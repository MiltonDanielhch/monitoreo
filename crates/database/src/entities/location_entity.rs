// crates/database/src/entities/location_entity.rs
// Entidad de mapeo relacional para tabla locations
// Vinculado con ADR-0004-persistencia-mysql-seaorm-docker.md

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "locations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    #[sea_orm(unique)]
    pub code: String,
    pub region: String,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub is_active: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}