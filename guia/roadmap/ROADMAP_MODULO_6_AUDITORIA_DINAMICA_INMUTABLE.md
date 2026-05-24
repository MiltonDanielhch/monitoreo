# 🗺️ Roadmap — Módulo 6: Auditoría Dinámica e Inmutable

> **Propósito:** Capturar de forma quirúrgica, estructurada y en tiempo real cada acción crítica realizada por los operadores sobre la red, garantizando un registro inalterable para auditorías técnicas e investigaciones de incidentes.
> **Entregable:** Sistema de telemetría de eventos con almacenamiento indexado en Sea-ORM, extractor middleware automatizado en Axum 0.8 para capturar IPs de origen, contextos de red y agentes de usuario, y un visor cronológico interactivo de auditoría en Svelte 5.
> **Regla de Pureza:** El registro de auditoría es un camino de solo escritura (`Append-Only`). El dominio prohíbe explícitamente cualquier operación de actualización (`UPDATE`) o eliminación física (`DELETE`) sobre las tablas de logs. Las estructuras de datos deben almacenar datos serializados y formateados de forma que no dependan del estado volátil de la base de datos en el futuro.
> **Stack:** Rust 2024 · Axum 0.8 · Sea-ORM 1.1 · SvelteKit 2 · Svelte 5 (Runes) · Tailwind v4 · Docker
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
| 6.1 | Esquema Estricto Append-Only y Estructura SQL | [ ] |
| 6.2 | Modelos de Eventos de Seguridad y Puertos (`crates/domain`) | [ ] |
| 6.3 | Entidad Inmutable y Repositorio de Auditoría (`crates/database`) | [ ] |
| 6.4 | Interceptores y Extractores de Contexto de Red en Axum 0.8 | [ ] |
| 6.5 | API de Consulta, Filtros Avanzados y Exportación | [ ] |
| 6.6 | UI de Línea de Tiempo de Auditoría Quirúrgica (Svelte 5 Runes) | [ ] |
| 6.7 | Alertas Visuales para Eventos Críticos Regionales | [ ] |
| 6.8 | Pruebas de Estrés, Intentos de Alteración de Logs y Simulación | [ ] |
| **Módulo 6 Total** |  | [ ] |

---

## Slice 6.1: Esquema Estricto Append-Only y Estructura SQL 🔥

> **Objetivo:** Crear la estructura física de almacenamiento diseñada específicamente para consultas rápidas y bloqueo absoluto de modificaciones.

```
[ ] Crear archivo de migración plano en `data/migrations/0006_audit_trail.sql`
    [ ] Definir tabla `audit_logs` (id, user_id, username_snapshot, action_code, ip_address, user_agent, resource_affected, structured_payload, created_at)
    [ ] *Nota de Diseño:* Guardar `username_snapshot` evita perder la legibilidad del log si un usuario es eliminado de la base de datos principal en el futuro.
    [ ] Configurar índices compuestos indispensables para auditoría forense:
        - `idx_audit_action_date` en `(action_code, created_at)`
        - `idx_audit_resource_date` en `(resource_affected, created_at)`
    [ ] Configurar un trigger o restricción a nivel base de datos (si la base de datos lo soporta) o aplicar un rol de conexión MySql/Postgres que revoque permisos de `UPDATE` y `DELETE` sobre esta tabla específica.

[ ] Ejecutar la migración dentro del contenedor:
    [ ] Comando: docker exec -i redes-db-dev mysql -u redes -predes redes_dev < data/migrations/0006_audit_trail.sql

```

---

## Slice 6.2: Modelos de Eventos de Seguridad y Puertos (`crates/domain`) 🔥

> **Objetivo:** Definir los tipos de acciones del sistema de forma estricta y tipada en la capa central del negocio.

```
[ ] Crear el archivo `crates/domain/src/models/audit.rs`
    [ ] Definir enum fuertemente tipado `AuditAction` que represente las operaciones sensibles:
        - `AlertAcknowledgment` (El caso: Reconocer la caída de un equipo)
        - `BackupDownload` (Descargar configuraciones de routers)
        - `TopologyMutation` (Modificar un esquema SVG)
        - `CredentialChange` (Modificaciones de seguridad de usuarios)
    [ ] Estructurar la entidad de dominio `AuditEvent` conteniendo metadatos como dirección IP, timestamp nativo y el contexto afectado (ej. `"switch-riberalta-01"`).
    [ ] Definir el trait `AuditLoggerPort` con la firma asíncrona única: `log_event(event: AuditEvent) -> Result<(), DomainError>`.

```

---

## Slice 6.3: Entidad Inmutable y Repositorio de Auditoría (`crates/database`) 🔥

> **Objetivo:** Implementar la persistencia con Sea-ORM garantizando que la capa de base de datos respete las reglas del dominio de solo escritura.

```
[ ] Crear entidad con Sea-ORM en `crates/database/src/entities/audit_log_entity.rs`
    [ ] Configurar el campo `structured_payload` mapeado a un tipo `Json` nativo para almacenar metadatos variables de las alertas (ej. IDs de equipos, estados anteriores, etc.).

[ ] Crear `crates/database/src/repositories/audit_repository.rs`
    [ ] Implementar el método de inserción derivado de `AuditLoggerPort`.
    [ ] Crear el método de lectura paginada `find_logs_filtered(...)` permitiendo acotar búsquedas por Sede, rango de fechas exacto y nivel de gravedad del código de acción.
    [ ] Omitir deliberadamente cualquier función que permita la actualización o borrado de registros.

```

---

## Slice 6.4: Interceptores y Extractores de Contexto de Red en Axum 0.8 🔥

> **Objetivo:** Capturar automáticamente los datos de red de las peticiones HTTP que ingresan al backend sin ensuciar la lógica del controlador.

```
[ ] Crear extractor utilitario en `crates/infrastructure/src/middleware/audit_extractor.rs`
    [ ] Escribir una función interceptora o extractor personalizado que obtenga la dirección IP real del operador analizando las cabeceras `X-Forwarded-For`, `X-Real-IP` o el socket nativo de conexión remota (`ConnectInfo`).
    [ ] Capturar el string de `User-Agent` del navegador para análisis e identificación del terminal.

[ ] Diseñar una macro o helper para registrar eventos de forma limpia en los controladores:
    [ ] Ejemplo de uso interno: `audit.log(AuditAction::AlertAcknowledgment, "switch-riberalta", &ctx).await;`

```

---

## Slice 6.5: API de Consulta, Filtros Avanzados y Exportación 🔥

> **Objetivo:** Proveer los puntos de acceso seguros para que el personal autorizado pueda auditar las operaciones históricas del sistema.

```
[ ] Crear el controlador en `crates/infrastructure/src/handlers/audit_handler.rs`
    [ ] Implementar `GET /api/v1/audit/logs` (Acceso exclusivo para el rol `SUPER_ADMIN` o `AUDITOR`):
        - Extraer parámetros de paginación (`page`, `limit`), filtros por texto, IP o acción.
        - Retornar el set de datos ordenado de manera descendente estricta (`ORDER BY created_at DESC`).

```

---

## Slice 6.6: UI de Línea de Tiempo de Auditoría Quirúrgica (Svelte 5 Runes) 🔥

> **Objetivo:** Desarrollar la interfaz visual de monitoreo en forma de una línea de tiempo limpia, interactiva y scannable a simple vista.

```
[ ] Crear la interfaz administrativa en `apps/web/src/routes/dashboard/audit/+page.svelte`
    [ ] Diseñar una línea de tiempo vertical estilizada con Tailwind v4.
    [ ] Utilizar la rune `$state` para manejar la lista reactiva de registros `logs` y los estados de los filtros seleccionados.
    [ ] Formatear el texto dinámicamente en el cliente para construir la cadena quirúrgica exacta requerida por el negocio.

```

Para asegurar que visualices cómo se estructurará la interfaz de auditoría en la plataforma web, el componente principal implementará este diseño interactivo de eventos en tiempo real:

* 14:05:22: Reconocimiento de Alerta Crítica
El usuario **Milton Daniel** reconoció la alerta de caída del **switch de Riberalta** desde la dirección IP `192.168.10.45` usando Firefox 139 en Linux.


* 12:30:11: Descarga de Respaldo de Configuración
El usuario **Operador_Beni** descargó el archivo de configuración `router-core-trinidad.cfg` correspondiente a la sede central. IP de origen: `192.168.1.100`.


* 09:15:00: Mutación de Esquema de Red
El usuario **Admin_Redes** actualizó el plano de topología SVG de la sede **Guayaramerín**. Archivo físico guardado con hash SHA-256 verificado de forma exitosa.


---

## Slice 6.7: Alertas Visuales para Eventos Críticos Regionales 🔥

> **Objetivo:** Resaltar los eventos que alteren el flujo normal de la infraestructura regional para una rápida identificación táctica.

```
[ ] Crear sub-componente en `apps/web/src/lib/components/AuditRow.svelte`
    [ ] Utilizar la rune `$derived` para computar clases visuales dinámicas según el código de la acción:
        - Si la acción es crítica (ej. caídas de equipos o accesos denegados), pintar un indicador lateral color rojo de alta prioridad o una etiqueta de advertencia destacada.
        - Si es operacional (ej. descargas o consultas rutinarias), aplicar un tono neutral que mantenga limpia la legibilidad general de la pantalla.

```

---

## Slice 6.8: Pruebas de Estrés, Intentos de Alteración de Logs y Simulación 🔥

> **Objetivo:** Forzar fallos y validar la resistencia de los mecanismos de seguridad antes de pasar a producción.

```
[ ] Prueba 1 (Inmutabilidad Estricta): Intentar ejecutar un comando SQL directo (`UPDATE audit_logs SET user_id = ...`) con las credenciales regulares de la aplicación backend. La base de datos o las restricciones de Sea-ORM deben denegar la operación de inmediato de manera segura.
[ ] Prueba 2 (Precisión Quirúrgica E2E): Simular la acción en la UI: loguearse como un operador de prueba, hacer clic en "Reconocer Caída" sobre el switch simulado de Riberalta y verificar de forma exhaustiva en la tabla de la base de datos que el registro se grabó con la estampa de tiempo exacta, la IP del cliente y el payload estructurado intacto.

```

---

## Entregable del Módulo 6

Al finalizar este módulo, tu sistema para la Gobernación del Beni contará con una caja negra digital transparente, inmutable y de altísima precisión. Cualquier acción crítica quedará estampada de por vida con los datos exactos del terminal de origen, dotando a la gerencia técnica de una herramienta analítica impecable para la gestión de su red de datos.

¡Hemos completado la estructura de los 6 módulos principales del núcleo de tu laboratorio! Quedo atento a tus instrucciones para ver cuál es nuestro siguiente paso en el código.