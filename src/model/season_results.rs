use std::fmt;
use {
    serde::{Deserialize, Serialize},
    serde_repr::{Deserialize_repr, Serialize_repr},
};

#[derive(Deserialize_repr, Serialize_repr, Clone, Copy, Debug)]
#[repr(u8)]
pub enum EventType {
    Practice = 2,
    Qualify = 3,
    TimeTrial = 4,
    Race = 5,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SeasonResults {
    results_list: Vec<Session>,
    event_type: EventType,
    success: bool,
    season_id: u32,
    race_week_num: Option<u32>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Session {
    race_week_num: u32,
    event_type: EventType,
    event_type_name: String,
    start_time: String, // timestamp
    session_id: u32,
    subsession_id: u32,
    official_session: bool,
    event_strength_of_field: i32,
    event_best_lap_time: i32,
    num_cautions: i32,
    num_caution_laps: i32,
    num_drivers: u32,
    track: Track,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Track {
    track_id: u32,
    track_name: String,
    config_name: Option<String>,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}
