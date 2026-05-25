// crates/infrastructure/src/config/mod.rs
// Módulo de configuración en tiempo de ejecución
// Vinculado con ADR-0015 (Asincronía con Tokio Jobs)

pub mod runtime_config;

pub use runtime_config::{RuntimeConfig, RuntimeConfigInner, HealthStatus};