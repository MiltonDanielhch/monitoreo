// crates/database/src/repositories/mod.rs
// Módulo de repositorios de persistencia
// Vinculado con ADR-0004-persistencia-mysql-seaorm-docker.md

pub mod auth_repository;
pub mod dashboard_repository;
pub mod network_file_repository;
pub mod notification_repository;
pub mod settings_repository;

pub use auth_repository::AuthRepository;
pub use dashboard_repository::DashboardRepository;
pub use network_file_repository::NetworkFileRepository;
pub use notification_repository::NotificationRepository;
pub use settings_repository::SettingsRepository;
