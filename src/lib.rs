use std::collections::HashMap;

/// Contains types related to the difficulty files
pub mod difficulty;
/// Contains types related to the `info.dat` file
pub mod info;

use difficulty::Difficulty;
use info::Info;
use info::info::difficulty_beatmap_set::{
    BeatmapCharacteristic,
    difficulty_beatmap::DifficultyRank,
};
use std::error::Error;
use std::path::Path;

/// Represents a Beat Saber map
#[derive(Debug)]
pub struct Beatmap {
    /// Beatmap info
    pub info: Info,
    /// Beatmap difficulty sets
    pub difficulties: HashMap<BeatmapCharacteristic, HashMap<DifficultyRank, Difficulty>>,
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

        let mut difficulties = HashMap::new();
        // For each characteristic, get the difficulty ranks
        for difficulty_beatmap_set in &info.difficulty_beatmap_sets {
            let mut sub_difficulties = HashMap::new();
            // For each difficulty rank, get the difficulty from its file
            for difficulty_beatmap in &difficulty_beatmap_set.difficulty_beatmaps {
                let difficulty_filename = Path::new(beatmap_dir).join(&difficulty_beatmap.beatmap_filename);
                let difficulty_contents = std::fs::read_to_string(difficulty_filename)?;
                let difficulty: Difficulty = serde_json::from_str(&difficulty_contents)?;

                sub_difficulties.insert(difficulty_beatmap.difficulty_rank.clone(), difficulty);
            }

            difficulties.insert(difficulty_beatmap_set.beatmap_characteristic_name.clone(), sub_difficulties);
        }

        // Create the Beatmap and return it
        Ok(Beatmap {
            info,
            difficulties,
            #[cfg(feature = "beatsaver")]
            key: None,
            #[cfg(feature = "audio")]
            length: 100.0,
        })
    }
}