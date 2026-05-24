# ADR 0011: Estándares de Desarrollo: Ciclo Lab → Puente → Producción
**Autores:** Milton Hipamo / Laboratorio 3030

**Relacionado con:** ADR 0001 (Hexagonal), ADR 0003 (Backend), ADR 0004 (MySQL), ADR 0007 (Errores), ADR 0010 (Testing)

---

## 📋 Contexto y Decisión

**Problema:** Los proyectos de software tienden a degradarse aceleradamente debido al acoplamiento de código, tiempos de compilación lentos, módulos masivos repletos de responsabilidades mezcladas, procesos de despliegue manuales y discrepancias estructurales. Esto reduce drásticamente la mantenibilidad y rompe la capacidad de la IA para co-programar de forma asertiva.

**Decisión:** Institucionalizar el ciclo iterativo **Lab → Puente → Producción** como el marco metodológico e infranqueable del monorepo. Se establece la siguiente escala de prioridades inmutable: **Simplicidad > Mantenibilidad > Rendimiento > Observabilidad > Automatización**, rechazando cualquier patrón corporativo abstracto que no resuelva un problema real inmediato.

---

## 🔄 Las 3 Fases del Ciclo de Software

```
 [ 1. LABORATORIO ] ──► Entorno Local (bacon + TDD + refactor continuo)
         │  (Validación local mediante Lefthook Git-Hooks)
         ▼
 [ 2. EL PUENTE ]   ──► Pipeline de CI (Compilación estricta, Sea-ORM, OpenAPI Drift, Cero Warnings)
         │  (Generación de artefactos binarios optimizados)
         ▼
 [ 3. PRODUCCIÓN ]  ──► Servidor Físico (Fail-fast atómico, Inmutabilidad, Cero Compilación)

```

### 1 — LABORATORIO (Desarrollo Local y Experimentación)

Es el espacio de prototipado rápido, TDD (Test-Driven Development) y refactorización agresiva. El objetivo principal es minimizar la fricción y asegurar un bucle de retroalimentación (**Feedback Loop**) menor a 5 segundos.

**Flujo obligatorio antes de realizar cualquier Commit:**

```bash
cargo check --workspace
cargo fmt
cargo clippy --workspace
cargo nextest run

```

### 2 — EL PUENTE (Integración Continua / Automatización de Calidad)

Entorno aislado de validación automatizada donde se verifica la rigidez de la arquitectura, la vigencia de los contratos, la seguridad criptográfica y la reproducibilidad del entorno.

**Comandos del Pipeline de Validación Unificado:**

```bash
cargo nextest run --all-targets                       # Ejecución de capas 1, 2 y 3
cargo clippy --all-targets -- -D warnings             # Cero tolerancia a alertas del linter
cargo deny check                                      # Auditoría de licencias y fuentes de dependencias
cargo audit                                           # Detección de vulnerabilidades en el supply chain
just check-contracts                                  # Validación sintáctica de entidades Sea-ORM
just check-types-drift                                # Verificación de sincronía OpenAPI vs Svelte 5 UI

```

**El pipeline en el Puente fallará inmediatamente si:**

* Existe un solo *warning* de compilación o del linter.
* Se detecta una dependencia vulnerable o no aprobada en `cargo-deny`.
* Existe una desincronización entre el esquema OpenAPI expuesto por el backend y las interfaces de tipos de Svelte 5.
* Un crate de la capa de Dominio rompe el aislamiento hexagonal importando dependencias web (`axum`) o de infraestructura física (`sea-orm`).

### 3 — PRODUCCIÓN (Entorno de Ejecución Soberano)

El entorno de producción bajo el estándar Código 3026 **no es un entorno de desarrollo ni de compilación**. Queda prohibido instalar toolchains de Rust, compiladores o ejecutar tareas de construcción en caliente en el servidor definitivo.

**Principios Operativos en Producción:**

* **Fail-Fast Absoluto:** Si una variable de entorno requerida falta, una migración de Sea-ORM falla, o el pool de conexiones a MySQL no se inicializa en los primeros milisegundos, el proceso binario debe provocar un `panic!` y morir inmediatamente.
* **Despliegue Inmutable:** El servidor solo recibe y ejecuta el binario nativo previamente compilado y validado en el Puente.
* **Trazabilidad Forense:** Cada petición aceptada por el servidor debe ser inyectada con un `request_id` único, propagado a través de todos los spans del sistema de logs estructurados (`tracing`).

---

## 📏 Los 8 Estándares Clave del Código 3026

### 1 — Restricciones de Atomicidad Estricta

Para garantizar un código limpio y fácilmente procesable por sistemas de Inteligencia Artificial, se imponen límites físicos de tamaño:

| Componente Lógico | Límite Máximo Recomendado | Acción Correctiva Obligatoria |
| --- | --- | --- |
| **Función / Método** | ~30 líneas de código real | Extraer bloques lógicos a funciones puras independientes. |
| **Archivo de Código (`.rs`)** | ~200 líneas de código | Segregar el archivo y estructurar un submódulo dedicado. |
| **Trait / Interfaz** | Responsabilidad Única | Segregación de interfaces (Interface Segregation Principle). |
| **Handler HTTP (Axum)** | Solo Orquestación | Prohibido procesar lógica de negocio; delegar al Caso de Uso. |

### 2 — Regla del Boy Scout

Todo commit aplicado al repositorio debe dejar el código en un estado más limpio, simple y eficiente de cómo fue encontrado. Se premia la eliminación de líneas duplicadas o muertas sobre la creación de abstracciones complejas.

### 3 — Código Autodocumentado y Semántico

Los comentarios en el código están reservados única y exclusivamente para responder al **"por qué"** (razones de negocio, decisiones arquitectónicas complejas o restricciones de hardware regional). Queda prohibido escribir comentarios que expliquen el **"cómo"**, lo cual debe ser evidente mediante un tipado fuerte y nombres semánticos.

### 4 — Tipos Fuertes sobre Primitivas (Anti-Primitive Obsession)

Evitar el paso de tipos genéricos que oculten el significado real de las propiedades.

* ❌ **Incorrecto:** `fn registrar_dispositivo(id: String, ip: String)`
* Space **Correcto:** `fn registrar_dispositivo(id: DeviceId, ip: Ipv4Addr)`

### 5 — Filosofía de Dependencias Minimalistas

Cada crate añadido al `Cargo.toml` debe justificarse exhaustivamente evaluando su impacto en el tiempo de compilación, superficie de ataque, acoplamiento y mantenimiento a largo plazo. Si un problema trivial puede resolverse con 10 líneas de código nativo usando la librería estándar (`stdlib`), se rechaza la introducción de una dependencia externa.

### 6 — Convenciones de Estructura para Colaboración con IA

El espacio de trabajo se diseña con un determinismo estructural absoluto para que los asistentes de IA identifiquen el contexto instantáneamente:

* **Nombres Explicitos y Descriptivos:** `create_user_use_case.rs`, `mysql_user_repository.rs`. Se prohíbe el uso de archivos genéricos y ambiguos como `utils.rs`, `helpers.rs` o `services.rs`.
* **Un solo concepto por archivo:** Facilita la lectura, reduce los conflictos de combinación de ramas (merges) y optimiza las ventanas de contexto de la IA.

### 7 — Política de Complejidad Operacional Minimalista

Operando sobre infraestructura y servidores físicos controlados, se descartan por completo arquitecturas distribuidas sobrediseñadas. Queda despriorizado el uso de Kubernetes, mallas de servicios (service mesh), brokers de mensajería masiva distribuidos (`Kafka`) o bases de datos NoSQL introducidas sin justificación técnica de alta carga. **La simplicidad de la infraestructura reduce la deuda técnica a cero.**

### 8 — Matriz Comparativa del Ciclo de Vida (SDLC)

| Dimensión de Desarrollo | Enfoque Corporativo Tradicional | Estándar Operativo Laboratorio 3030 |
| --- | --- | --- |
| **Feedback Loop** | Horas / Días (Dependiente de aprobaciones) | Segundos (Automatizado localmente por `bacon`) |
| **Despliegue** | Manual, artesanal y propenso a fallos. | Automatizado e inmutable basado en artefactos. |
| **Garantía de Calidad** | Pruebas de integración parciales o inexistentes. | Estructura piramidal cerrada de 4 capas. |
| **Documentación** | Wikis desactualizadas y aisladas. | Arquitectura viva documentada mediante ADRs y código. |
| **Infraestructura** | Servicios en la nube complejos y costosos. | Servidor físico optimizado con bajo costo operativo. |
| **Monitoreo** | Reactivo (Post-incidente). | Proactivo e integrado desde el diseño del Handler. |

---

## 🛠️ Herramientas de Desarrollo y Calidad Aprobadas

| Herramienta / Crate | Versión | Propósito Arquitectónico en el Workspace | Estado |
| --- | --- | --- | --- |
| `bacon` | `3.22.x` | Motor de feedback loop en tiempo real (Reemplaza a `cargo-watch`). | ✅ Activo |
| `cargo-nextest` | `0.9.x` | Runner oficial de paralelización de pruebas por procesos. | ✅ Activo |
| `cargo-deny` | `0.19.x` | Validación de licencias, duplicados y orígenes de dependencias. | ✅ Activo |
| `cargo-audit` | `0.22.x` | Escaneo activo de vulnerabilidades registradas en el RUSTSEC. | ✅ Activo |
| `cargo-mutants` | `27.0.x` | Pruebas de mutación para certificar la efectividad de las aserciones. | ✅ Activo |
| `typos` | `1.46.x` | Corrector ortográfico automatizado para mantener limpio el código fuente. | ✅ Activo |
| `just` | `1.51.x` | Automatizador ejecutor de comandos del monorepo independiente del shell. | ✅ Activo |
| `lefthook` | `2.1.x` | Gestión ultra-rápida de Git-Hooks en lenguaje Go para bloqueo local. | ✅ Activo |
| `tracing` | workspace | Framework unificado de telemetría, spans y diagnóstico estructurado. | ✅ Activo |

---

## 📜 Decisiones y Mandatos Derivados

* El compilador tiene la orden estricta de tratar los warnings como errores fatales en el Puente mediante la bandera `-D warnings`.
* `bacon` se consolida como el entorno por defecto en la terminal del desarrollador durante las jornadas de programación activa.
* Toda modificación estructural que impacte a las entidades debe acompañarse de la regeneración automática de los modelos de Sea-ORM, validando que no exista discrepancia sintáctica con la base de datos de pruebas MySQL.