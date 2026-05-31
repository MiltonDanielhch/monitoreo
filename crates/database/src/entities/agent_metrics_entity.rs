// crates/database/src/entities/agent_metrics_entity.rs
// Entidad Sea-ORM para métricas de agentes
// Vinculado con ADR-0004-persistencia-sea-orm.md

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "agent_metrics_batch")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub agent_id: String,
    pub cpu_usage_percent: Option<Decimal>,
    pub memory_usage_percent: Option<Decimal>,
    pub latency_ms: Option<i32>,
    pub packet_loss_percent: Option<Decimal>,
    pub bandwidth_mbps: Option<Decimal>,
    pub disk_usage_percent: Option<Decimal>,
    pub temperature_celsius: Option<Decimal>,
    pub uptime_seconds: Option<i64>,
    pub created_at: DateTime,
    pub metadata: Option<String>, // JSON como string
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
