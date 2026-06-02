// crates/database/src/entities/discovered_device_entity.rs
// Entidad Sea-ORM para dispositivos descubiertos en la red
// Vinculado con ADR-0004 (Persistencia con Sea-ORM)

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "discovered_devices")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub ip_address: String,
    pub mac_address: Option<String>,
    pub hostname: Option<String>,
    pub device_type: String,
    pub os_fingerprint: Option<String>,
    pub manufacturer: Option<String>,
    pub open_ports: Option<String>, // JSON como string
    pub services: Option<String>,   // JSON como string
    pub status: String,
    pub is_authorized: bool,
    pub last_seen: DateTime,
    pub first_seen: DateTime,
    pub scan_id: Option<String>,
    pub sede_id: Option<String>,
    pub metadata: Option<String>, // JSON como string
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
