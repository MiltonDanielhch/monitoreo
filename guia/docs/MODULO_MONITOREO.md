# 📊 Módulo de Monitoreo

## 🎯 Descripción General

El **Módulo de Monitoreo** es el corazón del sistema Lab 3030, proporcionando supervisión en tiempo real de toda la infraestructura de red de la Gobernación del Beni. Este módulo permite visualizar el estado de sedes regionales, dispositivos de red, métricas de rendimiento y alertas críticas desde una interfaz centralizada.

---

## 🏛️ Componentes del Módulo

### 1. Inicio (Dashboard Principal)

**Propósito**: Panel de control centralizado que proporciona una visión general del estado de toda la red.

**Funcionalidades**:
- Resumen de estado de todas las sedes
- Métricas agregadas en tiempo real
- Alertas críticas destacadas
- Gráficos de tendencias de rendimiento
- Estado general de la infraestructura

**Datos Mostrados**:
- Total de sedes activas/inactivas
- Número de dispositivos monitoreados
- Promedio de latencia de red
- Porcentaje de disponibilidad global
- Alertas activas por severidad

**Flujo de Uso**:
1. Usuario accede al dashboard principal
2. Sistema carga métricas en tiempo real
3. Se muestran widgets con información agregada
4. Usuario puede navegar a detalles específicos

---

### 2. Sedes

**Propósito**: Gestión y supervisión de sedes regionales distribuidas geográficamente.

**Funcionalidades**:
- Listado de todas las sedes regionales
- Estado de conectividad por sede
- Información de ubicación y contacto
- Métricas específicas por sede
- Historial de incidentes por sede

**Datos de Sede**:
- Nombre y código de identificación
- Ubicación geográfica (coordenadas)
- Dirección física
- Responsable de contacto
- Estado de conectividad
- Última actualización de telemetría

**Estados de Sede**:
- 🟢 **Conectada**: Telemetría recibida recientemente
- 🟡 **Inestable**: Conectividad intermitente
- 🔴 **Desconectada**: Sin comunicación por > 5 minutos
- ⚪ **Desconocida**: Sin datos históricos

**Flujo de Uso**:
1. Usuario navega a sección de sedes
2. Sistema muestra lista de todas las sedes
3. Usuario puede filtrar por estado/ubicación
4. Al hacer clic en una sede, se muestran detalles específicos

---

### 3. Dispositivos

**Propósito**: Monitoreo individual de dispositivos de red (routers, switches, servidores, etc.).

**Funcionalidades**:
- Inventario de dispositivos por sede
- Estado operativo de cada dispositivo
- Métricas de rendimiento por dispositivo
- Configuración de umbrales de alerta
- Historial de eventos por dispositivo

**Tipos de Dispositivos**:
- **Routers**: Enrutadores de red
- **Switches**: Conmutadores de red
- **Servers**: Servidores de aplicaciones
- **Firewalls**: Cortafuegos de seguridad
- **Access Points**: Puntos de acceso WiFi

**Métricas por Dispositivo**:
- Estado (online/offline)
- Uso de CPU
- Consumo de memoria
- Tráfico de red (entrada/salida)
- Temperatura (si disponible)
- Tiempo de actividad (uptime)

**Flujo de Uso**:
1. Usuario selecciona una sede
2. Sistema muestra dispositivos de esa sede
3. Usuario puede ver detalles de cada dispositivo
4. Se muestran métricas en tiempo real

---

### 4. Métricas

**Propósito**: Visualización detallada de indicadores de rendimiento de la red.

**Funcionalidades**:
- Gráficos de tendencias temporales
- Comparación entre sedes
- Análisis de patrones de uso
- Exportación de datos históricos
- Configuración de periodos de análisis

**Tipos de Métricas**:
- **Ping**: Tiempo de respuesta ICMP
- **Latencia**: Retardo en la transmisión de datos
- **Pérdida de Paquetes**: Porcentaje de paquetes perdidos
- **Ancho de Banda**: Capacidad de transferencia de datos
- **Jitter**: Variación en la latencia

**Umbrales Configurables**:
- **Normal**: Métrica dentro de rangos aceptables
- **Advertencia**: Métrica cerca de límites críticos
- **Crítico**: Métrica supera límites establecidos

**Flujo de Uso**:
1. Usuario selecciona tipo de métrica
2. Define periodo de análisis
3. Sistema genera gráficos y estadísticas
4. Usuario puede exportar datos

---

### 5. Alertas

**Propósito**: Gestión de incidentes críticos y notificaciones de problemas de red.

**Funcionalidades**:
- Listado de alertas activas
- Historial de alertas resueltas
- Filtrado por severidad y tipo
- Asignación de responsables
- Seguimiento de resolución

**Severidad de Alertas**:
- 🔴 **Crítica**: Impacto severo en operaciones
- 🟠 **Alta**: Degradación significativa
- 🟡 **Media**: Problema menor
- 🔵 **Baja**: Información solamente

**Tipos de Alertas**:
- **Conectividad**: Pérdida de enlace
- **Rendimiento**: Degradación de métricas
- **Dispositivo**: Fallo de hardware
- **Seguridad**: Intento de intrusión
- **Capacidad**: Límites de recursos

**Ciclo de Vida de Alerta**:
1. **Detección**: Sistema identifica anomalía
2. **Generación**: Se crea alerta en el sistema
3. **Notificación**: Se envía correo electrónico
4. **Asignación**: Responsable toma el caso
5. **Resolución**: Problema es solucionado
6. **Cierre**: Alerta es marcada como resuelta

**Flujo de Uso**:
1. Usuario accede a sección de alertas
2. Sistema muestra alertas activas primero
3. Usuario puede filtrar por criterios
4. Al hacer clic, se muestran detalles
5. Usuario puede actualizar estado

---

### 6. Agentes

**Propósito**: Supervisión de conectividad y estado de agentes remotos instalados en sedes.

**Funcionalidades**:
- Estado de conectividad de agentes
- Versión de software instalado
- Última comunicación recibida
- Configuración de intervalos de reporte
- Reinicio remoto de agentes

**Información del Agente**:
- Identificador único
- Sede asignada
- Dirección IP
- Versión de software
- Sistema operativo
- Estado de conexión
- Última sincronización

**Estados del Agente**:
- 🟢 **Activo**: Reportando normalmente
- 🟡 **Inestable**: Reportes intermitentes
- 🔴 **Inactivo**: Sin comunicación
- ⚪ **Desconocido**: Nunca se conectó

**Flujo de Uso**:
1. Usuario navega a sección de agentes
2. Sistema muestra lista de todos los agentes
3. Usuario puede ver detalles de cada agente
4. Puede reiniciar agentes remotamente si es necesario

---

## 🔄 Flujo de Datos

### Recolección de Telemetría

```
Agente Remoto → API Backend → Base de Datos → Frontend
```

1. **Agente**: Recolecta métricas cada X segundos
2. **API**: Recibe datos vía endpoint `/api/v1/telemetry/ingest`
3. **Base de Datos**: Almacena métricas en tabla de telemetría
4. **Frontend**: Consulta y visualiza datos en tiempo real

### Generación de Alertas

```
Métrica → Comparación con Umbrales → Generación de Alerta → Notificación
```

1. **Métrica**: Sistema recibe nuevo dato de telemetría
2. **Comparación**: Evalúa contra umbrales configurados
3. **Generación**: Si supera umbral, crea alerta
4. **Notificación**: Envía correo SMTP a responsables

---

## 📊 Métricas Clave

### Indicadores de Rendimiento

| Métrica | Unidad | Normal | Advertencia | Crítico |
|---------|--------|--------|-------------|---------|
| Ping | ms | < 100 | 100-500 | > 500 |
| Latencia | ms | < 150 | 150-800 | > 800 |
| Pérdida de Paquetes | % | < 5 | 5-15 | > 15 |
| Ancho de Banda | Mbps | > 50 | 10-50 | < 10 |
| Jitter | ms | < 10 | 10-30 | > 30 |

### Indicadores de Disponibilidad

- **SLA Objetivo**: 99.5% mensual
- **Tiempo de Inactividad Máximo**: 3.6 horas/mes
- **Tiempo de Recuperación Objetivo (RTO)**: 1 hora
- **Punto de Recuperación Objetivo (RPO)**: 5 minutos

---

## 🛠️ Configuración

### Umbrales de Alerta

Los umbrales pueden configurarse globalmente o por sede específica:

```rust
// Ejemplo de configuración en código
let thresholds = ThresholdSettings {
    ping_ms: ThresholdValue::new(100.0, 500.0).unwrap(),
    latency_ms: ThresholdValue::new(150.0, 800.0).unwrap(),
    packet_loss_percent: ThresholdValue::new(5.0, 15.0).unwrap(),
};
```

### Intervalos de Reporte

- **Telemetría**: Cada 30 segundos (configurable)
- **Alertas**: Inmediato al detectar anomalía
- **Dashboard**: Actualización cada 5 segundos
- **Histórico**: Retención de datos por 90 días

---

## 🔗 Integraciones

### Dependencias del Sistema

- **Módulo de Sistema**: Workers procesan telemetría en segundo plano
- **Módulo de Seguridad**: Detecta anomalías que pueden ser intrusiones
- **Módulo de Reportes**: Usa datos de monitoreo para generar SLA
- **Módulo de Notificaciones**: Envía alertas por correo electrónico

### APIs Externas

- **SMTP**: Envío de notificaciones de alertas
- **NTP**: Sincronización de tiempo para timestamps precisos
- **DNS**: Resolución de nombres para conectividad

---

## 📈 Casos de Uso

### Caso 1: Detección de Caída de Enlace

1. Agente deja de reportar telemetría
2. Sistema detecta inactividad por > 5 minutos
3. Genera alerta crítica de conectividad
4. Envía notificación a responsables
5. Dashboard muestra sede en estado "Desconectada"
6. Técnico recibe alerta y comienza investigación

### Caso 2: Degradación de Rendimiento

1. Latencia de una sede supera umbral de advertencia
2. Sistema genera alerta de severidad media
3. Dashboard muestra métrica en color amarillo
4. Usuario investiga en sección de métricas
5. Identifica patrón de congestión
6. Toma acción para optimizar enlace

### Caso 3: Análisis Histórico

1. Usuario accede a sección de métricas
2. Selecciona periodo de último mes
3. Sistema genera gráficos de tendencias
4. Usuario identifica picos de uso
5. Exporta datos para reporte
6. Usa información para planificación de capacidad

---

## 🚨 Procedimientos de Emergencia

### Pérdida de Conectividad Masiva

1. Verificar estado de agentes
2. Revisar logs de sistema
3. Contactar proveedor de servicios
4. Activar procedimientos de contingencia
5. Documentar incidente en auditoría

### Alerta de Seguridad Crítica

1. Inmediatamente revisar módulo de seguridad
2. Verificar correlación de eventos
3. Si es intrusión confirmada, activar protocolo de respuesta
4. Documentar en auditoría inmutable
5. Notificar a autoridades si es necesario

---

## 📚 Referencias

- [Roadmap del Módulo de Monitoreo](../roadmap/ROADMAP_MODULO_1_SEDES.md)
- [Documentación de API](../api/README.md)
- [Guía de Configuración](../configuracion/README.md)

---

**Última actualización**: Junio 2026
**Versión**: 1.0.0
