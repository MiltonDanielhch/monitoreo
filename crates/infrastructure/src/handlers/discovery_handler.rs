// crates/infrastructure/src/handlers/discovery_handler.rs
// Handlers HTTP para descubrimiento de red y escaneo de activos
// Vinculado con ADR-0003 (Stack Backend Rust Axum) y ADR-0006 (Seguridad)

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::AppState;
use domain::models::discovery::{
    DeviceType, DeviceStatus, DiscoveryFilters, DiscoveredDevice, ScanFilters,
    NetworkScan, ScanStatus, DiscoveryPort,
};
use ipnetwork::IpNetwork;

/// DTO para iniciar un escaneo de red
#[derive(Debug, Deserialize)]
pub struct StartScanRequest {
    pub ip_range: String,       // ej: "192.168.1.0/24"
    pub scan_type: String,      // "full", "partial", "targeted"
    pub ports_to_scan: Option<Vec<u16>>,
    pub timeout_ms: Option<u64>,
    pub max_concurrent: Option<usize>,
    pub sede_id: Option<String>,
}

/// DTO para respuesta de inicio de escaneo
#[derive(Debug, Serialize)]
pub struct StartScanResponse {
    pub scan_id: String,
    pub status: String,
    pub message: String,
}

/// DTO para filtro de dispositivos
#[derive(Debug, Deserialize)]
pub struct DiscoveryFiltersQuery {
    pub device_type: Option<String>,
    pub status: Option<String>,
    pub is_authorized: Option<bool>,
    pub sede_id: Option<String>,
    pub manufacturer: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

impl From<DiscoveryFiltersQuery> for DiscoveryFilters {
    fn from(q: DiscoveryFiltersQuery) -> Self {
        Self {
            device_type: q.device_type.map(|t| DeviceType::from(t)),
            status: q.status.map(|s| DeviceStatus::from(s)),
            is_authorized: q.is_authorized,
            sede_id: q.sede_id,
            manufacturer: q.manufacturer,
            date_from: q.date_from,
            date_to: q.date_to,
        }
    }
}

/// DTO para filtro de escaneos
#[derive(Debug, Deserialize)]
pub struct ScanFiltersQuery {
    pub status: Option<String>,
    pub sede_id: Option<String>,
    pub scan_type: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

impl From<ScanFiltersQuery> for ScanFilters {
    fn from(q: ScanFiltersQuery) -> Self {
        Self {
            status: q.status.map(|s| ScanStatus::from(s)),
            sede_id: q.sede_id,
            scan_type: q.scan_type,
            date_from: q.date_from,
            date_to: q.date_to,
        }
    }
}

/// DTO para respuesta de dispositivo descubierto
#[derive(Debug, Serialize)]
pub struct DiscoveredDeviceResponse {
    pub id: String,
    pub ip_address: String,
    pub mac_address: Option<String>,
    pub hostname: Option<String>,
    pub device_type: String,
    pub os_fingerprint: Option<String>,
    pub manufacturer: Option<String>,
    pub open_ports: Vec<u16>,
    pub services: Vec<String>,
    pub status: String,
    pub is_authorized: bool,
    pub last_seen: String,
    pub first_seen: String,
    pub scan_id: String,
    pub sede_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

impl From<DiscoveredDevice> for DiscoveredDeviceResponse {
    fn from(d: DiscoveredDevice) -> Self {
        Self {
            id: d.id,
            ip_address: d.ip_address,
            mac_address: d.mac_address,
            hostname: d.hostname,
            device_type: d.device_type.to_string(),
            os_fingerprint: d.os_fingerprint,
            manufacturer: d.manufacturer,
            open_ports: d.open_ports,
            services: d.services,
            status: d.status.to_string(),
            is_authorized: d.is_authorized,
            last_seen: d.last_seen,
            first_seen: d.first_seen,
            scan_id: d.scan_id,
            sede_id: d.sede_id,
            metadata: d.metadata,
        }
    }
}

/// DTO para respuesta de escaneo de red
#[derive(Debug, Serialize)]
pub struct NetworkScanResponse {
    pub id: String,
    pub scan_type: String,
    pub ip_range: String,
    pub status: String,
    pub devices_found: i32,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub duration_seconds: Option<i32>,
    pub sede_id: Option<String>,
    pub created_by: String,
}

impl From<NetworkScan> for NetworkScanResponse {
    fn from(s: NetworkScan) -> Self {
        Self {
            id: s.id,
            scan_type: s.scan_type,
            ip_range: s.ip_range,
            status: s.status.to_string(),
            devices_found: s.devices_found,
            started_at: s.started_at,
            completed_at: s.completed_at,
            duration_seconds: s.duration_seconds,
            sede_id: s.sede_id,
            created_by: s.created_by,
        }
    }
}

/// DTO para respuesta de progreso de escaneo
#[derive(Debug, Serialize)]
pub struct ScanProgressResponse {
    pub scan_id: String,
    pub total_ips: usize,
    pub scanned_ips: usize,
    pub devices_found: usize,
    pub percentage: f64,
}

/// DTO para respuesta de error
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// DTO para respuesta de éxito
#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
}

/// DTO para respuesta paginada de dispositivos
#[derive(Debug, Serialize)]
pub struct DevicesPaginatedResponse {
    pub devices: Vec<DiscoveredDeviceResponse>,
    pub total: usize,
}

/// DTO para respuesta paginada de escaneos
#[derive(Debug, Serialize)]
pub struct ScansPaginatedResponse {
    pub scans: Vec<NetworkScanResponse>,
    pub total: usize,
}

/// Handler para iniciar un escaneo de red
/// POST /api/v1/discovery/scan
pub async fn start_network_scan(
    State(_state): State<AppState>,
    Json(request): Json<StartScanRequest>,
) -> Result<Json<StartScanResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validar rango de IPs
    if request.ip_range.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "El rango de IPs no puede estar vacío".to_string(),
            }),
        ));
    }

    // Validar formato CIDR
    match request.ip_range.parse::<IpNetwork>() {
        Ok(_) => {}
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Rango de IP inválido: {}", request.ip_range),
                }),
            ));
        }
    }

    // Validar tipo de escaneo
    let valid_scan_types = ["full", "partial", "targeted"];
    if !valid_scan_types.contains(&request.scan_type.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!(
                    "Tipo de escaneo inválido: {}. Válidos: full, partial, targeted",
                    request.scan_type
                ),
            }),
        ));
    }

    // TODO: Extraer user_id del token de autorización (luego de implementar auth middleware)
    let created_by = "system".to_string();

    // Crear la configuración del escaneo (usado cuando se conecte al ScanEngine)
    let _scan_config = crate::discovery::ScanConfig {
        ip_range: request.ip_range.clone(),
        scan_type: request.scan_type.clone(),
        ports_to_scan: request.ports_to_scan.unwrap_or_default(),
        timeout_ms: request.timeout_ms.unwrap_or(1000),
        max_concurrent: request.max_concurrent.unwrap_or(100),
        sede_id: request.sede_id.clone(),
        created_by,
    };

    // Encolar el escaneo (el ScanEngine maneja la ejecución asíncrona)
    // Por ahora, retornamos el ID del escaneo
    let scan_id = format!("scan_{}", chrono::Utc::now().timestamp_millis());

    Ok(Json(StartScanResponse {
        scan_id,
        status: "pending".to_string(),
        message: "Escaneo enqueued exitosamente".to_string(),
    }))
}

/// Handler para obtener dispositivos descubiertos
/// GET /api/v1/discovery/devices
pub async fn get_discovered_devices(
    State(state): State<AppState>,
    Query(filters): Query<DiscoveryFiltersQuery>,
) -> Result<Json<DevicesPaginatedResponse>, (StatusCode, Json<ErrorResponse>)> {
    let discovery_repo = state.discovery_repo.clone();

    let domain_filters: DiscoveryFilters = filters.into();

    let devices = discovery_repo
        .get_devices(domain_filters)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Error al obtener dispositivos: {}", e),
                }),
            )
        })?;

    let total = devices.len();
    let device_responses: Vec<DiscoveredDeviceResponse> =
        devices.into_iter().map(DiscoveredDeviceResponse::from).collect();

    Ok(Json(DevicesPaginatedResponse {
        devices: device_responses,
        total,
    }))
}

/// Handler para obtener un dispositivo por IP
/// GET /api/v1/discovery/devices/ip/{ip}
pub async fn get_device_by_ip(
    State(state): State<AppState>,
    Path(ip): Path<String>,
) -> Result<Json<DiscoveredDeviceResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validar formato de IP
    if let Err(e) = DiscoveredDevice::validate_ip(&ip) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: e }),
        ));
    }

    let discovery_repo = state.discovery_repo.clone();

    let device = discovery_repo
        .get_device_by_ip(ip.clone())
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Error al buscar dispositivo: {}", e),
                }),
            )
        })?;

    match device {
        Some(d) => Ok(Json(DiscoveredDeviceResponse::from(d))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Dispositivo con IP {} no encontrado", ip),
            }),
        )),
    }
}

/// Handler para marcar dispositivo como autorizado
/// PUT /api/v1/discovery/devices/{id}/authorize
pub async fn mark_device_authorized(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let discovery_repo = state.discovery_repo.clone();

    discovery_repo
        .mark_device_authorized(id.clone())
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Error al autorizar dispositivo: {}", e),
                }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: format!("Dispositivo {} marcado como autorizado", id),
    }))
}

/// Handler para marcar dispositivo como no autorizado
/// PUT /api/v1/discovery/devices/{id}/unauthorize
pub async fn mark_device_unauthorized(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let discovery_repo = state.discovery_repo.clone();

    discovery_repo
        .mark_device_unauthorized(id.clone())
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Error al desautorizar dispositivo: {}", e),
                }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: format!("Dispositivo {} marcado como no autorizado", id),
    }))
}

/// Handler para obtener escaneos de red
/// GET /api/v1/discovery/scans
pub async fn get_network_scans(
    State(state): State<AppState>,
    Query(filters): Query<ScanFiltersQuery>,
) -> Result<Json<ScansPaginatedResponse>, (StatusCode, Json<ErrorResponse>)> {
    let discovery_repo = state.discovery_repo.clone();

    let domain_filters: ScanFilters = filters.into();

    let scans = discovery_repo
        .get_scans(domain_filters)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Error al obtener escaneos: {}", e),
                }),
            )
        })?;

    let total = scans.len();
    let scan_responses: Vec<NetworkScanResponse> =
        scans.into_iter().map(NetworkScanResponse::from).collect();

    Ok(Json(ScansPaginatedResponse {
        scans: scan_responses,
        total,
    }))
}

/// Handler para obtener un escaneo específico
/// GET /api/v1/discovery/scans/{id}
pub async fn get_network_scan_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<NetworkScanResponse>, (StatusCode, Json<ErrorResponse>)> {
    let discovery_repo = state.discovery_repo.clone();

    let scan = discovery_repo
        .get_scan(id.clone())
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Error al buscar escaneo: {}", e),
                }),
            )
        })?;

    match scan {
        Some(s) => Ok(Json(NetworkScanResponse::from(s))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Escaneo con ID {} no encontrado", id),
            }),
        )),
    }
}

/// Handler para obtener progreso de un escaneo
/// GET /api/v1/discovery/scan/{id}/progress
pub async fn get_scan_progress(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ScanProgressResponse>, (StatusCode, Json<ErrorResponse>)> {
    // El progreso real se obtendrá del ScanEngine via canal
    // Por ahora retornamos un estado placeholder
    Ok(Json(ScanProgressResponse {
        scan_id: id,
        total_ips: 0,
        scanned_ips: 0,
        devices_found: 0,
        percentage: 0.0,
    }))
}
