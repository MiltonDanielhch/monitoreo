// crates/database/src/entities/system_setting_entity.rs
// Entidad de mapeo relacional para tabla system_settings
// Vinculado con ADR-0004-persistencia-mysql-seaorm-docker.md

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "system_settings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    #[sea_orm(unique)]
    pub key_name: String,
    pub value: String,
    pub value_type: String,
    pub category: String,
    pub description: Option<String>,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}