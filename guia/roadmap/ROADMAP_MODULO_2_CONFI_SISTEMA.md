# 🗺️ Checklist de Seguimiento — Módulo 2: Configuración y Umbrales

### 🛠️ Gobernado por los Estándares de Arquitectura del Lab 3030

> **Entorno de Trabajo:** Local (`rustc 1.95.0`) + Sesión Directa en **MySQL Workbench**.

---

## 📊 Matriz de Trazabilidad y Progreso

```text
[x] Slice 2.1: El Almacén Físico (Workbench) ----> Vinculado a: ADR-0004 y ADR-0005
[x] Slice 2.2: El Guardián Lógico (Dominio) ----> Vinculado a: ADR-0001 y ADR-0007
[x] Slice 2.3: El Conector (Sea-ORM Local) -----> Vinculado a: ADR-0004
[x] Slice 2.4: El Escudo de RAM (Caché) ---------> Vinculado a: ADR-0015
[x] Slice 2.5: Las Compuertas API (Axum) --------> Vinculado a: ADR-0003, ADR-0006 y ADR-0016
[x] Slice 2.6: El Tablero Visual (Svelte 5) -----> Vinculado a: ADR-0017
[x] Slice 2.7: Certificación de Calidad ---------> Vinculado a: ADR-0010

```

---

## 🔍 Plan de Acción Explicado (Para "Tickear")

### 🗄️ Slice 2.1: Esquema Clave-Valor y Sedes SQL

* **Gobernanza:** `ADR-0004` (Persistencia MySQL) y `ADR-0005` (Migraciones y Seeding).
* **Explicación:** Crearemos las estructuras en tu base de datos local para que el sistema tenga un catálogo de dónde está parado (sedes del Beni) y pueda guardar variables sin alterar el código de Rust.
* [x] **2.1.1:** Escribir el script SQL limpio con las tablas de Sedes (`locations`) y Configuración Dinámica (`system_settings`) en `data/migrations/0002_system_settings.sql`.
* [x] **2.1.2:** Abrir **MySQL Workbench**, pegar el script, ejecutarlo (botón del rayo) y confirmar en el panel izquierdo que las tablas se crearon correctamente.
* [x] **2.1.3:** Insertar los datos semilla (los valores iniciales de Ping, Latencia y Pérdida de paquetes) mediante Workbench y verificar con un `SELECT *` que estén listos para ser consumidos.



### 🧠 Slice 2.2: Reglas de Negocio y Umbrales (`crates/domain`)

* **Gobernanza:** `ADR-0001` (Arquitectura Hexagonal) y `ADR-0007` (Manejo de Errores).
* **Explicación:** El corazón del negocio. Aquí le enseñamos a Rust qué cosas son lógicas y qué cosas son un peligro (como poner una alerta crítica más baja que una advertencia). No tocamos nada de bases de datos ni entorno web aquí.
* [x] **2.2.1:** Definir el modelo matemático/lógico de los umbrales de red en la capa del dominio puro.
* [x] **2.2.2:** Programar la regla de validación interna: rechazar cualquier intento de configuración si el umbral crítico es menor o igual al de advertencia.
* [x] **2.2.3:** Registrar la variante de fallo `InvalidSettingValue` dentro de tu árbol centralizado de errores (`DomainError`).
* [x] **2.2.4:** Correr `cargo check -p domain` en tu terminal para garantizar que compila perfectamente aislado del mundo exterior.



### 🔌 Slice 2.3: Repositorio de Configuración (`crates/database`)

* **Gobernanza:** `ADR-0004` (Persistencia Sea-ORM).
* **Explicación:** El puente que traduce las filas que ves en MySQL Workbench a objetos seguros dentro de Rust usando Sea-ORM.
* [x] **2.3.1:** Mapear la estructura de la tabla `system_settings` como una entidad de Sea-ORM.
* [x] **2.3.2:** Crear la función del repositorio para extraer todas las variables del sistema de golpe en el arranque.
* [x] **2.3.3:** Crear la función para actualizar una clave específica en Workbench cuando se altere desde la interfaz.



### ⚡ Slice 2.4: Servicio de Caché In-Memory (`crates/infrastructure`)

* **Gobernanza:** `ADR-0015` (Asincronía con Tokio Jobs).
* **Explicación:** Para que el sistema sea ultraveloz y no desgaste tu disco duro con miles de lecturas por segundo, cargamos los umbrales en la memoria RAM con protección para múltiples hilos (`RwLock`).
* [x] **2.4.1:** Diseñar el contenedor volátil `RuntimeConfig` que mantendrá los umbrales vivos en la RAM.
* [x] **2.4.2:** Implementar el mecanismo asíncrono seguro que permite actualizar la RAM en caliente en microsegundos sin bloquear el hilo principal.



### 🛣️ Slice 2.5: Endpoints de Configuración y Estado en Axum

* **Gobernanza:** `ADR-0003` (Backend Axum), `ADR-0006` (RBAC) y `ADR-0016` (OpenAPI Utoipa).
* **Explicación:** Abrimos las compuertas web seguras. Creamos los caminos para que el frontend pueda leer y escribir configuraciones, pero solo si el usuario tiene el rol autorizado.
* [x] **2.5.1:** Crear la ruta `GET /settings/thresholds` para entregar los datos instantáneamente desde la RAM.
* [x] **2.5.2:** Crear la ruta `PUT /settings/thresholds` que reciba los cambios de la web, obligue al dominio a validarlos, los guarde en Workbench y actualice la RAM.
* [x] **2.5.3:** Inyectar el middleware `RequireRole` para bloquear el acceso a intrusos y documentar la ruta usando las macros de Utoipa.



### 🎨 Slice 2.6: Formulario de Configuración Dinámico (Svelte 5 Runes)

* **Gobernanza:** `ADR-0017` (Frontend SvelteKit / Svelte 5).
* **Explicación:** Construimos el panel visual que tú vas a operar. Usamos la reactividad moderna de Svelte 5 para congelar la pantalla si pones datos erróneos.
* [x] **2.6.1** — Instalar e inicializar `shadcn-svelte` con Tailwind v4 en el proyecto web.
* [x] **2.6.2** — Crear el esquema de validación con `Zod` que replique las reglas de los umbrales de Rust.
* [x] **2.6.3** — Implementar `createQuery` y `createMutation` de `TanStack Query` para gestionar la lectura y escritura caliente contra Axum.
* [x] **2.6.4** — Montar la vista en `apps/web/src/routes/dashboard/settings/+page.svelte` usando los componentes de shadcn.



### 🏁 Slice 2.7: Pruebas de Integración y Cambio de Umbrales

* **Gobernanza:** `ADR-0010` (Testing y Calidad).
* **Explicación:** Rompemos las reglas a propósito para certificar que el sistema se defiende solo.
* [x] **2.7.1:** **Prueba de Rechazo:** Intentar guardar configuraciones incongruentes desde la UI y verificar que Axum devuelva un error de inmediato.
* [x] **2.7.2:** **Prueba de Impacto Local:** Guardar datos válidos y verificar visualmente en MySQL Workbench que los registros se modificaron en la tabla física.
* [x] **2.7.3:** **Prueba de Sincronización:** Confirmar que los nuevos parámetros se refleja en las lecturas de red sin necesidad de apagar o reiniciar el binario de Rust.
