// crates/domain/src/models/mod.rs
// Módulo de modelos del dominio - Entidades de negocio puras
// Vinculado con ADR-0001-arquitectura-hexagonal.md

pub mod user;
pub mod session;

pub use user::User;
pub use session::Session;
