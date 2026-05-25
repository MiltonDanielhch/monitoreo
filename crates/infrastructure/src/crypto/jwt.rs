// crates/infrastructure/src/crypto/jwt.rs
// Módulo de tokens JWT
// Vinculado con ADR-0008-seguridad-auth-paseto.md (modificado para JWT)

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use domain::DomainError;
use secrecy::{SecretString, ExposeSecret};
use chrono::{Duration, Utc};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
    iat: usize,
}

pub fn generate_access_token(
    user_id: &str,
    role: &str,
    secret: &SecretString,
    expires_in_hours: i64,
) -> Result<String, DomainError> {
    let secret_bytes = secret.expose_secret().as_bytes();

    let now = Utc::now();
    let exp = now + Duration::hours(expires_in_hours);

    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_bytes),
    ).map_err(|e| DomainError::Infrastructure(format!("Error generating JWT: {}", e)))
}

pub fn verify_access_token(
    token: &str,
    secret: &SecretString,
) -> Result<(String, String), DomainError> {
    let secret_bytes = secret.expose_secret().as_bytes();

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_bytes),
        &Validation::default(),
    ).map_err(|e| {
        if e.to_string().contains("expired") {
            DomainError::SessionExpired
        } else {
            DomainError::Infrastructure(format!("Error verifying JWT: {}", e))
        }
    })?;

    let user_id = token_data.claims.sub;
    let role = token_data.claims.role;

    Ok((user_id, role))
}