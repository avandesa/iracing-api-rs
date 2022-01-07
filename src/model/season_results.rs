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
    pub results_list: Vec<Session>,
    pub event_type: EventType,
    pub success: bool,
    pub season_id: u32,
    pub race_week_num: Option<u32>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Session {
    pub race_week_num: u32,
    pub event_type: EventType,
    pub event_type_name: String,
    pub start_time: String, // timestamp
    pub session_id: u32,
    pub subsession_id: u32,
    pub official_session: bool,
    pub event_strength_of_field: i32,
    pub event_best_lap_time: i32,
    pub num_cautions: i32,
    pub num_caution_laps: i32,
    pub num_drivers: u32,
    pub track: Track,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Track {
    pub track_id: u32,
    pub track_name: String,
    pub config_name: Option<String>,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}
