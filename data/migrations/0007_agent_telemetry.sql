-- Módulo 7: API de Telemetría
-- Slice 7.1: Esquema de Muestreos y Series de Tiempo (MySQL Workbench)
-- Tablas para agentes remotos y métricas de telemetría
-- Vinculado con ADR-0004-persistencia-sea-orm.md y ADR-0005-modelado-relacional.md

-- Tabla de agentes remotos (identidad de las antenas/nodos provinciales)
CREATE TABLE IF NOT EXISTS remote_agents (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL COMMENT 'Nombre del agente/nodo',
    sede_id VARCHAR(50) NOT NULL COMMENT 'Sede donde está ubicado el agente',
    agent_type VARCHAR(50) NOT NULL COMMENT 'Tipo: ROUTER, SWITCH, SERVER, etc.',
    ip_address VARCHAR(45) NOT NULL COMMENT 'Dirección IP del agente',
    api_token_hash VARCHAR(255) NOT NULL COMMENT 'Hash del token de autenticación',
    last_seen TIMESTAMP NULL COMMENT 'Última vez que el agente reportó',
    status VARCHAR(20) DEFAULT 'ACTIVE' COMMENT 'Estado: ACTIVE, INACTIVE, ERROR',
    metadata JSON COMMENT 'Información adicional del agente',
    INDEX idx_sede_id (sede_id),
    INDEX idx_status (status),
    INDEX idx_last_seen (last_seen),
    CONSTRAINT fk_agent_sede FOREIGN KEY (sede_id) REFERENCES sedes(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Tabla de métricas de telemetría (batch de métricas)
CREATE TABLE IF NOT EXISTS agent_metrics_batch (
    id VARCHAR(36) PRIMARY KEY,
    agent_id VARCHAR(36) NOT NULL COMMENT 'ID del agente que reportó',
    cpu_usage_percent DECIMAL(5,2) COMMENT 'Uso de CPU en porcentaje',
    memory_usage_percent DECIMAL(5,2) COMMENT 'Uso de memoria en porcentaje',
    latency_ms INT COMMENT 'Latencia en milisegundos',
    packet_loss_percent DECIMAL(5,2) COMMENT 'Pérdida de paquetes en porcentaje',
    bandwidth_mbps DECIMAL(10,2) COMMENT 'Ancho de banda en Mbps',
    disk_usage_percent DECIMAL(5,2) COMMENT 'Uso de disco en porcentaje',
    temperature_celsius DECIMAL(5,2) COMMENT 'Temperatura en grados Celsius',
    uptime_seconds BIGINT COMMENT 'Tiempo de actividad en segundos',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Timestamp de la métrica',
    metadata JSON COMMENT 'Información adicional de la métrica',
    INDEX idx_agent_date (agent_id, created_at DESC),
    INDEX idx_created_at (created_at DESC),
    CONSTRAINT fk_metrics_agent FOREIGN KEY (agent_id) REFERENCES remote_agents(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
