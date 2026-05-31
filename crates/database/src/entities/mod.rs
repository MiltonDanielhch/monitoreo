// crates/database/src/entities/mod.rs
// Módulo de entidades de mapeo relacional Sea-ORM
// Vinculado con ADR-0004-persistencia-mysql-seaorm-docker.md

pub mod active_alert_entity;
pub mod device_entity;
pub mod location_entity;
pub mod notification_channel_entity;
pub mod notification_template_entity;
pub mod notification_log_entity;
pub mod role_entity;
pub mod system_setting_entity;
pub mod user_entity;
pub mod user_session_entity;
pub mod used_refresh_token_entity;

pub use active_alert_entity::Entity as ActiveAlert;
pub use device_entity::Entity as Device;
pub use location_entity::Entity as Location;
pub use notification_channel_entity::Entity as NotificationChannel;
pub use notification_template_entity::Entity as NotificationTemplate;
pub use notification_log_entity::Entity as NotificationLog;
pub use role_entity::Entity as Role;
pub use system_setting_entity::Entity as SystemSetting;
pub use user_entity::Entity as User;
pub use user_session_entity::Entity as UserSession;
pub use used_refresh_token_entity::Entity as UsedRefreshToken;
