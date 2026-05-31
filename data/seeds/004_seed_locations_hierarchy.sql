-- Seed para áreas dentro de sedes (jerarquía parent_id)
-- Vinculado con ADR-0005 (Migraciones y Seeding)
-- Base de datos: redes_dev

-- Áreas de Trinidad
INSERT INTO locations (id, name, code, region, parent_id, latitude, longitude, is_active) VALUES
('018f0000-0000-0000-0000-000000000060', 'Rack Principal TDD', 'TDD-R1', 'Beni', '018f0000-0000-0000-0000-000000000020', -14.8333, -64.9000, TRUE),
('018f0000-0000-0000-0000-000000000061', 'Sala de Backup TDD', 'TDD-B1', 'Beni', '018f0000-0000-0000-0000-000000000020', -14.8333, -64.9000, TRUE),
('018f0000-0000-0000-0000-000000000062', 'Cuarto de Networking TDD', 'TDD-N1', 'Beni', '018f0000-0000-0000-0000-000000000020', -14.8333, -64.9000, TRUE)
ON DUPLICATE KEY UPDATE name=name;

-- Áreas de Riberalta
INSERT INTO locations (id, name, code, region, parent_id, latitude, longitude, is_active) VALUES
('018f0000-0000-0000-0000-000000000063', 'Centro de Datos RBT', 'RBT-CD1', 'Beni', '018f0000-0000-0000-0000-000000000021', -10.9833, -66.1000, TRUE),
('018f0000-0000-0000-0000-000000000064', 'Rack Secundario RBT', 'RBT-R2', 'Beni', '018f0000-0000-0000-0000-000000000021', -10.9833, -66.1000, TRUE)
ON DUPLICATE KEY UPDATE name=name;

-- Áreas de Guayaramerín
INSERT INTO locations (id, name, code, region, parent_id, latitude, longitude, is_active) VALUES
('018f0000-0000-0000-0000-000000000065', 'Sala de Equipos GYA', 'GYA-SE1', 'Beni', '018f0000-0000-0000-0000-000000000022', -10.8167, -65.7833, TRUE)
ON DUPLICATE KEY UPDATE name=name;

-- Áreas de San Ignacio de Moxos
INSERT INTO locations (id, name, code, region, parent_id, latitude, longitude, is_active) VALUES
('018f0000-0000-0000-0000-000000000066', 'Rack Principal SIM', 'SIM-R1', 'Beni', '018f0000-0000-0000-0000-000000000023', -14.5333, -65.7500, TRUE)
ON DUPLICATE KEY UPDATE name=name;

-- Áreas de Santa Ana de Yacuma
INSERT INTO locations (id, name, code, region, parent_id, latitude, longitude, is_active) VALUES
('018f0000-0000-0000-0000-000000000067', 'Cuarto de Servidores SAY', 'SAY-CS1', 'Beni', '018f0000-0000-0000-0000-000000000024', -13.7500, -67.1833, TRUE)
ON DUPLICATE KEY UPDATE name=name;