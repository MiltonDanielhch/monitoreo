// crates/database/src/lib.rs
// Adaptador de persistencia con Sea-ORM + MySQL
// Vinculado con ADR-0004-persistencia-mysql-seaorm-docker.md

// Este crate contiene la implementación de repositorios y acceso a datos
// Todas las operaciones de base de datos deben ocurrir estrictamente aquí

use sea_orm::{Database, DbErr};
use std::time::Duration;

pub mod entities;
pub mod repositories;

pub use repositories::AuthRepository;
pub use repositories::AuditRepository;
pub use repositories::DashboardRepository;
pub use repositories::NetworkFileRepository;
pub use repositories::NotificationRepository;
pub use repositories::SettingsRepository;

pub async fn establish_connection(db_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = sea_orm::ConnectOptions::new(db_url.to_owned());

    // Sintonía fina Código 3026: Reutilización de canales sin saturar el motor nativo
    opt.max_connections(10)
       .min_connections(2)
       .connect_timeout(Duration::from_secs(8))
       .idle_timeout(Duration::from_secs(8));

    let db = Database::connect(opt).await?;
    tracing::info!("Conexión exitosa a la instancia local de MySQL vía Workbench.");
    Ok(db)
}

// Reexportar DatabaseConnection para uso en infrastructure
pub use sea_orm::DatabaseConnection;
