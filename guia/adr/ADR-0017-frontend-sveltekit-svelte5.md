# ADR 0017: Frontend: SvelteKit + Svelte 5 Runes + SSE + Local-First
**Autores:** Milton Hipamo / Laboratorio 3030

**Relacionado con:** ADR 0003 (Backend Axum), ADR 0008 (PASETO Auth), ADR 0010 (Testing), ADR 0012 (Tooling), ADR 0016 (OpenAPI), ADR 0020 (Monitoreo Regional), ADR 0021 (Local-First Sync Offline), ADR 0022 (Agentes Distribuidos)

---

## 📋 Contexto y Decisión

**Problema:** El sistema de monitoreo regional del Laboratorio 3030 exige páneles de control (*dashboards*) de tiempo real altamente reactivos, con un consumo mínimo de memoria RAM/CPU en el cliente, carga instantánea en redes móviles inestables, soporte nativo para Renderizado en el Servidor (SSR), tipado compartido estricto con el backend en Rust y la capacidad de operar en modo offline en sedes regionales que sufran cortes de conectividad.

**Decisión:** Adoptar oficialmente **SvelteKit** combinado con **Svelte 5** explotando su nueva arquitectura de **Runes** (`$state`, `$derived`, `$effect`) como el framework oficial del frontend. Las transferencias de datos se gestionarán mediante llamadas REST validadas por el contrato inmutable de OpenAPI, utilizando **TanStack Query** para la caché de datos, **TailwindCSS v4** para estilos atómicos, **LayerChart** para analíticas visuales, e hilos **SSE (Server-Sent Events)** nativos para el flujo unidireccional de tiempo real. El soporte offline se delega a una base de datos local IndexedDB.

> Soluciones basadas en React o Next.js quedan completamente descartadas debido a su excesivo overhead, tamaño de bundle elevado y penalizaciones de rendimiento en dispositivos de gama baja.

---

## 🏗️ Arquitectura Frontend del Espacio de Trabajo

```
apps/web/
├── src/
│   ├── lib/
│   │   ├── components/
│   │   │   ├── ui/        ← Componentes atómicos base de shadcn-svelte
│   │   │   ├── layout/    ← Componentes estructurales (Sidebar, Topbar)
│   │   │   ├── dashboard/ ← Tarjetas KPI primarias y resúmenes analíticos
│   │   │   ├── devices/   ← Listados y formularios de control de hardware
│   │   │   ├── metrics/   ← Gráficos de telemetría y flujos de conexión SSE
│   │   │   ├── sync/      ← Indicadores visuales de sincronización local-first
│   │   │   └── auth/      ← Formularios de acceso seguros
│   │   ├── api/           ← Módulos cliente segregados por dominio del backend
│   │   ├── stores/        ← Estados reactivos globales en archivos .svelte.ts
│   │   ├── sync/          ← Motor de cola FIFO de persistencia en IndexedDB
│   │   ├── validation/    ← Esquemas de validación estricta con ArkType
│   │   └── generated/     ← api-types.ts (Esquema OpenAPI autogenerado, PROHIBIDO EDITAR)
│   ├── routes/
│   │   ├── +layout.svelte      ← Contenedor raíz de la interfaz de usuario
│   │   ├── +layout.server.ts   ← Verificación de sesión mediante cookies en SSR
│   │   ├── (auth)/             ← Grupo de rutas de acceso y recuperación
│   │   └── (dashboard)/        ← Páneles protegidos (sedes, dispositivos, alertas)
│   ├── service-worker.ts       ← Estrategias de caché e interceptor de red offline
│   └── hooks.server.ts         ← Extracción de cookies PASETO y trazabilidad de Request ID
├── static/
│   └── manifest.json           ← Configuración PWA nativa
└── tests/
    ├── unit/                   ← Pruebas unitarias ultrarápidas con Vitest
    └── e2e/                    ← Pruebas de extremo a extremo con Playwright

```

---

## 📊 Stack Tecnológico Homologado

| Componente del Stack | Tecnología Adoptada | Versión Fijada | Contexto Operativo |
| --- | --- | --- | --- |
| **Core Framework** | SvelteKit | **2.57.x** | SSR + Enrutamiento basado en archivos. |
| **Engine Reactivo** | Svelte 5 (Runes) | **5.55.x** | Compilación optimizada sin Virtual DOM. |
| **Runtime Base** | Node.js | **24.x LTS** | Estabilidad garantizada en contenedores Docker. |
| **Gestor de Paquetes** | pnpm | **10.x** | Descargas cacheadas en el espacio de trabajo. |
| **Estilos Atómicos** | TailwindCSS | **v4.0** | Compilador nativo vía Lightning CSS. |
| **Data Fetching** | TanStack Query | **6.1.x** | Orquestación y sincronización de caché asíncrona. |
| **Gestión de Tablas** | TanStack Table | **v8.x (Estable)** | Manipulación de datos masivos sin lag de renderizado. |
| **Validación de Tipos** | ArkType | **2.2.x** | Validación en tiempo de ejecución a velocidad nativa. |
| **Visualización** | LayerChart | **2.0.x** | Gráficos reactivos construidos sobre D3 y Svelte. |
| **Manejo de Fechas** | date-fns | **4.x** | Ajustes temporales condicionados a Bolivia. |
| **Contrato de API** | openapi-typescript | **7.13.x** | Conversión directa del JSON del backend a tipos TS. |
| **Flujo de Red** | Fetch Nativo | Estándar Web | Abstracción limpia mediante interceptores de tokens. |

---

## 🔑 Implementación Práctica de Svelte 5 Runes

El estado de la aplicación abandona los antiguos *Svelte Stores* (`writable`) y migra a clases tipadas con Runes nativas para un control fino de la reactividad:

```typescript
// src/lib/stores/auth.svelte.ts
import type { components } from '$lib/generated/api-types';

type User = components['schemas']['UserDto'];

class AuthStore {
    // Estado reactivo puro encapsulado
    #user = $state<User | null>(null);
    
    constructor(initialUser: User | null) {
        this.#user = initialUser;
    }

    // Valor computado reactivo que se actualiza automáticamente
    get user() { return this.#user; }
    get isAuthenticated() { return this.#user !== null; }
    get isAdmin() { return this.#user?.role === 'admin'; }

    setContext(user: User | null) {
        this.#user = user;
    }
}

export const authContext = new AuthStore(null);

```

---

## 🔐 Estrategia de Autenticación Inmune

* **Persistencia Segura en SSR:** El token de acceso PASETO v4 generado por el backend de Axum (ADR 0008) se almacena exclusivamente en una **Cookie con banderas `httpOnly`, `Secure`, `SameSite=Strict**`. Se prohíbe almacenar tokens en texto plano dentro del `localStorage` para neutralizar ataques XSS de extracción de credenciales.
*  handshakes automáticos: El archivo `hooks.server.ts` intercepta cada navegación en SvelteKit, lee la cookie de sesión y valida los privilegios del usuario antes de renderizar la página en el servidor, eliminando pantallas en blanco en el cliente.

---

## 📡 Canal de Tiempo Real mediante Server-Sent Events (SSE)

Para mantener los tableros actualizados con la telemetría perimetral de las sedes sin saturar el servidor físico, se implementa una arquitectura orientada a eventos unidireccionales:

| Característica | Implementación Técnica | Beneficio Operacional |
| --- | --- | --- |
| **Protocolo Primario** | **SSE (`EventSource` Nativo)** | Evita el overhead de negociación bidireccional de WebSockets utilizando transporte HTTP estándar. |
| **Mecanismo de Control** | Reconexión automática con retraso exponencial integrada en el cliente web. | Resiliencia frente a microcortes de red en la infraestructura. |
| **Estrategia Fallback** | Polling selectivo mediante llamadas REST tradicionales cada 30 segundos si la conexión SSE permanece caída por más de un minuto. | Degradación elegante de la interfaz de usuario. |

---

## 📴 Modelo de Persistencia Local-First (Offline)

* **Caché en Caliente:** Todas las consultas despachadas por TanStack Query se respaldan en una base de datos local gestionada por **IndexedDB**.
* **Invariabilidad del Dashboard:** Ante una desconexión total, la aplicación intercepta el fallo de red, congela los componentes visuales con el último snapshot válido guardado en IndexedDB e despliega un banner de estado degradado sin alterar la navegación.
* **Cola de Sincronización FIFO:** Las mutaciones ejecutadas por el usuario (creación de dispositivos, procesamiento de alertas) se registran temporalmente en una cola local estructurada bajo el principio *First-In, First-Out*. Al restablecerse la conectividad, el Service Worker descarga la cola hacia el backend, resolviendo disputas bajo la regla de **Última Escritura Gana (Last-Write-Wins)**.

---

## 🌎 Internacionalización y Filtros Regionales

* **Localización Estricta:** Configuración del idioma raíz en **`es` (Español)** con zona horaria inmutable fijada en **`America/La_Paz`**.
* **Formatters del Sistema:** Toda métrica expuesta en las tablas analíticas debe ser procesada obligatoriamente a través de las funciones de utilidad del espacio de trabajo:
* `formatLatency(ms)` -> Renderiza cadenas legibles (e.g., `12 ms`).
* `formatNetworkSpeed(bps)` -> Conversión automática a Mbps o Gbps según escala.
* `formatDate(timestamp)` -> Formateo de fechas conforme al estándar regional de Bolivia.



---

## 🛡️ Directrices de Seguridad Frontend

* **Content Security Policy (CSP):** SvelteKit inyectará encabezados CSP estrictos que bloqueen la ejecución de scripts en línea (*inline scripts*) no autorizados.
* **Validación Perimetral:** Ningún formulario enviará datos a la red sin haber sido validado localmente mediante los esquemas estáticos de **ArkType**, deteniendo peticiones malformadas antes de que toquen el backend.
* **Saneamiento de Datos:** Se implementa el bloqueo nativo de Svelte contra inyecciones HTML. Si se requiere renderizar contenido dinámico explícito, los datos pasarán obligatoriamente por un filtro de saneamiento estricto.