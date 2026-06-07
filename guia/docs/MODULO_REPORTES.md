# 📄 Módulo de Reportes

## 🎯 Descripción General

El **Módulo de Reportes** proporciona capacidades avanzadas de generación, análisis y exportación de reportes de Service Level Agreement (SLA) para todas las sedes regionales. Este módulo consolida millones de métricas mensuales para generar documentos oficiales con firmas criptográficas que garantizan la integridad y autenticidad de la información.

---

## 🏛️ Componentes del Módulo

### Reportes

**Propósito**: Generación y descarga de reportes de SLA con análisis de cumplimiento contractual.

**Funcionalidades**:
- Selección de periodo (mes/año) para reportes
- Generación de reportes por sede individual
- Resumen de SLA para todas las sedes
- Exportación a PDF con firma criptográfica
- Previsualización de datos antes de exportar

**Tipos de Reportes**:
- **Reporte Individual**: SLA de una sede específica
- **Reporte Consolidado**: Resumen de todas las sedes
- **Reporte Comparativo**: Comparación entre periodos
- **Reporte de Tendencias**: Análisis histórico de SLA

**Métricas de SLA**:
- **Disponibilidad**: Porcentaje de tiempo operativo
- **Tiempo de Inactividad**: Minutos de caída total
- **Tiempo Operativo**: Minutos de funcionamiento
- **Latencia Promedio**: Retardo promedio de red
- **Ancho de Banda Promedio**: Capacidad de transferencia
- **Número de Incidentes**: Conteo de alertas críticas

**Estados Contractuales**:
- 🟢 **Compliant**: Cumple con SLA objetivo (≥ 99.5%)
- 🟡 **At Risk**: Cerca de incumplimiento (95-99.5%)
- 🔴 **Breached**: Incumple SLA objetivo (< 95%)

**Flujo de Generación de Reporte**:
1. Usuario selecciona periodo (mes/año)
2. Sistema consulta métricas agregadas
3. Calcula SLA por sede
4. Determina estado contractual
5. Genera documento PDF
6. Firma criptográficamente el documento
7. Inyecta metadatos de integridad
8. Ofrece descarga del PDF firmado

---

## 📊 Cálculo de SLA

### Fórmula de Disponibilidad

```
SLA (%) = (Tiempo Operativo / Tiempo Total) × 100

Donde:
- Tiempo Operativo = Total de minutos del periodo - Tiempo de Inactividad
- Tiempo Total = Número de días del periodo × 24 × 60
```

### Ejemplo de Cálculo

Para un mes de 30 días:
- **Tiempo Total**: 30 × 24 × 60 = 43,200 minutos
- **Tiempo de Inactividad**: 120 minutos (2 horas)
- **Tiempo Operativo**: 43,200 - 120 = 43,080 minutos
- **SLA**: (43,080 / 43,200) × 100 = 99.72%

### Umbrales Contractuales

| Estado | Rango de SLA | Acción Requerida |
|--------|--------------|------------------|
| Compliant | ≥ 99.5% | Ninguna |
| At Risk | 95% - 99.5% | Monitoreo intensivo |
| Breached | < 95% | Plan de mejora |

---

## 🔐 Firma Criptográfica

### Proceso de Firma

1. **Generación de PDF**: Sistema crea documento PDF
2. **Cálculo de Hash**: SHA-256 del contenido del PDF
3. **Inyección de Metadatos**: Hash + timestamp + operador
4. **Registro en Auditoría**: Hash guardado en log inmutable
5. **Verificación**: Cualquier alteración invalida el documento

### Metadatos Inyectados

```json
{
  "document_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
  "timestamp": "2026-06-02T12:00:00Z",
  "operator_ip": "192.168.1.100",
  "operator_id": "admin@lab3030.bo",
  "report_id": "SLA-2026-06-TRINIDAD",
  "algorithm": "SHA-256"
}
```

### Verificación de Integridad

Para verificar que un PDF no ha sido manipulado:

1. Extraer hash del metadato del documento
2. Recalcular hash del contenido actual
3. Comparar ambos hashes
4. Si coinciden → Documento válido
5. Si no coinciden → Documento manipulado

---

## 📈 Análisis de Datos

### Consolidación de Métricas

El sistema procesa millones de registros de telemetría mensuales:

```
Entrada: 4,200,000 registros de telemetría
  ↓
Agregación SQL (SUM, COUNT, AVG)
  ↓
Salida: ~100 registros agregados por sede
```

### Consultas de Agregación

**Uptime por Sede**:
```sql
SELECT 
    sede_id,
    SUM(CASE WHEN status = 'online' THEN 1 ELSE 0 END) as uptime_events,
    SUM(CASE WHEN status = 'offline' THEN 1 ELSE 0 END) as downtime_events,
    AVG(latency_ms) as avg_latency,
    AVG(bandwidth_mbps) as avg_bandwidth
FROM telemetry
WHERE month = '06' AND year = 2026
GROUP BY sede_id
```

**Cálculo de Minutos de Inactividad**:
```sql
SELECT 
    sede_id,
    SUM(downtime_duration_minutes) as total_downtime_minutes
FROM downtime_events
WHERE month = '06' AND year = 2026
GROUP BY sede_id
```

---

## 🎨 Estructura del Reporte PDF

### Encabezado

```
┌─────────────────────────────────────────────────┐
│  GOBERNACIÓN DEL BENI                            │
│  Dirección de Telecomunicaciones                 │
│                                                  │
│  REPORTE DE SLA - JUNIO 2026                    │
│  Sede: Trinidad                                 │
└─────────────────────────────────────────────────┘
```

### Resumen Ejecutivo

- **Disponibilidad**: 99.72%
- **Tiempo de Inactividad**: 120 minutos (2 horas)
- **Estado Contractual**: Compliant ✅
- **Incidentes Críticos**: 3
- **Latencia Promedio**: 45ms
- **Ancho de Banda Promedio**: 85 Mbps

### Detalles de Incidentes

| Fecha | Hora | Tipo | Duración | Descripción |
|-------|------|------|----------|-------------|
| 2026-06-15 | 10:30 | Conectividad | 45 min | Caída de enlace principal |
| 2026-06-22 | 14:15 | Rendimiento | 30 min | Degradación de ancho de banda |
| 2026-06-28 | 08:00 | Dispositivo | 45 min | Fallo de router |

### Gráficos

- Gráfico de disponibilidad diaria
- Tendencia de latencia mensual
- Distribución de incidentes por tipo

### Pie de Firma

```
┌─────────────────────────────────────────────────┐
│  Hash de Integridad: e3b0c44298fc1c149afbf4c...  │
│  Generado: 2026-06-02 12:00:00 UTC               │
│  Operador: admin@lab3030.bo                      │
│                                                  │
│  ____________________    ____________________     │
│  Director Técnico        Director de TI          │
└─────────────────────────────────────────────────┘
```

---

## 🔗 Integraciones

### Dependencias del Sistema

- **Módulo de Monitoreo**: Fuente de datos de telemetría
- **Módulo de Auditoría**: Registro de hashes de documentos
- **Módulo de Seguridad**: Verificación de integridad
- **Módulo de Sistema**: Workers procesan agregación

### APIs Externas

- **PDF Renderer**: Generación de documentos PDF
- **Crypto Library**: Cálculo de hashes SHA-256
- **Database**: Almacenamiento de métricas agregadas

---

## 📈 Casos de Uso

### Caso 1: Generación de Reporte Mensual

1. Usuario accede a sección de Reportes
2. Selecciona mes "Junio" y año "2026"
3. Sistema carga resumen de todas las sedes
4. Usuario ve que Trinidad tiene 99.72% SLA (Compliant)
5. Hace clic en "Generar" para Trinidad
6. Sistema genera PDF con firma criptográfica
7. Usuario descarga documento firmado
8. Documento puede ser verificado en cualquier momento

### Caso 2: Análisis de Incumplimiento

1. Usuario selecciona periodo con sedes en estado "Breached"
2. Sistema muestra alerta visual de incumplimiento
3. Usuario hace clic en sede Riberalta (94.15% SLA)
4. Revisa detalles de incidentes en el periodo
5. Identifica que hubo 3 caídas prolongadas
6. Usa información para planificar mejoras
7. Genera reporte para proveedor de servicios

### Caso 3: Verificación de Integridad

1. Auditor externo solicita verificación de reporte
2. Usuario sube PDF al sistema de verificación
3. Sistema extrae hash del metadato
4. Recalcula hash del contenido
5. Compara ambos hashes
6. Confirma que documento es válido (no manipulado)
7. Genera certificado de verificación

---

## 🚨 Procedimientos de Emergencia

### Documento Manipulado

1. Sistema detecta desajuste de hash
2. Marca documento como "No Válido"
3. Alerta a administradores
4. Genera nuevo reporte con datos correctos
5. Investiga causa de manipulación
6. Documenta incidente en auditoría

### Error en Generación de Reporte

1. Usuario reporta error al generar PDF
2. Verificar disponibilidad de datos del periodo
3. Revisar logs de generación de PDF
4. Si es error de renderer, usar alternativa
5. Si es error de datos, corregir en base de datos
6. Reintentar generación
7. Documentar solución

---

## 📚 Referencias

- [Roadmap del Módulo de Reportes](../roadmap/ROADMAP_MODULO_9_REPORTES.md)
- [Documentación de Cálculo de SLA](../reports/sla-calculation.md)
- [Guía de Verificación de Documentos](../reports/verification.md)

---

**Última actualización**: Junio 2026
**Versión**: 1.0.0
