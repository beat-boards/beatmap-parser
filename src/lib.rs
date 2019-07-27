#[cfg(feature = "beatsaver")]
extern crate reqwest;
#[cfg(feature = "beatsaver")]
extern crate tempfile;
#[cfg(feature = "beatsaver")]
extern crate zip;

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

#[cfg(feature = "beatsaver")]
use std::io;
#[cfg(feature = "beatsaver")]
use std::io::Read;

#[cfg(feature = "audio")]
use ogg_metadata::OggFormat;
#[cfg(feature = "audio")]
use std::fs::File;

#[cfg(feature = "beatsaver")]
#[cfg(feature = "audio")]
use std::io::{Seek, SeekFrom};

/// Contains types related to the difficulty files
pub mod difficulty;
/// Contains types related to the `info.dat` file
pub mod info;

type DifficultyHashMap = HashMap<BeatmapCharacteristic, HashMap<DifficultyRank, Difficulty>>;

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
    #[cfg(feature = "audio")]
    fn calculate_ogg_length(formats: Vec<OggFormat>, mut length: f64) -> f64 {
        for format in formats {
            if let OggFormat::Vorbis(metadata) = format {
                length = (metadata.length_in_samples.unwrap_or(1) as f64
                    / metadata.sample_rate as f64)
                    / metadata.channels as f64;
                length *= 2.0;
                break;
            }
        }
        length
    }

    /// Returns a new Beatmap instance from an `info.dat` file
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

            length = Beatmap::calculate_ogg_length(formats, length);
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

    /// Returns a new Beatmap instance from a BeatSaver key
    #[cfg(feature = "beatsaver")]
    pub fn from_beatsaver_key(key: &str) -> Result<Beatmap, Box<dyn Error>> {
        // Download the file and store it temporarly
        let mut response =
            reqwest::get(&format!("https://beatsaver.com/api/download/key/{}", key))?;
        let mut temp_file = tempfile::tempfile()?;
        io::copy(&mut response, &mut temp_file)?;

        // Create the zip archive object
        let mut archive = zip::ZipArchive::new(temp_file)?;

        let mut info: Info;
        {
            // Get Info from info.dat
            let mut info_file = archive.by_name("info.dat")?;
            let mut info_contents = String::new();
            info_file.read_to_string(&mut info_contents)?;
            info = serde_json::from_str(&info_contents)?;
        }

        let mut difficulties: DifficultyHashMap = HashMap::new();
        // For each characteristic, get the difficulty ranks
        for difficulty_beatmap_set in &info.difficulty_beatmap_sets {
            let mut sub_difficulties = HashMap::new();
            // For each difficulty rank, get the difficulty from its file
            for difficulty_beatmap in &difficulty_beatmap_set.difficulty_beatmaps {
                let mut difficulty_file = archive.by_name(&difficulty_beatmap.beatmap_filename)?;
                let mut difficulty_contents = String::new();
                difficulty_file.read_to_string(&mut difficulty_contents)?;
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
            let mut audio_file = archive.by_name(&info.song_filename)?;
            let mut temp_audio_file = tempfile::tempfile()?;
            io::copy(&mut audio_file, &mut temp_audio_file)?;
            temp_audio_file.seek(SeekFrom::Start(0))?;
            let formats = ogg_metadata::read_format(&mut temp_audio_file)?;

            length = Beatmap::calculate_ogg_length(formats, length);
        }

        // Create the Beatmap and return it
        Ok(Beatmap {
            info,
            difficulties,
            #[cfg(feature = "beatsaver")]
            key: Some(String::from(key)),
            #[cfg(feature = "audio")]
            length,
        })
    }

    /// Returns a new Beatmap instance from a BeatSaver url
    #[cfg(feature = "beatsaver")]
    pub fn from_beatsaver_url(url: &str) -> Result<Beatmap, Box<dyn Error>> {
        let url_string = String::from(url);
        if url_string.starts_with("https://beatsaver.com/api/download/key/")
            || url_string.starts_with("https://beatsaver.com/beatmap/")
            || url_string.starts_with("beatsaver://")
        {
            let mut key = String::from(url_string.split("/").last().unwrap_or("invalid"));
            if key == "invalid" {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Can't extract key from url",
                )));
            }
            if key.ends_with("/") {
                key.pop();
            }

            Beatmap::from_beatsaver_key(&key)
        } else {
            Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid url",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Beatmap;
    use std::path::PathBuf;

    #[test]
    fn from_file_dat() {
        let mut filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        filename.push("resources/test/info.dat");

        let result = Beatmap::from_file_dat(filename.to_str().unwrap()).unwrap();
        println!("{:#?}", result);
    }

    #[cfg(feature = "beatsaver")]
    #[test]
    fn from_beatsaver_key() {
        let result = Beatmap::from_beatsaver_key("3cf5").unwrap();
        println!("{:#?}", result);
    }

    #[cfg(feature = "beatsaver")]
    #[test]
    fn from_beatsaver_url() {
        let result = Beatmap::from_beatsaver_url("https://beatsaver.com/beatmap/1fef").unwrap();
        println!("{:#?}", result);
    }
}
