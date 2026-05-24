# Resumen — ADR 0011: Estándares de Desarrollo: Ciclo Lab → Puente → Producción
**Autores:** Milton Hipamo / Laboratorio 3030

**Relacionado con:** ADR 0001 (Hexagonal), ADR 0003 (Backend), ADR 0004 (MySQL), ADR 0010 (Testing), ADR 0012 (just + lefthook)

---

## 📋 Contexto y Decisión

**Problema:** El desarrollo de software a gran escala tiende a degradarse aceleradamente debido a la acumulación de deuda técnica, compilaciones lentas, módulos gigantes repletos de responsabilidades cruzadas, procesos de despliegue manuales y arquitectura inconsistente. Esto reduce drásticamente la mantenibilidad y fractura la capacidad de una Inteligencia Artificial para co-programar de forma asertiva.

**Decisión:** Institucionalizar el ciclo iterativo **Lab → Puente → Producción** como el marco metodológico obligatorio e infranqueable del monorepo. Se establece la siguiente escala de prioridades inmutable: **Simplicidad > Mantenibilidad > Rendimiento > Observabilidad > Automatización**, rechazando cualquier patrón de diseño abstracto que no resuelva una restricción técnica o de negocio inmediata.

---

## 🔄 Las 3 Fases del Ciclo de Software

```
 [ 1. LABORATORIO ] ──► Entorno Local (bacon + TDD + refactor continuo)
         │  (Validación local instantánea mediante Lefthook Git-Hooks)
         ▼
 [ 2. EL PUENTE ]   ──► Pipeline de CI (Compilación estricta, Sea-ORM Entities, OpenAPI Drift)
         │  (Generación de artefactos binarios estáticos y ultra-optimizados)
         ▼
 [ 3. PRODUCCIÓN ]  ──► Servidor Físico (Fail-fast atómico, Inmutabilidad, Cero Compilación)

```

### 1 — LABORATORIO (Desarrollo Local y Experimentación)

Es el entorno de prototipado rápido, refactorización agresiva y desarrollo guiado por pruebas (TDD). El objetivo principal es minimizar la carga cognitiva del desarrollador y asegurar un bucle de retroalimentación (**Feedback Loop**) menor a 5 segundos utilizando herramientas nativas que no requieran reinicios pesados.

**Flujo obligatorio en la terminal local antes de realizar un Commit:**

```bash
cargo check --workspace
cargo fmt
cargo clippy --workspace
cargo nextest run

```

### 2 — EL PUENTE (Integración Continua y Automatización de Calidad)

Entorno aislado de validación automatizada en el servidor de control. Aquí se verifica la rigidez de la arquitectura hexagonal, la vigencia de los contratos de persistencia, la seguridad criptográfica del supply chain y la reproducibilidad del entorno.

**Comandos del Pipeline de Validación Unificado:**

```bash
cargo nextest run --all-targets                       # Ejecución de capas de pruebas 1, 2 y 3
cargo clippy --all-targets -- -D warnings             # Tolerancia cero a alertas o advertencias del linter
cargo deny check                                      # Auditoría estricta de licencias y fuentes de dependencias
cargo audit                                           # Detección de vulnerabilidades en el supply chain (RUSTSEC)
just check-entities                                   # Validación sintáctica de entidades de Sea-ORM vs MySQL
just check-types-drift                                # Verificación de sincronía OpenAPI vs Svelte 5 UI (Runes)

```

**El pipeline en el Puente fallará inmediatamente si:**

* Existe un solo *warning* de compilación o del linter.
* Se detecta una dependencia vulnerable o con licencias restrictivas en `cargo-deny`.
* Existe una desincronización entre el esquema OpenAPI expuesto por el backend en Rust y las interfaces de tipos TypeScript de la interfaz web.
* Un crate de la capa de Dominio rompe el aislamiento hexagonal importando dependencias web (`axum`) o de infraestructura física (`sea-orm`).

### 3 — PRODUCCIÓN (Entorno de Ejecución Soberano)

El entorno de producción bajo el estándar Código 3026 **no es un entorno de compilación**. Queda terminantemente prohibido instalar toolchains de Rust, compiladores, dependencias de node_modules de desarrollo o ejecutar tareas de construcción en caliente en el hardware definitivo.

**Principios Operativos en Producción:**

* **Fail-Fast Absoluto:** Si una variable de entorno requerida falta, una migración de Sea-ORM falla, o el pool de conexiones hacia la base de datos MySQL local no se inicializa en los primeros milisegundos, el proceso binario debe provocar un `panic!` y morir inmediatamente.
* **Despliegue Inmutable:** El servidor solo recibe y ejecuta el binario nativo previamente compilado, optimizado y validado en el Puente.
* **Trazabilidad Forense Obligatoria:** Cada petición aceptada por el servidor debe ser inyectada con un `request_id` único, propagado a través de todos los spans del sistema de logs estructurados (`tracing`), permitiendo aislar el comportamiento de un usuario o dispositivo en milisegundos.

---

## 📏 Los 8 Estándares Clave del Código 3026

### 1 — Restricciones de Atomicidad Estricta

Para garantizar un código limpio, legible y fácilmente procesable por sistemas de Inteligencia Artificial de forma asertiva, se imponen límites físicos de tamaño:

| Componente Lógico | Límite Máximo Recomendado | Acción Correctiva Obligatoria |
| --- | --- | --- |
| **Función / Método** | ~30 líneas de código real | Extraer bloques lógicos a funciones puras independientes. |
| **Archivo de Código (`.rs`)** | ~200 líneas de código | Segregar el archivo y estructurar un submódulo dedicado. |
| **Trait / Interfaz** | Responsabilidad Única | Segregación de interfaces (Interface Segregation Principle). |
| **Handler HTTP (Axum)** | Solo Orquestación | Prohibido procesar lógica de negocio; delegar al Caso de Uso. |

### 2 — Regla del Boy Scout

Todo commit aplicado al repositorio debe dejar el código en un estado más limpio, simple y eficiente de cómo fue encontrado. Se premia la eliminación de líneas duplicadas, dependencias muertas o nombres ambiguos sobre la creación de abstracciones complejas complejas.

### 3 — Código Autodocumentado y Semántico

Los comentarios en el código están reservados única y exclusivamente para responder al **"por qué"** (razones de negocio, decisiones arquitectónicas complejas o restricciones de hardware regional). Queda prohibido escribir comentarios que expliquen el **"cómo"**, lo cual debe ser evidente mediante un tipado fuerte y nombres semánticos.

### 4 — Tipos Fuertes sobre Primitivas (Anti-Primitive Obsession)

Evitar el paso de tipos genéricos que oculten el significado real de las propiedades.

* ❌ **Incorrecto:** `fn registrar_dispositivo(id: String, ip: String)`
* ✓ **Correcto:** `fn registrar_dispositivo(id: DeviceId, ip: Ipv4Addr)`

### 5 — Filosofía de Dependencias Minimalistas

Cada crate añadido al `Cargo.toml` debe justificarse exhaustivamente evaluando su impacto en el tiempo de compilación, superficie de ataque, acoplamiento y mantenimiento a largo plazo. Si un problema puede resolverse con 10 líneas de código nativo usando la librería estándar (`stdlib`), se rechaza la introducción de una dependencia externa.

### 6 — Convenciones de Estructura para Colaboración con IA

El espacio de trabajo se diseña con un determinismo estructural absoluto para que los asistentes de IA identifiquen el contexto instantáneamente:

* **Nombres Explicitos y Descriptivos:** `create_user_use_case.rs`, `mysql_user_repository.rs`. Se prohíbe el uso de archivos genéricos y ambiguos como `utils.rs`, `helpers.rs o services.rs`.
* **Un solo concepto por archivo:** Facilita la lectura, reduce los conflictos de combinación de ramas (merges) y optimiza las ventanas de contexto de la IA.

### 7 — Política de Complejidad Operacional Minimalista

Operando sobre infraestructura y servidores físicos controlados, se descartan por completo arquitecturas distribuidas sobrediseñadas. Queda despriorizado el uso de Kubernetes, mallas de servicios (service mesh), brokers de mensajería masiva distribuidos (`Kafka`) o bases de datos NoSQL introducidas sin justificación técnica de alta carga. **La simplicidad de la infraestructura reduce la deuda técnica a cero.**

### 8 — Matriz Comparativa del Ciclo de Vida (SDLC)

| Dimensión de Desarrollo | Enfoque Corporativo Tradicional | Estándar Operativo Laboratorio 3030 |
| --- | --- | --- |
| **Feedback Loop** | Horas / Días (Dependiente de aprobaciones) | Segundos (Automatizado localmente por `bacon`) |
| **Despliegue** | Manual, artesanal y propenso a fallos. | Automatizado e inmutable basado en artefactos binarios. |
| **Garantía de Calidad** | Pruebas de integración parciales o inexistentes. | Estructura piramidal cerrada de 4 capas con cargo-nextest. |
| **Documentación** | Wikis desactualizadas y aisladas. | Arquitectura viva documentada mediante ADRs y código limpio. |
| **Infraestructura** | En la nube, hiper-distribuida y costosa. | Servidor físico optimizado con bajo costo operativo. |
| **Monitoreo** | Reactivo (Post-incidente). | Proactivo e integrado desde el diseño del Handler vía tracing. |

---

## 🛠️ Herramientas de Desarrollo y Calidad Aprobadas

| Herramienta / Crate | Propósito Arquitectónico en el Workspace | Versión | Estado |
| --- | --- | --- | --- |
| `bacon` | Motor de feedback loop en tiempo real (Reemplaza a `cargo-watch`). | `3.22.x` | ✅ Activa |
| `cargo-nextest` | Runner oficial de paralelización de pruebas por procesos aislados. | `0.9.x` | ✅ Activa |
| `cargo-deny` | Validación de licencias, duplicados y orígenes de dependencias. | `0.19.x` | ✅ Activa |
| `cargo-audit` | Escaneo activo de vulnerabilidades registradas en el RUSTSEC. | `0.22.x` | ✅ Activa |
| `cargo-mutants` | Pruebas de mutación para certificar la efectividad de las aserciones. | `27.0.x` | ✅ Activa |
| `typos` | Corrector ortográfico automatizado para mantener limpio el código fuente. | `1.46.x` | ✅ Activa |
| `just` | Automatizador ejecutor de comandos del monorepo independiente del shell. | `1.51.x` | ✅ Activa |
| `lefthook` | Gestión ultra-rápida de Git-Hooks en lenguaje Go para bloqueo local. | `2.1.x` | ✅ Activa |
| `tracing` | Framework unificado de telemetría, spans y diagnóstico estructurado. | workspace | ✅ Activa |

**Nota de sintonía:** `bacon` reemplaza oficialmente a `cargo-watch` debido a su consumo eficiente de CPU en entornos de desarrollo de alta frecuencia de guardado y su renderizado óptimo para errores estructurados de Rust 2024.

---

## 📜 Mandatos Derivados Inmutables

* El compilador tiene la orden estricta de tratar los warnings como errores fatales en el Puente mediante la bandera `-D warnings`.
* `bacon` se consolida como el entorno por defecto en la terminal de desarrollo durante las jornadas de programación activa.
* Se elimina cualquier rastro o referencia a la suite offline de SQLx de los comandos de automatización de tareas. Todo el control de esquemas y validaciones pasa a través del ecosistema nativo de Sea-ORM.