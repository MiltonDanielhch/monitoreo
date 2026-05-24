# ADR 0013: Infraestructura: Docker Compose + Distroless + Red Privada
**Autores:** Milton Hipamo / Laboratorio 3030

**Relacionado con:** ADR 0004 (MySQL), ADR 0006 (Sea-ORM), ADR 0011 (Estándares), ADR 0012 (Tooling), ADR 0019 (Coolify)

---

## 📋 Contexto y Decisión

**Problema:** Necesidad de un entorno de despliegue simple, de bajo mantenimiento, con aislamiento estricto entre servicios, alta reproducibilidad y mínimo consumo de recursos en un servidor físico/VPS, optimizando el ciclo de vida del software para un equipo de desarrollo ágil y con presupuesto controlado.

**Decisión:** Adoptar oficialmente **Docker y Docker Compose** como el motor de orquestación estándar del monorepo. Se rechaza el uso de Kubernetes o Swarm por introducir capas de complejidad innecesarias para esta fase del laboratorio. La infraestructura se diseñará utilizando redes privadas internas aisladas, volúmenes locales persistentes e imágenes de ejecución **Distroless** para el backend de Rust, garantizando la mínima superficie de ataque y un peso de imagen inferior a 20MB.

---

## 📁 Estructura de Archivos

```
infra/
├── docker/
│   ├── backend.Dockerfile       # Multi-stage Rust 2024 + cargo-chef + Distroless static
│   ├── frontend.Dockerfile      # Interfaz Svelte 5 (Runes) + Node 26 Alpine
│   ├── .dockerignore            # Exclusiones de contexto optimizadas para Docker
│   └── docker-compose.yml       # Orquestación oficial unificada
│
└── coolify/
    └── docker-compose.yml       # Override específico para despliegues en Coolify

```

---

## 🐳 Especificaciones de Dockerfiles (Multi-Stage)

### Backend (Rust 2024 + cargo-chef + Distroless)

Para optimizar los tiempos de compilación en el Puente y asegurar la inmutabilidad, se estructura el proceso en 4 etapas limpias:

1. **Chef:** Imagen base de Rust oficial con `cargo-chef` instalado para preparar el entorno.
2. **Planner:** Analiza el espacio de trabajo del monorepo y genera el archivo `recipe.json` con el árbol de dependencias puras.
3. **Builder:** Compila de forma aislada las dependencias de la receta para congelar la caché de Docker. Posteriormente, compila la lógica de negocio usando el target estático `x86_64-unknown-linux-musl`.
4. **Runtime:** `gcr.io/distroless/static-debian13:nonroot`. Carece por completo de shells (`sh`, `bash`), administradores de paquetes o binarios del sistema innecesarios.

**Seguridad:** El proceso se ejecuta bajo el usuario seguro del contenedor `nonroot` (UID 65532), impidiendo que una brecha en la aplicación comprometa los privilegios del sistema anfitrión.

### Frontend (Svelte 5 + Node 24 Alpine)

1. **Builder:** Compilación y empaquetado de la interfaz reactiva (Runes) utilizando **pnpm 10**.
2. **Runtime:** Node.js 24 sobre Alpine Linux, delegando la ejecución al usuario sin privilegios `svelteui` (UID 1001).

---

## 🔧 Docker Compose Oficial (`docker-compose.yml`)

**Matriz de Servicios Estables:**

| Servicio | Imagen / Origen | Puerto Interno | Dependencia Crítica |
| --- | --- | --- | --- |
| `mysql_db` | `mysql:8.0-debian` | 3306 (Oculto al exterior) | Ninguna |
| `backend` | Build Backend Multi-stage | 8080 | `mysql_db` (Healthcheck aprobado) |
| `frontend` | Build Frontend Multi-stage | 3000 | `backend` (Healthcheck aprobado) |

**Características de la Red y Persistencia:**

* **Red Privada (`red-laboratorio`):** Subred virtualizada y aislada mediante el driver de puente nativo de Docker. Los servicios se comunican internamente por nombres de servicio DNS.
* **Fail-Fast (Healthchecks):** El backend no inicia hasta que el healthcheck de MySQL valide que el motor relacional está listo para recibir conexiones de Sea-ORM.
* **Persistencia:** Volumen local indexado `mysql_3026_data` gestionado de forma directa por Docker. **La base de datos MySQL nunca expone puertos públicos al exterior.**

### Mapeo de Desarrollo Local (`docker-compose.override.yml`)

* Mapea el puerto `3306` exclusivamente a la interfaz local `127.0.0.1` del desarrollador para permitir auditorías rápidas de datos sin exponer la red.
* Activa el montaje de volúmenes en caliente para el Frontend con el fin de acelerar la recarga del servidor de desarrollo.

---

## 🛡️ Matriz de Seguridad

| Capa de Seguridad | Implementación en el Código 3026 |
| --- | --- |
| **Aislamiento de Red** | Red interna hermética. MySQL es invisible desde el exterior del servidor físico. |
| **Superficie Mínima** | Uso de `gcr.io/distroless/static-debian13:nonroot` para el backend. Cero vulnerabilidades de OS. |
| **Ejecución Segura** | Restricción estricta de usuarios `nonroot` (UID 65532) y `svelteui` (UID 1001). Prohibido correr procesos como root dentro del contenedor. |
| **Vinculación Estática** | Compilación nativa en Rust apuntando al target `x86_64-unknown-linux-musl` para eliminar dependencias dinámicas. |

---

## 🚀 Comandos Operativos Unificados (Justfile)

Para asegurar que los comandos locales sean idénticos a los del CI/Puente, Docker Compose se abstrae mediante tareas semánticas en el `justfile`:

```bash
# Levantar la infraestructura completa en segundo plano
just infra-up       # Ejecuta: docker compose -f infra/docker/docker-compose.yml up -d

# Monitorear logs estructurados del monorepo
just infra-logs     # Ejecuta: docker compose -f infra/docker/docker-compose.yml logs -f

# Detener los servicios liberando memoria RAM
just infra-down     # Ejecuta: docker compose -f infra/docker/docker-compose.yml down

# Reconstruir el binario de Rust ignorando capas de caché rotas
just infra-rebuild  # Ejecuta: docker compose -f infra/docker/docker-compose.yml build --no-cache backend

```

---

## 🔗 Integración con Coolify (ADR 0019)

El servidor de control y automatización de despliegues (Coolify) procesa el archivo inmutable del monorepo:

1. **Source de Verdad:** Escucha la rama principal apuntando directamente al directorio `infra/docker/docker-compose.yml`.
2. **Build Automático:** Ejecuta la compilación multi-stage optimizando el uso de las capas de `cargo-chef` guardadas en el disco del servidor.
3. **Inyección de Entorno:** Mapea las variables confidenciales (claves de encriptación de tokens y credenciales de MySQL) directamente a las variables de entorno inyectadas en tiempo de ejecución.
4. **Proxy de Entrada:** Coolify asocia el dominio SSL de forma automática, redirigiendo el tráfico de red directo al puerto expuesto de nuestra interfaz web.

---

## 🛠️ Herramientas de Gestión Aprobadas

| Herramienta | Propósito Arquitectónico | Versión | Estado |
| --- | --- | --- | --- |
| `cargo-chef` | Cachear y optimizar capas de dependencias Rust en Docker. | `0.1.77` | ✅ Activa |
| `lazydocker` | Interfaz de terminal (TUI) para administración rápida de contenedores. | `0.25.x` | ✅ Activa |
| `dive` | Analizador de eficiencia y optimización de espacio por capas de imagen. | latest | ✅ Activa |

---
