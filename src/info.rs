// https://github.com/Kylemc1413/SongCore#readme

extern crate semver;
extern crate serde;
extern crate serde_repr;

use semver::Version;
use serde::{Deserialize, Serialize};

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
                #[serde(rename = "_warning")]
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
