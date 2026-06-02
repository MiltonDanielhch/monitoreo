// crates/database/src/entities/security_event_entity.rs
// Entidad Sea-ORM para eventos de seguridad
// Vinculado con ADR-0004 (Persistencia con Sea-ORM)

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "security_events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub event_type: String,
    pub severity: String,
    pub status: String,
    pub source_ip: String,
    pub source_mac: Option<String>,
    pub target_device_id: Option<String>,
    pub target_sede_id: Option<String>,
    pub description: String,
    pub metadata: Option<String>, // JSON como string
    pub detected_at: DateTime,
    pub resolved_at: Option<DateTime>,
    pub resolved_by: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
