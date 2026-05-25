// apps/api/src/main.rs
// Binario ejecutable - Orquestador y arranque del backend
// Vinculado con ADR-0003-stack-backend-rust-axum.md y ADR-0007-manejo-errores.md

// Este es el punto de entrada principal del servidor Axum
// Realiza la inyección de dependencias y el arranque del runtime Tokio

use database::establish_connection;
use database::AuthRepository;
use infrastructure::{create_router, AppState};
use tokio::net::TcpListener;
use std::env;
use std::sync::Arc;
use secrecy::SecretString;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configurar logging en consola y archivo
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "debug".to_string());
    
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_target(true)
                .with_level(true)
                .with_thread_ids(true)
        )
        .with(tracing_subscriber::EnvFilter::new(log_level))
        .init();

    // Fallback seguro en caso de requerir valores por defecto nativos
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:Milton123@127.0.0.1:3306/redes_dev".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());

    tracing::info!("Conectando a base de datos...");
    let db_connection = establish_connection(&database_url).await
        .map_err(|e| anyhow::anyhow!("Fallo crítico al enlazar MySQL local: {}", e))?;

    tracing::info!("Inicializando repositorio de autenticación...");
    let auth_repo = Arc::new(AuthRepository::new(db_connection.clone()));
    let paseto_secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default-secret-32-bytes-long-12345678".to_string());
    
    let state = AppState { 
        db: db_connection,
        auth_repo,
        paseto_secret: SecretString::new(paseto_secret.into()),
    };
    
    tracing::info!("Creando router Axum...");
    let app = create_router(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Servidor Axum escuchando activamente en http://{}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
