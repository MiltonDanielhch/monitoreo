// crates/infrastructure/tests/security_tests.rs
// Tests para el módulo de seguridad y detección de intrusiones
// Vinculado con ADR-0015 (Tokio Jobs)

use infrastructure::security::{DetectionEngine, DetectionMessage, TrafficData, DeviceData};
use domain::models::security::{SecurityEvent, IntrusionType, Severity, SecurityPort};
use std::sync::Arc;
use std::time::Instant;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_detection_engine_creation() {
    // Crear un repositorio mock para el test
    // En un caso real, usaríamos un mock o un repositorio de prueba
    let security_repo = Arc::new(MockSecurityRepository::new());
    
    let engine = DetectionEngine::new(security_repo);
    
    // El motor debería haberse creado exitosamente
    // No hay assertion directa, pero si no panic, es exitoso
    let _ = engine;
}

#[tokio::test]
async fn test_traffic_data_analysis() {
    let security_repo = Arc::new(MockSecurityRepository::new());
    let engine = DetectionEngine::new(security_repo.clone());
    
    let traffic = TrafficData {
        source_ip: "192.168.1.100".to_string(),
        source_mac: Some("00:11:22:33:44:55".to_string()),
        target_device_id: Some("device-1".to_string()),
        target_sede_id: Some("sede-1".to_string()),
        port: 80,
        bytes: 1024,
        timestamp: Instant::now(),
    };
    
    let result = engine.analyze_traffic(traffic).await;
    assert!(result.is_ok(), "El análisis de tráfico debería ser exitoso");
}

#[tokio::test]
async fn test_device_check() {
    let security_repo = Arc::new(MockSecurityRepository::new());
    let engine = DetectionEngine::new(security_repo.clone());
    
    let device = DeviceData {
        device_id: "device-1".to_string(),
        sede_id: Some("sede-1".to_string()),
        cpu_usage: 50.0,
        memory_usage: 60.0,
        bandwidth_usage: 70.0,
        timestamp: Instant::now(),
    };
    
    let result = engine.check_device(device).await;
    assert!(result.is_ok(), "La verificación de dispositivo debería ser exitosa");
}

#[tokio::test]
async fn test_detection_engine_shutdown() {
    let security_repo = Arc::new(MockSecurityRepository::new());
    let engine = DetectionEngine::new(security_repo.clone());
    
    // Enviar algunos mensajes
    let traffic = TrafficData {
        source_ip: "192.168.1.100".to_string(),
        source_mac: None,
        target_device_id: None,
        target_sede_id: None,
        port: 80,
        bytes: 1024,
        timestamp: Instant::now(),
    };
    
    let _ = engine.analyze_traffic(traffic).await;
    
    // Apagar el motor
    engine.shutdown().await;
    
    // Esperar un poco para asegurar que se apagó
    sleep(Duration::from_millis(100)).await;
}

#[tokio::test]
async fn test_port_scan_detection() {
    let security_repo = Arc::new(MockSecurityRepository::new());
    let engine = DetectionEngine::new(security_repo.clone());
    
    // Simular escaneo de puertos enviando tráfico a múltiples puertos
    for port in 1..=25 {
        let traffic = TrafficData {
            source_ip: "192.168.1.100".to_string(),
            source_mac: None,
            target_device_id: Some("device-1".to_string()),
            target_sede_id: None,
            port,
            bytes: 1024,
            timestamp: Instant::now(),
        };
        
        let _ = engine.analyze_traffic(traffic).await;
        sleep(Duration::from_millis(10)).await;
    }
    
    // Esperar a que el motor procese
    sleep(Duration::from_millis(500)).await;
    
    // Verificar que se detectó el escaneo
    let events = security_repo.get_events();
    assert!(events.len() > 0, "Debería haber detectado al menos un evento");
}

#[tokio::test]
async fn test_ddos_detection() {
    let security_repo = Arc::new(MockSecurityRepository::new());
    let engine = DetectionEngine::new(security_repo.clone());
    
    // Simular DDoS enviando muchas conexiones
    for _ in 0..1001 {
        let traffic = TrafficData {
            source_ip: "192.168.1.100".to_string(),
            source_mac: None,
            target_device_id: None,
            target_sede_id: None,
            port: 80,
            bytes: 1024,
            timestamp: Instant::now(),
        };
        
        let _ = engine.analyze_traffic(traffic).await;
    }
    
    // Esperar a que el motor procese
    sleep(Duration::from_millis(500)).await;
    
    // Verificar que se detectó el DDoS
    let events = security_repo.get_events();
    assert!(events.len() > 0, "Debería haber detectado al menos un evento");
}

#[tokio::test]
async fn test_device_anomaly_detection() {
    let security_repo = Arc::new(MockSecurityRepository::new());
    let engine = DetectionEngine::new(security_repo.clone());
    
    // Simular uso crítico de CPU
    let device = DeviceData {
        device_id: "device-1".to_string(),
        sede_id: None,
        cpu_usage: 98.0,
        memory_usage: 50.0,
        bandwidth_usage: 50.0,
        timestamp: Instant::now(),
    };
    
    let _ = engine.check_device(device).await;
    
    // Esperar a que el motor procese
    sleep(Duration::from_millis(500)).await;
    
    // Verificar que se detectó la anomalía
    let events = security_repo.get_events();
    assert!(events.len() > 0, "Debería haber detectado al menos un evento");
}

// Mock del repositorio de seguridad para tests
struct MockSecurityRepository {
    events: std::sync::Mutex<Vec<SecurityEvent>>,
}

impl MockSecurityRepository {
    fn new() -> Self {
        Self {
            events: std::sync::Mutex::new(Vec::new()),
        }
    }
    
    fn get_events(&self) -> Vec<SecurityEvent> {
        self.events.lock().unwrap().clone()
    }
}

#[async_trait::async_trait]
impl SecurityPort for MockSecurityRepository {
    async fn log_event(&self, event: SecurityEvent) -> Result<(), domain::models::security::DomainError> {
        self.events.lock().unwrap().push(event);
        Ok(())
    }
    
    async fn get_events(&self, _filters: domain::models::security::SecurityFilters) -> Result<Vec<SecurityEvent>, domain::models::security::DomainError> {
        Ok(self.events.lock().unwrap().clone())
    }
    
    async fn get_event_by_id(&self, _id: String) -> Result<Option<SecurityEvent>, domain::models::security::DomainError> {
        Ok(None)
    }
    
    async fn resolve_event(&self, _id: String, _resolved_by: String) -> Result<(), domain::models::security::DomainError> {
        Ok(())
    }
    
    async fn mark_false_positive(&self, _id: String) -> Result<(), domain::models::security::DomainError> {
        Ok(())
    }
    
    async fn get_events_by_severity(&self, _severity: Severity) -> Result<Vec<SecurityEvent>, domain::models::security::DomainError> {
        Ok(self.events.lock().unwrap().clone())
    }
    
    async fn get_events_by_status(&self, _status: domain::models::security::SecurityStatus) -> Result<Vec<SecurityEvent>, domain::models::security::DomainError> {
        Ok(self.events.lock().unwrap().clone())
    }
    
    async fn get_events_by_device(&self, _device_id: String) -> Result<Vec<SecurityEvent>, domain::models::security::DomainError> {
        Ok(self.events.lock().unwrap().clone())
    }
}
