// crates/domain/src/models/session.rs
// Modelo de sesión del dominio - Lógica de expiración y vigencia
// Vinculado con ADR-0001-arquitectura-hexagonal.md

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Modelo de sesión del dominio
/// Vinculado con ADR-0001-arquitectura-hexagonal.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token_hash: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Session {
    /// Crea una nueva sesión
    pub fn new(
        id: Uuid,
        user_id: Uuid,
        refresh_token_hash: String,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id,
            user_id,
            refresh_token_hash,
            expires_at,
            created_at: now,
        }
    }

    /// Verifica si la sesión está expirada
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() > self.expires_at
    }

    /// Verifica si la sesión está próxima a expirar (menos de 5 minutos)
    pub fn is_expiring_soon(&self) -> bool {
        let five_minutes_from_now = chrono::Utc::now() + chrono::Duration::minutes(5);
        self.expires_at < five_minutes_from_now
    }
}
