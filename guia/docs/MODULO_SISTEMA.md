# ⚙️ Módulo de Sistema

## 🎯 Descripción General

El **Módulo de Sistema** proporciona las herramientas de administración y configuración necesarias para mantener el funcionamiento óptimo del sistema Lab 3030. Este módulo gestiona tareas en segundo plano, configuración de workers, documentación de infraestructura y notificaciones del sistema.

---

## 🏛️ Componentes del Módulo

### 1. Workers

**Propósito**: Monitoreo y gestión de tareas en segundo plano que procesan datos asíncronamente.

**Funcionalidades**:
- Visualización de workers activos
- Estado de ejecución de cada worker
- Métricas de rendimiento de workers
- Historial de ejecuciones
- Control de inicio/parada de workers

**Tipos de Workers**:
- **Telemetry Processor**: Procesa datos de telemetría entrantes
- **Alert Generator**: Genera alertas basadas en umbrales
- **Notification Sender**: Envía notificaciones por correo
- **Data Aggregator**: Agrega datos para reportes
- **Health Checker**: Verifica estado de componentes del sistema

**Estados del Worker**:
- 🟢 **Running**: Ejecutándose normalmente
- 🟡 **Paused**: Pausado temporalmente
- 🔴 **Stopped**: Detenido
- ⚪ **Error**: Error en ejecución

**Métricas del Worker**:
- Tiempo de ejecución
- Número de tareas procesadas
- Tasa de éxito/fallo
- Uso de memoria
- CPU consumida

**Flujo de Uso**:
1. Usuario accede a sección de workers
2. Sistema muestra lista de workers configurados
3. Usuario puede ver estado y métricas
4. Puede pausar/reiniciar workers individualmente

---

### 2. Config. Workers

**Propósito**: Configuración de intervalos, parámetros y comportamiento de workers.

**Funcionalidades**:
- Configuración de intervalos de ejecución
- Definición de parámetros por worker
- Gestión de colas de tareas
- Configuración de reintentos
- Límites de concurrencia

**Parámetros Configurables**:
- **Intervalo**: Frecuencia de ejecución (segundos)
- **Batch Size**: Número de tareas por lote
- **Max Retries**: Número máximo de reintentos
- **Timeout**: Tiempo máximo de ejecución
- **Concurrency**: Número de workers simultáneos

**Configuración por Worker**:

```json
{
  "telemetry_processor": {
    "interval": 30,
    "batch_size": 100,
    "max_retries": 3,
    "timeout": 60,
    "concurrency": 4
  },
  "alert_generator": {
    "interval": 10,
    "batch_size": 50,
    "max_retries": 5,
    "timeout": 30,
    "concurrency": 2
  }
}
```

**Flujo de Uso**:
1. Usuario accede a configuración de workers
2. Selecciona worker a configurar
3. Modifica parámetros según necesidades
4. Guarda cambios (requiere reinicio del worker)
5. Sistema valida configuración antes de aplicar

---

### 3. Infraestructura

**Propósito**: Gestión de documentación técnica, topologías de red y archivos multimedia del sistema.

**Funcionalidades**:
- Carga de archivos técnicos (PDF, imágenes, diagramas)
- Organización por categorías y sedes
- Búsqueda de documentos
- Control de versiones de archivos
- Descarga de documentos

**Tipos de Archivos**:
- **Diagramas de Red**: Topologías de conexión
- **Manuales Técnicos**: Documentación de equipos
- **Configuraciones**: Archivos de configuración de dispositivos
- **Planos**: Esquemas físicos de instalaciones
- **Fotos**: Imágenes de infraestructura

**Categorías**:
- **Topología**: Diagramas de red general
- **Equipos**: Documentación por tipo de equipo
- **Sedes**: Documentación específica por sede
- **Procedimientos**: Guías de operación
- **Políticas**: Documentos de políticas internas

**Flujo de Uso**:
1. Usuario accede a sección de infraestructura
2. Puede navegar por categorías
3. Sube nuevos archivos con metadatos
4. Busca documentos por nombre/categoría
5. Descarga archivos para consulta

---

### 4. Notificaciones

**Propósito**: Historial y gestión de alertas enviadas por el sistema a través de correo electrónico.

**Funcionalidades**:
- Historial de notificaciones enviadas
- Estado de entrega de correos
- Reenvío de notificaciones fallidas
- Filtrado por tipo y fecha
- Análisis de estadísticas de envío

**Información de Notificación**:
- ID único de notificación
- Tipo de alerta
- Destinatarios
- Fecha y hora de envío
- Estado de entrega
- Contenido del mensaje
- Intentos de reenvío

**Estados de Notificación**:
- ✅ **Enviada**: Entregada exitosamente
- ⏳ **Pendiente**: En cola para envío
- ❌ **Fallida**: Error en envío
- 🔄 **Reintentando**: Reenviando después de fallo

**Tipos de Notificaciones**:
- **Alerta Crítica**: Incidentes severos
- **Alerta de Advertencia**: Degradación de servicio
- **Alerta de Información**: Actualizaciones de estado
- **Reporte Generado**: Reportes disponibles
- **Mantenimiento**: Ventanas de mantenimiento programado

**Flujo de Uso**:
1. Usuario accede a sección de notificaciones
2. Sistema muestra historial de envíos
3. Puede filtrar por criterios
4. Al hacer clic, ve detalles del mensaje
5. Puede reenviar notificaciones fallidas

---

## 🔄 Arquitectura de Workers

### Modelo de Ejecución

```
┌─────────────────────────────────────────────────────────┐
│                 Queue de Tareas                          │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
   ┌────▼────┐  ┌───▼────┐  ┌───▼────┐
   │Worker 1 │  │Worker 2│  │Worker 3│
   └────┬────┘  └───┬────┘  └───┬────┘
        │           │           │
        └───────────┼───────────┘
                    │
           ┌────────▼────────┐
           │  Base de Datos  │
           └─────────────────┘
```

### Ciclo de Vida de una Tarea

1. **Inserción**: Tarea se agrega a la cola
2. **Asignación**: Worker disponible toma la tarea
3. **Procesamiento**: Worker ejecuta la tarea
4. **Resultado**: Éxito o fallo
5. **Persistencia**: Resultado se guarda en DB
6. **Limpieza**: Tarea se marca como completada

---

## 📊 Métricas del Sistema

### Indicadores de Workers

| Métrica | Descripción | Objetivo |
|---------|-------------|----------|
| Throughput | Tareas procesadas por minuto | > 1000/min |
| Latencia | Tiempo promedio de procesamiento | < 500ms |
| Error Rate | Porcentaje de tareas fallidas | < 1% |
| Queue Size | Número de tareas pendientes | < 1000 |
| Uptime | Tiempo de actividad del worker | > 99% |

### Indicadores de Notificaciones

| Métrica | Descripción | Objetivo |
|---------|-------------|----------|
| Delivery Rate | Porcentaje de correos entregados | > 95% |
| Delivery Time | Tiempo promedio de entrega | < 30s |
| Bounce Rate | Porcentaje de correos rechazados | < 5% |
| Retry Rate | Porcentaje de reenvíos necesarios | < 10% |

---

## 🛠️ Configuración

### Configuración SMTP

Para el envío de notificaciones, se requiere configurar el servidor SMTP:

```env
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USER=noreply@lab3030.bo
SMTP_PASS=your-password
SMTP_FROM=Lab 3030 <noreply@lab3030.bo>
SMTP_TLS=true
```

### Configuración de Almacenamiento

Para el módulo de infraestructura, se configura el directorio de almacenamiento:

```env
INFRASTRUCTURE_STORAGE_PATH=./data/infrastructure
MAX_FILE_SIZE=50MB
ALLOWED_EXTENSIONS=pdf,png,jpg,jpeg,svg,docx,xlsx
```

---

## 🔗 Integraciones

### Dependencias del Sistema

- **Módulo de Monitoreo**: Workers procesan telemetría y generan alertas
- **Módulo de Seguridad**: Workers verifican eventos de seguridad
- **Módulo de Reportes**: Workers agregan datos para reportes
- **Módulo de Auditoría**: Todas las acciones de workers se registran

### Servicios Externos

- **SMTP Server**: Envío de notificaciones por correo
- **File System**: Almacenamiento de archivos de infraestructura
- **Database**: Persistencia de configuración y estado

---

## 📈 Casos de Uso

### Caso 1: Configuración de Worker de Telemetría

1. Usuario accede a Config. Workers
2. Selecciona "Telemetry Processor"
3. Ajusta intervalo de 30 a 15 segundos
4. Aumenta batch size de 100 a 200
5. Guarda configuración
6. Sistema reinicia worker con nueva configuración
7. Verifica que worker procesa más rápido

### Caso 2: Carga de Documento de Topología

1. Usuario accede a sección de Infraestructura
2. Selecciona categoría "Topología"
3. Clic en "Subir Archivo"
4. Selecciona diagrama de red PDF
5. Agrega metadatos (nombre, descripción, sede)
6. Sube archivo
7. Sistema valida y almacena archivo
8. Documento aparece en lista y es searchable

### Caso 3: Reenvío de Notificación Fallida

1. Usuario accede a sección de Notificaciones
2. Filtra por estado "Fallida"
3. Encuentra notificación crítica no entregada
4. Hace clic en "Reenviar"
5. Worker de notificaciones reintenta envío
6. Sistema actualiza estado a "Enviada"
7. Confirma entrega al destinatario

---

## 🚨 Procedimientos de Emergencia

### Worker Atascado

1. Identificar worker con estado "Error"
2. Revisar logs del worker
3. Si es error transitorio, reiniciar worker
4. Si es error persistente, revisar configuración
5. Si es error de código, notificar a desarrolladores
6. Documentar incidente en auditoría

### Cola de Tareas Saturada

1. Verificar tamaño de cola en dashboard
2. Si > 1000 tareas pendientes, aumentar concurrencia
3. Si persiste, verificar rendimiento de base de datos
4. Considerar agregar más workers
5. Monitorear hasta que cola se normalice
6. Documentar ajustes realizados

### Fallo en Envío de Notificaciones

1. Verificar configuración SMTP
2. Probar conexión con servidor SMTP
3. Revisar credenciales de autenticación
4. Verificar que servidor no esté en lista negra
5. Si es problema del proveedor, usar SMTP alternativo
6. Reenviar notificaciones fallidas manualmente

---

## 📚 Referencias

- [Roadmap del Módulo de Sistema](../roadmap/ROADMAP_MODULO_8_TAREA_SEGUNDO_PLANO_AUTOMATIZACION.md)
- [Documentación de Workers](../workers/README.md)
- [Guía de Configuración SMTP](../configuracion/smtp.md)

---

**Última actualización**: Junio 2026
**Versión**: 1.0.0
