// crates/database/src/repositories/auth_repository.rs
// Repositorio de autenticación con Sea-ORM
// Vinculado con ADR-0004-persistencia-mysql-seaorm-docker.md

use crate::entities::{user_entity, user_session_entity, used_refresh_token_entity};
use domain::{DomainError, Result};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, TransactionTrait};
use uuid::Uuid;
use chrono::Utc;

/// Repositorio de autenticación
pub struct AuthRepository {
    db: DatabaseConnection,
}

impl AuthRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Buscar sesión por hash del refresh token
    pub async fn find_session_by_hash(&self, hash: &str) -> Result<Option<domain::Session>> {
        let session = user_session_entity::Entity::find()
            .filter(user_session_entity::Column::RefreshTokenHash.eq(hash))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        if let Some(session_entity) = session {
            Ok(Some(domain::Session::new(
                Uuid::parse_str(&session_entity.id).map_err(|_| DomainError::Infrastructure("Invalid UUID".to_string()))?,
                Uuid::parse_str(&session_entity.user_id).map_err(|_| DomainError::Infrastructure("Invalid UUID".to_string()))?,
                session_entity.refresh_token_hash,
                session_entity.expires_at.and_utc(),
            )))
        } else {
            Ok(None)
        }
    }

    /// Crear nueva sesión
    pub async fn create_session(&self, session: domain::Session) -> Result<()> {
        let session_model = user_session_entity::ActiveModel {
            id: sea_orm::Set(session.id.to_string()),
            user_id: sea_orm::Set(session.user_id.to_string()),
            refresh_token_hash: sea_orm::Set(session.refresh_token_hash),
            expires_at: sea_orm::Set(session.expires_at.naive_utc()),
            created_at: sea_orm::Set(session.created_at.naive_utc()),
            updated_at: sea_orm::Set(session.created_at.naive_utc()),
        };

        user_session_entity::Entity::insert(session_model)
            .exec_without_returning(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(format!("Error inserting session: {}", e)))?;

        tracing::info!("Sesión insertada exitosamente");
        Ok(())
    }

    /// Eliminar una sesión específica por hash del refresh token
    pub async fn delete_session(&self, refresh_token_hash: &str) -> Result<()> {
        user_session_entity::Entity::delete_many()
            .filter(user_session_entity::Column::RefreshTokenHash.eq(refresh_token_hash))
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;
        Ok(())
    }

    /// Rotar sesión (eliminar vieja y crear nueva)
    pub async fn rotate_session(&self, old_hash: &str, new_session: domain::Session) -> Result<()> {
        let txn = self.db.begin().await.map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        // Eliminar sesión vieja
        user_session_entity::Entity::delete_many()
            .filter(user_session_entity::Column::RefreshTokenHash.eq(old_hash))
            .exec(&txn)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        // Crear nueva sesión
        let session_model = user_session_entity::ActiveModel {
            id: sea_orm::Set(new_session.id.to_string()),
            user_id: sea_orm::Set(new_session.user_id.to_string()),
            refresh_token_hash: sea_orm::Set(new_session.refresh_token_hash),
            expires_at: sea_orm::Set(new_session.expires_at.naive_utc()),
            created_at: sea_orm::Set(new_session.created_at.naive_utc()),
            updated_at: sea_orm::Set(new_session.created_at.naive_utc()),
        };

        user_session_entity::Entity::insert(session_model)
            .exec_without_returning(&txn)
            .await
            .map_err(|e| DomainError::Infrastructure(format!("Error inserting session during rotation: {}", e)))?;

        txn.commit().await.map_err(|e| DomainError::Infrastructure(e.to_string()))?;
        Ok(())
    }

    /// Purgar todas las sesiones de un usuario (para ataques de reutilización)
    pub async fn purge_user_sessions(&self, user_id: &Uuid) -> Result<()> {
        user_session_entity::Entity::delete_many()
            .filter(user_session_entity::Column::UserId.eq(user_id.to_string()))
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        Ok(())
    }

    /// Buscar usuario por email
    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<domain::User>> {
        let user = user_entity::Entity::find()
            .filter(user_entity::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        if let Some(user_entity) = user {
            let user_id = Uuid::parse_str(&user_entity.id)
                .map_err(|e| DomainError::Infrastructure(format!("Invalid UUID for user_id: {} - error: {}", user_entity.id, e)))?;
            let role_id = Uuid::parse_str(&user_entity.role_id)
                .map_err(|e| DomainError::Infrastructure(format!("Invalid UUID for role_id: {} - error: {}", user_entity.role_id, e)))?;
            
            Ok(Some(domain::User::new(
                user_id,
                user_entity.email,
                user_entity.password_hash,
                role_id,
            )?))
        } else {
            Ok(None)
        }
    }

    /// Eliminar sesiones expiradas (para limpieza periódica)
    pub async fn delete_expired_sessions(&self) -> Result<u64> {
        let result = user_session_entity::Entity::delete_many()
            .filter(user_session_entity::Column::ExpiresAt.lt(Utc::now().naive_utc()))
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        Ok(result.rows_affected)
    }

    /// Marcar un refresh token como usado (para RTR replay protection)
    pub async fn mark_token_as_used(&self, token_hash: &str, user_id: &Uuid) -> Result<()> {
        let used_token = used_refresh_token_entity::ActiveModel {
            id: sea_orm::Set(Uuid::now_v7().to_string()),
            token_hash: sea_orm::Set(token_hash.to_string()),
            user_id: sea_orm::Set(user_id.to_string()),
            used_at: sea_orm::Set(Utc::now().naive_utc()),
        };

        used_refresh_token_entity::Entity::insert(used_token)
            .exec_without_returning(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(format!("Error marking token as used: {}", e)))?;

        Ok(())
    }

    /// Verificar si un refresh token ya fue usado (RTR replay detection)
    pub async fn is_token_reused(&self, token_hash: &str) -> Result<Option<String>> {
        let used_token = used_refresh_token_entity::Entity::find()
            .filter(used_refresh_token_entity::Column::TokenHash.eq(token_hash))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        Ok(used_token.map(|t| t.user_id))
    }
}
