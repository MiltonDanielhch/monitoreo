// crates/database/src/repositories/notification_repository.rs
// Repositorio de notificaciones - Persistencia con Sea-ORM
// Vinculado con ADR-0004-persistencia-mysql-seaorm-docker.md
// Módulo 4: Motor de Notificaciones

use crate::entities::{notification_channel_entity, notification_log_entity, notification_template_entity};
use domain::{NotificationEventType, NotificationTemplate, NotificationRequest, Result};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use uuid::uuid;

/// Repositorio de notificaciones
/// Maneja toda la persistencia de canales, plantillas y logs de notificaciones
pub struct NotificationRepository {
    db: DatabaseConnection,
}

impl NotificationRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    // ==================== CANALES DE NOTIFICACIÓN ====================

    /// Obtener un canal por ID
    pub async fn get_channel(&self, channel_id: &str) -> Result<notification_channel_entity::Model> {
        notification_channel_entity::Entity::find_by_id(channel_id.to_string())
            .one(&self.db)
            .await
            .map_err(|e| domain::DomainError::Infrastructure(e.to_string()))?
            .ok_or_else(|| domain::DomainError::ChannelUnavailable)
    }

    /// Obtener todos los canales activos
    pub async fn get_active_channels(&self) -> Result<Vec<notification_channel_entity::Model>> {
        notification_channel_entity::Entity::find()
            .filter(notification_channel_entity::Column::IsActive.eq(true))
            .filter(notification_channel_entity::Column::DeletedAt.is_null())
            .all(&self.db)
            .await
            .map_err(|e| domain::DomainError::Infrastructure(e.to_string()))
    }

    // ==================== PLANTILLAS DE NOTIFICACIÓN ====================

    /// Obtener una plantilla por ID
    pub async fn get_template(
        &self,
        template_id: &str,
    ) -> Result<notification_template_entity::Model> {
        notification_template_entity::Entity::find_by_id(template_id.to_string())
            .one(&self.db)
            .await
            .map_err(|e| domain::DomainError::Infrastructure(e.to_string()))?
            .ok_or_else(|| domain::DomainError::TemplateNotFound)
    }

    /// Obtener plantilla por tipo de evento
    pub async fn get_template_by_type(
        &self,
        event_type: NotificationEventType,
    ) -> Result<notification_template_entity::Model> {
        let event_type_str = match event_type {
            NotificationEventType::NodeDown => "NODE_DOWN",
            NotificationEventType::HighLatency => "HIGH_LATENCY",
            NotificationEventType::BandwidthSaturation => "BANDWIDTH_SATURATION",
            NotificationEventType::DeviceUnauthorized => "DEVICE_UNAUTHORIZED",
        };

        notification_template_entity::Entity::find()
            .filter(notification_template_entity::Column::TemplateType.eq(event_type_str))
            .filter(notification_template_entity::Column::IsActive.eq(true))
            .filter(notification_template_entity::Column::DeletedAt.is_null())
            .one(&self.db)
            .await
            .map_err(|e| domain::DomainError::Infrastructure(e.to_string()))?
            .ok_or_else(|| domain::DomainError::TemplateNotFound)
    }

    /// Convertir entidad de plantilla a modelo del dominio
    pub fn entity_to_template(
        &self,
        entity: notification_template_entity::Model,
    ) -> Result<NotificationTemplate> {
        let template_type = match entity.template_type.as_str() {
            "NODE_DOWN" => NotificationEventType::NodeDown,
            "HIGH_LATENCY" => NotificationEventType::HighLatency,
            "BANDWIDTH_SATURATION" => NotificationEventType::BandwidthSaturation,
            "DEVICE_UNAUTHORIZED" => NotificationEventType::DeviceUnauthorized,
            _ => {
                return Err(domain::DomainError::TemplateRenderError(
                    "Tipo de plantilla desconocido".to_string(),
                ))
            }
        };

        Ok(NotificationTemplate {
            id: entity.id,
            name: entity.name,
            template_type,
            subject: entity.subject,
            body: entity.body,
            is_active: entity.is_active,
        })
    }

    // ==================== LOGS DE NOTIFICACIÓN ====================

    /// Crear un nuevo log de notificación en estado PENDING
    pub async fn create_notification_log(
        &self,
        request: &NotificationRequest,
    ) -> Result<String> {
        let log_id = uuid!("550e8400-e29b-41d4-a716-446655440000").to_string();

        let new_log = notification_log_entity::ActiveModel {
            id: Set(log_id.clone()),
            channel_id: Set(request.channel_id.clone()),
            template_id: Set(request.template_id.clone()),
            recipient: Set(request.recipient.clone()),
            status: Set("PENDING".to_string()),
            attempt_count: Set(0),
            max_attempts: Set(3),
            error_message: Set(None),
            sent_at: Set(None),
            ..Default::default()
        };

        notification_log_entity::Entity::insert(new_log)
            .exec(&self.db)
            .await
            .map_err(|e| domain::DomainError::Infrastructure(e.to_string()))?;

        Ok(log_id)
    }

    /// Actualizar estado de un log a SENT
    pub async fn mark_as_sent(&self, log_id: &str) -> Result<()> {
        let log = notification_log_entity::Entity::find_by_id(log_id.to_string())
            .one(&self.db)
            .await
            .map_err(|e| domain::DomainError::Infrastructure(e.to_string()))?
            .ok_or_else(|| domain::DomainError::TemplateNotFound)?;

        let mut active_log: notification_log_entity::ActiveModel = log.into();
        active_log.status = Set("SENT".to_string());
        active_log.sent_at = Set(Some(chrono::Utc::now().into()));
        active_log.updated_at = Set(chrono::Utc::now().into());

        active_log
            .update(&self.db)
            .await
            .map_err(|e| domain::DomainError::Infrastructure(e.to_string()))?;

        Ok(())
    }

    /// Actualizar estado de un log a FAILED con mensaje de error
    pub async fn mark_as_failed(&self, log_id: &str, error_message: &str) -> Result<()> {
        let log = notification_log_entity::Entity::find_by_id(log_id.to_string())
            .one(&self.db)
            .await
            .map_err(|e| domain::DomainError::Infrastructure(e.to_string()))?
            .ok_or_else(|| domain::DomainError::TemplateNotFound)?;

        let mut active_log: notification_log_entity::ActiveModel = log.into();
        active_log.status = Set("FAILED".to_string());
        active_log.error_message = Set(Some(error_message.to_string()));
        active_log.attempt_count = Set(active_log.attempt_count.unwrap() + 1);
        active_log.updated_at = Set(chrono::Utc::now().into());

        active_log
            .update(&self.db)
            .await
            .map_err(|e| domain::DomainError::Infrastructure(e.to_string()))?;

        Ok(())
    }

    /// Incrementar contador de intentos y marcar como RETRYING
    pub async fn mark_for_retry(&self, log_id: &str) -> Result<()> {
        let log = notification_log_entity::Entity::find_by_id(log_id.to_string())
            .one(&self.db)
            .await
            .map_err(|e| domain::DomainError::Infrastructure(e.to_string()))?
            .ok_or_else(|| domain::DomainError::TemplateNotFound)?;

        // Verificar si excedió el máximo de intentos
        if log.attempt_count >= log.max_attempts {
            return Err(domain::DomainError::MaxRetriesExceeded);
        }

        let mut active_log: notification_log_entity::ActiveModel = log.into();
        active_log.status = Set("RETRYING".to_string());
        active_log.attempt_count = Set(active_log.attempt_count.unwrap() + 1);
        active_log.updated_at = Set(chrono::Utc::now().into());

        active_log
            .update(&self.db)
            .await
            .map_err(|e| domain::DomainError::Infrastructure(e.to_string()))?;

        Ok(())
    }

    /// Obtener logs pendientes para procesamiento en background
    pub async fn get_pending_logs(&self, limit: u64) -> Result<Vec<notification_log_entity::Model>> {
        notification_log_entity::Entity::find()
            .filter(
                Condition::any()
                    .add(notification_log_entity::Column::Status.eq("PENDING"))
                    .add(notification_log_entity::Column::Status.eq("RETRYING")),
            )
            .filter(notification_log_entity::Column::DeletedAt.is_null())
            .order_by_desc(notification_log_entity::Column::CreatedAt)
            .limit(limit)
            .all(&self.db)
            .await
            .map_err(|e| domain::DomainError::Infrastructure(e.to_string()))
    }

    /// Obtener logs paginados para la interfaz visual
    pub async fn get_logs_paginated(
        &self,
        page: u64,
        per_page: u64,
    ) -> Result<Vec<notification_log_entity::Model>> {
        let offset = (page - 1) * per_page;

        notification_log_entity::Entity::find()
            .filter(notification_log_entity::Column::DeletedAt.is_null())
            .order_by_desc(notification_log_entity::Column::CreatedAt)
            .paginate(&self.db, per_page)
            .fetch_page(offset)
            .await
            .map_err(|e| domain::DomainError::Infrastructure(e.to_string()))
    }

    /// Contar total de logs para paginación
    pub async fn count_logs(&self) -> Result<u64> {
        notification_log_entity::Entity::find()
            .filter(notification_log_entity::Column::DeletedAt.is_null())
            .count(&self.db)
            .await
            .map_err(|e| domain::DomainError::Infrastructure(e.to_string()))
    }
}
