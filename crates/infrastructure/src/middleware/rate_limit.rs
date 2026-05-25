// crates/infrastructure/src/middleware/rate_limit.rs
// Middleware de Rate Limiting con axum-governor
// Vinculado con ADR-0009-rate-limiting.md

use axum_governor::{GovernorConfigBuilder, GovernorLayer, Quota, nz, extractor::PeerIp};
use tower::Layer;

/// Configuración de Rate Limiting para endpoints de autenticación
pub fn auth_rate_limit() -> impl Layer<axum::routing::Route> {
    // 10 peticiones cada 5 minutos (protección anti fuerza bruta)
    let cfg = GovernorConfigBuilder::default()
        .with_extractor(PeerIp::default())
        .expect_connect_info()
        .quota_default(Quota::requests_per_minute(nz!(10u32)))
        .finish()
        .unwrap();

    GovernorLayer::new(cfg)
}

/// Configuración de Rate Limiting para endpoints de refresh
pub fn refresh_rate_limit() -> impl Layer<axum::routing::Route> {
    // 20 peticiones cada 1 minuto (soporte para alta concurrencia legítima)
    let cfg = GovernorConfigBuilder::default()
        .with_extractor(PeerIp::default())
        .expect_connect_info()
        .quota_default(Quota::requests_per_second(nz!(20u32)))
        .finish()
        .unwrap();

    GovernorLayer::new(cfg)
}

/// Configuración de Rate Limiting para API general
pub fn api_rate_limit() -> impl Layer<axum::routing::Route> {
    // 30 peticiones cada 60 segundos
    let cfg = GovernorConfigBuilder::default()
        .with_extractor(PeerIp::default())
        .expect_connect_info()
        .quota_default(Quota::requests_per_minute(nz!(30u32)))
        .finish()
        .unwrap();

    GovernorLayer::new(cfg)
}
