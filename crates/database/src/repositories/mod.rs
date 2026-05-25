// crates/database/src/repositories/mod.rs
// Módulo de repositorios de persistencia
// Vinculado con ADR-0004-persistencia-mysql-seaorm-docker.md

pub mod auth_repository;
pub mod settings_repository;

pub use auth_repository::AuthRepository;
pub use settings_repository::SettingsRepository;
