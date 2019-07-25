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
    use super::*;

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
}