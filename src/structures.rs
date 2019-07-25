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
            pub role: String,
            pub name: String,
            pub icon_path: String,
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CustomData {
        pub contributors: Vec<custom_data::Contributor>,
        pub custom_environment: String,
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
                pub difficulty_label: String,
                pub editor_offset: f64,
                pub editor_old_offset: f64,
                pub color_left: Option<custom_data::Color>,
                pub color_right: Option<custom_data::Color>,
                pub warning: Vec<String>,
                pub information: Vec<String>,
                pub suggestions: Vec<String>,
                pub requirements: Vec<String>,
            }
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct DifficultyBeatmap {
            pub difficulty: difficulty_beatmap::Difficulty,
            pub difficulty_rank: difficulty_beatmap::DifficultyRank,
            pub beatmap_filename: String,
            pub note_jump_movement_speed: f64,
            pub note_jump_start_beat_offset: f64,
            pub custom_data: difficulty_beatmap::CustomData,
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DifficultyBeatmapSet {
        pub beatmap_characteristic_name: difficulty_beatmap_set::BeatmapCharacteritic,
        pub difficulty_beatmaps: Vec<difficulty_beatmap_set::DifficultyBeatmap>,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Beatmap {
    pub version: Version,
    pub song_name: String,
    pub song_sub_name: String,
    pub song_author_name: String,
    pub level_author_name: String,
    pub beats_per_minute: f64,
    pub song_time_offset: f64,
    pub shuffle: f64,
    pub shuffle_period: f64,
    pub preview_start_time: f64,
    pub preview_duration: f64,
    pub song_filename: String,
    pub cover_image_filename: String,
    pub environment_name: beatmap::Environment,
    pub custom_data: beatmap::CustomData,
    pub difficulty_beatmap_sets: Vec<beatmap::DifficultyBeatmapSet>,
}
