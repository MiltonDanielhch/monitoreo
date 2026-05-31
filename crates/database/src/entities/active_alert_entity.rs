// crates/database/src/entities/active_alert_entity.rs
// Entidad de mapeo relacional para tabla active_alerts
// Vinculado con ADR-0004-persistencia-mysql-seaorm-docker.md

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "active_alerts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub device_id: Option<String>,
    pub location_id: Option<String>,
    pub severity: String,
    pub title: String,
    pub description: Option<String>,
    #[sea_orm(column_name = "metric_name")]
    pub metric_name: Option<String>,
    #[sea_orm(column_name = "metric_value")]
    pub metric_value: Option<Decimal>,
    #[sea_orm(column_name = "threshold_value")]
    pub threshold_value: Option<Decimal>,
    #[sea_orm(column_name = "is_acknowledged")]
    pub is_acknowledged: bool,
    #[sea_orm(column_name = "acknowledged_by")]
    pub acknowledged_by: Option<String>,
    #[sea_orm(column_name = "acknowledged_at")]
    pub acknowledged_at: Option<DateTime>,
    #[sea_orm(column_name = "resolved_at")]
    pub resolved_at: Option<DateTime>,
    #[sea_orm(column_name = "created_at")]
    pub created_at: DateTime,
    #[sea_orm(column_name = "updated_at")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}