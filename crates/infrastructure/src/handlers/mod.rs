// crates/infrastructure/src/handlers/mod.rs
// Módulo de controladores HTTP
// Vinculado con ADR-0003-stack-backend-rust-axum.md

pub mod auth_handler;

pub use auth_handler::{login, refresh};
