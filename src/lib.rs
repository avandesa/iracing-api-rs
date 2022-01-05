pub mod auth;
pub mod client;
pub mod model;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use client::IracingApiClient;
