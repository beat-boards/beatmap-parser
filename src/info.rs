extern crate semver;
extern crate serde;
extern crate serde_repr;

use semver::Version;
use serde::{Deserialize, Serialize};

/// Contains custom types used by Info
pub mod info {
    use super::{Deserialize, Serialize};

    /// Represents a game environment
    #[derive(Serialize, Deserialize, Debug)]
    pub enum Environment {
        DefaultEnvironment,
        BigMirrorEnvironment,
        TriangleEnvironment,
        NiceEnvironment,
        KDAEnvironment,
        MonstercatEnvironment,
    }

    /// Contains custom types used by info::CustomData
    pub mod custom_data {
        use super::{Deserialize, Serialize};

        /// Represents a contributor
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Contributor {
            /// Role of the contributor
            #[serde(rename = "_role")]
            pub role: String,
            /// Name of the contributor
            #[serde(rename = "_name")]
            pub name: String,
            /// Contributor icon file name
            #[serde(rename = "_iconPath")]
            pub icon_path: String,
        }
    }

    /// Represents custom data applying to all characteristics and difficulties
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CustomData {
        /// People who contributed to this map
        #[serde(rename = "_contributors")]
        pub contributors: Vec<custom_data::Contributor>,
        /// Custom platform override, will use "environmentName" if CustomPlatforms isn't installed or is disabled
        #[serde(rename = "_customEnvironment")]
        pub custom_environment: String,
        /// The hash found on ModelSaber, used to download missing platforms
        #[serde(rename = "_customEnvironmentHash")]
        pub custom_environment_hash: String,
    }

    /// Contains custom types used by info::DifficultyBeatmapSet
    pub mod difficulty_beatmap_set {
        use super::{Deserialize, Serialize};

        /// Represent a characteristic
        #[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Hash, Debug)]
        pub enum BeatmapCharacteristic {
            Standard,
            NoArrows,
            OneSaber,
            Lawless,
            Lightshow,
        }

        /// Contains custom types used by info::difficulty_beatmap_set::DifficultyBeatmap
        pub mod difficulty_beatmap {
            use super::{Deserialize, Serialize};
            use serde_repr::*;

            /// Represents a difficulty name
            #[derive(Serialize, Deserialize, Debug)]
            pub enum Difficulty {
                Easy,
                Normal,
                Hard,
                Expert,
                ExpertPlus,
            }

            /// Represents a difficulty rank
            #[derive(
                Serialize_repr, Deserialize_repr, PartialEq, PartialOrd, Eq, Ord, Clone, Hash, Debug,
            )]
            #[repr(u8)]
            pub enum DifficultyRank {
                Easy = 1,
                Normal = 3,
                Hard = 5,
                Expert = 7,
                ExpertPlus = 9,
            }

            /// Contains custom types used by info::difficulty_beatmap_set::difficulty_beatmap::CustomData
            pub mod custom_data {
                use super::{Deserialize, Serialize};

                /// Represents aa RGB color
                #[derive(Serialize, Deserialize, Debug)]
                pub struct Color {
                    /// Amount of red [0-1]
                    pub r: f64,
                    /// Amount of green [0-1]
                    pub g: f64,
                    /// Amount of blue [0-1]
                    pub b: f64,
                }

                impl Color {
                    /// Returns an unprefixed hex string representing the color
                    pub fn to_hex(&self) -> String {
                        let r = (self.r * 255.0) as u8;
                        let g = (self.g * 255.0) as u8;
                        let b = (self.b * 255.0) as u8;

                        format!("{:x}{:x}{:x}", r, g, b)
                    }
                }
            }

            /// Represents custom data applying to a specific characteristic and difficulty
            #[derive(Serialize, Deserialize, Debug)]
            pub struct CustomData {
                /// The name to display for the difficulty
                #[serde(rename = "_difficultyLabel")]
                pub difficulty_label: String,
                #[serde(rename = "_editorOffset")]
                pub editor_offset: i32,
                #[serde(rename = "_editorOldOffset")]
                pub editor_old_offset: i32,
                /// Left side color override if CustomColors is installed and enabled
                #[serde(rename = "_colorLeft")]
                pub color_left: Option<custom_data::Color>,
                /// Right side color override if CustomColors is installed and enabled
                #[serde(rename = "_colorRight")]
                pub color_right: Option<custom_data::Color>,
                /// Any warnings the player should be aware of before playing the song
                #[serde(rename = "_warnings")]
                pub warning: Vec<String>,
                /// Any general information the player should be aware of before playing the song
                #[serde(rename = "_information")]
                pub information: Vec<String>,
                /// Any mods the player is suggested to use for playing the song
                #[serde(rename = "_suggestions")]
                pub suggestions: Vec<String>,
                /// Any mods the player is required to use for playing the song
                #[serde(rename = "_requirements")]
                pub requirements: Vec<String>,
            }
        }

        /// Represents a single difficulty beatmap
        #[derive(Serialize, Deserialize, Debug)]
        pub struct DifficultyBeatmap {
            /// Name of the difficulty
            #[serde(rename = "_difficulty")]
            pub difficulty: difficulty_beatmap::Difficulty,
            /// Rank of the difficulty, should match the difficulty name
            #[serde(rename = "_difficultyRank")]
            pub difficulty_rank: difficulty_beatmap::DifficultyRank,
            /// Beatmap file name
            #[serde(rename = "_beatmapFilename")]
            pub beatmap_filename: String,
            #[serde(rename = "_noteJumpMovementSpeed")]
            pub note_jump_movement_speed: f64,
            #[serde(rename = "_noteJumpStartBeatOffset")]
            pub note_jump_start_beat_offset: f64,
            /// Custom data used by mods
            #[serde(rename = "_customData")]
            pub custom_data: difficulty_beatmap::CustomData,
        }
    }

    /// Represents a set of difficulty beatmaps for a specific characteristic
    #[derive(Serialize, Deserialize, Debug)]
    pub struct DifficultyBeatmapSet {
        /// Characteristic of the beatmap set
        #[serde(rename = "_beatmapCharacteristicName")]
        pub beatmap_characteristic_name: difficulty_beatmap_set::BeatmapCharacteristic,
        /// Set of difficulty beatmaps for the current characteristic
        #[serde(rename = "_difficultyBeatmaps")]
        pub difficulty_beatmaps: Vec<difficulty_beatmap_set::DifficultyBeatmap>,
    }
}

/// Represents an `info.dat` file
#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    /// Format version
    #[serde(rename = "_version")]
    pub version: Version,
    /// Name of the song
    #[serde(rename = "_songName")]
    pub song_name: String,
    /// Text rendered in smaller letters next to song name
    #[serde(rename = "_songSubName")]
    pub song_sub_name: String,
    /// Author of the song itself
    #[serde(rename = "_songAuthorName")]
    pub song_author_name: String,
    /// Mapper of the song
    #[serde(rename = "_levelAuthorName")]
    pub level_author_name: String,
    /// BPM of the song
    #[serde(rename = "_beatsPerMinute")]
    pub beats_per_minute: f64,
    /// Offset playing the audio, in seconds
    #[serde(rename = "_songTimeOffset")]
    pub song_time_offset: f64,
    #[serde(rename = "_shuffle")]
    pub shuffle: f64,
    #[serde(rename = "_shufflePeriod")]
    pub shuffle_period: f64,
    /// Audio preview starting point, in seconds
    #[serde(rename = "_previewStartTime")]
    pub preview_start_time: f64,
    /// Audio preview length, in seconds
    #[serde(rename = "_previewDuration")]
    pub preview_duration: f64,
    /// Audio file name
    #[serde(rename = "_songFilename")]
    pub song_filename: String,
    /// Cover file name
    #[serde(rename = "_coverImageFilename")]
    pub cover_image_filename: String,
    /// Game environment to use
    #[serde(rename = "_environmentName")]
    pub environment_name: info::Environment,
    /// Custom data used by mods
    #[serde(rename = "_customData")]
    pub custom_data: info::CustomData,
    /// A set of maps for a given characteristic
    #[serde(rename = "_difficultyBeatmapSets")]
    pub difficulty_beatmap_sets: Vec<info::DifficultyBeatmapSet>,
}
