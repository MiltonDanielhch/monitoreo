# ADR 0019 — Deploy con Coolify + MySQL 8.0

| Campo | Valor |
| --- | --- |
| **Autores** | Milton Hipamo / Laboratorio 3030 |
| **Relacionado con** | ADR 0001 (Hexagonal), ADR 0004 (MySQL 8.0), ADR 0006 (Sea-ORM), ADR 0013 (Infraestructura Docker), ADR 0015 (Jobs + Apalis), ADR 0018 (Sintonía CLI), ADR 0020 (Monitoreo Regional) |

---

## 📋 Contexto

Coolify es una plataforma de despliegue todo-en-uno auto-alojada (*self-hosted*) que opera sobre el VPS de destino, ofreciendo una experiencia estilo PaaS mediante un dashboard web intuitivo, despliegues automáticos desde Git webhooks, gestión visual de certificados SSL y manejo centralizado de variables de entorno.

Considerando que el despliegue principal de producción del MVP del sistema de Monitoreo Regional se gestiona mediante **Kamal** (ADR 0013) sobre entornos ligeros, surge la necesidad de evaluar y validar a **Coolify** como una infraestructura de despliegue alternativa oficialmente soportada para escenarios multi-proyecto o para entornos donde el equipo operativo requiera administración visual.

La pregunta clave a resolver es:

> ¿Puede Coolify orquestar de manera eficiente nuestro stack de alto rendimiento basado en Rust, Axum y MySQL 8.0 sin introducir sobrecostos ocultos ni romper el aislamiento de la arquitectura hexagonal?

---

## 🎯 Decisión

Se adopta **Coolify v4+** como el sistema de despliegue visual alternativo oficial del Laboratorio 3030.

Esta adopción se rige bajo las siguientes directrices arquitectónicas:

* **Cero Intrusión:** Coolify no altera las capas de software, no modifica la estructura del Containerfile de producción ni influye en las reglas del núcleo de negocio.
* **Preservación del Almacenamiento:** El motor central inmutable del servidor sigue siendo **MySQL 8.0** (ADR 0004) gestionado a través de **Sea-ORM** (ADR 0006). No se permite la introducción de PostgreSQL u otros motores relacionales.
* **Segmentación de Recursos:** Kamal sigue siendo la recomendación estricta para VPS optimizados de 1GB de RAM. Coolify queda validado exclusivamente para instancias con un mínimo de 2GB de RAM debido al consumo base de su suite de telemetría y proxy interno.

---

## ⚙️ Compatibilidad del Stack Tecnológico

| Componente del Sistema | Estado de Compatibilidad | Notas Operativas en Coolify |
| --- | --- | --- |
| **MySQL 8.0** | ✅ Compatible | Instanciado como servicio interno persistente dentro de la red del proyecto. |
| **Axum 0.8** | ✅ Compatible | Expone el puerto `8080` nativo hacia el reverse proxy sin modificaciones. |
| **Healthcheck `/health**` | ✅ Obligatorio | Utilizado por el orquestador de Coolify para validar el traffic swap zero-downtime. |
| **tokio Jobs** | ✅ Compatible | Almacenamiento de colas unificado sobre el motor MySQL principal. |
| **Manejo de Backups** | ✅ Rediseñado | Migrado por completo a `mysqldump` y Percona XtraBackup hacia almacenamiento S3. |
| **SQLite Wasm** | ❌ **No aplica en Servidor** | SQLite se despliega estrictamente en el cliente web para la estrategia Local-First. |
| **Distroless CC** | ✅ Compatible | El Containerfile de runtime optimizado (~32MB) se ejecuta de forma nativa. |

---

## 🏗️ Flujo de Arquitectura en Red de Despliegue

```text
Git Push (Rama Main)
       ↓
Coolify Webhook Trigger
       ↓
Build de Contenedor (Rust 1.95.0 Alpine -> Distroless CC)
       ↓
Instanciación de la API (Axum 0.8)
       ↓
Conexión a MySQL 8.0 Service (Red Docker Interna AISLADA)
       ↓
Persistencia Física -> /var/lib/mysql (Volumen Dedicado en Host)
       ↓
Validación de Endpoint: /health (Verifica Pool Sea-ORM + App)
       ↓
Traffic Swap Exitoso (Traefik 3.7.1 gestiona enrutamiento y SSL)

```

> ⚠️ **Restricción de Seguridad Crítica:** El puerto `3306` de MySQL jamás debe ser expuesto al exterior por el dashboard de Coolify. Toda comunicación con la API se realiza dentro de la red virtual aislada generada por la plataforma.

---

## 💾 Estrategia de Persistencia y Backups MySQL

El almacenamiento relacional se delega a un volumen físico mapeado en el host para evitar cualquier pérdida de datos ante rollbacks de la aplicación:

```text
Coolify Persistent Storage Mount:
Source: /var/data/mysql_production
Destination Path: /var/lib/mysql

```

La URL de conexión inyectada al binario de Rust se estandariza bajo el controlador de MySQL:

```env
DATABASE_URL=mysql://user:password@mysql_host:3306/monitoreo_regional

```

### Plan de Respaldos Automatizados (Purger de Litestream)

Habiendo descartado Litestream (exclusivo de SQLite), la seguridad de los datos se divide en dos niveles integrados en las tareas de infraestructura:

1. **Respaldo Lógico Estructurado (`mysqldump`):** Ejecutado diariamente a través de un cron en el host o un worker de Apalis, generando esquemas limpios listos para restauración inmediata:
```bash
mysqldump -h mysql_host -u root -p'password' --single-transaction --quick --lock-tables=false monitoreo_regional > /backup/mysql-$(date +%Y%m%d).sql

```



```
2. **Respaldo Físico Caliente (Percona XtraBackup):** Empleado como herramienta avanzada para bases de datos de alta transaccionalidad, permitiendo snapshots de los archivos de datos de InnoDB sin bloquear las escrituras del sistema de Monitoreo Regional.

---

## 🐳 Containerfile de Producción Unificado

El proceso de construcción multi-stage no sufre alteraciones, asegurando binarios estáticos ultraligeros con las últimas optimizaciones del compilador:

```dockerfile
# Stage 1: Compilación de alta eficiencia
FROM rust:1.95-alpine AS builder
RUN apk add --no-cache musl-dev openssl-dev mariadb-dev
WORKDIR /app
COPY . .
RUN cargo build --release --bin api

# Stage 2: Entorno de ejecución seguro libre de vulnerabilidades
FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/api /api
EXPOSE 8080
ENTRYPOINT ["/api"]

```

---

## 📊 Matriz Comparativa: Kamal vs. Coolify

| Criterio Operativo | Estándar Kamal (Principal MVP) | Estándar Coolify (Alternativa) |
| --- | --- | --- |
| **Enfoque de Diseño** | Minimalista, orientado a terminal (CLI-first) | Entorno PaaS visual mediante interfaz web |
| **Sobrecarga de Servidor** | ~0 MB (Solo corren tus contenedores) | ~500 MB - 1 GB (Servicios de control internos) |
| **Proxy y Capa SSL** | Caddy 2.11.3 (Configuración vía Justfile) | Traefik 3.7.1 (Automatizado nativamente) |
| **Estrategia de Rollback** | Comandado por CLI (`kamal rollback`) | Ejecución visual con un solo click en UI |
| **Infraestructura Mínima** | VPS económico ($5) - 1GB RAM | VPS optimizado ($10+) - 2GB RAM mínimo |
| **Protección Perimetral** | Fail2ban 1.1.0 configurado en Host | Fail2ban 1.1.0 configurado en Host |

---

## 🛠️ Toolchain de Versiones Fijadas (Edición 2026-05-23)

Para garantizar la estabilidad predictiva del entorno de ejecución, se congelan de manera estricta las siguientes versiones de la infraestructura:

* **Coolify:** `v4.0.0-stable` o superior (Rama principal de actualizaciones controladas).
* **Traefik:** `v3.7.1` (Orquestador de enrutamiento y aprovisionamiento automático de certificados Let's Encrypt).
* **Caddy:** `v2.11.3` (Mantenido exclusivamente para la línea de despliegue basada en Kamal).
* **MySQL Server:** `8.0.x` (Imagen oficial de contenedor optimizada para arquitecturas de almacenamiento InnoDB).
* **Fail2ban:** `v1.1.0` (Instalador imperativo en el sistema operativo del host para el bloqueo de ataques de fuerza bruta en los puertos SSH y HTTP expuestos).

---

## 📋 Consecuencias del Diseño

### ✅ Impactos Positivos

* **Mitigación de la Fricción DevOps:** Logs en tiempo real, gráficas de consumo de hardware y despliegues automáticos al confirmar cambios en Git sin salir de la interfaz web.
* **Aislamiento Multitenant Real:** Permite alojar el frontend de Svelte 5, la API en Rust y entornos de pruebas separados en un único VPS sin colisión de puertos de red.
* **Cumplimiento de Diseño:** La base de datos MySQL y la arquitectura de inyección de dependencias por puertos y adaptadores permanecen idénticas a los entornos locales de desarrollo.

### ⚠️ Trade-offs / Riesgos Operativos

* **Penalización de Recursos:** El plano de control de Coolify devora ciclos de CPU y memoria RAM que en Kamal quedan libres para absorber ráfagas de tráfico en la API.
* **Mayor Superficie de Ataque:** Al exponer un panel web de administración de infraestructura, se vuelve obligatorio proteger el acceso al dashboard mediante autenticación de doble factor (2FA) y reglas estrictas de restricción de IP en el firewall del VPS.