// crates/database/src/repositories/audit_repository.rs
// Repositorio para auditoría inmutable
// Vinculado con ADR-0004-persistencia-sea-orm.md

use crate::entities::audit_entity;
use domain::models::audit::{AuditLog, AuditAction, AuditFilters, AuditPort};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};
use serde_json;

pub struct AuditRepository {
    db: DatabaseConnection,
}

impl AuditRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Convierte un modelo de Sea-ORM a entidad de dominio
    fn model_to_domain(&self, model: audit_entity::Model) -> AuditLog {
        let action = AuditAction::from_str(&model.action);
        let old_value = model.old_value.and_then(|s| serde_json::from_str(&s).ok());
        let new_value = model.new_value.and_then(|s| serde_json::from_str(&s).ok());
        let metadata = model.metadata.and_then(|s| serde_json::from_str(&s).ok());

        // Convertir NaiveDateTime a DateTime<Utc> usando el método actual
        let timestamp: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_naive_utc_and_offset(model.timestamp, chrono::Utc);

        AuditLog {
            id: model.id,
            timestamp: timestamp.to_rfc3339(),
            user_id: model.user_id,
            action,
            entity_type: model.entity_type,
            entity_id: model.entity_id,
            old_value,
            new_value,
            ip_address: model.ip_address,
            user_agent: model.user_agent,
            metadata,
        }
    }
}

#[async_trait::async_trait]
impl AuditPort for AuditRepository {
    /// Registra un evento de auditoría (append-only)
    async fn log_event(&self, log: AuditLog) -> Result<(), domain::errors::DomainError> {
        let old_value_json = log.old_value.map(|v| serde_json::to_string(&v).unwrap_or_default());
        let new_value_json = log.new_value.map(|v| serde_json::to_string(&v).unwrap_or_default());
        let metadata_json = log.metadata.map(|m| serde_json::to_string(&m).unwrap_or_default());

        let new_log = audit_entity::ActiveModel {
            id: Set(log.id),
            timestamp: Set(chrono::Utc::now().naive_utc()),
            user_id: Set(log.user_id),
            action: Set(log.action.to_string()),
            entity_type: Set(log.entity_type),
            entity_id: Set(log.entity_id),
            old_value: Set(old_value_json),
            new_value: Set(new_value_json),
            ip_address: Set(log.ip_address),
            user_agent: Set(log.user_agent),
            metadata: Set(metadata_json),
        };

        new_log.insert(&self.db).await.map_err(|e| {
            domain::errors::DomainError::Infrastructure(format!("Error inserting audit log: {}", e))
        })?;

        Ok(())
    }

    /// Consulta registros de auditoría con filtros
    async fn query_logs(
        &self,
        filters: AuditFilters,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<AuditLog>, domain::errors::DomainError> {
        let mut query = audit_entity::Entity::find();

        // Aplicar filtros
        if let Some(user_id) = filters.user_id {
            query = query.filter(audit_entity::Column::UserId.eq(user_id));
        }
        if let Some(action) = filters.action {
            query = query.filter(audit_entity::Column::Action.eq(action.to_string()));
        }
        if let Some(entity_type) = filters.entity_type {
            query = query.filter(audit_entity::Column::EntityType.eq(entity_type));
        }
        if let Some(entity_id) = filters.entity_id {
            query = query.filter(audit_entity::Column::EntityId.eq(entity_id));
        }
        if let Some(ip_address) = filters.ip_address {
            query = query.filter(audit_entity::Column::IpAddress.eq(ip_address));
        }

        let results = query
            .order_by_desc(audit_entity::Column::Timestamp)
            .paginate(&self.db, limit as u64)
            .fetch_page(offset as u64)
            .await
            .map_err(|e| {
                domain::errors::DomainError::Infrastructure(format!("Error fetching audit logs: {}", e))
            })?;

        Ok(results.into_iter().map(|m| self.model_to_domain(m)).collect())
    }

    /// Obtiene un registro específico por ID (solo lectura)
    async fn get_log_by_id(&self, id: String) -> Result<Option<AuditLog>, domain::errors::DomainError> {
        let result = audit_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                domain::errors::DomainError::Infrastructure(format!(
                    "Error fetching audit log: {}",
                    e
                ))
            })?;

        Ok(result.map(|m| self.model_to_domain(m)))
    }

    /// Obtiene el historial de cambios de una entidad específica
    async fn get_entity_history(
        &self,
        entity_type: String,
        entity_id: String,
    ) -> Result<Vec<AuditLog>, domain::errors::DomainError> {
        let results = audit_entity::Entity::find()
            .filter(audit_entity::Column::EntityType.eq(entity_type))
            .filter(audit_entity::Column::EntityId.eq(entity_id))
            .order_by_desc(audit_entity::Column::Timestamp)
            .all(&self.db)
            .await
            .map_err(|e| {
                domain::errors::DomainError::Infrastructure(format!(
                    "Error fetching entity history: {}",
                    e
                ))
            })?;

        Ok(results.into_iter().map(|m| self.model_to_domain(m)).collect())
    }
}
