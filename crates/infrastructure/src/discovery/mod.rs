// crates/infrastructure/src/discovery/mod.rs
// Módulo de descubrimiento de red y escaneo de activos
// Vinculado con ADR-0015 (Tokio Jobs) y ADR-0001 (Arquitectura Hexagonal)

pub mod scan_engine;
pub mod oui_lookup;

pub use scan_engine::{ScanEngine, ScanConfig, ScanMessage, ScanProgress};
pub use oui_lookup::OuiLookupService;
