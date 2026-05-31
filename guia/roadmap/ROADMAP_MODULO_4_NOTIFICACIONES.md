# 🗺️ Roadmap Actualizado — Módulo 4: Notificaciones en Segundo Plano

### 🖼️ Integrado al Ecosistema del Lab 3030 y la Interfaz "Redes Beni"

```text
Propósito: Construir el motor de procesamiento y despacho asíncrono de alertas críticas, garantizando que el envío de correos o mensajería no bloquee el hilo web y aislando el dominio de los protocolos de red (SMTP/HTTP).
Entregable: Historial de alertas reactivo en Svelte 5, procesamiento asíncrono en background mediante una cola nativa sobre MySQL y despacho de notificaciones con plantillas dinámicas.
Regla de Pureza: El dominio no conoce protocolos como SMTP ni APIs externas; solo entiende eventos de red que requieren difusión, plantillas de texto y contratos de despacho.
Estados: [ ] Pendiente   [~] En progreso   [x] Completado   [!] Bloqueado

```

### 📊 Matriz de Progreso General

| Slice | Nombre | Referencia ADR | Progreso |
| --- | --- | --- | --- |
| **4.1** | Esquema y Migración SQL (MySQL Workbench) | `ADR-0004`, `ADR-0005` | [x] |
| **4.2** | Eventos de Notificación y Errores de Dominio | `ADR-0001`, `ADR-0007` | [x] |
| **4.3** | Entidades y Repositorios de Persistencia (Sea-ORM) | `ADR-0004` | [x] |
| **4.4** | Adaptadores de Red (SMTP) y Cola en Background | `ADR-0014`, `ADR-0015` | [x] |
| **4.5** | Endpoints de Historial y Pruebas en Axum | `ADR-0003`, `ADR-0006` | [x] |
| **4.6** | UI del Historial de Alertas (Svelte 5 + UI) | `ADR-0017` [TANSTACK/SHADCN] | [x] |
| **4.7** | Formulario de Configuración y Test (Zod + UI) | `ADR-0011`, `ADR-0017` [ZOD] | [x] |
| **4.8** | Pruebas de Carga de la Cola y Simulación Offline | `ADR-0010` | [ ] |
| **M4** | **Módulo 4 Total** |  | **[x]** |

---

## Slice 4.1: Esquema y Migración SQL de Alertas (MySQL Workbench) 🗄️

> **Objetivo:** Diseñar el modelo de datos relacional en tu base de datos local para guardar canales de envío, plantillas de mensajes e historial de reintentos.

* [ ] **4.1.1 — Diseño del Archivo SQL:**
* Crear el archivo plano en `data/migrations/0004_notification_engine.sql`.
* Diseñar la estructura de tres tablas clave: `notification_channels` (canales activos de email/Telegram), `notification_templates` (plantillas preconfiguradas con texto dinámico) y `notification_logs` (historial detallado de envíos, intentos y mensajes de error).


* [ ] **4.1.2 — Ejecución Directa en Workbench:**
* Abrir el archivo `.sql` en tu **MySQL Workbench**.
* Ejecutar el script completo usando el botón del rayo para crear físicamente las tablas e índices en tu servidor local.


* [ ] **4.1.3 — Inyección de Semillas (Seeds):**
* Correr sentencias `INSERT` directamente desde Workbench para registrar las plantillas por defecto del sistema (ej. alertar caídas de nodos o latencias críticas usando parámetros adaptables como `{{host}}` o `{{latency}}`).


* [ ] **4.1.4 — Inspección de Tablas:**
* Verificar en el panel de Workbench la integridad y las llaves foráneas indexadas de las nuevas tablas creadas.



---

## Slice 4.2: Eventos de Notificación y Errores del Dominio 🧠

> **Objetivo:** Modelar de forma pura las necesidades de negocio del sistema de alertas en Rust, aislando la lógica de los mecanismos físicos de transporte.

* [ ] **4.2.1 — Extensión del Sistema de Errores:**
* Registrar en tu catálogo central de fallos del dominio (`DomainError`) variantes específicas como plantillas no encontradas, destinatarios inválidos o exceso en el límite de reintentos.


* [ ] **4.2.2 — Modelado del Negocio de Alertas:**
* Definir en la capa de dominio puro las variantes de carga útiles (como los datos de un nodo caído o una alerta por saturación de tráfico).
* Declarar los contratos y puertos de interfaz abstractos que definirán cómo se encolará una notificación sin especificar bases de datos.



---

## Slice 4.3: Entidades y Repositorios de Persistencia (Sea-ORM) 🔌

> **Objetivo:** Implementar los métodos de acceso a datos para registrar el disparo inicial de alertas y transicionar sus estados.

* [ ] **4.3.1 — Generación de Entidades:**
* Configurar en el crate de datos las nuevas entidades mapeadas de Sea-ORM correspondientes a los canales, plantillas e historial de logs.


* [ ] **4.3.2 — Repositorio de Notificaciones:**
* Desarrollar las funciones del repositorio para guardar registros iniciales en estado de espera (`PENDING`), transicionar sus estados a completado o fallido, y realizar consultas paginadas ordenadas cronológicamente para la interfaz visual.



---

## Slice 4.4: Adaptadores de Red y Lógica de Colas en Segundo Plano ⚙️

> **Objetivo:** Configurar el motor que interactúa con la red física (servidor de correo) y el procesador asíncrono para ejecutar tareas en segundo plano usando Tokio.

* [ ] **4.4.1 — Integración de Herramientas de Correo:**
* Habilitar en la infraestructura los conectores de mensajería locales y un motor ligero para el renderizado del texto dinámico de las plantillas de base de datos.


* [ ] **4.4.2 — Programación de la Tarea en Background:**
* Desarrollar el Worker asíncrono que despierta en segundo plano: extrae la tarea de la base de datos, compila la plantilla con los datos del incidente, despacha el correo y actualiza el estado en Workbench de forma transparente sin frenar el flujo web principal.



---

## Slice 4.5: Endpoints de Historial y Pruebas en Axum 🛣️

> **Objetivo:** Exponer los canales de datos para que la interfaz web consulte la bitácora y proveer herramientas de prueba para el administrador.

* [ ] **4.5.1 — Construcción de Handlers:**
* Crear la ruta `GET /api/v1/notifications/logs` protegida por el sistema de roles de seguridad (RBAC) para entregar la lista pagipada de envíos.
* Crear la ruta `POST /api/v1/notifications/test-smtp` que fuerce de manera síncrona el envío de una alerta fantasma para certificar instantáneamente las conexiones del servidor local.



---

## Slice 4.6: UI del Historial de Alertas (Svelte 5 + TanStack Query + shadcn) 🎨

> **Objetivo:** Crear la pantalla de visualización del historial en la ruta `/dashboard/notifications`, respetando el estilo oscuro de tu captura de pantalla de "Redes Beni".

* [ ] **4.6.1 — Gestión del Estado del Servidor con TanStack Query:**
* Implementar `createQuery` para devorar los logs de notificaciones desde Axum, administrando la paginación de la tabla de forma asíncrona y fluida.


* [ ] **4.6.2 — Diseño de la Tabla con shadcn-svelte:**
* Maquetar el historial usando los componentes limpios de tabla de **shadcn-svelte** bajo la estética Zinc Ultra Dark.
* Organizar las columnas esenciales de control: *Destinatario*, *Tipo de Alerta*, *Canal*, *Intentos*, *Estado* y *Fecha*.


* [ ] **4.6.3 — Estados de Envío Reactivos:**
* Usar lógica condicional nativa de Svelte 5 para pintar badges dinámicos basados en el estado del envío (verde suave para completados, amarillo para pendientes y rojo con indicación del error para envíos fallidos).



---

## Slice 4.7: Formulario de Configuración y Test-Email (Svelte 5 + Zod) 🛠️

> **Objetivo:** Proveer un panel administrativo interactivo en la ruta `/dashboard/notifications/settings` para cambiar parámetros de red sin tocar el código fuente.

* [ ] **4.7.1 — Contrato de Validación con Zod:**
* Crear el esquema estricto en el frontend con **Zod** para validar en tiempo real los inputs de configuración SMTP (servidor, puerto numérico, usuario y contraseñas obligatorias) antes de enviarlos a la API de Rust.


* [ ] **4.7.2 — Construcción del Formulario con shadcn:**
* Renderizar los campos de texto usando componentes de formulario de **shadcn-svelte**, vinculando las variables de forma reactiva con los estados del cliente.


* [ ] **4.7.3 — Botón de Test y Mutación:**
* Implementar un `createMutation` de TanStack Query enlazado al botón de "Probar Conexión" para disparar la alerta fantasma, capturar respuestas y desplegar diagnósticos crudos en pantalla en caso de fallos de red.



---

## Slice 4.8: Pruebas de Carga de la Cola y Simulación Offline 🏁

> **Objetivo:** Garantizar que el subsistema es inmune a interrupciones y caídas del servidor local.

* [ ] **4.8.1 — Prueba de Red Caída:**
* Cortar intencionalmente el acceso al puerto SMTP de pruebas, simular un incidente y verificar visualmente en **MySQL Workbench** que los logs registran el estado de fallo junto con la descripción exacta del socket desconectado.


* [ ] **4.8.2 — Prueba de Concurrencia Masiva:**
* Inyectar en lote un número elevado de registros ficticios en la base de datos y validar mediante tus consolas de Bacon que el backend de Axum sigue atendiendo peticiones en milisegundos mientras el hilo de background trabaja de forma secuencial.


* [ ] **4.8.3 — Certificación E2E de Extremo a Extremo:**
* Realizar un flujo completo: simular una falla en Workbench, verificar el almacenamiento asíncrono, el encolamiento en segundo plano y confirmar que el registro aparece actualizado en la UI de Svelte 5 de manera transparente.
