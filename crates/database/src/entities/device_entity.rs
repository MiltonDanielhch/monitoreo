// crates/database/src/entities/device_entity.rs
// Entidad de mapeo relacional para tabla devices
// Vinculado con ADR-0004-persistencia-mysql-seaorm-docker.md

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "devices")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub location_id: Option<String>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    #[sea_orm(column_name = "bandwidth_gbps")]
    pub bandwidth_gbps: Option<Decimal>,
    pub status: String,
    #[sea_orm(column_name = "is_active")]
    pub is_active: bool,
    #[sea_orm(column_name = "last_seen")]
    pub last_seen: Option<DateTime>,
    #[sea_orm(column_name = "created_at")]
    pub created_at: DateTime,
    #[sea_orm(column_name = "updated_at")]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}