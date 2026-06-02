// crates/database/src/repositories/security_repository.rs
// Repositorio de eventos de seguridad con Sea-ORM
// Vinculado con ADR-0004 (Persistencia con Sea-ORM)

use crate::entities::security_event_entity;
use crate::entities::security_event_entity::Entity as SecurityEventEntity;
use crate::entities::security_event_entity::Column;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set, ActiveModelTrait, ColumnTrait};
use domain::models::security::{
    SecurityEvent, SecurityFilters, SecurityPort, DomainError, IntrusionType, Severity, SecurityStatus
};
use std::sync::Arc;

pub struct SecurityRepository {
    db: Arc<DatabaseConnection>,
}

impl SecurityRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    /// Convertir modelo de dominio a entidad de base de datos
    fn domain_to_entity(event: &SecurityEvent) -> security_event_entity::ActiveModel {
        let metadata_json = event.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap_or_default());

        security_event_entity::ActiveModel {
            id: Set(event.id.clone()),
            event_type: Set(event.event_type.to_string()),
            severity: Set(event.severity.to_string()),
            status: Set(event.status.to_string()),
            source_ip: Set(event.source_ip.clone()),
            source_mac: Set(event.source_mac.clone()),
            target_device_id: Set(event.target_device_id.clone()),
            target_sede_id: Set(event.target_sede_id.clone()),
            description: Set(event.description.clone()),
            metadata: Set(metadata_json),
            detected_at: Set(chrono::DateTime::parse_from_rfc3339(&event.detected_at)
                .unwrap_or_else(|_| chrono::Utc::now().into())
                .naive_utc()),
            resolved_at: if let Some(r) = event.resolved_at.as_ref() {
                Set(Some(chrono::DateTime::parse_from_rfc3339(r)
                    .unwrap_or_else(|_| chrono::Utc::now().into())
                    .naive_utc()))
            } else {
                Set(None)
            },
            resolved_by: Set(event.resolved_by.clone()),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
        }
    }

    /// Convertir entidad de base de datos a modelo de dominio
    fn entity_to_domain(entity: &security_event_entity::Model) -> Result<SecurityEvent, DomainError> {
        let metadata = entity.metadata.as_ref()
            .and_then(|m| serde_json::from_str(m).ok());

        let detected_at: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_naive_utc_and_offset(
            entity.detected_at,
            chrono::Utc
        );

        let resolved_at = entity.resolved_at.as_ref().map(|r| {
            let dt: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_naive_utc_and_offset(*r, chrono::Utc);
            dt.to_rfc3339()
        });

        Ok(SecurityEvent {
            id: entity.id.clone(),
            event_type: IntrusionType::from(entity.event_type.clone()),
            severity: Severity::from(entity.severity.clone()),
            status: SecurityStatus::from(entity.status.clone()),
            source_ip: entity.source_ip.clone(),
            source_mac: entity.source_mac.clone(),
            target_device_id: entity.target_device_id.clone(),
            target_sede_id: entity.target_sede_id.clone(),
            description: entity.description.clone(),
            metadata,
            detected_at: detected_at.to_rfc3339(),
            resolved_at,
            resolved_by: entity.resolved_by.clone(),
        })
    }
}

#[async_trait::async_trait]
impl SecurityPort for SecurityRepository {
    async fn log_event(&self, event: SecurityEvent) -> Result<(), DomainError> {
        let active_model = Self::domain_to_entity(&event);
        SecurityEventEntity::insert(active_model)
            .exec(&*self.db)
            .await
            .map_err(|e| DomainError::from(format!("Error al insertar evento: {}", e)))?;
        Ok(())
    }

    async fn get_events(&self, filters: SecurityFilters) -> Result<Vec<SecurityEvent>, DomainError> {
        let mut query = SecurityEventEntity::find();

        if let Some(severity) = filters.severity {
            query = query.filter(Column::Severity.eq(severity.to_string()));
        }

        if let Some(status) = filters.status {
            query = query.filter(Column::Status.eq(status.to_string()));
        }

        if let Some(source_ip) = filters.source_ip {
            query = query.filter(Column::SourceIp.eq(source_ip));
        }

        if let Some(target_device_id) = filters.target_device_id {
            query = query.filter(Column::TargetDeviceId.eq(target_device_id));
        }

        if let Some(target_sede_id) = filters.target_sede_id {
            query = query.filter(Column::TargetSedeId.eq(target_sede_id));
        }

        if let Some(date_from) = filters.date_from {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_from) {
                query = query.filter(Column::DetectedAt.gte(dt.naive_utc()));
            }
        }

        if let Some(date_to) = filters.date_to {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_to) {
                query = query.filter(Column::DetectedAt.lte(dt.naive_utc()));
            }
        }

        query = query.order_by_desc(Column::DetectedAt);

        let entities = query
            .all(&*self.db)
            .await
            .map_err(|e| DomainError::from(format!("Error al obtener eventos: {}", e)))?;

        entities.iter()
            .map(Self::entity_to_domain)
            .collect()
    }

    async fn get_event_by_id(&self, id: String) -> Result<Option<SecurityEvent>, DomainError> {
        let entity = SecurityEventEntity::find_by_id(id.clone())
            .one(&*self.db)
            .await
            .map_err(|e| DomainError::from(format!("Error al obtener evento: {}", e)))?;

        entity.map(|e| Self::entity_to_domain(&e)).transpose()
    }

    async fn resolve_event(&self, id: String, resolved_by: String) -> Result<(), DomainError> {
        let entity = SecurityEventEntity::find_by_id(id.clone())
            .one(&*self.db)
            .await
            .map_err(|e| DomainError::from(format!("Error al obtener evento: {}", e)))?;

        if let Some(entity) = entity {
            let mut active_model: security_event_entity::ActiveModel = entity.into();
            active_model.status = Set(SecurityStatus::Resolved.to_string());
            active_model.resolved_at = Set(Some(chrono::Utc::now().naive_utc()));
            active_model.resolved_by = Set(Some(resolved_by));
            active_model.updated_at = Set(chrono::Utc::now().naive_utc());

            active_model.update(&*self.db)
                .await
                .map_err(|e| DomainError::from(format!("Error al resolver evento: {}", e)))?;
        }

        Ok(())
    }

    async fn mark_false_positive(&self, id: String) -> Result<(), DomainError> {
        let entity = SecurityEventEntity::find_by_id(id.clone())
            .one(&*self.db)
            .await
            .map_err(|e| DomainError::from(format!("Error al obtener evento: {}", e)))?;

        if let Some(entity) = entity {
            let mut active_model: security_event_entity::ActiveModel = entity.into();
            active_model.status = Set(SecurityStatus::FalsePositive.to_string());
            active_model.updated_at = Set(chrono::Utc::now().naive_utc());

            active_model.update(&*self.db)
                .await
                .map_err(|e| DomainError::from(format!("Error al marcar falso positivo: {}", e)))?;
        }

        Ok(())
    }

    async fn get_events_by_severity(&self, severity: Severity) -> Result<Vec<SecurityEvent>, DomainError> {
        let entities = SecurityEventEntity::find()
            .filter(Column::Severity.eq(severity.to_string()))
            .order_by_desc(Column::DetectedAt)
            .all(&*self.db)
            .await
            .map_err(|e| DomainError::from(format!("Error al obtener eventos por severidad: {}", e)))?;

        entities.iter()
            .map(Self::entity_to_domain)
            .collect()
    }

    async fn get_events_by_status(&self, status: SecurityStatus) -> Result<Vec<SecurityEvent>, DomainError> {
        let entities = SecurityEventEntity::find()
            .filter(Column::Status.eq(status.to_string()))
            .order_by_desc(Column::DetectedAt)
            .all(&*self.db)
            .await
            .map_err(|e| DomainError::from(format!("Error al obtener eventos por estado: {}", e)))?;

        entities.iter()
            .map(Self::entity_to_domain)
            .collect()
    }

    async fn get_events_by_device(&self, device_id: String) -> Result<Vec<SecurityEvent>, DomainError> {
        let entities = SecurityEventEntity::find()
            .filter(Column::TargetDeviceId.eq(device_id))
            .order_by_desc(Column::DetectedAt)
            .all(&*self.db)
            .await
            .map_err(|e| DomainError::from(format!("Error al obtener eventos por dispositivo: {}", e)))?;

        entities.iter()
            .map(Self::entity_to_domain)
            .collect()
    }
}
