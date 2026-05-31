// crates/database/src/repositories/telemetry_repository.rs
// Repositorio para telemetría de agentes
// Vinculado con ADR-0004-persistencia-sea-orm.md

use crate::entities::{agent_metrics_entity, remote_agent_entity};
use domain::models::telemetry::{
    RemoteAgent, AgentType, AgentStatus, TelemetryMetrics, TelemetryBatch, TelemetryPort,
};
use num_traits::cast::ToPrimitive;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};
use serde_json;

pub struct TelemetryRepository {
    db: DatabaseConnection,
}

impl TelemetryRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Convierte un modelo de Sea-ORM a entidad de dominio para RemoteAgent
    fn model_to_agent(&self, model: remote_agent_entity::Model) -> RemoteAgent {
        let agent_type = AgentType::from_str(&model.agent_type);
        let status = AgentStatus::from_str(&model.status);
        let metadata = model.metadata.and_then(|s| serde_json::from_str(&s).ok());

        RemoteAgent {
            id: model.id,
            name: model.name,
            sede_id: model.sede_id,
            agent_type,
            ip_address: model.ip_address,
            api_token_hash: model.api_token_hash,
            last_seen: model.last_seen.map(|dt| {
                let timestamp: chrono::DateTime<chrono::Utc> =
                    chrono::DateTime::from_naive_utc_and_offset(dt, chrono::Utc);
                timestamp.to_rfc3339()
            }),
            status,
            metadata,
        }
    }

    /// Convierte un modelo de Sea-ORM a entidad de dominio para TelemetryBatch
    fn model_to_batch(&self, model: agent_metrics_entity::Model) -> TelemetryBatch {
        let metadata = model.metadata.and_then(|s| serde_json::from_str(&s).ok());

        TelemetryBatch {
            id: model.id,
            agent_id: model.agent_id,
            metrics: TelemetryMetrics {
                cpu_usage_percent: model.cpu_usage_percent.and_then(|d| d.to_string().parse().ok()),
                memory_usage_percent: model.memory_usage_percent.and_then(|d| d.to_string().parse().ok()),
                latency_ms: model.latency_ms,
                packet_loss_percent: model.packet_loss_percent.and_then(|d| d.to_string().parse().ok()),
                bandwidth_mbps: model.bandwidth_mbps.and_then(|d| d.to_string().parse().ok()),
                disk_usage_percent: model.disk_usage_percent.and_then(|d| d.to_string().parse().ok()),
                temperature_celsius: model.temperature_celsius.and_then(|d| d.to_string().parse().ok()),
                uptime_seconds: model.uptime_seconds,
            },
            created_at: {
                let timestamp: chrono::DateTime<chrono::Utc> =
                    chrono::DateTime::from_naive_utc_and_offset(model.created_at, chrono::Utc);
                timestamp.to_rfc3339()
            },
            metadata,
        }
    }
}

#[async_trait::async_trait]
impl TelemetryPort for TelemetryRepository {
    /// Registra un agente remoto
    async fn register_agent(&self, agent: RemoteAgent) -> Result<(), domain::errors::DomainError> {
        let metadata_json = agent.metadata.map(|m| serde_json::to_string(&m).unwrap_or_default());

        let new_agent = remote_agent_entity::ActiveModel {
            id: Set(agent.id),
            name: Set(agent.name),
            sede_id: Set(agent.sede_id),
            agent_type: Set(agent.agent_type.to_string()),
            ip_address: Set(agent.ip_address),
            api_token_hash: Set(agent.api_token_hash),
            last_seen: Set(agent.last_seen.and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(&s)
                    .ok()
                    .map(|dt| dt.naive_utc())
            })),
            status: Set(agent.status.to_string()),
            metadata: Set(metadata_json),
        };

        new_agent.insert(&self.db).await.map_err(|e| {
            domain::errors::DomainError::Infrastructure(format!("Error inserting agent: {}", e))
        })?;

        Ok(())
    }

    /// Obtiene un agente por ID
    async fn get_agent(&self, id: String) -> Result<Option<RemoteAgent>, domain::errors::DomainError> {
        let result = remote_agent_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                domain::errors::DomainError::Infrastructure(format!("Error fetching agent: {}", e))
            })?;

        Ok(result.map(|m| self.model_to_agent(m)))
    }

    /// Obtiene un agente por token hash
    async fn get_agent_by_token(
        &self,
        token_hash: String,
    ) -> Result<Option<RemoteAgent>, domain::errors::DomainError> {
        let result = remote_agent_entity::Entity::find()
            .filter(remote_agent_entity::Column::ApiTokenHash.eq(token_hash))
            .one(&self.db)
            .await
            .map_err(|e| {
                domain::errors::DomainError::Infrastructure(format!(
                    "Error fetching agent by token: {}",
                    e
                ))
            })?;

        Ok(result.map(|m| self.model_to_agent(m)))
    }

    /// Actualiza el estado de un agente
    async fn update_agent_status(
        &self,
        id: String,
        status: AgentStatus,
    ) -> Result<(), domain::errors::DomainError> {
        let agent = remote_agent_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                domain::errors::DomainError::Infrastructure(format!("Error fetching agent: {}", e))
            })?;

        if let Some(agent) = agent {
            let mut active_agent: remote_agent_entity::ActiveModel = agent.into();
            active_agent.status = Set(status.to_string());
            active_agent.last_seen = Set(Some(chrono::Utc::now().naive_utc()));
            active_agent.update(&self.db).await.map_err(|e| {
                domain::errors::DomainError::Infrastructure(format!(
                    "Error updating agent status: {}",
                    e
                ))
            })?;
        }

        Ok(())
    }

    /// Registra un lote de métricas
    async fn ingest_metrics(&self, batch: TelemetryBatch) -> Result<(), domain::errors::DomainError> {
        let metadata_json = batch.metadata.map(|m| serde_json::to_string(&m).unwrap_or_default());

        let new_batch = agent_metrics_entity::ActiveModel {
            id: Set(batch.id),
            agent_id: Set(batch.agent_id),
            cpu_usage_percent: Set(batch.metrics.cpu_usage_percent.map(|v| {
                sea_orm::prelude::Decimal::from_f64_retain(v)
                    .unwrap_or_default()
            })),
            memory_usage_percent: Set(batch.metrics.memory_usage_percent.map(|v| {
                sea_orm::prelude::Decimal::from_f64_retain(v)
                    .unwrap_or_default()
            })),
            latency_ms: Set(batch.metrics.latency_ms),
            packet_loss_percent: Set(batch.metrics.packet_loss_percent.map(|v| {
                sea_orm::prelude::Decimal::from_f64_retain(v)
                    .unwrap_or_default()
            })),
            bandwidth_mbps: Set(batch.metrics.bandwidth_mbps.map(|v| {
                sea_orm::prelude::Decimal::from_f64_retain(v)
                    .unwrap_or_default()
            })),
            disk_usage_percent: Set(batch.metrics.disk_usage_percent.map(|v| {
                sea_orm::prelude::Decimal::from_f64_retain(v)
                    .unwrap_or_default()
            })),
            temperature_celsius: Set(batch.metrics.temperature_celsius.map(|v| {
                sea_orm::prelude::Decimal::from_f64_retain(v)
                    .unwrap_or_default()
            })),
            uptime_seconds: Set(batch.metrics.uptime_seconds.map(|v| v as i64)),
            created_at: Set(chrono::Utc::now().naive_utc()),
            metadata: Set(metadata_json),
        };

        new_batch.insert(&self.db).await.map_err(|e| {
            domain::errors::DomainError::Infrastructure(format!("Error inserting metrics: {}", e))
        })?;

        Ok(())
    }

    /// Registra múltiples lotes de métricas (inserción masiva)
    async fn ingest_metrics_batch(
        &self,
        batches: Vec<TelemetryBatch>,
    ) -> Result<(), domain::errors::DomainError> {
        for batch in batches {
            self.ingest_metrics(batch).await?;
        }
        Ok(())
    }

    /// Obtiene métricas de un agente en un rango de tiempo
    async fn get_agent_metrics(
        &self,
        agent_id: String,
        start: String,
        end: String,
    ) -> Result<Vec<TelemetryBatch>, domain::errors::DomainError> {
        let start_dt = chrono::DateTime::parse_from_rfc3339(&start)
            .map_err(|e| {
                domain::errors::DomainError::Infrastructure(format!("Invalid start date: {}", e))
            })?
            .naive_utc();
        let end_dt = chrono::DateTime::parse_from_rfc3339(&end)
            .map_err(|e| {
                domain::errors::DomainError::Infrastructure(format!("Invalid end date: {}", e))
            })?
            .naive_utc();

        let results = agent_metrics_entity::Entity::find()
            .filter(agent_metrics_entity::Column::AgentId.eq(agent_id))
            .filter(agent_metrics_entity::Column::CreatedAt.gte(start_dt))
            .filter(agent_metrics_entity::Column::CreatedAt.lte(end_dt))
            .order_by_desc(agent_metrics_entity::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|e| {
                domain::errors::DomainError::Infrastructure(format!(
                    "Error fetching agent metrics: {}",
                    e
                ))
            })?;

        Ok(results.into_iter().map(|m| self.model_to_batch(m)).collect())
    }

    /// Obtiene métricas recientes de todos los agentes
    async fn get_recent_metrics(
        &self,
        limit: usize,
    ) -> Result<Vec<TelemetryBatch>, domain::errors::DomainError> {
        let results = agent_metrics_entity::Entity::find()
            .order_by_desc(agent_metrics_entity::Column::CreatedAt)
            .limit(limit as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                domain::errors::DomainError::Infrastructure(format!(
                    "Error fetching recent metrics: {}",
                    e
                ))
            })?;

        Ok(results.into_iter().map(|m| self.model_to_batch(m)).collect())
    }
}
