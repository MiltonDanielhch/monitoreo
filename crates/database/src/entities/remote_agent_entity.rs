// crates/database/src/entities/remote_agent_entity.rs
// Entidad Sea-ORM para agentes remotos
// Vinculado con ADR-0004-persistencia-sea-orm.md

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "remote_agents")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    pub sede_id: String,
    pub agent_type: String,
    pub ip_address: String,
    pub api_token_hash: String,
    pub last_seen: Option<DateTime>,
    pub status: String,
    pub metadata: Option<String>, // JSON como string
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
