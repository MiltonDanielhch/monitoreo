// crates/database/src/entities/notification_log_entity.rs
// Entidad de mapeo relacional para tabla notification_logs
// Vinculado con ADR-0004-persistencia-mysql-seaorm-docker.md
// Módulo 4: Motor de Notificaciones

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "notification_logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub channel_id: String,
    pub template_id: String,
    pub recipient: String,
    pub status: String, // ENUM en MySQL, String en Sea-ORM
    pub attempt_count: i32,
    pub max_attempts: i32,
    pub error_message: Option<String>,
    pub sent_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::notification_channel_entity::Entity",
        from = "Column::ChannelId",
        to = "super::notification_channel_entity::Column::Id"
    )]
    Channel,
    #[sea_orm(
        belongs_to = "super::notification_template_entity::Entity",
        from = "Column::TemplateId",
        to = "super::notification_template_entity::Column::Id"
    )]
    Template,
}

impl ActiveModelBehavior for ActiveModel {}
