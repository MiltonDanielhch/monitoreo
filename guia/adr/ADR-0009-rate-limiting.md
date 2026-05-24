# Resumen — ADR 0009: Rate Limiting con tower-governor

**Autores:** Milton Hipamo / Laboratorio 3030

**Relacionado con:** ADR 0003 (Axum + Tower), ADR 0006 (RBAC), ADR 0007 (Errores), ADR 0008 (Seguridad)

---

## 📋 Contexto y Decisión

**Problema:** La API expuesta en el entorno regional es vulnerable a ataques de fuerza bruta por diccionario, denegación de servicio (DoS) por inundación de peticiones, scraping automatizado de la topología de red y agotamiento de recursos de CPU provocado por llamadas maliciosas masivas a los endpoints de autenticación que procesan el algoritmo criptográfico pesado `argon2id` (~200ms por iteración).

**Decisión:** Implementar **`tower-governor`** como la capa de middleware única y centralizada de Rate Limiting, perfectamente acoplada al ecosistema de Axum y Tower. El control de flujo se divide estratégicamente en tres políticas:

* Límites severos y con ventanas de tiempo extendidas en endpoints críticos de identidad.
* Límites de ráfagas dinámicos para el consumo ordinario de la API del dashboard.
* Exclusión explícita y de cero coste para rutas internas de monitorización y documentación.

---

## 🏗️ Arquitectura de Intercepción de Tráfico

```
  [ Cliente Externo ]
         │
         ▼
  [ Caddy Proxy Reverso ]   ──► Inyecta: X-Real-IP / X-Forwarded-For
         │
         ▼
  [ tower-governor Middleware ]
         │
         ├──► Política Auth (Ventana por minuto, ráfaga milimétrica)
         ├──► Política API General (Ventana por segundo, alta concurrencia)
         └──► bypass: Rutas Excluidas (/health, /docs)
         │
         ▼
  [ Handlers Axum (Rust 1.95) ]

```

---

## ⚡ Configuración Estricta de Límites (Código 3026)

### 1. Capa de Identidad (Protección Anti Fuerza Bruta)

| Endpoint Destino | Límite Base | Ráfaga (Burst) | Objetivo de Seguridad |
| --- | --- | --- | --- |
| `/auth/register` | 5 / 15 min | 3 | Mitigación de spam automatizado de cuentas. |
| `/auth/login` | 10 / 5 min | 5 | Bloqueo radical de ataques de fuerza bruta. |
| `/auth/forgot-password` | 3 / 60 min | 2 | Bloqueo de enumeración de cuentas y spam de correo. |
| `/auth/reset-password` | 5 / 15 min | 3 | Prevención de secuestro de flujos de recuperación. |
| `/auth/refresh` | 20 / 1 min | 10 | Soporte para alta concurrencia legítima de UI. |

> **Nota de Diseño:** Los límites de identidad se evalúan en ventanas de minutos. La combinación de la latencia controlada de `argon2id` con la restricción GCRA de `tower-governor` vuelve económicamente inviable un ataque de fuerza bruta por software.

### 2. Capa de API Autenticada General

| Prefijo de Ruta | Límite Base | Ráfaga (Burst) | Objetivo de Seguridad |
| --- | --- | --- | --- |
| `/api/v1/*` | 30 / 60 s | 20 | Consumo fluido del dashboard en tiempo real. |

### 3. Rutas Excluidas Excluyentes (Bypass)

| Endpoint | Justificación Operativa |
| --- | --- |
| `/health` | Monitoreo constante de disponibilidad por el orquestador (Evita falsos positivos). |
| `/docs` | Renderizado de la interfaz de usuario para documentación técnica. |
| `/openapi.json` | Consumo estático del esquema OpenAPI del sistema. |

---

## 🔧 Integración Técnica en Axum

La segmentación se realiza aislando los contextos de enrutamiento mediante la aplicación selectiva de capas (`.layer`):

```rust
// Aislamiento de rutas de autenticación con rate limiting específico
let auth_routes = Router::new()
    .route("/auth/register", post(register_handler))
    .layer(auth_register_rate_limit())
    .route("/auth/login", post(login_handler))
    .layer(auth_login_rate_limit());

// Rutas protegidas del ecosistema de la API
let api_routes = Router::new()
    .route("/api/v1/devices", get(list_devices))
    .layer(auth_middleware) // Validación PASETO v4 Local
    .layer(api_general_rate_limit());

// Servicios públicos sin coste de Rate Limiting
let app = Router::new()
    .merge(auth_routes)
    .merge(api_routes)
    .route("/health", get(health_handler))
    .into_make_service_with_connect_info::<SocketAddr>(); // Requisito ineludible para la captura de red

```

---

## 📤 Estructura de Respuesta HTTP 429 Unificada

En estricta conformidad con la arquitectura de manejo de errores fijada en el **ADR 0007**, cuando un cliente excede su cuota, el middleware intercepta la petición y devuelve los encabezados semánticos de red junto con el payload JSON estandarizado:

```http
HTTP/1.1 429 Too Many Requests
Content-Type: application/json
Retry-After: 30
x-ratelimit-limit: 30
x-ratelimit-remaining: 0
x-ratelimit-after: 30

```

```json
{
  "error": "RATE_LIMITED",
  "message": "Se ha excedido el límite de peticiones permitido. Por favor, intente de nuevo más tarde.",
  "details": {
    "retry_after_seconds": 30
  },
  "request_id": "uuid-generado-por-el-middleware-de-trazabilidad"
}

```

---

## 🌐 Extracción de Identidad de Red (Detrás de Caddy)

Para evitar el "Efecto Embudo" (donde todas las peticiones externas parecen provenir de la IP local del proxy inverso `127.0.0.1`, causando un bloqueo masivo accidental a usuarios legítimos), la directiva de Caddy debe forzar el reenvío de la identidad de red real:

```caddyfile
# Configuración del proxy en producción
reverse_proxy localhost:8080 {
    header_up X-Real-IP {remote_host}
    header_up X-Forwarded-For {remote_host}
}

```

El backend procesa esta información utilizando `SmartIpKeyExtractor` configurado para priorizar `ClientIpSource::XRealIp`, aislando las cuotas de peticiones de manera atómica por cada dirección IP externa real.

---

## 🛠️ Herramientas de Control de Flujo Aprobadas

| Herramienta / Crate | Versión | Propósito en el Ecosistema | Justificación / Mecanismo |
| --- | --- | --- | --- |
| `tower-governor` | `0.8.x` | Middleware GCRA para Tower/Axum. | Provee la integración directa con los routers de Axum sin degradar el rendimiento por hilos. |
| `axum-client-ip` | `1.3.x` | Extracción segura de IPs reales. | Decodifica de forma segura los encabezados de proxies reversos protegiendo contra IP-Spoofing. |
| `governor` | `0.10.x` | Algoritmo GCRA subyacente. | Algoritmo genérico de tasa de celdas virtuales (GCRA). Almacena el estado en tipos primitivos `AtomicU64` con operaciones CAS (*Compare-And-Swap*), libre de locks pesados. |
| `tracing` | workspace | Telemetría estructural. | Registra alertas críticas cuando una IP entra en estado de bloqueo prolongado para auditoría forense. |