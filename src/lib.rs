#[cfg(feature = "audio")]
extern crate ogg_metadata;

use difficulty::Difficulty;
use info::info::difficulty_beatmap_set::{
    difficulty_beatmap::DifficultyRank, BeatmapCharacteristic,
};
use info::Info;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

#[cfg(feature = "audio")]
use ogg_metadata::OggFormat;
#[cfg(feature = "audio")]
use std::fs::File;

/// Contains types related to the difficulty files
pub mod difficulty;
/// Contains types related to the `info.dat` file
pub mod info;

/// Convenience type for the difficulty hash map
pub type DifficultyHashMap = HashMap<BeatmapCharacteristic, HashMap<DifficultyRank, Difficulty>>;

/// Represents a Beat Saber map
#[derive(Debug)]
pub struct Beatmap {
    /// Beatmap info
    pub info: Info,
    /// Beatmap difficulty sets
    pub difficulties: DifficultyHashMap,
    /// BeatSaver key
    #[cfg(feature = "beatsaver")]
    pub key: Option<String>,
    /// Audio file length, in seconds
    #[cfg(feature = "audio")]
    pub length: f64,
}

impl Beatmap {
    /// Returns a new `Beatmap` instance from an `info.dat` file
    pub fn from_file_dat(filename: &str) -> Result<Beatmap, Box<dyn Error>> {
        // Get Info from info.dat
        let info_contents = std::fs::read_to_string(filename)?;
        let info: Info = serde_json::from_str(&info_contents)?;

        // Get the directory containing the map
        let beatmap_dir = Path::new(filename).parent().unwrap_or(Path::new("."));

        let mut difficulties: DifficultyHashMap = HashMap::new();
        // For each characteristic, get the difficulty ranks
        for difficulty_beatmap_set in &info.difficulty_beatmap_sets {
            let mut sub_difficulties = HashMap::new();
            // For each difficulty rank, get the difficulty from its file
            for difficulty_beatmap in &difficulty_beatmap_set.difficulty_beatmaps {
                let difficulty_filename =
                    Path::new(beatmap_dir).join(&difficulty_beatmap.beatmap_filename);
                let difficulty_contents = std::fs::read_to_string(difficulty_filename)?;
                let difficulty: Difficulty = serde_json::from_str(&difficulty_contents)?;

                sub_difficulties.insert(difficulty_beatmap.difficulty_rank.clone(), difficulty);
            }

            difficulties.insert(
                difficulty_beatmap_set.beatmap_characteristic_name.clone(),
                sub_difficulties,
            );
        }

        // Calculate the audio file length
        #[cfg(feature = "audio")]
        let mut length = 0.0;
        #[cfg(feature = "audio")]
        {
            let audio_filename = Path::new(beatmap_dir).join(&info.song_filename);
            let mut audio_file = File::open(audio_filename)?;
            let formats = ogg_metadata::read_format(&mut audio_file)?;

            for format in formats {
                if let OggFormat::Vorbis(metadata) = format {
                    length = (metadata.length_in_samples.unwrap_or(1) as f64
                        / metadata.sample_rate as f64)
                        / metadata.channels as f64;
                    length *= 2.0;
                    break;
                }
            }
        }

        // Create the Beatmap and return it
        Ok(Beatmap {
            info,
            difficulties,
            #[cfg(feature = "beatsaver")]
            key: None,
            #[cfg(feature = "audio")]
            length,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Beatmap;
    use std::path::PathBuf;

    fn cargo_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    #[test]
    fn from_file_dat() {
        let mut filename = cargo_dir();
        filename.push("resources/test/info.dat");

        let result = Beatmap::from_file_dat(filename.to_str().unwrap()).unwrap();
        println!("{:#?}", result);
    }
}
