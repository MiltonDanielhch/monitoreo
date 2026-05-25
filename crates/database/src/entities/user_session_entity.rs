// crates/database/src/entities/user_session_entity.rs
// Entidad de mapeo relacional para tabla user_sessions
// Vinculado con ADR-0004-persistencia-mysql-seaorm-docker.md

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_sessions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub user_id: String,
    pub refresh_token_hash: String,
    pub expires_at: DateTime,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user_entity::Entity",
        from = "Column::UserId",
        to = "super::user_entity::Column::Id"
    )]
    User,
}

impl ActiveModelBehavior for ActiveModel {}
