// crates/database/src/entities/used_refresh_token_entity.rs
// Entidad de mapeo relacional para tabla used_refresh_tokens (RTR replay protection)
// Vinculado con ADR-0006-rbac-sessions-audit.md

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "used_refresh_tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    #[sea_orm(unique)]
    pub token_hash: String,
    pub user_id: String,
    pub used_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}