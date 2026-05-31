-- Slice 3.1: Seeds para Devices y Active Alerts
-- Vinculado con ADR-0005 (Migraciones y Seeding)
-- Base de datos: redes_dev

-- Devices de prueba por sede
INSERT INTO devices (id, name, device_type, location_id, ip_address, bandwidth_gbps, status, is_active, last_seen) VALUES
-- Trinidad
('018f0000-0000-0000-0000-000000000040', 'Router Principal Trinidad', 'router', '018f0000-0000-0000-0000-000000000020', '192.168.1.1', 10.50, 'online', TRUE, NOW()),
('018f0000-0000-0000-0000-000000000041', 'Switch Core Trinidad', 'switch', '018f0000-0000-0000-0000-000000000020', '192.168.1.2', 8.25, 'online', TRUE, NOW()),
-- Riberalta
('018f0000-0000-0000-0000-000000000042', 'Router Principal Riberalta', 'router', '018f0000-0000-0000-0000-000000000021', '192.168.2.1', 5.00, 'online', TRUE, NOW()),
('018f0000-0000-0000-0000-000000000043', 'Switch Edge Riberalta', 'switch', '018f0000-0000-0000-0000-000000000021', '192.168.2.2', 3.75, 'degraded', TRUE, NOW()),
-- Guayaramerín
('018f0000-0000-0000-0000-000000000044', 'Router Guayaramerín', 'router', '018f0000-0000-0000-0000-000000000022', '192.168.3.1', 2.00, 'online', TRUE, NOW()),
-- San Ignacio de Moxos
('018f0000-0000-0000-0000-000000000045', 'Switch San Ignacio', 'switch', '018f0000-0000-0000-0000-000000000023', '192.168.4.1', 1.50, 'offline', TRUE, NOW()),
-- Santa Ana
('018f0000-0000-0000-0000-000000000046', 'Router Santa Ana', 'router', '018f0000-0000-0000-0000-000000000024', '192.168.5.1', 1.00, 'online', TRUE, NOW())
ON DUPLICATE KEY UPDATE name=name;

-- Active Alerts de prueba
INSERT INTO active_alerts (id, device_id, location_id, severity, title, description, metric_name, metric_value, threshold_value) VALUES
('018f0000-0000-0000-0000-000000000050', '018f0000-0000-0000-0000-000000000043', '018f0000-0000-0000-0000-000000000021', 'warning', 'Alta latencia detectada', 'Switch Edge Riberalta mostrando degradación', 'latency_ms', 650.00, 500.00),
('018f0000-0000-0000-0000-000000000051', '018f0000-0000-0000-0000-000000000045', '018f0000-0000-0000-0000-000000000023', 'critical', 'Dispositivo fuera de línea', 'Switch San Ignacio no responde hace 15 minutos', 'ping_ms', 0.00, 500.00),
('018f0000-0000-0000-0000-000000000052', NULL, '018f0000-0000-0000-0000-000000000020', 'info', 'Mantenimiento programado', 'Se realizará mantenimiento en Rack Principal Trinidad', 'packet_loss_percent', 0.10, 15.00)
ON DUPLICATE KEY UPDATE title=title;