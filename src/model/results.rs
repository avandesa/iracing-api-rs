use {
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SessionResult {
    pub subsession_id: u32,

    pub season_id: u32,
    pub season_name: String,
    pub season_short_name: String,
    pub season_year: u32,
    pub season_quarter: u32,

    pub series_id: u32,
    pub series_name: String,
    pub series_short_name: String,
    pub series_logo: Option<String>,
    pub race_week_num: u32,

    pub session_id: u32,

    pub license_category: String,
    pub license_category_id: u32,

    pub private_session_id: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,

    pub num_laps_for_qual_average: u32,
    pub num_laps_for_solo_average: u32,

    pub corners_per_lap: u32,
    pub caution_type: u32, // TODO: Identify meaning and convert to enum
    pub event_type: u32,
    pub event_type_name: String,

    pub driver_changes: bool,
    pub min_team_drivers: u32,
    pub max_team_drivers: u32,
    pub driver_change_rule: u32,
    pub driver_change_param1: i32,
    pub driver_change_param2: i32,

    pub max_weeks: i32,
    pub points_type: String,
    pub event_strength_of_field: i32,
    pub event_average_lap: i32, // TODO: convert to human-readable time
    pub event_laps_complete: i32,
    pub num_cautions: i32,
    pub num_caution_laps: i32,
    pub num_lead_changes: i32,
    pub official_session: bool,
    pub heat_info_id: i32,
    pub special_event_type: i32,
    pub damage_model: u32, // enum

    pub can_protest: bool,
    pub cooldown_minutes: u32,
    pub limit_minutes: u32,

    pub track: Track,
    pub weather: Weather,
    pub track_state: TrackState,
    pub session_results: Vec<SubsessionResult>,
    pub car_classes: Vec<CarClass>,
    pub allowed_licenses: Option<Vec<SessionAlowedLicense>>,
    pub race_summary: Option<RaceSummary>,

    pub results_restricted: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SessionAlowedLicense {
    pub group_name: String,
    pub license_group: i32,
    pub max_license_level: i32,
    pub min_license_level: i32,
    pub parent_id: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CarClass {
    pub car_class_id: i32,
    pub cars_in_class: Vec<CarInClass>,
    pub name: String,
    pub short_name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CarInClass {
    pub car_id: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RaceSummary {
    pub average_lap: i32,
    pub field_strength: i32,
    pub has_opt_path: bool,
    pub laps_complete: i32,
    pub num_caution_laps: i32,
    pub num_cautions: i32,
    pub num_lead_changes: i32,
    pub num_opt_laps: i32,
    pub special_event_type: i32,
    pub special_event_type_text: String,
    pub subsession_id: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SubsessionResult {
    pub results: Vec<SubsessionDriverResult>,
    pub simsession_name: String,
    pub simsession_number: i32,
    pub simsession_subtype: i32, // TODO: convert to enum
    pub simsession_type: i32,    // TODO: convert to enum
    pub simsession_type_name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SubsessionDriverResult {
    pub aggregate_champ_points: i32,
    pub ai: bool,
    pub average_lap: i32,
    pub best_lap_num: i32,
    pub best_lap_time: i32,
    pub best_nlaps_num: i32,
    pub best_nlaps_time: i32,
    pub best_qual_lap_at: DateTime<Utc>,
    pub best_qual_lap_num: i32,
    pub best_qual_lap_time: i32,
    pub car_class_id: i32,
    pub car_id: i32,
    pub champ_points: i32,
    pub class_interval: i32,
    pub club_id: i32,
    pub club_name: String,
    pub club_points: i32,
    pub club_shortname: String,
    pub cust_id: i32,
    pub display_name: String,
    pub division: i32,
    pub division_name: Option<String>,
    pub drop_race: bool,
    pub finish_position: i32,
    pub finish_position_in_class: i32,
    pub friend: bool,
    pub helmet: Helmet,
    pub incidents: i32,
    pub interval: i32,
    pub laps_complete: i32,
    pub laps_lead: i32,
    pub league_agg_points: i32,
    pub license_change_oval: i32,
    pub license_change_road: i32,
    pub livery: Livery,
    pub max_pct_fuel_fill: i32,
    pub multiplier: i32,
    pub new_cpi: i32,
    pub new_license_level: i32,
    pub new_sub_level: i32,
    pub new_ttrating: i32,
    pub newi_rating: i32,
    pub old_cpi: i32,
    pub old_license_level: i32,
    pub old_sub_level: i32,
    pub old_ttrating: i32,
    pub oldi_rating: i32,
    pub opt_laps_complete: i32,
    pub position: i32,
    pub qual_lap_time: i32,
    pub reason_out: String,
    pub reason_out_id: i32, // enum
    pub starting_position: i32,
    pub suit: Suit,
    pub watched: bool,
    pub weight_penalty_kg: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Helmet {
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub face_type: i32,
    pub helmet_type: i32,
    pub pattern: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Livery {
    pub car_id: i32,
    pub car_number: String,
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub number_color1: String,
    pub number_color2: String,
    pub number_color3: String,
    pub number_font: i32,
    pub number_slant: i32,
    pub pattern: i32,
    pub rim_type: i32,
    pub sponsor1: i32,
    pub sponsor2: i32,
    pub wheel_color: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Suit {
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub pattern: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Track {
    pub category: String, // enum
    pub category_id: i32, // enum??
    pub config_name: String,
    pub track_id: i32,
    pub track_name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TrackState {
    pub leave_marbles: bool,
    pub practice_grip_compound: i32,
    pub practice_rubber: i32,
    pub qualify_grip_compound: i32,
    pub qualify_rubber: i32,
    pub race_grip_compound: i32,
    pub race_rubber: i32,
    pub warmup_grip_compound: i32,
    pub warmup_rubber: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Weather {
    fog: i32,
    rel_humidity: i32,
    simulated_start_utc_offset: i32, // Offset
    simulated_start_utc_time: DateTime<Utc>,
    skies: i32,      // enum?
    temp_units: i32, // enum?
    temp_value: i32,
    time_of_day: i32,
    r#type: i32, // enum?
    weather_var_initial: i32,
    weather_var_ongoing: i32,
    wind_dir: i32,   // enum?
    wind_units: i32, // enum?
    wind_value: i32,
}
