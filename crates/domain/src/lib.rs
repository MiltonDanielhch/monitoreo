// crates/domain/src/lib.rs
// Núcleo puro - Entidades y Errores (0 dependencias externas de frameworks)
// Vinculado con ADR-0001-arquitectura-hexagonal.md y ADR-0007-manejo-errores.md

// Este crate contiene la lógica de negocio pura e inmutable
// No se permiten dependencias de frameworks web, bases de datos o librerías de red

pub mod errors;
pub mod models;

pub use errors::{DomainError, Result};
pub use models::user::User;
pub use models::session::Session;
