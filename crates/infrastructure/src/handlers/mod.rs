// crates/infrastructure/src/handlers/mod.rs
// Módulo de controladores HTTP
// Vinculado con ADR-0003-stack-backend-rust-axum.md

pub mod auth_handler;
pub mod dashboard_handler;
pub mod devices_handler;
pub mod locations_handler;
pub mod settings_handler;

pub use auth_handler::{login, refresh, logout};
pub use dashboard_handler::{get_dashboard_stats, get_recent_alerts};
pub use devices_handler::get_devices;
pub use locations_handler::get_locations;
pub use settings_handler::{get_thresholds, update_thresholds};
