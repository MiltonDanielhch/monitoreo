-- data/migrations/0009_sla_aggregation_queries.sql
-- Consultas de agregación para cálculo de SLA
-- Vinculado con ADR-0004 (Persistencia con Sea-ORM) y ADR-0005 (Diseño de esquema relacional)

-- Vista para cálculo de uptime/downtime por sede por mes
CREATE OR REPLACE VIEW v_sla_monthly AS
SELECT 
    sede_id,
    DATE_FORMAT(detected_at, '%Y-%m') AS month,
    COUNT(*) AS total_events,
    SUM(CASE WHEN status = 'up' THEN 1 ELSE 0 END) AS uptime_events,
    SUM(CASE WHEN status = 'down' THEN 1 ELSE 0 END) AS downtime_events,
    SUM(CASE WHEN status = 'down' THEN 1 ELSE 0 END) * 5 AS estimated_downtime_minutes, -- Asumiendo 5 minutos por evento down
    SUM(CASE WHEN status = 'up' THEN 1 ELSE 0 END) * 5 AS estimated_uptime_minutes,
    ROUND(
        (SUM(CASE WHEN status = 'up' THEN 1 ELSE 0 END) * 100.0 / COUNT(*)),
        2
    ) AS sla_percentage,
    AVG(latency_ms) AS avg_latency_ms,
    AVG(bandwidth_mbps) AS avg_bandwidth_mbps,
    MIN(detected_at) AS period_start,
    MAX(detected_at) AS period_end
FROM telemetry_metrics
WHERE detected_at >= DATE_SUB(NOW(), INTERVAL 12 MONTH)
GROUP BY sede_id, DATE_FORMAT(detected_at, '%Y-%m')
ORDER BY sede_id, month DESC;

-- Consulta para obtener SLA de una sede específica en un periodo
-- SELECT * FROM v_sla_monthly WHERE sede_id = 'sede-1' AND month = '2024-05';

-- Consulta para obtener resumen de SLA de todas las sedes en un mes
-- SELECT * FROM v_sla_monthly WHERE month = '2024-05' ORDER BY sla_percentage ASC;

-- Consulta para detectar sedes que incumplen el SLA (menos de 99.5%)
-- SELECT sede_id, month, sla_percentage, estimated_downtime_minutes 
-- FROM v_sla_monthly 
-- WHERE sla_percentage < 99.5 
-- ORDER BY sla_percentage ASC;

-- Consulta para obtener tendencias de SLA de una sede en los últimos 6 meses
-- SELECT sede_id, month, sla_percentage, avg_latency_ms, avg_bandwidth_mbps
-- FROM v_sla_monthly
-- WHERE sede_id = 'sede-1'
-- ORDER BY month DESC
-- LIMIT 6;
