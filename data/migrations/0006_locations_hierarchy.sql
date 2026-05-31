-- Slice 3.1: Agregar jerarquía de áreas a locations
-- Vinculado con ADR-0005 (Migraciones y Seeding)
-- Base de datos: redes_dev

-- Agregar columna parent_id para relación jerárquica
ALTER TABLE locations
ADD COLUMN parent_id VARCHAR(36) NULL COMMENT 'FK a locations.id para jerarquía' AFTER region,
ADD CONSTRAINT fk_locations_parent FOREIGN KEY (parent_id) REFERENCES locations(id) ON DELETE SET NULL;

-- Agregar índice para búsquedas por padre
CREATE INDEX idx_locations_parent ON locations(parent_id);