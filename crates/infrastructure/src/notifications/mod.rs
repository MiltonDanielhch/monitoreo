// crates/infrastructure/src/notifications/mod.rs
// Módulo de notificaciones - Adaptadores y Workers
// Vinculado con ADR-0014-monitoreo-tareas-criticas.md y ADR-0015-tokio-jobs.md
// Módulo 4: Motor de Notificaciones

pub mod smtp_adapter;
pub mod worker;

pub use smtp_adapter::{SmtpAdapter, SmtpConfig};
pub use worker::NotificationWorker;
