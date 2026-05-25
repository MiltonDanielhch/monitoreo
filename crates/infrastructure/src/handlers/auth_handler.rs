// crates/infrastructure/src/handlers/auth_handler.rs
// Controladores HTTP de autenticación
// Vinculado con ADR-0003-stack-backend-rust-axum.md, ADR-0006-rbac-sessions-audit.md

use axum::{Json, extract::State, http::StatusCode};
use shared_types::{LoginRequest, TokenRefreshRequest, AuthResponse, UserInfo, LogoutRequest};
use crate::crypto::{verify_password, generate_access_token, generate_opaque_token, hash_token};
use crate::AppState;
use domain::Session;
use uuid::Uuid;
use chrono::{Duration, Utc};
use secrecy::SecretString;
use tracing::{info, error, instrument};

/// Endpoint /api/auth/login
#[instrument(skip(state, req))]
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    info!("Intentando login para email: {}", req.email);
    
    // Buscar usuario por email
    let user = state.auth_repo.find_user_by_email(&req.email)
        .await
        .map_err(|e| {
            error!("Error buscando usuario: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Credenciales inválidas".to_string()))?;

    info!("Usuario encontrado: {}", user.email);

    // Verificar contraseña
    let password_secret = SecretString::new(req.password.into());
    let password_valid = verify_password(&user.password_hash, &password_secret)
        .map_err(|e| {
            error!("Error verificando contraseña: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    
    if !password_valid {
        info!("Contraseña inválida para usuario: {}", user.email);
        return Err((StatusCode::UNAUTHORIZED, "Credenciales inválidas".to_string()));
    }

    info!("Contraseña válida para usuario: {}", user.email);

    // Generar token de acceso JWT (1 hora de expiración)
    let access_token = generate_access_token(
        &user.id.to_string(),
        &user.role_id.to_string(),
        &state.paseto_secret,
        1,
    ).map_err(|e| {
        error!("Error generando access token: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    info!("Access token generado para usuario: {}", user.email);

    // Generar refresh token opaco
    let refresh_token = generate_opaque_token();
    let refresh_token_hash = hash_token(&refresh_token);

    // Crear sesión en base de datos (7 días de expiración)
    let session = Session::new(
        uuid::Uuid::now_v7(),
        user.id,
        refresh_token_hash,
        Utc::now() + Duration::days(7),
    );

    state.auth_repo.create_session(session)
        .await
        .map_err(|e| {
            error!("Error creando sesión: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    info!("Sesión creada para usuario: {}", user.email);

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        user_info: UserInfo {
            id: user.id,
            email: user.email.clone(),
            role: user.role_id.to_string(),
        },
    }))
}

/// Endpoint /api/auth/refresh con RTR (Refresh Token Rotation) y protección anti-replay
#[instrument(skip(state, req))]
pub async fn refresh(
    State(state): State<AppState>,
    Json(req): Json<TokenRefreshRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let refresh_token_hash = hash_token(&req.refresh_token);

    if let Some(user_id) = state.auth_repo.is_token_reused(&refresh_token_hash)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    {
        error!("⚠️ ATAQUE RTR DETECTADO: Refresh token reutilizado para usuario {}", user_id);
        let user_uuid = Uuid::parse_str(&user_id).map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Invalid user ID".to_string()))?;
        state.auth_repo.purge_user_sessions(&user_uuid)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        error!("🚨 Todas las sesiones del usuario {} han sido purgadas debido a ataque RTR", user_id);
        return Err((StatusCode::UNAUTHORIZED, "Token de refresco inválido - posible ataque detectado".to_string()));
    }

    let session = state.auth_repo.find_session_by_hash(&refresh_token_hash)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Token de refresco inválido".to_string()))?;

    if session.expires_at < Utc::now() {
        return Err((StatusCode::UNAUTHORIZED, "Sesión expirada".to_string()));
    }

    state.auth_repo.mark_token_as_used(&refresh_token_hash, &session.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let new_refresh_token = generate_opaque_token();
    let new_refresh_token_hash = hash_token(&new_refresh_token);
    let new_session = Session::new(
        uuid::Uuid::now_v7(),
        session.user_id,
        new_refresh_token_hash,
        Utc::now() + Duration::days(7),
    );

    state.auth_repo.rotate_session(&refresh_token_hash, new_session)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let new_access_token = generate_access_token(
        &session.user_id.to_string(),
        "user",
        &state.paseto_secret,
        1,
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse {
        access_token: new_access_token,
        refresh_token: new_refresh_token,
        user_info: UserInfo {
            id: session.user_id,
            email: "user@example.com".to_string(),
            role: "user".to_string(),
        },
    }))
}

/// Endpoint /api/auth/logout - Destruir sesión y limpiar hash de token
#[instrument(skip(state, req))]
pub async fn logout(
    State(state): State<AppState>,
    Json(req): Json<LogoutRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let refresh_token_hash = hash_token(&req.refresh_token);

    let session = state.auth_repo.find_session_by_hash(&refresh_token_hash)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Sesión no encontrada".to_string()))?;

    state.auth_repo.delete_session(&refresh_token_hash)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    state.auth_repo.mark_token_as_used(&refresh_token_hash, &session.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    info!("Sesión destruida para usuario {:?}", session.user_id);

    Ok(Json(serde_json::json!({
        "message": "Sesión cerrada exitosamente"
    })))
}
