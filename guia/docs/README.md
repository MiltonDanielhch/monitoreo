# 📚 Documentación del Sistema de Monitoreo Lab 3030

## 🎯 Visión General

**Lab 3030** es un sistema integral de monitoreo de redes y telecomunicaciones diseñado para la Gobernación del Beni. El sistema permite supervisar en tiempo real la conectividad, rendimiento y disponibilidad de sedes regionales, dispositivos de red y servicios críticos a través de una arquitectura moderna basada en Rust (backend) y Svelte (frontend).

### 🏗️ Arquitectura del Sistema

```
┌─────────────────────────────────────────────────────────────┐
│                     Frontend (Svelte 5)                     │
│                   Dashboard Web UI                           │
└──────────────────────┬──────────────────────────────────────┘
                       │ HTTP/REST API
┌──────────────────────▼──────────────────────────────────────┐
│              Backend (Rust + Axum)                          │
│         API Server + Business Logic                         │
└──────────────────────┬──────────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────────┐
│              Database (MySQL)                                │
│         Telemetry + Configuration + Audit                    │
└─────────────────────────────────────────────────────────────┘
```

### 📦 Componentes Principales

- **Backend (Rust)**: API RESTful con Axum, procesamiento de telemetría, gestión de alertas
- **Frontend (Svelte 5)**: Dashboard interactivo con visualización en tiempo real
- **Base de Datos (MySQL)**: Almacenamiento de métricas, configuración y auditoría
- **Agentes Remotos**: Software instalado en sedes para recolección de datos

---

## 🚀 Características Principales

### 1. Monitoreo en Tiempo Real
- Supervisión continua de sedes regionales
- Métricas de ping, latencia, pérdida de paquetes
- Ancho de banda y rendimiento de enlaces
- Estado de dispositivos de red

### 2. Gestión de Alertas
- Detección automática de incidentes
- Notificaciones por correo electrónico (SMTP)
- Historial de alertas enviadas
- Umbrales configurables por tipo de métrica

### 3. Tareas en Segundo Plano
- Workers para procesamiento asíncrono
- Configuración de intervalos de ejecución
- Monitoreo de estado de workers
- Gestión de trabajos programados

### 4. Seguridad y Auditoría
- Detección de intrusiones y anomalías
- Historial inmutable de acciones (audit log)
- Autenticación con tokens PASETO
- Correlación de eventos de seguridad

### 5. Reportes y SLA
- Generación de reportes de Service Level Agreement
- Cálculo de disponibilidad por sede
- Exportación a PDF con firma criptográfica
- Análisis de cumplimiento contractual

### 6. Gestión de Infraestructura
- Archivos técnicos y topologías
- Documentación de redes
- Diagramas y configuraciones
- Gestión de archivos multimedia

---

## 📁 Estructura del Proyecto

```
monitoreo/
├── apps/
│   ├── api/              # Backend Rust (Axum)
│   └── web/              # Frontend SvelteKit
├── crates/
│   ├── domain/           # Lógica de negocio y modelos
│   ├── database/         # Repositorios y entidades
│   └── infrastructure/   # Handlers, configuración, utilidades
├── data/
│   └── migrations/       # Migraciones de base de datos
└── guia/
    ├── docs/             # Documentación del sistema
    └── roadmap/          # Roadmap de módulos
```

---

## 🎛️ Módulos del Sistema

El sistema está organizado en 5 módulos principales accesibles desde el sidebar:

### 📊 Monitoreo
- **Inicio**: Dashboard principal con resumen general
- **Sedes**: Gestión de sedes regionales
- **Dispositivos**: Monitoreo de dispositivos de red
- **Métricas**: Indicadores de rendimiento detallados
- **Alertas**: Gestión de incidentes críticos
- **Agentes**: Conectividad de agentes remotos

### 🔍 Descubrimiento
- **Dispositivos**: Dispositivos descubiertos en la red
- **Nuevo Escaneo**: Iniciar escaneo de red para detectar nuevos dispositivos

### ⚙️ Sistema
- **Workers**: Monitoreo de tareas en segundo plano
- **Config. Workers**: Configuración de workers
- **Infraestructura**: Archivos técnicos y topologías
- **Notificaciones**: Historial de alertas enviadas

### 🔒 Seguridad
- **Seguridad**: Detección de intrusiones
- **Auditoría**: Historial inmutable de acciones

### 📄 Reportes
- **Reportes**: Generación de reportes SLA
- **Config. SMTP**: Configuración de servidor de correo
- **Configuración**: Ajustes generales del sistema

---

## 🛠️ Tecnologías Utilizadas

### Backend
- **Rust**: Lenguaje principal del backend
- **Axum**: Framework web para API REST
- **Sea-ORM**: ORM para base de datos
- **Tokio**: Runtime asíncrono
- **PASETO**: Tokens de autenticación
- **SQLx**: Consultas SQL tipo-safe

### Frontend
- **Svelte 5**: Framework de UI reactivo
- **SvelteKit**: Framework de aplicación web
- **Tailwind CSS**: Framework de estilos
- **Lucide Svelte**: Iconos
- **TanStack Query**: Gestión de datos

### Base de Datos
- **MySQL**: Sistema de base de datos relacional
- **Workbench**: Herramienta de administración

---

## 🚀 Inicio Rápido

### Requisitos Previos
- Rust 1.70+
- Node.js 18+
- MySQL 8.0+
- PowerShell 7+ (recomendado)

### Instalación

1. **Clonar el repositorio**
```bash
git clone <repository-url>
cd monitoreo
```

2. **Configurar variables de entorno**
```bash
cp .env.example .env.local
# Editar .env.local con tus credenciales
```

3. **Ejecutar migraciones**
```bash
just migrate
```

4. **Iniciar el backend**
```bash
just run-api
```

5. **Iniciar el frontend**
```bash
just run-web
```

6. **Acceder al sistema**
```
Frontend: http://localhost:5173
API: http://localhost:3000
```

---

## 📖 Documentación por Módulo

Para información detallada sobre cada módulo, consulta:

- [Módulo de Monitoreo](./MODULO_MONITOREO.md)
- [Módulo de Descubrimiento](./MODULO_DESCUBRIMIENTO.md)
- [Módulo de Sistema](./MODULO_SISTEMA.md)
- [Módulo de Seguridad](./MODULO_SEGURIDAD.md)
- [Módulo de Reportes](./MODULO_REPORTES.md)

---

## 🔧 Configuración

### Variables de Entorno

```env
DATABASE_URL=mysql://user:password@localhost:3306/database
SERVER_PORT=3000
JWT_SECRET=your-secret-key
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USER=user@example.com
SMTP_PASS=password
```

### Umbrales de Alerta

- **Ping**: Normal < 100ms, Crítico > 500ms
- **Latencia**: Normal < 150ms, Crítico > 800ms
- **Pérdida de Paquetes**: Normal < 5%, Crítico > 15%

---

## 🤝 Contribución

Para contribuir al desarrollo del sistema:

1. Revisa el [roadmap](../roadmap/) para了解 los módulos en desarrollo
2. Sigue las ADR (Architecture Decision Records) definidas
3. Mantén el código limpio y bien documentado
4. Realiza pruebas antes de hacer commit

---

## 📞 Soporte

Para reportar problemas o solicitar ayuda, contacta al equipo de desarrollo.

---

**Última actualización**: Junio 2026
**Versión**: 1.0.0
