// crates/database/src/entities/network_scan_entity.rs
// Entidad Sea-ORM para registros de escaneos de red
// Vinculado con ADR-0004 (Persistencia con Sea-ORM)

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "network_scans")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub scan_type: String,
    pub ip_range: String,
    pub status: String,
    pub devices_found: i32,
    pub started_at: DateTime,
    pub completed_at: Option<DateTime>,
    pub duration_seconds: Option<i32>,
    pub sede_id: Option<String>,
    pub created_by: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
