// crates/domain/src/errors.rs
// Errores tipificados del dominio - Árbol de fallos con thiserror
// Vinculado con ADR-0007-manejo-errores.md

use thiserror::Error;

/// Errores del dominio de autenticación
/// Vinculado con ADR-0007-manejo-errores.md
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Formato de correo electrónico inválido")]
    InvalidEmail,

    #[error("La contraseña no cumple con los requisitos de fortaleza mínima")]
    WeakPassword,

    #[error("Credenciales inválidas")]
    InvalidCredentials,

    #[error("Usuario suspendido")]
    UserSuspended,

    #[error("Sesión expirada")]
    SessionExpired,

    #[error("Detectado ataque por reutilización de token")]
    ReusedTokenAttack,

    #[error("Límite de velocidad excedido")]
    RateLimitExceeded,

    #[error("Valor de configuración inválido: {0}")]
    InvalidSettingValue(String),

    #[error("Error de infraestructura: {0}")]
    Infrastructure(String),
}

pub type Result<T> = std::result::Result<T, DomainError>;
