# # Resumen — ADR 0004: Persistencia con MySQL 8.0 + Sea-ORM + Docker
**Autores:** Milton Hipamo / Laboratorio 3030
---

## 📋 Contexto y Decisión

**Problema:** El sistema de monitoreo e inventario de red requiere un almacenamiento de datos altamente consistente, que soporte transacciones complejas, auditorías permanentes, procesamiento eficiente de logs y un consumo controlado de recursos. Todo esto debe ejecutarse de forma aislada, reproducible y fácil de desplegar en servidores VPS pequeños.

**Decisión:** Adoptar **MySQL 8.0+ (Alpine-based)** ejecutado en un entorno de contenedores **Docker** como el motor principal de persistencia centralizada. La capa de datos en Rust se gobernará mediante **Sea-ORM 1.1** para garantizar un modelado tipado, asíncrono y desacoplado del core del negocio.

> **Nota sobre SQLite:** Se reserva **únicamente** para la persistencia local de los agentes remotos distribuidos (`crates/sync` / `apps/agent`) y estrategias offline/local-first en el cliente. Jamás se empleará como base de datos del servidor central.

### Integración Concreta con Rust (Código 3026)

* **Sea-ORM 1.1:** Mapeo objeto-relacional asíncrono nativo con generación de entidades fuertemente tipeadas, derivando directamente del diseño táctico del dominio.
* **Compilación Segura:** Validación de tipos de datos en tiempo de desarrollo, eliminando discrepancias entre los esquemas físicos de la base de datos y las estructuras de negocio del backend.
* **Pool de Conexiones Asíncrono:** Gestión eficiente del ciclo de vida de las conexiones mediante el pool nativo de Sea-ORM configurado con TLS via `rustls`.

---

## 📦 Stack de Persistencia Aprobado

| Componente | Versión Base | Propósito Específico en el Ecosistema |
| --- | --- | --- |
| **MySQL** | `8.x-alpine` | Motor de base de datos relacional primario del servidor central. |
| **Sea-ORM** | `1.1.x` | ORM asíncrono y tipado para la abstracción segura de consultas en Rust. |
| **Docker / Compose** | `v2.x+` / `v5.x+` | Infraestructura de contenedores para la orquestación y despliegue del entorno local. |
| **Moka Cache** | `0.12.x` | Capa de caché en memoria intermedia para evitar sobrecargar el pool de MySQL. |
| **Sea-ORM CLI** | `1.1.x` | Herramienta para la generación de entidades de Rust a partir del esquema vivo. |
| **Just** | `v1.x` | Automatizador de comandos (`just db-migrate`, `just db-seed`) para el equipo de desarrollo. |

---

## 🔄 Estrategia de Migraciones y Gestión de Esquemas

El ciclo de vida de la base de datos se rige por un aislamiento absoluto entre el esquema estructural y los datos de inicialización (seeds), aplicando las siguientes directrices mecánicas:

| Regla | Nombre | Descripción |
| --- | --- | --- |
| **R1** | Migraciones Puras | Toda migración estructural se escribe en código SQL nativo o mediante el constructor tipado de Sea-ORM, incluyendo obligatoriamente lógica de aplicación (`up`) y reversión (`down`). |
| **R2** | Idempotencia Estricta | Los scripts de migración deben diseñarse para poder ejecutarse múltiples veces sin alterar el estado del sistema ni duplicar restricciones (`CREATE TABLE IF NOT EXISTS`). |
| **R3** | Entorno de Reversión | Los métodos de destrucción (`down`) se ejecutan única y exclusivamente en entornos de desarrollo local para pruebas de regresión. **Queda prohibido su uso automático en entornos de producción.** |
| **R4** | Segregación de Semillas | Los datos de prueba, catálogos iniciales y credenciales de administración raíz (*Seeds*) se gestionan de manera completamente aislada a los scripts de estructura física. |
| **R5** | Control de Despliegue | En producción, las migraciones se aplican de forma manual o controlada a través del pipeline de integración continua, impidiendo que el binario altere el esquema de la base de datos de forma automática en el arranque. |

---

## 🛡️ Configuración y Consistencia

* **Persistencia de Volúmenes:** Los datos de MySQL se montan estrictamente en volúmenes gestionados por Docker o rutas locales del host debidamente securizadas, impidiendo la pérdida de información al reiniciar o recrear los contenedores.
* **Aislamiento de Red:** El contenedor de la base de datos se comunica con la app de Axum a través de una red interna privada de Docker (*bridge network*), exponiendo el puerto `3306` hacia el exterior únicamente si el entorno de desarrollo local lo requiere de manera explícita.