extern crate semver;
extern crate serde;
extern crate serde_repr;

#[cfg(feature = "beatsaver")]
extern crate reqwest;
#[cfg(feature = "beatsaver")]
extern crate tempfile;
#[cfg(feature = "beatsaver")]
extern crate zip;

use semver::Version;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[cfg(feature = "beatsaver")]
use std::io;
#[cfg(feature = "beatsaver")]
use std::io::Read;

pub mod beatmap {
    use super::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Environment {
        DefaultEnvironment,
        BigMirrorEnvironment,
        TriangleEnvironment,
        NiceEnvironment,
        KDAEnvironment,
        MonstercatEnvironment,
    }

    pub mod custom_data {
        use super::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Contributor {
            #[serde(rename = "_role")]
            pub role: String,
            #[serde(rename = "_name")]
            pub name: String,
            #[serde(rename = "_iconPath")]
            pub icon_path: String,
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CustomData {
        #[serde(rename = "_contributors")]
        pub contributors: Vec<custom_data::Contributor>,
        #[serde(rename = "_customEnvironment")]
        pub custom_environment: String,
        #[serde(rename = "_customEnvironmentHash")]
        pub custom_environment_hash: String,
    }

    pub mod difficulty_beatmap_set {
        use super::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        pub enum BeatmapCharacteritic {
            Standard,
            NoArrows,
            OneSaber,
            Lawless,
            Lightshow,
        }

        pub mod difficulty_beatmap {
            use super::{Deserialize, Serialize};
            use serde_repr::*;

            #[derive(Serialize, Deserialize, Debug)]
            pub enum Difficulty {
                Easy,
                Normal,
                Hard,
                Expert,
                ExpertPlus,
            }

            #[derive(Serialize_repr, Deserialize_repr, PartialEq, PartialOrd, Debug)]
            #[repr(u8)]
            pub enum DifficultyRank {
                Easy = 1,
                Normal = 3,
                Hard = 5,
                Expert = 7,
                ExpertPlus = 9,
            }

            pub mod custom_data {
                use super::{Deserialize, Serialize};

                #[derive(Serialize, Deserialize, Debug)]
                pub struct Color {
                    pub r: f64,
                    pub g: f64,
                    pub b: f64,
                }
            }

            #[derive(Serialize, Deserialize, Debug)]
            pub struct CustomData {
                #[serde(rename = "_difficultyLabel")]
                pub difficulty_label: String,
                #[serde(rename = "_editorOffset")]
                pub editor_offset: f64,
                #[serde(rename = "_editorOldOffset")]
                pub editor_old_offset: f64,
                #[serde(rename = "_colorLeft")]
                pub color_left: Option<custom_data::Color>,
                #[serde(rename = "_colorRight")]
                pub color_right: Option<custom_data::Color>,
                #[serde(rename = "_warnings")]
                pub warning: Vec<String>,
                #[serde(rename = "_information")]
                pub information: Vec<String>,
                #[serde(rename = "_suggestions")]
                pub suggestions: Vec<String>,
                #[serde(rename = "_requirements")]
                pub requirements: Vec<String>,
            }
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct DifficultyBeatmap {
            #[serde(rename = "_difficulty")]
            pub difficulty: difficulty_beatmap::Difficulty,
            #[serde(rename = "_difficultyRank")]
            pub difficulty_rank: difficulty_beatmap::DifficultyRank,
            #[serde(rename = "_beatmapFilename")]
            pub beatmap_filename: String,
            #[serde(rename = "_noteJumpMovementSpeed")]
            pub note_jump_movement_speed: f64,
            #[serde(rename = "_noteJumpStartBeatOffset")]
            pub note_jump_start_beat_offset: f64,
            #[serde(rename = "_customData")]
            pub custom_data: difficulty_beatmap::CustomData,
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DifficultyBeatmapSet {
        #[serde(rename = "_beatmapCharacteristicName")]
        pub beatmap_characteristic_name: difficulty_beatmap_set::BeatmapCharacteritic,
        #[serde(rename = "_difficultyBeatmaps")]
        pub difficulty_beatmaps: Vec<difficulty_beatmap_set::DifficultyBeatmap>,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Beatmap {
    #[serde(rename = "_version")]
    pub version: Version,
    #[serde(rename = "_songName")]
    pub song_name: String,
    #[serde(rename = "_songSubName")]
    pub song_sub_name: String,
    #[serde(rename = "_songAuthorName")]
    pub song_author_name: String,
    #[serde(rename = "_levelAuthorName")]
    pub level_author_name: String,
    #[serde(rename = "_beatsPerMinute")]
    pub beats_per_minute: f64,
    #[serde(rename = "_songTimeOffset")]
    pub song_time_offset: f64,
    #[serde(rename = "_shuffle")]
    pub shuffle: f64,
    #[serde(rename = "_shufflePeriod")]
    pub shuffle_period: f64,
    #[serde(rename = "_previewStartTime")]
    pub preview_start_time: f64,
    #[serde(rename = "_previewDuration")]
    pub preview_duration: f64,
    #[serde(rename = "_songFilename")]
    pub song_filename: String,
    #[serde(rename = "_coverImageFilename")]
    pub cover_image_filename: String,
    #[serde(rename = "_environmentName")]
    pub environment_name: beatmap::Environment,
    #[serde(rename = "_customData")]
    pub custom_data: beatmap::CustomData,
    #[serde(rename = "_difficultyBeatmapSets")]
    pub difficulty_beatmap_sets: Vec<beatmap::DifficultyBeatmapSet>,
}

impl Beatmap {
    /// Returns a new `Beatmap` instance from an `info.dat` file
    pub fn from_file_dat(filename: &str) -> Result<Beatmap, Box<dyn Error>> {
        let contents = std::fs::read_to_string(filename)?;
        Ok(serde_json::from_str(&contents)?)
    }

    /// Returns a new `Beatmap` instance from a BeatSaver key
    #[cfg(feature = "beatsaver")]
    pub fn from_beatsaver_key(key: &str) -> Result<Beatmap, Box<dyn Error>> {
        let mut response =
            reqwest::get(&format!("https://beatsaver.com/api/download/key/{}", key))?;
        let mut temp_file = tempfile::tempfile()?;
        io::copy(&mut response, &mut temp_file)?;

        let mut archive = zip::ZipArchive::new(temp_file)?;
        let mut info_file = archive.by_name("info.dat")?;
        let mut contents = String::new();
        info_file.read_to_string(&mut contents)?;
        Ok(serde_json::from_str(&contents)?)
    }

    /// Returns a new `Beatmap` instance from a BeatSaver key
    #[cfg(feature = "beatsaver")]
    pub fn from_beatsaver_url(url: &str) -> Result<Beatmap, Box<dyn Error>> {
        let url_string = String::from(url);
        if url_string.starts_with("https://beatsaver.com/api/download/key/")
            || url_string.starts_with("https://beatsaver.com/beatmap/")
            || url_string.starts_with("beatsaver://")
        {
            Beatmap::from_beatsaver_key(url_string.split("/").last().unwrap_or("invalid"))
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

    #[cfg(feature = "beatsaver")]
    #[test]
    fn from_beatsaver_key() {
        let result = Beatmap::from_beatsaver_key("570").unwrap();
        println!("{:#?}", result);
    }

    #[cfg(feature = "beatsaver")]
    #[test]
    fn from_beatsaver_url() {
        let result = Beatmap::from_beatsaver_url("https://beatsaver.com/beatmap/570").unwrap();
        println!("{:#?}", result);
    }
}
