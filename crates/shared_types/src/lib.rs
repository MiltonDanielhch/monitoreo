// crates/shared_types/src/lib.rs
// Tipos de contrato compartidos entre backend y frontend
// Vinculado con ADR-0011-estandares-desarrollo.md y ADR-0016-openapi.md

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// DTO para solicitud de login
/// Vinculado con ADR-0008-seguridad-auth-paseto.md
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// DTO para solicitud de refresh token
/// Vinculado con ADR-0008-seguridad-auth-paseto.md
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TokenRefreshRequest {
    pub refresh_token: String,
}

/// DTO para respuesta de autenticación
/// Vinculado con ADR-0008-seguridad-auth-paseto.md
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user_info: UserInfo,
}

/// Información del usuario en respuesta de autenticación
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
    pub role: String,
}

/// DTO para solicitud de logout
/// Vinculado con ADR-0008-seguridad-auth-paseto.md
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LogoutRequest {
    pub refresh_token: String,
}
