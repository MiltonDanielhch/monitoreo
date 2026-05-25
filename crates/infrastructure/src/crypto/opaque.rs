// crates/infrastructure/src/crypto/opaque.rs
// Módulo de tokens opacos y hashing SHA-256
// Vinculado con ADR-0008-seguridad-auth-paseto.md

use rand_core::{OsRng, RngCore};
use sha2::{Sha256, Digest};

/// Generar token opaco de alta entropía (64 caracteres hexadecimales)
pub fn generate_opaque_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Hashear token opaco con SHA-256
pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}
