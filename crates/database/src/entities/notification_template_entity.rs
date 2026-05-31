// crates/database/src/entities/notification_template_entity.rs
// Entidad de mapeo relacional para tabla notification_templates
// Vinculado con ADR-0004-persistencia-mysql-seaorm-docker.md
// Módulo 4: Motor de Notificaciones

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "notification_templates")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    pub template_type: String, // ENUM en MySQL, String en Sea-ORM
    pub subject: String,
    pub body: String,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
