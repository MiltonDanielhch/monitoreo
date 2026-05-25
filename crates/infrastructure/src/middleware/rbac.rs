// crates/infrastructure/src/middleware/rbac.rs
// Middleware RBAC con extractor RequireRole
// Vinculado con ADR-0006-rbac-sessions-audit.md

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode, header::AUTHORIZATION},
};
use crate::crypto::verify_access_token;
use secrecy::SecretString;
use std::future::Future;

/// Extractor de autenticación que valida JWT
pub struct AuthenticatedUser {
    pub user_id: String,
    pub role: String,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            // Extraer header Authorization
            let auth_header = parts
                .headers
                .get(AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Falta header Authorization".to_string()))?;

            // Anti-JWT: rechazar tokens que empiezan con "eyJ" (prefijo JWT base64)
            // Nota: Ahora usamos JWT, así que esta validación ya no aplica
            // if auth_header.starts_with("eyJ") {
            //     return Err((StatusCode::UNAUTHORIZED, "JWT no permitido - usar PASETO v4".to_string()));
            // }

            // Extraer token (formato: "Bearer <token>")
            let token = auth_header
                .strip_prefix("Bearer ")
                .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Formato de Authorization inválido".to_string()))?;

            // Obtener el secreto JWT del estado de la aplicación
            let jwt_secret = std::env::var("JWT_SECRET")
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "JWT_SECRET no configurado".to_string()))?;

            let secret = SecretString::new(jwt_secret.into());

            // Verificar token JWT
            let (user_id, role) = verify_access_token(token, &secret)
                .map_err(|_| (StatusCode::UNAUTHORIZED, "Token inválido o expirado".to_string()))?;

            Ok(AuthenticatedUser { user_id, role })
        }
    }
}
