// crates/infrastructure/src/handlers/report_handler.rs
// Handlers HTTP para generación y descarga de reportes SLA
// Vinculado con ADR-0003 (Stack Backend Rust Axum) y ADR-0013 (Streaming)

use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use domain::models::report::SlaReport;
use crate::reports::{PdfRenderer, CryptoSigner};
use crate::AppState;

/// Parámetros de query para generación de reporte SLA
#[derive(Debug, Deserialize)]
pub struct GenerateSlaReportQuery {
    /// ID de la sede
    pub sede_id: String,
    /// Mes (formato MM)
    pub month: String,
    /// Año
    pub year: i32,
}

/// Respuesta de generación de reporte
#[derive(Debug, Serialize)]
pub struct GenerateReportResponse {
    /// ID del reporte generado
    pub report_id: String,
    /// Hash SHA-256 del documento
    pub document_hash: String,
    /// Timestamp de generación
    pub generated_at: String,
}

/// Generar reporte SLA mensual
pub async fn generate_sla_report(
    State(state): State<AppState>,
    Query(params): Query<GenerateSlaReportQuery>,
) -> Result<Json<GenerateReportResponse>, (StatusCode, String)> {
    // Obtener datos de SLA del repositorio
    let sla_data = state.report_repo
        .get_sla_by_sede_month(&params.sede_id, &params.month)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error al obtener datos SLA: {}", e)))?;

    let sla_data = sla_data.ok_or_else(|| {
        (StatusCode::NOT_FOUND, "No se encontraron datos SLA para el periodo especificado".to_string())
    })?;

    // Crear reporte de dominio
    let report = SlaReport::new(
        params.sede_id.clone(),
        format!("Sede {}", params.sede_id),
        params.month.clone(),
        params.year,
        99.5, // Objetivo SLA por defecto
        sla_data.sla_percentage,
        sla_data.estimated_uptime_minutes,
        sla_data.estimated_downtime_minutes,
        sla_data.total_events,
        sla_data.avg_latency_ms.unwrap_or(0.0),
        sla_data.avg_bandwidth_mbps.unwrap_or(0.0),
        0, // Incident count (se puede calcular de otra fuente)
        "system".to_string(),
    );

    // Generar PDF
    let renderer = PdfRenderer::default();
    let pdf_bytes = renderer
        .generate_sla_report(&report)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error al generar PDF: {}", e)))?;

    // Firmar documento criptográficamente
    let signer = CryptoSigner::default();
    let signature = signer.sign_document(&pdf_bytes, "system".to_string());

    // Inyectar metadatos
    let signed_document = signer
        .inject_metadata(&pdf_bytes, &signature)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error al inyectar metadatos: {}", e)))?;

    // Aquí se guardaría el documento en almacenamiento
    // Por ahora, retornamos la información de la firma

    Ok(Json(GenerateReportResponse {
        report_id: report.id,
        document_hash: signature.document_hash,
        generated_at: report.generated_at,
    }))
}

/// Descargar reporte SLA como PDF
pub async fn download_sla_report(
    State(state): State<AppState>,
    Query(params): Query<GenerateSlaReportQuery>,
) -> Result<Response, (StatusCode, String)> {
    // Obtener datos de SLA del repositorio
    let sla_data = state.report_repo
        .get_sla_by_sede_month(&params.sede_id, &params.month)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error al obtener datos SLA: {}", e)))?;

    let sla_data = sla_data.ok_or_else(|| {
        (StatusCode::NOT_FOUND, "No se encontraron datos SLA para el periodo especificado".to_string())
    })?;

    // Crear reporte de dominio
    let report = SlaReport::new(
        params.sede_id.clone(),
        format!("Sede {}", params.sede_id),
        params.month.clone(),
        params.year,
        99.5,
        sla_data.sla_percentage,
        sla_data.estimated_uptime_minutes,
        sla_data.estimated_downtime_minutes,
        sla_data.total_events,
        sla_data.avg_latency_ms.unwrap_or(0.0),
        sla_data.avg_bandwidth_mbps.unwrap_or(0.0),
        0,
        "system".to_string(),
    );

    // Generar PDF
    let renderer = PdfRenderer::default();
    let pdf_bytes = renderer
        .generate_sla_report(&report)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error al generar PDF: {}", e)))?;

    // Firmar documento
    let signer = CryptoSigner::default();
    let signature = signer.sign_document(&pdf_bytes, "system".to_string());

    // Inyectar metadatos
    let signed_document = signer
        .inject_metadata(&pdf_bytes, &signature)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error al inyectar metadatos: {}", e)))?;

    // Configurar cabeceras para descarga
    let filename = format!("sla_report_{}_{}_{}.pdf", params.sede_id, params.month, params.year);

    Ok((
        [
            (header::CONTENT_TYPE, "application/pdf"),
            (header::CONTENT_DISPOSITION, &format!("attachment; filename=\"{}\"", filename)),
            (header::CONTENT_LENGTH, signed_document.len().to_string().as_str()),
        ],
        signed_document,
    )
        .into_response())
}

/// Obtener resumen de SLA para todas las sedes
pub async fn get_sla_summary(
    State(state): State<AppState>,
    Query(params): Query<GenerateSlaReportQuery>,
) -> Result<Json<Vec<SlaReport>>, (StatusCode, String)> {
    let sla_data = state.report_repo
        .get_sla_all_sedes_month(&params.month)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error al obtener resumen SLA: {}", e)))?;

    let reports: Vec<SlaReport> = sla_data
        .into_iter()
        .map(|data| {
            SlaReport::new(
                data.sede_id.clone(),
                format!("Sede {}", data.sede_id),
                params.month.clone(),
                params.year,
                99.5,
                data.sla_percentage,
                data.estimated_uptime_minutes,
                data.estimated_downtime_minutes,
                data.total_events,
                data.avg_latency_ms.unwrap_or(0.0),
                data.avg_bandwidth_mbps.unwrap_or(0.0),
                0,
                "system".to_string(),
            )
        })
        .collect();

    Ok(Json(reports))
}
