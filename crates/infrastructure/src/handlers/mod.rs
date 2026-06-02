// crates/infrastructure/src/handlers/mod.rs
// Módulo de controladores HTTP
// Vinculado con ADR-0003-stack-backend-rust-axum.md

pub mod auth_handler;
pub mod audit_handler;
pub mod dashboard_handler;
pub mod devices_handler;
pub mod infrastructure_file_handler;
pub mod locations_handler;
pub mod notification_handler;
pub mod report_handler;
pub mod security_handler;
pub mod settings_handler;
pub mod telemetry_handler;
pub mod worker_stats_handler;
pub mod worker_config_handler;

pub use auth_handler::{login, refresh, logout};
pub use audit_handler::{get_audit_logs, get_entity_history};
pub use dashboard_handler::{get_dashboard_stats, get_recent_alerts};
pub use devices_handler::get_devices;
pub use infrastructure_file_handler::{download_file, list_files, upload_file};
pub use locations_handler::get_locations;
pub use notification_handler::{get_notification_logs, test_smtp_connection};
pub use report_handler::{generate_sla_report, download_sla_report, get_sla_summary};
pub use security_handler::{
    log_security_event, get_security_events, get_security_event_by_id,
    resolve_security_event, mark_false_positive, get_events_by_severity
};
pub use settings_handler::{get_thresholds, update_thresholds};
pub use telemetry_handler::{ingest_telemetry, register_agent};
pub use worker_stats_handler::{get_worker_stats, WorkerStats};
pub use worker_config_handler::{get_worker_config, update_worker_config};
