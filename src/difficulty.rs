extern crate semver;
extern crate serde;
extern crate serde_repr;

use semver::Version;
use serde::{Deserialize, Serialize};

pub mod difficulty {
    use super::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct BPMChange {
        pub bpm: f64,
        pub time: f64,
        pub beats_per_bar: u32,
        pub metronome_offset: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Event {
        pub time: f64,
        pub event_type: u8,
        pub value: u8,
    }

    #[derive(Serialize_repr, Deserialize_repr, PartialEq, PartialOrd, Debug)]
    #[repr(u8)]
    pub enum LineIndex {
        FarLeft = 0,
        MidLeft = 1,
        MidRight = 2,
        FarRight = 3,
    }

    pub mod note {
        #[derive(Serialize_repr, Deserialize_repr, PartialEq, PartialOrd, Debug)]
        #[repr(u8)]
        pub enum LineLayer {
            Bottom = 0,
            Middle = 1,
            Top = 2,
        }

        #[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
        #[repr(u8)]
        pub enum NoteType {
            Red = 0,
            Blue = 1,
            Bomb = 3,
        }

        #[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
        #[repr(u8)]
        pub enum CutDirection {
            Up = 0,
            Down = 1,
            Left = 2,
            Right = 3,
            UpLeft = 4,
            UpRight = 5,
            DownLeft = 6,
            DownRight = 7,
            Dot = 8,
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Note {
        pub time: f64,
        pub line_index: LineIndex,
        pub line_layer: note::LineLayer,
        pub note_type: note::NoteType,
        pub cut_direction: note::CutDirection,
    }

    pub mod obstacle {
        #[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
        #[repr(u8)]
        pub enum ObstacleType {
            Wall = 0,
            Ceiling = 1,
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Obstacle {
        pub time: f64,
        pub line_index: LineIndex,
        pub obstacle_type: obstacle::ObstacleType,
        pub duration: f64,
        pub width: u8,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Bookmark {
        time: f64,
        name: String,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Difficulty {
    pub version: Version,
    pub bpm_changes: Vec<difficulty::BPMChange>,
    pub events: Vec<difficulty::Event>,
    pub notes: Vec<difficulty::Note>,
    pub obstacles: Vec<difficulty::Obstacle>,
    pub bookmarks: Vec<difficulty::Bookmark>,
}
