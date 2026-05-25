-- Migración para Slice 2.1: Esquema Clave-Valor y Sedes SQL
-- Vinculado con ADR-0004 (Persistencia MySQL) y ADR-0005 (Migraciones)
-- Base de datos: redes_dev

-- Tabla de Sedes (Locations) - Catálogo del Beni
CREATE TABLE IF NOT EXISTS locations (
    id VARCHAR(36) PRIMARY KEY COMMENT 'UUIDv7 para indexación eficiente',
    name VARCHAR(100) NOT NULL COMMENT 'Nombre de la sede (ej. Trinidad, Riberalta)',
    code VARCHAR(20) NOT NULL UNIQUE COMMENT 'Código corto (ej. TDD, RBT)',
    region VARCHAR(50) NOT NULL DEFAULT 'Beni' COMMENT 'Región administrativa',
    latitude DECIMAL(10, 8) COMMENT 'Coordenada geográfica',
    longitude DECIMAL(11, 8) COMMENT 'Coordenada geográfica',
    is_active BOOLEAN DEFAULT TRUE COMMENT 'Estado de la sede',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_locations_region (region),
    INDEX idx_locations_active (is_active)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Tabla de Configuración Dinámica (System Settings) - Umbrales de red
CREATE TABLE IF NOT EXISTS system_settings (
    id VARCHAR(36) PRIMARY KEY COMMENT 'UUIDv7 para indexación eficiente',
    key_name VARCHAR(100) NOT NULL UNIQUE COMMENT 'Nombre de la clave (ej. threshold_ping_ms)',
    value VARCHAR(255) NOT NULL COMMENT 'Valor configurado',
    value_type VARCHAR(20) NOT NULL DEFAULT 'integer' COMMENT 'Tipo: integer, float, string, boolean',
    category VARCHAR(50) NOT NULL DEFAULT 'thresholds' COMMENT 'Categoría (thresholds, alerts, system)',
    description TEXT COMMENT 'Descripción del propósito',
    min_value VARCHAR(50) COMMENT 'Valor mínimo permitido',
    max_value VARCHAR(50) COMMENT 'Valor máximo permitido',
    is_active BOOLEAN DEFAULT TRUE COMMENT 'Si está activo',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_system_settings_category (category),
    INDEX idx_system_settings_active (is_active)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;