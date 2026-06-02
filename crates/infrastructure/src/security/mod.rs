// crates/infrastructure/src/security/mod.rs
// Módulo de seguridad y detección de intrusiones
// Vinculado con ADR-0015 (Tokio Jobs)

pub mod detection_engine;

pub use detection_engine::{
    DetectionEngine, DetectionMessage, TrafficData, DeviceData,
    CorrelatedEvent, CorrelationType
};
