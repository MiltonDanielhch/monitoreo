# 🗺️ Roadmap — Módulo 9: Motor de Reportes y Exportación de Documentos SLA

> **Propósito:** Consolidar, procesar analíticamente y empaquetar los miles de millones de métricas recolectadas durante el mes para generar reportes en formato PDF con el acuerdo de nivel de servicio (SLA) de cada sede regional.
> **Entregable:** Pipeline de agregación de datos en Sea-ORM, motor de renderizado de PDFs nativo en Rust con inserción de firmas criptográficas para auditoría, y un selector de periodos históricos reactivo en Svelte 5.
> **Regla de Pureza:** El dominio no sabe qué es una coordenada X/Y en un papel, qué es una fuente tipográfica o un layout de página. El dominio recibe la fecha de inicio/fin, calcula las matemáticas del SLA (
> $$SLA = \frac{\text{Tiempo de Actividad}}{\text{Tiempo Total}} \times 100$$
> 
> 
> ) y delega la maquetación a los adaptadores de infraestructura correspondientes.
> **Stack:** Rust 2024 · Axum 0.8 · Sea-ORM 1.1 · `genpdf` / `printpdf` (Generación de PDF nativo) · SvelteKit 2 · Svelte 5 (Runes) · Tailwind v4 · docker
> **Última Revisión:** Mayo 2026

---

## Estados

```
[ ] Pendiente   [~] En progreso   [x] Completado   [!] Bloqueado

```

---

## Progreso General

| Slice | Nombre | Progreso |
| --- | --- | --- |
| 9.1 | Consultas de Agregación y Matemáticas de SLA en Base de Datos | [ ] |
| 9.2 | Modelos de Reportes y Estructuras de Datos (`crates/domain`) | [ ] |
| 9.3 | Adaptador de Renderizado PDF Nativo (`crates/infrastructure`) | [ ] |
| 9.4 | Firma Criptográfica de Integridad Documental (SHA-256 / Metadatos) | [ ] |
| 9.5 | Endpoints de Axum para Generación Asíncrona y Descarga Streaming | [ ] |
| 9.6 | UI de Selección de Periodos y Previsualización de SLA (Svelte 5) | [ ] |
| 9.7 | Panel Administrativo de Firmas y Logos Institucionales | [ ] |
| 9.8 | Pruebas de Carga en Consolidación de Millones de Registros | [ ] |
| **Módulo 9 Total** |  | [ ] |

---

## Slice 9.1: Consultas de Agregación y Matemáticas de SLA en Base de Datos 🔥

> **Objetivo:** Diseñar consultas de alto rendimiento en Sea-ORM que agrupen los datos históricos mensuales sin saturar la base de datos de desarrollo.

```
[ ] Crear las consultas SQL de agregación temporales en `crates/database/src/repositories/telemetry_repository.rs`:
    [ ] Implementar un método que extraiga el total de minutos caídos (*downtime*) y minutos operativos (*uptime*) agrupados por `sede_id` y por mes.
    [ ] Calcular el porcentaje real de disponibilidad de ancho de banda y latencia promedio usando funciones de agregación nativas de la base de datos para evitar traer datos crudos al backend.

```

---

## Slice 9.2: Modelos de Reportes y Estructuras de Datos (`crates/domain`) 🔥

> **Objetivo:** Definir las reglas del negocio encargadas de certificar si una sede cumplió o no con el SLA mínimo estipulado.

```
[ ] Crear el archivo `crates/domain/src/models/report.rs`
    [ ] Definir el struct `SlaReport` que consolide: `sede_name`, `target_sla` (ej. 99.5%), `achieved_sla`, `average_bandwidth_usage`, `packet_loss_avg`, y `incident_count`.
    [ ] Implementar reglas puras en el dominio para clasificar el estado del reporte: si el `achieved_sla` es menor al objetivo, marcar automáticamente el documento con un estado de incumplimiento contractual (*Breached*).
    [ ] Definir el trait/puerto `PdfGeneratorPort` que recibirá el `SlaReport` mapeado.

```

---

## Slice 9.3: Adaptador de Renderizado PDF Nativo (`crates/infrastructure`) 🔥

> **Objetivo:** Programar el generador físico del documento utilizando binarios puros de Rust, garantizando velocidad y bajo consumo de memoria.

```
[ ] Agregar dependencias en `crates/infrastructure/Cargo.toml` (`genpdf` o `printpdf`).
[ ] Crear el servicio en `crates/infrastructure/src/reporting/pdf_adapter.rs`
    [ ] Diseñar un layout elegante y corporativo que incluya: membrete de la Gobernación del Beni, tabla estilizada de métricas por sede, gráficos vectoriales simples de comportamiento y el bloque de firmas técnico-administrativo al pie de página.
    [ ] Configurar la incrustación de fuentes tipográficas (*fonts*) estándar para asegurar que el PDF se renderice exactamente igual en cualquier sistema operativo.

```

---

## Slice 9.4: Firma Criptográfica de Integridad Documental 🔥

> **Objetivo:** Estampar un hash único y metadatos de auditoría al final del PDF para asegurar que el documento no sea alterado tras su emisión.

```
[ ] Implementar una rutina de sellado en `crates/infrastructure/src/reporting/signer.rs`
    [ ] Calcular el hash criptográfico SHA-256 del flujo de bytes del PDF generado en caliente.
    [ ] Inyectar el hash resultante, junto al ID del usuario emisor, la fecha exacta y la IP de origen, dentro de los metadatos internos del archivo PDF y en tu tabla de auditoría inmutable (Módulo 6). Esto permite verificar en el futuro si el documento físico presentado a las autoridades fue manipulado.

```

---

## Slice 9.5: Endpoints de Axum para Generación Asíncrona y Descarga Streaming 🔥

> **Objetivo:** Exponer la API web para solicitar informes mensuales, transmitiendo los bytes de forma directa al navegador.

```
[ ] Crear el handler en `crates/infrastructure/src/handlers/report_handler.rs`
    [ ] Implementar el endpoint `GET /api/v1/reports/sla/monthly` (Acceso restringido por RBAC a `ADMIN` y `DIRECTOR`):
        - Leer parámetros de consulta: `month` (ej. `2026-05`) y `sede_id` (opcional, para reportes individuales o consolidados).
        - Procesar la solicitud y retornar un cuerpo de respuesta fluido `StreamBody` configurando las cabeceras HTTP correctas: `Content-Type: application/pdf` y `Content-Disposition: attachment; filename="reporte_sla_mayo_2026.pdf"`.

```

---

## Slice 9.6: UI de Selección de Periodos y Previsualización de SLA (Svelte 5) 🔥

> **Objetivo:** Desarrollar la consola visual administrativa donde los ingenieros puedan auditar las métricas antes de exportar el documento final.

```
[ ] Diseñar la vista en `apps/web/src/routes/dashboard/reports/+page.svelte`
    [ ] Maquetar selectores limpios con Tailwind v4 para elegir el año, el mes y las sedes a auditar.
    [ ] Usar la rune `$state` para controlar la respuesta analítica previa en pantalla antes de gatillar la descarga del PDF.
    [ ] Implementar indicadores visuales dinámicos utilizando la rune `$derived` para resaltar de un vistazo qué sedes provinciales no alcanzaron el rendimiento mínimo acordado en el mes seleccionado.

```

Para darte una visión clara de cómo se estructurará la secuencia de entrega de estos informes oficiales en tu centro de datos, el flujo analítico seguirá este orden de eventos:

* 08:00:00: Consolidación de Métricas Mensuales
El pipeline analítico agrupó de forma exitosa **4,200,000 registros de telemetría** del mes de mayo correspondientes a todas las sedes del Beni. Tiempo de ejecución de la consulta: `1.4 segundos`.


* 08:00:01: Generación y Compilación de PDF
El motor nativo de Rust compiló las estructuras del documento. Se generó el reporte oficial consolidando un **SLA del 99.82% para Trinidad** y un **94.15% para Riberalta** (Incumplimiento detectado por caída de enlace).


* 08:00:02: Sellado e Inyección Criptográfica
Se calculó e inyectó la firma digital en el PDF. Hash SHA-256 verificado: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`. Registro de auditoría anexado de por vida en la caja negra del sistema.


---

## Slice 9.7: Panel Administrativo de Firmas y Logos Institucionales 🔥

> **Objetivo:** Permitir la personalización de los datos que van impresos en el encabezado y pie de página de los reportes de la Gobernación.

```
[ ] Crear componente en `apps/web/src/routes/dashboard/reports/config/+page.svelte`
    [ ] Diseñar formularios reactivos con Svelte 5 para actualizar el nombre del Director de Telecomunicaciones actual, cargos institucionales y cargar imágenes del escudo oficial del Beni (Módulo 5).
    [ ] Garantizar que estos parámetros modifiquen el estado en base de datos para ser inyectados dinámicamente en los próximos PDFs generados.

```

---

## Slice 9.8: Pruebas de Carga en Consolidación de Millones de Registros 🔥

> **Objetivo:** Forzar al motor de base de datos y al generador de PDFs a procesar un volumen extremo de registros para asegurar estabilidad total en producción.

```
[ ] Prueba 1 (Eficiencia de Memoria RAM): Simular la generación de un reporte consolidado con más de 10 millones de métricas simuladas en la base de datos de desarrollo. Medir el uso de memoria en el contenedor de Axum: este debe permanecer bajo y estable gracias al uso de streaming y agregaciones directas en el motor de base de datos.
[ ] Prueba 2 (Validación Antimanipulación): Alterar manualmente un solo byte de texto dentro de un PDF ya generado empleando un editor binario. Al pasar el archivo alterado por tu validador interno del sistema, este debe marcar inmediatamente el reporte como **No Válido / Corrupto** debido al desajuste con el hash SHA-256 original.

```

---

## Entregable del Módulo 9

Al finalizar este noveno módulo, tu plataforma de infraestructura estará equipada con un sistema de reportaría de grado gubernamental. Los reportes mensuales se generarán al instante, con un diseño impecable, datos analíticos consolidados con precisión milimétrica y blindaje criptográfico integral listo para ser presentado ante cualquier autoridad de la Gobernación.

¡El módulo de analítica y exportación del **Código 3026** queda completamente en marcha! Dime, Milton, ¿estamos listos para dar el siguiente paso técnico o prefieres refinar algún detalle de este flujo?