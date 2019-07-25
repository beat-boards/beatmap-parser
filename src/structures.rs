// https://github.com/Kylemc1413/SongCore#readme

extern crate semver;
extern crate serde;

use semver::Version;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Environment {
    DefaultEnvironment,
    BigMirrorEnvironment,
    TriangleEnvironment,
    NiceEnvironment,
    KDAEnvironment,
    MonstercatEnvironment,
}

pub mod custom_data {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct Contributor {
        pub role: String,
        pub name: String,
        pub icon_path: String,
    }
}

#[derive(Serialize, Deserialize)]
pub struct CustomData {
    pub contributors: Vec<custom_data::Contributor>,
    pub custom_environment: String,
    pub custom_environment_hash: String,
}

pub mod difficulty_beatmap_set {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub enum BeatmapCharacteritic {
        Standard,
        NoArrows,
        OneSaber,
        Lawless,
        Lightshow,
    }

    #[derive(Serialize, Deserialize)]
    pub enum Difficulty {
        Easy,
        Normal,
        Hard,
        Expert,
        ExpertPlus,
    }

    #[derive(Serialize, Deserialize)]
    pub struct CustomData {
        pub difficulty_label: String,
        
    }

    #[derive(Serialize, Deserialize)]
    pub struct DifficultyBeatmap {
        pub difficulty: Difficulty,
        pub difficulty_rank: u32,
        pub beatmap_filename: String,
        pub note_jump_movement_speed: f64,
        pub note_jump_start_beat_offset: f64,
        pub custom_data: CustomData
    }
}

#[derive(Serialize, Deserialize)]
pub struct DifficultyBeatmapSet {
    pub beatmap_characteristic_name: difficulty_beatmap_set::BeatmapCharacteritic,
    pub difficulty_beatmaps: Vec<difficulty_beatmap_set::DifficultyBeatmap>
}

#[derive(Serialize, Deserialize)]
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
    pub environment_name: Environment,
    pub custom_data: CustomData,
    pub difficulty_beatmap_sets: Vec<DifficultyBeatmapSet>,
}