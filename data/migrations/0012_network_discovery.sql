-- data/migrations/0012_network_discovery.sql
-- Módulo 12: Descubrimiento de Red y Asset Discovery
-- Tablas para dispositivos descubiertos y registros de escaneos de red
-- Vinculado con ADR-0004 (Persistencia con Sea-ORM) y ADR-0005 (Diseño de esquema relacional)

-- Crear tabla de dispositivos descubiertos
CREATE TABLE IF NOT EXISTS discovered_devices (
    id VARCHAR(255) PRIMARY KEY COMMENT 'ID único del dispositivo descubierto',
    ip_address VARCHAR(45) NOT NULL COMMENT 'Dirección IP del dispositivo (IPv4 o IPv6)',
    mac_address VARCHAR(17) NULL COMMENT 'Dirección MAC del dispositivo (formato XX:XX:XX:XX:XX:XX)',
    hostname VARCHAR(255) NULL COMMENT 'Nombre del host obtenido por DNS reverse lookup',
    device_type VARCHAR(50) NOT NULL DEFAULT 'unknown' COMMENT 'Tipo: router, switch, server, pc, mobile, iot, printer, unknown',
    os_fingerprint VARCHAR(100) NULL COMMENT 'Sistema operativo detectado por fingerprinting',
    manufacturer VARCHAR(100) NULL COMMENT 'Fabricante obtenido por OUI lookup',
    open_ports TEXT NULL COMMENT 'JSON array con puertos abiertos detectados',
    services TEXT NULL COMMENT 'JSON array con servicios detectados',
    status VARCHAR(50) NOT NULL DEFAULT 'unknown' COMMENT 'Estado: online, offline, unknown',
    is_authorized BOOLEAN NOT NULL DEFAULT FALSE COMMENT 'Si el dispositivo está autorizado en la red',
    last_seen DATETIME NOT NULL COMMENT 'Última vez que se detectó el dispositivo',
    first_seen DATETIME NOT NULL COMMENT 'Primera vez que se detectó el dispositivo',
    scan_id VARCHAR(255) NULL COMMENT 'ID del escaneo que descubrió este dispositivo',
    sede_id VARCHAR(255) NULL COMMENT 'ID de la sede donde se descubrió',
    metadata TEXT NULL COMMENT 'JSON con detalles adicionales del dispositivo',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Timestamp de creación del registro',
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT 'Timestamp de última actualización',
    INDEX idx_discovered_devices_ip (ip_address),
    INDEX idx_discovered_devices_mac (mac_address),
    INDEX idx_discovered_devices_status (status),
    INDEX idx_discovered_devices_device_type (device_type),
    INDEX idx_discovered_devices_last_seen (last_seen DESC),
    INDEX idx_discovered_devices_sede (sede_id),
    INDEX idx_discovered_devices_scan (scan_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='Dispositivos descubiertos por escaneo de red';

-- Crear tabla de registros de escaneos de red
CREATE TABLE IF NOT EXISTS network_scans (
    id VARCHAR(255) PRIMARY KEY COMMENT 'ID único del escaneo',
    scan_type VARCHAR(50) NOT NULL DEFAULT 'partial' COMMENT 'Tipo: full, partial, targeted',
    ip_range VARCHAR(100) NOT NULL COMMENT 'Rango de IPs escaneado (ej: 192.168.1.0/24)',
    status VARCHAR(50) NOT NULL DEFAULT 'pending' COMMENT 'Estado: pending, running, completed, failed',
    devices_found INT NOT NULL DEFAULT 0 COMMENT 'Cantidad de dispositivos encontrados',
    started_at DATETIME NOT NULL COMMENT 'Timestamp de inicio del escaneo',
    completed_at DATETIME NULL COMMENT 'Timestamp de finalización del escaneo',
    duration_seconds INT NULL COMMENT 'Duración del escaneo en segundos',
    sede_id VARCHAR(255) NULL COMMENT 'ID de la sede donde se ejecutó el escaneo',
    created_by VARCHAR(255) NOT NULL COMMENT 'Usuario que inició el escaneo',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Timestamp de creación del registro',
    INDEX idx_network_scans_status (status),
    INDEX idx_network_scans_sede (sede_id),
    INDEX idx_network_scans_started (started_at DESC)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='Registros de escaneos de red ejecutados';
