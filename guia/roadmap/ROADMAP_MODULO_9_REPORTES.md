# 🗺️ Roadmap Actualizado — Módulo 9: Reportes y SLA

### 📊 Motor de Consolidación Analítica y Exportación de Documentación Inmutable

```text
Propósito: Consolidar, procesar analíticamente y empaquetar los millones de métricas mensuales para generar reportes oficiales en PDF con el acuerdo de nivel de servicio (SLA) de cada sede regional.
Entregable: Pipeline de agregación de datos con Sea-ORM, motor de renderizado de PDFs nativo en Rust con inyección de firmas criptográficas de integridad y un selector reactivo de periodos en Svelte 5.
Regla de Pureza: El dominio no entiende de coordenadas X/Y en un papel ni de fuentes tipográficas. Recibe fechas, calcula las matemáticas de SLA (Uptime / Tiempo Total) y delega la maquetación física a los adaptadores.
Estados: [ ] Pendiente   [~] En progreso   [x] Completado   [!] Bloqueado

```

### 📊 Matriz de Progreso General

| Slice | Nombre | Referencia ADR | Progreso |
| --- | --- | --- | --- |
| **9.1** | Consultas de Agregación y Matemáticas de SLA (Workbench) | `ADR-0004`, `ADR-0005` | [ ] |
| **9.2** | Modelos de Reportes y Estados Contractuales del Dominio | `ADR-0001` | [ ] |
| **9.3** | Adaptador de Renderizado PDF Nativo (`genpdf` / `printpdf`) | `ADR-0012` | [ ] |
| **9.4** | Firma Criptográfica de Integridad Documental (SHA-256) | `ADR-0009` | [ ] |
| **9.5** | Endpoints de Axum para Generación y Descarga Streaming | `ADR-0003`, `ADR-0013` | [ ] |
| **9.6** | UI de Selección de Periodos y Previsualización (Svelte 5) | `ADR-0017` [REPORTS] | [ ] |
| **9.7** | Panel Administrativo de Firmas y Logos Institucionales | `ADR-0017` [CONFIG] | [ ] |
| **9.8** | Pruebas de Estrés en Consolidación y Validación Antifraude | `ADR-0010` | [ ] |
| **M9** | **Módulo 9 Total** |  | **[ ]** |

---

## Slice 9.1: Consultas de Agregación y Matemáticas de SLA (MySQL Workbench) 🗄️

> **Objetivo:** Diseñar consultas optimizadas de alto rendimiento que agrupen los datos históricos en el servidor local sin sobrecargar la memoria de la aplicación.

* [ ] **9.1.1 — Pipeline de Agregación en SQL:**
* Desarrollar los métodos de Sea-ORM que traduzcan funciones de agregación nativas (`SUM`, `COUNT`) en tu base de datos local para extraer el total de minutos operativos (*uptime*) y caídos (*downtime*) por cada `sede_id`.
* Forzar que los cálculos matemáticos del SLA y promedios de ancho de banda se resuelvan directamente en el motor de base de datos administrado en tu **MySQL Workbench** para evitar transferir gigabytes de datos crudos al backend.



---

## Slice 9.2: Modelos de Reportes y Estados del Dominio 🧠

> **Objetivo:** Blindar la capa del negocio con las reglas que certifican si un nodo provincial cumplió con las exigencias del contrato de telecomunicaciones.

* [ ] **9.2.1 — Lógica de Validación de SLA:**
* Crear la estructura de dominio `SlaReport` que agrupe: metas de disponibilidad, latencia promedio y conteo de incidentes.
* Programar la regla de negocio pura: si el SLA alcanzado es menor al umbral objetivo acordado para la Gobernación (ej. 99.5%), tachar el registro de forma automática con el estado de incumplimiento contractual (`Breached`).



---

## Slice 9.3: Adaptador de Renderizado PDF Nativo 🔌

> **Objetivo:** Codificar el componente de infraestructura que ensambla los archivos binarios del PDF de manera veloz utilizando librerías nativas de Rust sin depender de herramientas externas pesadas.

* [ ] **9.3.1 — Layout Institucional del Beni:**
* Configurar las dependencias de tipografías y maquetar la estructura visual del documento: membrete oficial de la Gobernación del Beni, tablas de rendimiento técnico estilizadas y el área inferior reservada para las firmas autorizadas.



---

## Slice 9.4: Firma Criptográfica de Integridad Documental ⚙️

> **Objetivo:** Estampar una huella digital única en los metadatos del PDF para garantizar la inmutabilidad y transparencia de los informes entregados.

* [ ] **9.4.1 — Sellado Digital al Vuelo:**
* Desarrollar la rutina en Rust que calcule el hash SHA-256 sobre el flujo de bytes del archivo PDF recién generado en memoria.
* Inyectar dicho hash junto con la IP del operador y la fecha exacta en los metadatos del archivo y cruzarlos inmediatamente con tu bitácora de auditoría inmutable (Módulo 6) para auditorías forenses futuras.



---

## Slice 9.5: Endpoints de Axum para Generación y Descarga Streaming 🛣️

> **Objetivo:** Exponer los accesos a la API web que permitan a los directores descargar los documentos mediante flujos eficientes de datos sin saturar la RAM del servidor.

* [ ] **9.5.1 — Despacho con StreamBody:**
* Crear la ruta protegida por roles `GET /api/v1/reports/sla/monthly` (Acceso exclusivo para `ADMIN` y `DIRECTOR`).
* Configurar la respuesta utilizando extractores de flujo asíncronos (`StreamBody`) e inyectar las cabeceras HTTP correctas (`Content-Type: application/pdf`) para forzar que el navegador inicie la descarga de forma limpia.



---

## Slice 9.6: UI de Selección de Periodos y Previsualización (Svelte 5 + TanStack Query) 🎨

> **Objetivo:** Construir el centro operativo de reportes en la ruta `/dashboard/reports` clonando el diseño Ultra Dark Zinc que maneja tu panel principal.

* [ ] **9.6.1 — Consulta Analítica de Datos:**
* Implementar un `createQuery` de TanStack Query para traer el resumen analítico preliminar de las sedes al seleccionar el mes y año en los controles deslizantes de la interfaz.


* [ ] **9.6.2 — Runes para Alertas de Incumplimiento:**
* Diseñar la UI usando selectores modernos de **shadcn-svelte** combinados con Tailwind v4.
* Utilizar la rune `$derived` para evaluar los porcentajes en tiempo real: si una sede en provincias bajó de los límites establecidos, pintar instantáneamente una advertencia visual destacada antes de que el técnico presione el botón de exportación.



---

## Slice 9.7: Panel de Firmas y Logos Institucionales (Svelte 5 + Zod) 📊

> **Objetivo:** Crear el formulario administrativo de personalización de datos de cabeceras y pies de página.

* [ ] **9.7.1 — Validación Estricta de Parámetros:**
* Implementar esquemas de **Zod** en la vista para asegurar que nombres de directores, cargos y rutas de imágenes institucionales cumplan con la longitud y formato requeridos.


* [ ] **9.7.2 — Persistencia en Caliente:**
* Diseñar el formulario en la ruta `/dashboard/reports/config` usando **Svelte 5 Runes**. Al guardar, actualizar los metadatos globales en la base de datos para que los próximos reportes generados asuman de inmediato los nuevos cambios.



---

## Slice 9.8: Pruebas de Estrés y Validación Antifraude 🏁

> **Objetivo:** Forzar cargas de procesamiento extremas en tu entorno local para validar que el motor analítico sea invulnerable a corrupciones o fallos.

* [ ] **9.8.1 — Simulación de Carga Mensual Forense:**
* Inyectar millones de filas de pruebas en tus tablas de telemetría y disparar la consolidación. Monitorear mediante tus herramientas de diagnóstico que el consumo de hardware en tu backend permanezca plano y estable gracias al uso de streams optimizados.


* [ ] **9.8.2 — Rompimiento Binario de Seguridad:**
* **Prueba de Fuego:** Tomar un PDF generado, abrirlo en un editor hexadecimal y alterar manualmente un solo carácter de texto en el documento. Al subirlo al verificador del sistema, la aplicación debe rechazar el archivo inmediatamente y marcarlo como **Documento Manipulado / No Válido** debido al desajuste matemático contra el hash SHA-256 guardado en el Módulo 6.



---

### 🚀 Flujo Operativo en Consola del Centro de Control

Cuando todo el engranaje del Módulo 9 esté operando en producción, el comportamiento analítico del sistema imprimirá registros impecables con el siguiente orden cronológico:

* **08:00:00 [Consolidación]:** Pipeline analítico agrupó con éxito **4,200,000 registros de telemetría** del mes de mayo. Tiempo de ejecución en base de datos local: `1.4 segundos`.
* **08:00:01 [Compilación PDF]:** Motor nativo de Rust compiló las estructuras. Generado informe mensual con un **SLA del 99.82% para Trinidad** y un **94.15% para Riberalta** (*Alerta: Incumplimiento de contrato detectado por caídas de enlace*).
* **08:00:02 [Sello Digital]:** Calculada huella digital del archivo. Hash SHA-256 verificado: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`. Registro inmutable inyectado con éxito en la caja negra.