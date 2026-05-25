// crates/infrastructure/src/crypto/password.rs
// Módulo de hashing de contraseñas con Argon2id
// Vinculado con ADR-0008-seguridad-auth-paseto.md

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use domain::DomainError;
use secrecy::{SecretString, ExposeSecret};

/// Hash de contraseña usando Argon2id
pub fn hash_password(password: &SecretString) -> Result<String, DomainError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    let password_hash = argon2
        .hash_password(password.expose_secret().as_bytes(), &salt)
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?
        .to_string();
    
    Ok(password_hash)
}

/// Verificar contraseña contra hash
pub fn verify_password(hash: &str, password: &SecretString) -> Result<bool, DomainError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| DomainError::Infrastructure(e.to_string()))?;
    
    let argon2 = Argon2::default();
    Ok(argon2.verify_password(password.expose_secret().as_bytes(), &parsed_hash).is_ok())
}
