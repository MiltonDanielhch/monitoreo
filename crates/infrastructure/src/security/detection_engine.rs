// crates/infrastructure/src/security/detection_engine.rs
// Motor de detección de patrones anómalos usando Tokio Channels
// Vinculado con ADR-0015 (Tokio Jobs)

use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use domain::models::security::{SecurityEvent, IntrusionType, Severity, SecurityPort};

/// Mensajes para el motor de detección
#[derive(Debug, Clone)]
pub enum DetectionMessage {
    AnalyzeTraffic(TrafficData),
    CheckDevice(DeviceData),
    Shutdown,
}

/// Datos de tráfico de red
#[derive(Debug, Clone)]
pub struct TrafficData {
    pub source_ip: String,
    pub source_mac: Option<String>,
    pub target_device_id: Option<String>,
    pub target_sede_id: Option<String>,
    pub port: u16,
    pub bytes: u64,
    pub timestamp: Instant,
}

/// Datos de dispositivo
#[derive(Debug, Clone)]
pub struct DeviceData {
    pub device_id: String,
    pub sede_id: Option<String>,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub bandwidth_usage: f64,
    pub timestamp: Instant,
}

/// Evento correlacionado
#[derive(Debug, Clone)]
pub struct CorrelatedEvent {
    pub primary_event: SecurityEvent,
    pub related_events: Vec<SecurityEvent>,
    pub correlation_type: CorrelationType,
    pub confidence: f64,
}

/// Tipos de correlación
#[derive(Debug, Clone, PartialEq)]
pub enum CorrelationType {
    /// Múltiples escaneos de puertos desde la misma IP
    PortScanSequence,
    /// DDoS distribuido desde múltiples IPs
    DistributedDDoS,
    /// Exfiltración de datos a través de múltiples eventos
    DataExfiltrationSequence,
    /// Acceso no autorizado seguido de actividad anómala
    UnauthorizedAccessFollowedByAnomaly,
    /// Patrón de malware detectado
    MalwarePattern,
}

/// Motor de detección de patrones anómalos
pub struct DetectionEngine<T: SecurityPort + Send + Sync> {
    tx: mpsc::Sender<DetectionMessage>,
    _handle: JoinHandle<()>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: SecurityPort + Send + Sync + 'static> DetectionEngine<T> {
    /// Crea un nuevo motor de detección
    pub fn new(security_repo: Arc<T>) -> Self {
        let (tx, mut rx) = mpsc::channel(1000);

        let handle = tokio::spawn(async move {
            // Contadores para detección de patrones
            let mut port_scan_tracker: HashMap<String, Vec<u16>> = HashMap::new();
            let mut connection_tracker: HashMap<String, u64> = HashMap::new();
            let mut ddos_tracker: HashMap<String, u64> = HashMap::new();

            // Buffers para correlación de eventos
            let mut recent_events: Vec<SecurityEvent> = Vec::new();
            let max_buffer_size = 100;

            while let Some(message) = rx.recv().await {
                match message {
                    DetectionMessage::AnalyzeTraffic(traffic) => {
                        // Detectar escaneo de puertos (múltiples puertos desde misma IP)
                        let ports = port_scan_tracker.entry(traffic.source_ip.clone()).or_insert_with(Vec::new);
                        if !ports.contains(&traffic.port) {
                            ports.push(traffic.port);
                        }

                        // Si más de 20 puertos diferentes en 1 minuto, es escaneo
                        if ports.len() > 20 {
                            let event = SecurityEvent::new(
                                IntrusionType::PortScan,
                                Severity::High,
                                traffic.source_ip.clone(),
                                format!(
                                    "Escaneo de puertos detectado desde IP {} hacia dispositivo {}",
                                    traffic.source_ip,
                                    traffic.target_device_id.as_deref().unwrap_or("desconocido")
                                ),
                            );
                            let _ = security_repo.log_event(event.clone()).await;
                            
                            // Correlacionar con eventos recientes
                            Self::correlate_event(&event, &recent_events, &security_repo).await;
                            
                            recent_events.push(event);
                            if recent_events.len() > max_buffer_size {
                                recent_events.remove(0);
                            }
                            ports.clear(); // Resetear tracker
                        }

                        // Detectar DDoS (alto volumen de conexiones)
                        let counter = connection_tracker.entry(traffic.source_ip.clone()).or_insert(0);
                        *counter += 1;

                        if *counter > 1000 {
                            let event = SecurityEvent::new(
                                IntrusionType::DDoS,
                                Severity::Critical,
                                traffic.source_ip.clone(),
                                format!(
                                    "Posible ataque DDoS detectado desde IP {} ({} conexiones)",
                                    traffic.source_ip, counter
                                ),
                            );
                            let _ = security_repo.log_event(event.clone()).await;
                            
                            // Correlacionar con eventos recientes
                            Self::correlate_event(&event, &recent_events, &security_repo).await;
                            
                            recent_events.push(event);
                            if recent_events.len() > max_buffer_size {
                                recent_events.remove(0);
                            }
                            *counter = 0; // Resetear contador
                        }

                        // Detectar tráfico anómalo por sede
                        if let Some(sede_id) = &traffic.target_sede_id {
                            let sede_counter = ddos_tracker.entry(sede_id.clone()).or_insert(0);
                            *sede_counter += traffic.bytes;

                            // Si más de 1GB en 1 minuto, es sospechoso
                            if *sede_counter > 1_000_000_000 {
                                let event = SecurityEvent::new(
                                    IntrusionType::DataExfiltration,
                                    Severity::High,
                                    traffic.source_ip.clone(),
                                    format!(
                                        "Tráfico anómalo detectado en sede {} ({} bytes)",
                                        sede_id, sede_counter
                                    ),
                                );
                                let _ = security_repo.log_event(event.clone()).await;
                                
                                // Correlacionar con eventos recientes
                                Self::correlate_event(&event, &recent_events, &security_repo).await;
                                
                                recent_events.push(event);
                                if recent_events.len() > max_buffer_size {
                                    recent_events.remove(0);
                                }
                                *sede_counter = 0; // Resetear contador
                            }
                        }
                    }
                    DetectionMessage::CheckDevice(device) => {
                        // Detectar anomalías en uso de recursos
                        if device.cpu_usage > 95.0 {
                            let event = SecurityEvent::new(
                                IntrusionType::UnauthorizedAccess,
                                Severity::Medium,
                                "system".to_string(),
                                format!(
                                    "Uso de CPU crítico en dispositivo {}: {}%",
                                    device.device_id, device.cpu_usage
                                ),
                            );
                            let _ = security_repo.log_event(event.clone()).await;
                            
                            // Correlacionar con eventos recientes
                            Self::correlate_event(&event, &recent_events, &security_repo).await;
                            
                            recent_events.push(event);
                            if recent_events.len() > max_buffer_size {
                                recent_events.remove(0);
                            }
                        }

                        if device.memory_usage > 95.0 {
                            let event = SecurityEvent::new(
                                IntrusionType::UnauthorizedAccess,
                                Severity::Medium,
                                "system".to_string(),
                                format!(
                                    "Uso de memoria crítico en dispositivo {}: {}%",
                                    device.device_id, device.memory_usage
                                ),
                            );
                            let _ = security_repo.log_event(event.clone()).await;
                            
                            // Correlacionar con eventos recientes
                            Self::correlate_event(&event, &recent_events, &security_repo).await;
                            
                            recent_events.push(event);
                            if recent_events.len() > max_buffer_size {
                                recent_events.remove(0);
                            }
                        }

                        if device.bandwidth_usage > 95.0 {
                            let event = SecurityEvent::new(
                                IntrusionType::DDoS,
                                Severity::High,
                                "system".to_string(),
                                format!(
                                    "Saturación de ancho de banda en dispositivo {}: {}%",
                                    device.device_id, device.bandwidth_usage
                                ),
                            );
                            let _ = security_repo.log_event(event.clone()).await;
                            
                            // Correlacionar con eventos recientes
                            Self::correlate_event(&event, &recent_events, &security_repo).await;
                            
                            recent_events.push(event);
                            if recent_events.len() > max_buffer_size {
                                recent_events.remove(0);
                            }
                        }
                    }
                    DetectionMessage::Shutdown => {
                        break;
                    }
                }
            }
        });

        Self {
            tx,
            _handle: handle,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Correlacionar un evento con eventos recientes
    async fn correlate_event(
        event: &SecurityEvent,
        recent_events: &[SecurityEvent],
        security_repo: &Arc<T>,
    ) {
        // Buscar eventos relacionados por IP de origen
        let related_by_ip: Vec<_> = recent_events
            .iter()
            .filter(|e| e.source_ip == event.source_ip && e.id != event.id)
            .cloned()
            .collect();

        if related_by_ip.len() >= 3 {
            // Detectar secuencia de escaneo de puertos
            let port_scan_count = related_by_ip
                .iter()
                .filter(|e| matches!(e.event_type, IntrusionType::PortScan))
                .count();

            if port_scan_count >= 3 {
                let correlated = CorrelatedEvent {
                    primary_event: event.clone(),
                    related_events: related_by_ip.clone(),
                    correlation_type: CorrelationType::PortScanSequence,
                    confidence: 0.85,
                };
                Self::log_correlated_event(correlated, security_repo).await;
            }
        }

        // Buscar eventos relacionados por dispositivo objetivo
        if let Some(device_id) = &event.target_device_id {
            let related_by_device: Vec<_> = recent_events
                .iter()
                .filter(|e| e.target_device_id.as_ref() == Some(device_id) && e.id != event.id)
                .cloned()
                .collect();

            if related_by_device.len() >= 2 {
                // Detectar patrón de acceso no autorizado seguido de anomalía
                let unauthorized_count = related_by_device
                    .iter()
                    .filter(|e| matches!(e.event_type, IntrusionType::UnauthorizedAccess))
                    .count();

                if unauthorized_count > 0 {
                    let correlated = CorrelatedEvent {
                        primary_event: event.clone(),
                        related_events: related_by_device,
                        correlation_type: CorrelationType::UnauthorizedAccessFollowedByAnomaly,
                        confidence: 0.75,
                    };
                    Self::log_correlated_event(correlated, security_repo).await;
                }
            }
        }
    }

    /// Registrar evento correlacionado
    async fn log_correlated_event(
        correlated: CorrelatedEvent,
        security_repo: &Arc<T>,
    ) {
        let description = format!(
            "Evento correlacionado: {:?}. Eventos relacionados: {}. Confianza: {:.2}",
            correlated.correlation_type,
            correlated.related_events.len(),
            correlated.confidence
        );

        let event = SecurityEvent::new(
            IntrusionType::Other("CorrelatedEvent".to_string()),
            Severity::High,
            correlated.primary_event.source_ip.clone(),
            description,
        );

        let _ = security_repo.log_event(event).await;
    }

    /// Enviar datos de tráfico para análisis
    pub async fn analyze_traffic(&self, traffic: TrafficData) -> Result<(), String> {
        self.tx.send(DetectionMessage::AnalyzeTraffic(traffic))
            .await
            .map_err(|e| format!("Error al enviar tráfico: {}", e))
    }

    /// Verificar estado de dispositivo
    pub async fn check_device(&self, device: DeviceData) -> Result<(), String> {
        self.tx.send(DetectionMessage::CheckDevice(device))
            .await
            .map_err(|e| format!("Error al verificar dispositivo: {}", e))
    }

    /// Detener el motor de detección
    pub async fn shutdown(self) {
        let _ = self.tx.send(DetectionMessage::Shutdown).await;
    }
}
