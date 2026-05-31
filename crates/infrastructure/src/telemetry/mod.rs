// crates/infrastructure/src/telemetry/mod.rs
// Módulo de telemetría
// Vinculado con ADR-0015-ingestion-asincrona.md

pub mod ingestion_engine;

pub use ingestion_engine::{IngestionMessage, TelemetryIngestionEngine};
