-- Slice 3.1: Tablas de Devices y Alerts para Dashboard
-- Vinculado con ADR-0004 (Persistencia MySQL) y ADR-0005 (Migraciones)
-- Base de datos: redes_dev

-- Tabla de Dispositivos de Red
CREATE TABLE IF NOT EXISTS devices (
    id VARCHAR(36) PRIMARY KEY COMMENT 'UUIDv7 para indexación eficiente',
    name VARCHAR(100) NOT NULL COMMENT 'Nombre del dispositivo (ej. Router Trinidad)',
    device_type VARCHAR(50) NOT NULL COMMENT 'Tipo: router, switch, firewall, ap',
    location_id VARCHAR(36) COMMENT 'FK a locations.id',
    ip_address VARCHAR(45) COMMENT 'Dirección IPv4 o IPv6',
    mac_address VARCHAR(17) COMMENT 'Dirección MAC',
    bandwidth_gbps DECIMAL(10, 2) DEFAULT 0.00 COMMENT 'Ancho de banda actual en Gbps',
    status ENUM('online', 'offline', 'degraded') DEFAULT 'offline' COMMENT 'Estado de conectividad',
    is_active BOOLEAN DEFAULT TRUE COMMENT 'Si está activo en el sistema',
    last_seen DATETIME COMMENT 'Última vez que respondió',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_devices_location (location_id),
    INDEX idx_devices_status (status),
    INDEX idx_devices_type (device_type),
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Tabla de Alertas Activas
CREATE TABLE IF NOT EXISTS active_alerts (
    id VARCHAR(36) PRIMARY KEY COMMENT 'UUIDv7 para indexación eficiente',
    device_id VARCHAR(36) COMMENT 'FK a devices.id',
    location_id VARCHAR(36) COMMENT 'FK a locations.id',
    severity ENUM('critical', 'warning', 'info') NOT NULL COMMENT 'Nivel de severidad',
    title VARCHAR(200) NOT NULL COMMENT 'Título breve de la alerta',
    description TEXT COMMENT 'Descripción detallada',
    metric_name VARCHAR(50) COMMENT 'Nombre de la métrica que disparó (ping_ms, latency_ms, packet_loss)',
    metric_value DECIMAL(10, 3) COMMENT 'Valor que disparó la alerta',
    threshold_value DECIMAL(10, 3) COMMENT 'Umbral que fue excedido',
    is_acknowledged BOOLEAN DEFAULT FALSE COMMENT 'Si fue reconocida por un operador',
    acknowledged_by VARCHAR(36) COMMENT 'ID del usuario que reconoció',
    acknowledged_at DATETIME COMMENT 'Cuando fue reconocida',
    resolved_at DATETIME COMMENT 'Cuando fue resuelta',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_alerts_device (device_id),
    INDEX idx_alerts_location (location_id),
    INDEX idx_alerts_severity (severity),
    INDEX idx_alerts_acknowledged (is_acknowledged),
    INDEX idx_alerts_created (created_at),
    FOREIGN KEY (device_id) REFERENCES devices(id) ON DELETE SET NULL,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;