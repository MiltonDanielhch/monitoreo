# ADR 0021: Local-First y Sincronización Offline

| Campo | Valor |
| --- | --- |
| **Autores** | Milton Hipamo / Laboratorio 3030 |
| **Relacionado con** | ADR 0001 (Hexagonal), ADR 0004 (MySQL 8.0), ADR 0006 (Sea-ORM), ADR 0017 (SvelteKit), ADR 0020 (Monitoreo Regional), ADR 0022 (Agentes Distribuidos) |

---

## 📋 Contexto y Decisión

**Problema:** Las sedes regionales de la Gobernación del Beni operan en entornos con conectividad WAN deficiente, propensos a microcortes prolongados y latencias severas. El personal técnico debe interactuar con el inventario, gestionar alertas y visualizar topologías locales de manera ininterrumpida, incluso en ausencia total de internet.

**Decisión:** Adoptar una arquitectura de datos **Local-First** regida bajo las siguientes directrices:

1. El cliente web (SvelteKit) delega su estado de lectura persistente a una base de datos **SQLite compilada en WebAssembly** dentro del navegador.
2. Todas las acciones de escritura mutables realizadas offline se encolan de manera imperativa en una tabla local de operaciones pendientes.
3. Al restablecerse el enlace de red, un motor de sincronización concilia los lotes diferidos con el servidor central.
4. Los demonios de captura periféricos (ADR 0022) implementan su propio buffering aislado mediante SQLite embebido en Rust.

---

## 🏗️ Arquitectura de Sincronización de Datos

```
             Frontend Web (Svelte 5 / Runes)
                          |
     =====================|=====================
     |            SQLite Wasm Core             |
     |  ├─ cache_dispositivos, cache_sedes     |
     |  └─ sync_queue (Cola de Mutaciones)     |
     ===========================================
                          |
             Sync Engine (TypeScript)
       [apps/web/src/lib/sync/ — Controladores]
                          |
        HTTPS / REST (Batch Push & Delta Pull)
                          |
                          ↓
             API en Axum 0.8 (apps/api)
                          |
            Capa de Datos (Sea-ORM 1.1.x)
                          |
                          ↓
         Base de Datos Central (MySQL 8.0)
       [Tablas de negocio + sync_log operativo]

```

> 📌 **Aclaración de Fronteras:** El subproyecto `crates/sync/` escrito en Rust maneja con exclusividad la lógica de transporte de métricas del agente de red. La sincronización interactiva de la interfaz de usuario se procesa nativamente en TypeScript dentro de `apps/web/src/lib/sync/`.

---

## 📝 Especificación de la Cola de Sincronización

### Firma de Datos de Mutación (TypeScript)

```typescript
interface SyncOperation {
  id: string;           // UUID v7 secuencial temporal generado en cliente (uuid@11.0.0)
  type: 'create' | 'update' | 'acknowledge' | 'resolve';
  entity: 'device' | 'sede' | 'alert' | 'intrusion';
  entity_id: string;
  payload: Record<string, unknown>;
  timestamp: number;    // Epoch milisegundos del cliente
  retry_count: number;
  status: 'pending' | 'syncing' | 'failed' | 'resolved';
}

```

* **Exclusión de Eliminaciones:** No existen operaciones de tipo `delete`. Toda remoción se asimila como un comando `update` con la marca de tiempo asignada al campo `deleted_at` para respetar el estándar de Soft Delete (ADR 0006).

---

## ⚔️ Políticas de Resolución de Conflictos

La estrategia global por defecto se define como **Last-Write-Wins (LWW)** analizando las marcas de tiempo. Las excepciones lógicas se segmentan por la naturaleza del dominio:

| Entidad de Dominio | Estrategia de Resolución | Justificación Operativa |
| --- | --- | --- |
| `devices` / `sedes` | **Last-Write-Wins (LWW)** | Prevalece el último cambio físico registrado en campo. |
| `alerts` | **Remote Wins (Servidor)** | El motor de persistencia central y tokio jobs dictan la verdad analítica. |
| `metric_readings` | **Append-Only (Adición)** | Lecturas inmutables históricas de telemetría; no sufren modificaciones. |
| `intrusions` | **Remote Wins (Servidor)** | Criterio de seguridad restrictivo; el servidor evalúa la whitelist global. |

---

## 🗄️ Esquema DDL Relacional Local (Client-Side)

```sql
-- Cache local de activos de red
CREATE TABLE devices_cache (
    id TEXT PRIMARY KEY,
    hostname TEXT NOT NULL,
    ip TEXT,
    mac TEXT,
    device_type TEXT,
    status TEXT,
    sede_id TEXT,
    last_seen_at INTEGER,
    updated_at INTEGER,
    is_dirty BOOLEAN DEFAULT FALSE
);

-- Cola transaccional de operaciones pendientes
CREATE TABLE sync_queue (
    id TEXT PRIMARY KEY,
    op_type TEXT NOT NULL CHECK(op_type IN ('create','update','acknowledge','resolve')),
    entity TEXT NOT NULL CHECK(entity IN ('device','sede','alert','intrusion')),
    entity_id TEXT NOT NULL,
    payload TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    retry_count INTEGER DEFAULT 0,
    status TEXT DEFAULT 'pending' CHECK(status IN ('pending','syncing','failed','resolved')),
    error_message TEXT
);

-- Inicialización de variables de control de sincronización
CREATE TABLE sync_meta (key TEXT PRIMARY KEY, value TEXT);
INSERT INTO sync_meta VALUES ('last_sync', '0');

```

---

## 🔐 Estado de Sesión y Autenticación Desconectada

Para mitigar la indisponibilidad de llamadas de validación hacia el backend, se implementa un **Periodo de Gracia de 24 horas** en los tokens PASETO v4:

* El secreto del Refresh Token se almacena de forma encriptada en la base de datos local del navegador.
* Si el cliente detecta estado desconectado, el motor permite la navegación de lectura completa sobre los datos cacheados y admite la inserción de operaciones en la `sync_queue`.
* Las mutaciones administrativas de alta jerarquía (como alteración de roles o purga de inventario) quedan explícitamente bloqueadas hasta que el endpoint `/api/v1/sync/push` valide la firma criptográfica en línea.

---

## ⚙️ Aislamiento y Configuración del Bundler (Vite)

Para posibilitar el uso del Origin Private File System (OPFS) de alto rendimiento y el subprocesamiento con hilos compartidos, es imperativo forzar el aislamiento del contexto web mediante cabeceras HTTP:

```typescript
// vite.config.ts
export default defineConfig({
  server: {
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
  },
  optimizeDeps: { exclude: ['@sqlite.org/sqlite-wasm'] },
});

```

---

## 📦 Manifiesto Estricto de Dependencias (package.json)

```json
{
  "dependencies": {
    "@sqlite.org/sqlite-wasm": "3.45.0",
    "sql.js": "1.10.3",
    "uuid": "11.0.0"
  }
}

```

> 🔄 `sql.js` se acopla como el **fallback automático en memoria** del sistema de persistencia en caso de ejecutarse en navegadores heredados que carezcan de soporte nativo para primitivas OPFS.

---

## ✅ Pros vs. ⚠️ Trade-offs

### Ventajas Centrales

* **Tolerancia Absoluta a Cortes de Red:** La aplicación no interrumpe su flujo visual ni arroja errores de conexión fatales ante las caídas de enlaces WAN en el Beni.
* **Velocidad de Respuesta Sub-milisección:** Las consultas de rendering leen directamente del almacenamiento Wasm localizado en la memoria del cliente.
* **Ordenación Confiable:** La adopción de UUID v7 basados en tiempo garantiza que los eventos encolados se fusionen en el servidor central respetando la secuencia exacta en que ocurrieron.

### Limitaciones de Diseño

* **Sobrecarga del Bundle Inicial:** La inclusión de binarios Wasm inyecta aproximadamente 450KB de peso al cliente web.
* *Mitigación:* Carga diferida (*lazy loading*) controlada del módulo de base de datos únicamente al inicializar el core del dashboard técnico.


* **Desfase de Datos Temporales:** Riesgo latente de operar con datos modificados concurrentemente en otra sede regional.
* *Mitigación:* Marcado visual explícito de registros asíncronos en la UI mediante un indicador de "Estado Pendiente de Sincronización" (`is_dirty`).