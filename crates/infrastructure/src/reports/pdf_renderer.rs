// crates/infrastructure/src/reports/pdf_renderer.rs
// Adaptador de renderizado PDF nativo
// Vinculado con ADR-0012 (Renderizado de Documentos)
// NOTA: Implementación stub - se puede mejorar con librería PDF específica

use domain::models::report::SlaReport;

/// Renderer de PDF para reportes SLA
pub struct PdfRenderer {
    /// Título del documento
    title: String,
    /// Nombre de la organización
    organization: String,
    /// Logo (opcional, en base64)
    logo: Option<String>,
}

impl PdfRenderer {
    /// Crear un nuevo renderer de PDF
    pub fn new(title: String, organization: String) -> Self {
        Self {
            title,
            organization,
            logo: None,
        }
    }

    /// Establecer el logo (en base64)
    pub fn with_logo(mut self, logo: String) -> Self {
        self.logo = Some(logo);
        self
    }

    /// Generar PDF a partir de un reporte SLA
    /// NOTA: Implementación stub - genera un PDF básico
    pub fn generate_sla_report(&self, report: &SlaReport) -> Result<Vec<u8>, String> {
        // Implementación stub - genera un PDF básico
        // En producción, usar librería como genpdf, printpdf o lopdf
        let pdf_content = format!(
            "PDF REPORT - {}\n\n\
             {}\n\
             GOBERNACIÓN DEL BENI\n\
             Sistema de Monitoreo de Infraestructura\n\n\
             REPORTE MENSUAL DE SLA\n\n\
             Sede: {}\n\
             Periodo: {} {}\n\
             Fecha de generación: {}\n\
             Generado por: {}\n\n\
             MÉTRICAS DE DISPONIBILIDAD\n\n\
             Objetivo SLA: {:.2}%\n\
             SLA Alcanzado: {:.2}%\n\
             Estado: {}\n\n\
             APROBACIÓN Y FIRMAS\n\n\
             Director de TI\n\
             Administrador de Red\n\
             Jefe de Operaciones\n\n\
             Fecha: {}",
            self.title,
            self.organization,
            report.sede_name,
            self.get_month_name(&report.month),
            report.year,
            report.generated_at,
            report.generated_by,
            report.availability_target,
            report.availability_achieved,
            self.format_status(&report.status),
            report.generated_at
        );

        // Retornar contenido como bytes (stub)
        Ok(pdf_content.into_bytes())
    }

    /// Formatear estado del SLA
    fn format_status(&self, status: &domain::models::report::SlaStatus) -> String {
        match status {
            domain::models::report::SlaStatus::Compliant => "CUMPLIDO".to_string(),
            domain::models::report::SlaStatus::Breached => "INCUMPLIDO".to_string(),
            domain::models::report::SlaStatus::AtRisk => "EN RIESGO".to_string(),
        }
    }

    /// Obtener nombre del mes
    fn get_month_name(&self, month: &str) -> String {
        match month {
            "01" => "Enero",
            "02" => "Febrero",
            "03" => "Marzo",
            "04" => "Abril",
            "05" => "Mayo",
            "06" => "Junio",
            "07" => "Julio",
            "08" => "Agosto",
            "09" => "Septiembre",
            "10" => "Octubre",
            "11" => "Noviembre",
            "12" => "Diciembre",
            _ => month,
        }.to_string()
    }
}

impl Default for PdfRenderer {
    fn default() -> Self {
        Self::new(
            "Reporte SLA".to_string(),
            "GOBERNACIÓN DEL BENI".to_string(),
        )
    }
}
