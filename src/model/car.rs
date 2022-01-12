use std::path::PathBuf;
use {
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Car {
    pub ai_enabled: bool,
    pub allow_number_colors: bool,
    pub allow_number_font: bool,
    pub allow_sponsor1: bool,
    pub allow_sponsor2: bool,
    pub allow_wheel_color: bool,
    pub award_exempt: bool,
    pub car_dirpath: String,
    pub car_id: u32,
    pub car_name: String,
    pub car_name_abbreviated: String,
    pub car_types: Vec<CarType>,
    pub car_weight: u32,
    pub categories: Vec<String>,
    pub created: DateTime<Utc>,
    pub free_with_subscription: bool,
    pub has_headlights: bool,
    pub has_multiple_dry_tire_types: bool,
    pub hp: u32,
    pub max_power_adjust_pct: u32,
    pub max_weight_penalty_kg: u32,
    pub min_power_adjust_pct: i32,
    pub package_id: u32,
    pub patterns: u32,
    pub price: f32,
    pub retired: bool,
    pub search_filters: String,
    pub sku: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CarType {
    pub car_type: String,
}

// TODO: car assets, if/when they fix the response serialization
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CarAssets {
    pub car_id: u32,
    pub detail_copy: String,
    pub detail_screen_shot_images: String,
    pub detail_techspecs_copy: Option<String>,
    pub folder: PathBuf,
    pub gallery_images: Option<String>,
    pub gallery_prefix: Option<String>,
    pub group_image: Option<String>,
    pub group_name: Option<String>,
    pub large_image: PathBuf,
    pub logo: Option<PathBuf>,
    pub small_image: Option<PathBuf>,
    pub sponsor_logo: Option<String>,
    pub template_path: Option<PathBuf>,
}
