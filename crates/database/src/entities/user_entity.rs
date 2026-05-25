// crates/database/src/entities/user_entity.rs
// Entidad de mapeo relacional para tabla users
// Vinculado con ADR-0004-persistencia-mysql-seaorm-docker.md

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub role_id: String,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::role_entity::Entity",
        from = "Column::RoleId",
        to = "super::role_entity::Column::Id"
    )]
    Role,
}

impl ActiveModelBehavior for ActiveModel {}
