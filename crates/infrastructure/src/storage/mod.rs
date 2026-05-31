// crates/infrastructure/src/storage/mod.rs
// Módulo de almacenamiento de archivos de infraestructura
// Vinculado con ADR-0012-adaptador-almacenamiento.md

pub mod regional_storage;

pub use regional_storage::RegionalStorageAdapter;
