// crates/domain/src/models/mod.rs
// Módulo de modelos del dominio - Entidades de negocio puras
// Vinculado con ADR-0001-arquitectura-hexagonal.md

pub mod user;
pub mod session;
pub mod settings;

pub use user::User;
pub use session::Session;
pub use settings::{SystemSettings, ThresholdSettings, ThresholdValue, Location, SystemSetting, SettingValueType};
