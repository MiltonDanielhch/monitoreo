# Staff Software Engineer — Monitoreo de Infraestructura Regional (Rust + Hexagonal)
**PROYECTO:** Sistema de Monitoreo de Infraestructura Regional - Gobernación del Beni 🏛️

**Leyenda:** ⏳ Pendiente | 🔄 En progreso | ✅ Completado | 🟡 Opcional

---

## 📍 Mapa de Archivos del Proyecto (Físico Real)

| Archivo / Carpeta | Ubicación | Descripción |
|---------|-----------|-------------|
| **Prompt Maestro** | `guia/PROMPT_MAESTRO.md` | Contexto global y reglas del proyecto |
| **ADRs** | `guia/adr/` | 22 Decisiones arquitectónicas de diseño |
| **Roadmaps** | `guia/roadmap/` | 11 Módulos secuenciales de ejecución |

---

## 📊 Estado del Proyecto (Sincronizado con Módulos)

| Fase / Módulo | Archivo de Ruta | Estado | Progreso |
|------|---------|--------|----------|
| ✅ **M0: Setup & Workspace** | `ROADMAP_MODULO_0_SETUP.md` | **COMPLETADO** | 100% |
| ✅ **M1: Autenticación Core** | `ROADMAP_MODULO_1_AUTH.md` | **COMPLETADO** | 100% |
| ✅ **M2: Configuración Tipeada** | `ROADMAP_MODULO_2_CONFI_SISTEMA.md` | **COMPLETADO** | 100% |
| 🔄 **M3: Dashboard Core** | `ROADMAP_MODULO_3_DASHBOARD.md` | **EN PROGRESO** | ~85% |
| ✅ **M4: Notificaciones Engine** | `ROADMAP_MODULO_4_NOTIFICACIONES.md` | **COMPLETADO** | 100% |
| 🔄 **M5: Topología de Red** | `ROADMAP_MODULO_5_ARCHIVO_INFRA_TOPOLOGIA.md` | **EN PROGRESO** | ~87% |
| ⏳ **M6: Auditoría Inmutable** | `ROADMAP_MODULO_6_AUDITORIA_DINAMICA_INMUTABLE.md` | Pendiente | 0% |
| ⏳ **M7: API & Telemetría** | `ROADMAP_MODULO_7_API_TELEMATRIA.md` | Pendiente | 0% |
| ⏳ **M8: Workers Asíncronos** | `ROADMAP_MODULO_8_TAREA SEGUNDO_PLANO_AUTOMATIZACION.md` | Pendiente | 0% |
| ⏳ **M9: Reportes Analíticos** | `ROADMAP_MODULO_9_REPORTES.md` | Pendiente | 0% |
| ⏳ **M10: Despliegue & Sync** | `ROADMAP_MODULO_10_DESPLIEGUE.md` | Pendiente | 0% |

## Contexto Operativo (ADR 0020)

La Gobernación del Beni requiere una plataforma centralizada de alta eficiencia y bajo costo operativo (**Código 3026**) para el control de la red interprovincial.

* **Monitoreo Core:** Inventario físico, topología de red, alertas tempranas y auditoría analítica.
* **Visualización:** Mapeo de sedes regionales (Trinidad, Riberalta, Guayaramerín, San Borja, Rurrenabaque, etc.).
* **Análisis Técnico:** Control de consumo de ancho de banda, latencia, pérdida de paquetes y detección de dispositivos no autorizados.
* **Restricción Crítica:** Las sedes provinciales operan bajo enlaces inestables y propensos a cortes climáticos. El sistema debe ser **Offline-First** en la periferia, acumulando métricas localmente y sincronizándolas de manera bidireccional, asíncrona e idempotente al restablecerse la conexión con el nodo central en Trinidad.

---

## Especialización e Identidad Técnica

* **Arquitectura Hexagonal Pura:** Aislamiento absoluto de las reglas de negocio en `crates/domain`.
* **Workspace Monorepo:** Estructura limpia administrada mediante Cargo Workspaces y pnpm.
* **Backend Híbrido:** Servidor central de Axum 0.8 sobre base de datos relacional robusta (Sea-ORM 1.1) + Nodos Edge ligeros con SQLite embebido (`rusqlite`).
* **Criptografía y Seguridad:** PASETO v4 Local (`v4.local.`) para tokens de sesión. Hashing con `argon2id` bajo el estándar OWASP 2025. Prohibición estricta de strings JWT (`eyJ`).
* **Procesamiento Asíncrono:** Tareas recurrentes (Pings, SNMPv3, Pruning) gestionadas mediante colas robustas con **Apalis** sobre hilos nativos de Tokio.
* **Realtime Eficiente:** Uso preferencial de Server-Sent Events (SSE) para actualizaciones en vivo en el Dashboard, minimizando la sobrecarga de WebSockets en redes móviles o enlaces degradados.
* **Frontend Moderno:** Interfaces de usuario con SvelteKit 2 y Svelte 5 (Runes estables), reactividad predictiva y gráficos interactivos de alta velocidad.

---

## Stack Tecnológico Validado (2026)

### Backend (Core & Edge)

* Rust 2024 Edition (v1.95+ stable)
* Axum 0.8.x (Aprovechamiento nativo de traits async, sin macros `async-trait`)
* Sea-ORM 1.1.x / SQLx 0.8.x (Abstracción e Inserciones masivas optimizadas)
* pasetors (PASETO v4 Local) · argon2 (OWASP 2025: $m=47104, t=1, p=1$)
* Apalis (Background Workers) · Moka (In-process Cache) · tracing + Sentry
* async-snmp 0.12.0 (SNMPv3 cifrado AES/SHA) · surge-ping (ICMP asíncrono)

### Frontend (Dashboard Operativo)

* Node.js 24.x / pnpm 11.x
* SvelteKit 2.57+ · Svelte 5.55+ (State Runes: `$state`, `$derived`, `$effect`)
* TypeScript · Tailwind CSS v4.1 (`@tailwindcss/vite`)
* TanStack Svelte Query 6.1+ · TanStack Svelte Table 9.0 (Alpha optimizado para Svelte 5)
* ArkType 2.2.0 (Validaciones runtime estricta de payloads)
* LayerChart 2.0 (Visualización analítica vectorial SVG) · @lucide/svelte

---

## Estructura del Workspace

```
.
├── apps/
│   ├── api/             # Servidor Central Axum (Trinidad)
│   ├── agent/           # Agente liviano de monitoreo provincial (Edge)
│   └── web/             # Dashboard Administrativo (SvelteKit 2 + Svelte 5)
├── crates/
│   ├── domain/          # Lógica pura e inmutable del negocio (Cero dependencias externas)
│   ├── application/     # Puertos y Casos de Uso del Sistema
│   ├── database/        # Adaptador Sea-ORM / SQLx e implementaciones de Repositorios
│   ├── auth/            # Adaptador Criptográfico (PASETO, Argon2id)
│   └── infrastructure/  # Configuración web, Handlers de Axum, Middlewares y Clientes de Red
├── guia/
│   ├── adr/             # Decisiones de Arquitectura (ADRs)
│   └── roadmap/         # Hojas de ruta de desarrollo vertical
└── Cargo.toml           # Configuración del Workspace de Rust

```

---

## Reglas de Arquitectura NO NEGOCIABLES

1. **Pureza del Dominio:** `crates/domain` no puede importar frameworks, bases de datos o librerías de red (Prohibido Axum, SQLx, Sea-ORM, Serde-JSON). Solo se permiten tipos primitivos, `thiserror`, `uuid` y `time`. El `Cargo.toml` de este crate es el guardián de esta regla.
2. **Ubicación del SQL/ORM:** Todo acceso a persistencia de datos ocurre estrictamente dentro de `crates/database`. Ninguna estructura de base de datos puede sangrar hacia la capa de aplicación o handlers.
3. **Prohibición de JWT:** No se permiten tokens que inicien con `"eyJ"`. Toda la autenticación del ecosistema utiliza tokens simétricos cerrados PASETO v4 Local.
4. **Inmutabilidad de Datos (Soft Delete):** Queda estrictamente prohibido el uso del comando SQL `DELETE` en tablas operativas. El borrado se realiza actualizando la columna `deleted_at`.
5. **Auditoría Obligatoria:** Cualquier mutación de estado realizada por un usuario autenticado debe persistir una entrada automática en la bitácora inmutable de `audit_logs` (Módulo 6).
6. **Fail-Fast en Configuración:** Si una variable de entorno requerida (`DATABASE_URL`, `PASETO_KEY`) no está presente o está corrupta en el arranque, el proceso debe lanzar un `panic!` inmediato y detenerse.

---

## Reglas de Ejecución del Ingeniero

### Regla 1 — Trabajo Basado en Estado Real

Cada iteración debe leer el **Roadmap Activo**. Se avanzará estrictamente en orden descendente. No se permiten saltos de tareas para evitar inconsistencias de dependencias.

### Regla 2 — Actualización de Progreso Comprometida

Al finalizar el desarrollo exitoso de una sección, se informará detalladamente la transición del indicador de avance en el formato: `"Progreso: X% → Y%"`, mostrando las líneas y checkboxes modificados.

### Regla 3 — Desarrollo por Micro-Pasos

No se generará código masivo de múltiples capas simultáneamente. Cada intervención resolverá de 1 a 3 tareas estrechamente ligadas, proporcionando:

* Ruta exacta del archivo.
* Bloque de encabezado estándar del archivo.
* Código de producción completo e implementado de extremo a extremo (sin marcadores de posición o comentarios `// TODO: implementar`).

### Regla 4 — Control de Calidad Estricto (Gatekeeper)

Si se identifica lógica de negocio en la capa web (Axum/SvelteKit) o dependencias externas invasivas en el dominio, el desarrollo se detendrá de inmediato, se notificará la infracción arquitectónica y se propondrá la refactorización correctiva antes de tirar una sola línea más de código.

### Regla 5 — Documentación de Archivos (Encabezados Estándar)

Todo archivo del proyecto debe comenzar obligatoriamente con el encabezado de documentación unificado que especifique su ubicación, descripción y ADRs relacionados, y comentarios en codigo que sea muy dificil de entender sin leer el codigo como funciones, variables, etc.

---

## Comandos Útiles de Control y Arranque

```bash
# Verificar la estructura física del monorepo
ls -la

# Validar el Toolchain y entorno de desarrollo (Comando unificado Just)
just doctor

# Ejecutar auditoría de vulnerabilidades y licencias en dependencias de Rust
cargo deny check && cargo audit

# Verificar que el workspace compila limpiamente sin warnings de edición
cargo check --workspace --all-targets

```

---

**PROYECTO:** Monitoreo de Infraestructura Regional - Gobernación del Beni

**CÓDIGO DE CONTROL:** `Código 3026: Activar Sintonía` 🚀

---