# ADR 0016: OpenAPI: Utoipa + Scalar + IA-Ready
**Autores:** Milton Hipamo / Laboratorio 3030

**Relacionado con:** ADR 0001 (Arquitectura Hexagonal), ADR 0003 (Backend Axum), ADR 0008 (PASETO Auth), ADR 0010 (Testing), ADR 0015 (Canales Tokio Nativos), ADR 0020 (Monitoreo Regional)

---

## 📋 Contexto y Decisión

**Problema:** La documentación manual de las APIs tiende inevitablemente a desincronizarse del código fuente de negocio a medida que el backend evoluciona. La existencia de múltiples clientes, la necesidad de un frontend type-safe y la futura integración con agentes de IA requieren un contrato de API inmutable, autogenerado y de confianza absoluta que actúe como una única fuente de verdad.

**Decisión:** Adoptar oficialmente **Utoipa v5.5.0** para la generación estática y automática del esquema OpenAPI desde las macros de Rust, **Scalar v0.3.0** como la interfaz de usuario moderna para desarrollo y documentación interactiva, y el endpoint `/openapi.json` como el contrato de datos crudo optimizado para consumo automatizado (IA-ready). El frontend consumirá este archivo mediante **`openapi-typescript`** para derivar tipos puros en Svelte 5 en tiempo de compilación.

### Flujo de Datos del Contrato Inmutable:

```
  [ Rust Handlers + DTOs ] (Con macros #[utoipa::path] y #[derive(ToSchema)])
            │
            ▼
  [ OpenAPI Generator (Utoipa) ]
            │
            ▼
  [ /openapi.json ] ───▶ [ Agentes de IA / Consumo Autónomo ]
            │
            ├───▶ [ Scalar UI ] (Documentación Visual Interactiva)
            │
            ▼
  [ openapi-typescript ] ───▶ [ Tipos Estrictos Svelte 5 (Runes) ]

```

---

## 🛠️ Herramientas y Toolchain Aprobado

| Herramienta / Crate | Versión Fijada | Propósito Arquitectónico | Estado |
| --- | --- | --- | --- |
| `utoipa` | **5.5.0** | Generación del esquema OpenAPI a través de macros compiladas. Se activa la feature `time` para homologar el manejo de fechas. | ✅ Activa |
| `utoipa-scalar` | **0.3.0** | Renderizado visual e interactivo de la documentación técnica. | ✅ Activa |
| `utoipa-axum` | **0.2.0** | Extensión de bindings nativos compatible con los extractores de **Axum 0.8**. | ✅ Activa |
| `openapi-typescript` | **7.13.0** | Generación de tipos TypeScript puros para Svelte 5 desde el JSON, eliminando sobrecarga en tiempo de ejecución. | ✅ Activa |
| `@stoplight/spectral-cli` | **6.15.x** | Linter automatizado en el Puente para asegurar la validez del esquema OpenAPI. | ✅ Activa |
| `cargo-udeps` | latest | Auditor de dependencias del espacio de trabajo para eliminar crates no utilizados en la compilación final. | ✅ Activa |

---

## 🏷️ Estructura Semántica de Tags (ADR 0020)

Para mantener la coherencia con el monitoreo regional y los módulos de la arquitectura hexagonal, el esquema OpenAPI segmentará los endpoints bajo los siguientes tags estrictos de primer nivel:

* `auth`: Ciclo de vida de sesiones, apretón de manos y validación de tokens PASETO.
* `users`: Gestión soberana de identidades y credenciales de acceso.
* `sedes`: Nodos físicos y demarcaciones geográficas del Laboratorio.
* `devices`: Telemetría e inventariado de hardware desplegado.
* `metrics`: Volcado y lectura de series de rendimiento del sistema.
* `alerts`: Despacho e historial de eventos de emergencia y switches automáticos.
* `topology`: Mapeo de la red privada del ecosistema.
* `intrusions`: Registro y auditoría del sistema de seguridad perimetral.
* `agents`: Endpoints optimizados para el control e interacción de software autónomo.

---

## 💻 Ejemplo de Implementación Estricta

Los DTOs (*Data Transfer Objects*) del backend deben implementar las anotaciones requeridas para nutrir el esquema sin alterar la lógica de negocio pura:

```rust
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct DeviceTelemetryDto {
    /// Identificador único del dispositivo de red
    #[schema(example = "dev_01j8m4x9b2...")]
    pub device_id: String,
    
    /// Valor neto de la lectura de carga útil
    #[schema(example = 42.85)]
    pub payload_value: f64,
    
    /// Timestamp exacto de la captura mapeado mediante el crate time
    #[schema(value_type = String, format = DateTime, example = "2026-05-23T15:30:00Z")]
    pub captured_at: OffsetDateTime,
}

```

---

## 🛡️ Directrices de Seguridad y Despliegue

* **Entornos Restringidos:** La interfaz visual de Scalar (`/scalar` u `/docs`) estará habilitada de forma nativa en el entorno de desarrollo local. En producción, su acceso estará estrictamente condicionado tras la validación de tokens de administración o desactivada mediante variables de entorno en Coolify, manteniendo expuesto únicamente el `/openapi.json` crudo para los servicios autorizados.
* **Validación en el Pipeline (CI):** El pipeline del Puente compilará el esquema e invocará a `@stoplight/spectral-cli` de forma obligatoria. Si el linter detecta descripciones vacías, faltas de ejemplos en los DTOs o tags inválidos, la construcción se considerará fallida y no se procederá al despliegue.
* **Sincronización del Frontend:** El proceso de generación de tipos TypeScript se ejecutará mediante un script automatizado del `justfile` antes de cada compilación de la interfaz de usuario, garantizando que el frontend de Svelte 5 rompa en tiempo de compilación si el backend modificó la firma de algún contrato de datos.

---