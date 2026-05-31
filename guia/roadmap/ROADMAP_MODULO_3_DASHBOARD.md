# 🗺️ Roadmap Actualizado — Módulo 3: Dashboard de Monitoreo de Red

```text
Propósito: Consolidar y agregar el estado operativo de toda la infraestructura de red en tiempo real para el operador.
Entregable: Panel principal (Home) en Svelte 5 alimentado por TanStack Query (sondeo corto), clonando la interfaz visual de la captura "Redes Beni": Grid de KPIs específico, Feed de Alertas Recientes y Estado de Dispositivos.
Regla de Pureza: El handler del Dashboard no calcula métricas; consume una vista optimizada o un servicio de agregación del dominio para no bloquear el hilo asíncrono.
Estados: [ ] Pendiente   [~] En progreso   [x] Completado   [!] Bloqueado

```

### Progreso General

| Slice | Nombre | Referencia ADR | Progreso |
| --- | --- | --- | --- |
| **3.1** | Tablas de Nodos y Alertas (MySQL Workbench) | `ADR-0004`, `ADR-0005` | [x] |
| **3.2** | Modelo de Telemetría Ajustado al Diseño | `ADR-0001` | [x] |
| **3.3** | Consultas de Agregación Real con Sea-ORM | `ADR-0004` | [x] |
| **3.4** | Endpoint de Agregación de KPIs (`/api/dashboard/stats`) | `ADR-0003`, `ADR-0006` | [x] |
| **3.5** | Grid de KPIs Real de la Captura (Svelte 5 + shadcn) | `ADR-0017` | [x] |
| **3.6** | Feed de Alertas Recientes y Estado de Componentes | `ADR-0011`, `ADR-0017` | [x] |
| **3.7** | Simulación de Caídas Directamente desde Workbench | `ADR-0010` | [ ] |
| **M3** | **Módulo 3 Total** |  | **[~]** |

---

## Slice 3.1: Tablas de Nodos y Conectividad (MySQL Workbench) 🗄️

> **Objetivo:** Crear el esquema base en tu entorno de base de datos local para soportar las métricas de ancho de banda y alertas de la captura.

* [ ] **3.1.1 — Crear archivo de migración en `data/migrations/0003_network_devices.sql`:**
* Diseñar la tabla `devices` agregando la columna `bandwidth_gbps` (para alimentar la cuarta tarjeta de la captura) y el estado de conectividad.
* Diseñar la tabla `active_alerts` con su relación de llave foránea explícita hacia los dispositivos.


* [ ] **3.1.2 — Ejecución Local:**
* Abrir el archivo `.sql` directamente en **MySQL Workbench**.
* Ejecutar el script usando el icono del rayo para impactar tu base de datos local.


* [ ] **3.1.3 — Inyección de Semillas de Prueba:**
* Ejecutar sentencias `INSERT` nativas en Workbench para cargar nodos de prueba con datos reales (ruteadores en Trinidad, switches en Riberalta con consumos de ancho de banda específicos como `12 Gbps`).



---

## Slice 3.2: Modelo de Telemetría Ajustado al Diseño 🧠

> **Objetivo:** Adaptar las estructuras del dominio en Rust para que reflejen fielmente el contenido visual del nuevo Dashboard.

* [ ] **3.2.1 — Actualizar el modelo del Dashboard en `crates/domain/src/models/dashboard.rs`:**
* Definir la estructura abstracta que el backend transmitirá. Debe contener los campos exactos de las tarjetas: `active_locations`, `online_devices`, `pending_alerts`, `critical_alerts_count` y `total_bandwidth_gbps`.



---

## Slice 3.3: Consultas de Telemetría con Sea-ORM 🔌

> **Objetivo:** Traducir el inventario físico en datos agrupados ultrarrápidos utilizando el ORM de Rust.

* [ ] **3.3.1 — Configurar el Repositorio de Datos:**
* Generar o actualizar las entidades de Sea-ORM para que lean las tablas de dispositivos y alertas.
* Programar la consulta asíncrona de agregación utilizando sentencias `COUNT` y `SUM` integradas en el ORM para obtener el estado de salud global de la red en una sola llamada a la base de datos.



---

## Slice 3.4: Endpoint de Agregación de KPIs en Axum 🛣️

> **Objetivo:** Crear la compuerta web segura para que el cliente de Svelte extraiga la información.

* [ ] **3.4.1 — Implementar el Handler y Rutas:**
* Crear la ruta asíncrona protected en Axum bajo la dirección `/api/dashboard/stats`.
* Integrar el extractor de roles (`RequireRole`) para asegurar que solo los operadores autorizados visualicen el panel de control regional del Beni.



---

## Slice 3.5: Grid de KPIs Real de la Captura (Svelte 5 + shadcn-svelte) 🎨

> **Objetivo:** Replicar con precisión milimétrica la estructura superior de 4 columnas de tu imagen de referencia utilizando estados reactivos avanzados.

* [ ] **3.5.1 — Integrar TanStack Query en la vista principal:**
* Configurar `createQuery` en tu ruta del dashboard para gestionar el sondeo corto (polling automático), sincronizando la UI con el backend de Axum de manera transparente.


* [ ] **3.5.2 — Diseñar el layout de Tarjetas:**
* Utilizar componentes primitivos de **shadcn-svelte** para armar el panel oscuro (fondo en tonos Zinc intensos).
* Maquetar las 4 tarjetas idénticas a tu captura:
1. **Sedes Activas:** Mostrando el número de municipios monitoreados y la variación de tendencia.
2. **Dispositivos Online:** Con el porcentaje de disponibilidad de la infraestructura regional.
3. **Alertas Pendientes:** Resaltando la cantidad de incidencias críticas.
4. **Ancho de Banda:** Reflejando la carga total de tráfico de datos en **Gbps**.





---

## Slice 3.6: Feed de Alertas Recientes y Componentes de Estado 🚨

> **Objetivo:** Implementar los bloques inferiores de la captura para dar salida al flujo cronológico de incidentes.

* [ ] **3.6.1 — Validación con Zod:**
* Crear el esquema estricto en TypeScript con Zod para asegurar la consistencia del feed que se dibuja en la pantalla.


* [ ] **3.6.2 — Implementar el bloque "Alertas Recientes":**
* Diseñar la vista usando estados condicionales de Svelte 5. Si la lista está limpia, renderizar tu icono verde con la leyenda "Sin alertas pendientes". Si hay incidencias, listar los ruteadores afectados aplicando los estilos visuales oscuros correspondientes.


* [ ] **3.6.3 — Estructurar "Estado de Dispositivos":**
* Preparar el contenedor derecho para albergar la lista de componentes de red y remover el mensaje por defecto de "Sin datos disponibles".



---

## Slice 3.7: Simulación de Caídas Directamente desde Workbench 🏁

> **Objetivo:** Certificar bajo estrés controlado que todo el circuito reacciona de forma automática sin intervención manual.

* [ ] **3.7.1 — Verificar Rendimiento:**
* Validar con Bacon en tu terminal local que las peticiones asíncronas recurrentes de TanStack Query no saturen las Runes ni congelen los hilos del navegador.


* [ ] **3.7.2 — Ejecutar Simulación de Incidente:**
* Ejecutar una consulta de actualización directa desde la grilla de comandos de **MySQL Workbench** (ej. tumbar un nodo cambiando su estado a 'OFFLINE').


* [ ] **3.7.3 — Validación de Extremo a Extremo:**
* Confirmar que, sin reiniciar el servidor de Rust ni recargar la pestaña del navegador, el contador de "Dispositivos Online" de tu interfaz disminuya, se incremente el indicador de "Alertas Pendientes" y aparezca la nueva fila de error en el feed inferior automáticamente.
