// crates/database/src/entities/network_file_entity.rs
// Entidad Sea-ORM para archivos de infraestructura de red
// Vinculado con ADR-0004-persistencia-sea-orm.md

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "network_files")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub filename: String,
    pub file_type: String, // ENUM: TOPOLOGY_SVG, RACK_IMAGE, CONFIG_BACKUP
    pub file_size_bytes: i64,
    pub storage_key: String,
    pub sha256_checksum: String,
    pub sede_id: String,
    pub user_id: Option<String>,
    pub created_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
