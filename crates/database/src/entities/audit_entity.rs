// crates/database/src/entities/audit_entity.rs
// Entidad Sea-ORM para auditoría inmutable
// Vinculado con ADR-0004-persistencia-sea-orm.md

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "audit_trail")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub timestamp: DateTime,
    pub user_id: Option<String>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<String>,
    pub old_value: Option<String>, // JSON como string
    pub new_value: Option<String>, // JSON como string
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<String>, // JSON como string
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
