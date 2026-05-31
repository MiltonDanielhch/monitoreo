-- Módulo 5: Infraestructura y Topologías
-- Slice 5.1: Esquema Relacional de Activos por Sede
-- Tabla para almacenar metadatos de archivos técnicos (SVG, imágenes, respaldos de configuración)

CREATE TABLE IF NOT EXISTS network_files (
    id VARCHAR(36) PRIMARY KEY,
    filename VARCHAR(255) NOT NULL,
    file_type ENUM('TOPOLOGY_SVG', 'RACK_IMAGE', 'CONFIG_BACKUP') NOT NULL,
    file_size_bytes BIGINT NOT NULL,
    storage_key VARCHAR(512) NOT NULL COMMENT 'Ruta física del archivo en almacenamiento local',
    sha256_checksum VARCHAR(64) NOT NULL COMMENT 'Hash SHA-256 para integridad y deduplicación',
    sede_id VARCHAR(36) NOT NULL COMMENT 'Referencia a la sede (Trinidad, Riberalta, Guayaramerín, etc.)',
    user_id VARCHAR(36) COMMENT 'Usuario que subió el archivo',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP NULL COMMENT 'Soft delete para recuperación de archivos eliminados',
    
    CONSTRAINT fk_network_files_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Índice compuesto para búsquedas regionales rápidas por sede y tipo de archivo
CREATE INDEX idx_files_sede_type ON network_files(sede_id, file_type, deleted_at);

-- Índice para búsquedas por checksum (deduplicación)
CREATE INDEX idx_files_checksum ON network_files(sha256_checksum);

-- Índice para búsquedas por usuario
CREATE INDEX idx_files_user ON network_files(user_id);

-- Índice temporal para consultas cronológicas
CREATE INDEX idx_files_created_at ON network_files(created_at DESC);
