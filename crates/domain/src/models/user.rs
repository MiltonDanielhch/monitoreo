// crates/domain/src/models/user.rs
// Modelo de usuario del dominio - Validaciones y lógica de negocio pura
// Vinculado con ADR-0001-arquitectura-hexagonal.md

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Modelo de usuario del dominio
/// Vinculado con ADR-0001-arquitectura-hexagonal.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role_id: Uuid,
    pub is_active: bool,
}

impl User {
    /// Crea un nuevo usuario con validaciones
    pub fn new(
        id: Uuid,
        email: String,
        password_hash: String,
        role_id: Uuid,
    ) -> Result<Self, crate::errors::DomainError> {
        // Validar formato de correo electrónico
        if !Self::is_valid_email(&email) {
            return Err(crate::errors::DomainError::InvalidEmail);
        }

        // Validar fortaleza de contraseña (verificando el hash)
        if password_hash.len() < 32 {
            return Err(crate::errors::DomainError::WeakPassword);
        }

        Ok(Self {
            id,
            email,
            password_hash,
            role_id,
            is_active: true,
        })
    }

    /// Valida formato de correo electrónico (validación básica)
    fn is_valid_email(email: &str) -> bool {
        // Validación simple de formato de correo
        email.contains('@') && email.contains('.') && email.len() > 5
    }

    /// Suspende al usuario
    pub fn suspend(&mut self) {
        self.is_active = false;
    }

    /// Activa al usuario
    pub fn activate(&mut self) {
        self.is_active = true;
    }
}
