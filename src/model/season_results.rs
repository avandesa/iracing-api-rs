use std::fmt;
use {
    serde::{Deserialize, Serialize},
    serde_repr::{Deserialize_repr, Serialize_repr},
};

pub struct SeasonResultsQuery {
    season_id: u32,
    event_type: Option<EventType>,
    race_week_num: Option<u32>,
}

impl SeasonResultsQuery {
    pub fn new(season_id: u32) -> Self {
        Self {
            season_id,
            event_type: None,
            race_week_num: None,
        }
    }

    pub fn event_type(mut self, event_type: EventType) -> Self {
        self.event_type = Some(event_type);
        self
    }

    pub fn race_week_num(mut self, race_week_num: u32) -> Self {
        self.race_week_num = Some(race_week_num);
        self
    }

    pub fn as_query_params(&self) -> Vec<(&'static str, String)> {
        let mut query = vec![("season_id", self.season_id.to_string())];
        if let Some(event_type) = self.event_type {
            query.push(("event_type", event_type.to_string()));
        }
        if let Some(race_week_num) = self.race_week_num {
            query.push(("race_week_num", race_week_num.to_string()));
        }
        query
    }
}

#[derive(Deserialize_repr, Serialize_repr, Clone, Copy, Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod test {
    use crate::model::season_results::EventType;

    use super::SeasonResultsQuery;

    #[test]
    fn season_results_query_no_options() {
        let params = SeasonResultsQuery::new(1).as_query_params();
        assert_eq!(params, &[("season_id", "1".into())]);
    }

    #[test]
    fn season_results_query_event_type() {
        let params = SeasonResultsQuery::new(1)
            .event_type(EventType::Practice)
            .as_query_params();
        assert_eq!(
            params,
            &[("season_id", "1".into()), ("event_type", "2".into()),]
        )
    }

    #[test]
    fn season_results_query_race_week_num() {
        let params = SeasonResultsQuery::new(1)
            .race_week_num(5)
            .as_query_params();
        assert_eq!(
            params,
            &[("season_id", "1".into()), ("race_week_num", "5".into()),]
        )
    }

    #[test]
    fn season_results_query_all_options() {
        let params = SeasonResultsQuery::new(1)
            .event_type(EventType::Practice)
            .race_week_num(5)
            .as_query_params();
        assert_eq!(
            params,
            &[
                ("season_id", "1".into()),
                ("event_type", "2".into()),
                ("race_week_num", "5".into()),
            ]
        )
    }
}
