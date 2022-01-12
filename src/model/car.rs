use std::path::PathBuf;
use {
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Car {
    ai_enabled: bool,
    allow_number_colors: bool,
    allow_number_font: bool,
    allow_sponsor1: bool,
    allow_sponsor2: bool,
    allow_wheel_color: bool,
    award_exempt: bool,
    car_dirpath: String,
    car_id: u32,
    car_name: String,
    car_name_abbreviated: String,
    car_types: Vec<CarType>,
    car_weight: u32,
    categories: Vec<String>,
    created: DateTime<Utc>,
    free_with_subscription: bool,
    has_headlights: bool,
    has_multiple_dry_tire_types: bool,
    hp: u32,
    max_power_adjust_pct: u32,
    max_weight_penalty_kg: u32,
    min_power_adjust_pct: i32,
    package_id: u32,
    patterns: u32,
    price: f32,
    retired: bool,
    search_filters: String,
    sku: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CarType {
    car_type: String,
}

// TODO: car assets, if/when they fix the response serialization
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CarAssets {
    car_id: u32,
    detail_copy: String,
    detail_screen_shot_images: String,
    detail_techspecs_copy: Option<String>,
    folder: PathBuf,
    gallery_images: Option<String>,
    gallery_prefix: Option<String>,
    group_image: Option<String>,
    group_name: Option<String>,
    large_image: PathBuf,
    logo: Option<PathBuf>,
    small_image: Option<PathBuf>,
    sponsor_logo: Option<String>,
    template_path: Option<PathBuf>,
}
