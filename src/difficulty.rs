extern crate semver;
extern crate serde;
extern crate serde_repr;

use semver::Version;
use serde::{Deserialize, Serialize};

/// Contains custom types used by Difficulty
pub mod difficulty {
    use super::{Deserialize, Serialize};
    use serde_repr::*;

    /// Represents a BPM change
    #[derive(Serialize, Deserialize, Debug)]
    pub struct BPMChange {
        /// New BPM
        #[serde(rename = "_BPM")]
        pub bpm: f64,
        /// Time of the BPM change, in beats
        #[serde(rename = "_time")]
        pub time: f64,
        #[serde(rename = "_beatsPerBar")]
        pub beats_per_bar: u32,
        #[serde(rename = "_metronomeOffset")]
        pub metronome_offset: u32,
    }

    /// Represents a lighting event and other various events
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Event {
        /// Time of the event, in beats
        #[serde(rename = "_time")]
        pub time: f64,
        /// Type of the event
        #[serde(rename = "_type")]
        pub event_type: u8,
        /// Value of the event
        #[serde(rename = "_value")]
        pub value: u32,
    }

    /// Represents an horizontal line index
    #[derive(Serialize_repr, Deserialize_repr, PartialEq, PartialOrd, Eq, Ord, Debug)]
    #[repr(u8)]
    pub enum LineIndex {
        FarLeft = 0,
        MidLeft = 1,
        MidRight = 2,
        FarRight = 3,
    }

    /// Contains custom types used by difficulty::Note
    pub mod note {
        use serde_repr::*;

        /// Represents a vertical line layer
        #[derive(Serialize_repr, Deserialize_repr, PartialEq, PartialOrd, Eq, Ord, Debug)]
        #[repr(u8)]
        pub enum LineLayer {
            Bottom = 0,
            Middle = 1,
            Top = 2,
        }

        /// Represents a note type
        #[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug)]
        #[repr(u8)]
        pub enum NoteType {
            Red = 0,
            Blue = 1,
            Bomb = 3,
        }

        /// Represents a note cut direction
        #[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug)]
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
        /// Time of the note, in beats
        #[serde(rename = "_time")]
        pub time: f64,
        /// Horizontal line index of the note
        #[serde(rename = "_lineIndex")]
        pub line_index: LineIndex,
        /// Vertical line layer of the note
        #[serde(rename = "_lineLayer")]
        pub line_layer: note::LineLayer,
        /// Type of the note
        #[serde(rename = "_type")]
        pub note_type: note::NoteType,
        /// Cut direction of the note
        #[serde(rename = "_cutDirection")]
        pub cut_direction: note::CutDirection,
    }

    /// Contains custom types used by difficulty::Obstacle
    pub mod obstacle {
        use serde_repr::*;

        #[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug)]
        #[repr(u8)]
        pub enum ObstacleType {
            Wall = 0,
            Ceiling = 1,
        }
    }

    /// Represents an obstacle
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Obstacle {
        /// Time of the obstacle in beats
        #[serde(rename = "_time")]
        pub time: f64,
        /// Horizontal line index of the obstacle
        #[serde(rename = "_lineIndex")]
        pub line_index: LineIndex,
        /// Type of the obstacle
        #[serde(rename = "_type")]
        pub obstacle_type: obstacle::ObstacleType,
        /// Length of the obstacle, in beats
        #[serde(rename = "_duration")]
        pub duration: f64,
        /// Width of the obstacle in lines, extending to the right
        #[serde(rename = "_width")]
        pub width: u8,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Bookmark {
        #[serde(rename = "_time")]
        time: f64,
        #[serde(rename = "_name")]
        name: String,
    }
}

/// Represents a difficulty file
#[derive(Serialize, Deserialize, Debug)]
pub struct Difficulty {
    /// Format version
    #[serde(rename = "_version")]
    pub version: Version,
    /// BPM changes
    #[serde(rename = "_BPMChanges")]
    pub bpm_changes: Vec<difficulty::BPMChange>,
    /// Lighting and other various events
    #[serde(rename = "_events")]
    pub events: Vec<difficulty::Event>,
    /// Notes
    #[serde(rename = "_notes")]
    pub notes: Vec<difficulty::Note>,
    /// Walls and ceilings
    #[serde(rename = "_obstacles")]
    pub obstacles: Vec<difficulty::Obstacle>,
    /// Bookmarks, used by editors
    #[serde(rename = "_bookmarks")]
    pub bookmarks: Vec<difficulty::Bookmark>,
}
