// crates/infrastructure/src/crypto/mod.rs
// Módulo de criptografía - Argon2id, JWT, SHA-256

pub mod password;
pub mod jwt;
pub mod opaque;

pub use password::{hash_password, verify_password};
pub use jwt::{generate_access_token, verify_access_token};
pub use opaque::{generate_opaque_token, hash_token};
