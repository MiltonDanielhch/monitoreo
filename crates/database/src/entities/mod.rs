// crates/database/src/entities/mod.rs
// Módulo de entidades de mapeo relacional Sea-ORM
// Vinculado con ADR-0004-persistencia-mysql-seaorm-docker.md

pub mod active_alert_entity;
pub mod agent_metrics_entity;
pub mod audit_entity;
pub mod device_entity;
pub mod discovered_device_entity;
pub mod location_entity;
pub mod network_file_entity;
pub mod network_scan_entity;
pub mod notification_channel_entity;
pub mod notification_template_entity;
pub mod notification_log_entity;
pub mod remote_agent_entity;
pub mod role_entity;
pub mod security_event_entity;
pub mod system_setting_entity;
pub mod user_entity;
pub mod user_session_entity;
pub mod used_refresh_token_entity;

pub use active_alert_entity::Entity as ActiveAlert;
pub use agent_metrics_entity::Entity as AgentMetrics;
pub use audit_entity::Entity as AuditTrail;
pub use device_entity::Entity as Device;
pub use discovered_device_entity::Entity as DiscoveredDevice;
pub use location_entity::Entity as Location;
pub use network_file_entity::Entity as NetworkFile;
pub use network_scan_entity::Entity as NetworkScan;
pub use notification_channel_entity::Entity as NotificationChannel;
pub use notification_template_entity::Entity as NotificationTemplate;
pub use notification_log_entity::Entity as NotificationLog;
pub use remote_agent_entity::Entity as RemoteAgent;
pub use role_entity::Entity as Role;
pub use security_event_entity::Entity as SecurityEvent;
pub use system_setting_entity::Entity as SystemSetting;
pub use user_entity::Entity as User;
pub use user_session_entity::Entity as UserSession;
pub use used_refresh_token_entity::Entity as UsedRefreshToken;
