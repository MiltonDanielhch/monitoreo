# 🛠️ Auditoría de Software — Lab 3030

> Generado: `2026-05-23 23:00`

## Resumen

| Métrica | Valor |
| :--- | :--- |
| **Proyecto** | `monitoreo` |
| **Líneas de Código (Netas)** | 3951 LoC |
| **Peso Total del Proyecto** | 296.31KB |
| **Timestamp** | 2026-05-23 23:00 |
| **Estado** | Activa |

## Breakdown por Capa

| Capa / Archivo | LoC | Peso | % LoC |
| :--- | ---: | ---: | ---: |
| `guia` | 3805 | 290.88KB | 96.3% ███████████████████ |
| `audit.py` | 146 | 5.43KB | 3.7%  |
| **TOTAL** | **3951** | **296.31KB** | 100% |

## Mapa de Arquitectura

```text
monitoreo/
├── audit.py (146 LoC | 5.43KB)
└── guia/ [290.88KB]
    ├── PROMPT_MAESTRO.md (93 LoC | 7.65KB)
    ├── adr/ [168.57KB]
    │   ├── ADR-0001-arquitectura-hexagonal.md (94 LoC | 8.49KB)
    │   ├── ADR-0002-configuracion-tipeada-secretos.md (48 LoC | 5.18KB)
    │   ├── ADR-0003-stack-backend-rust-axum.md (59 LoC | 6.26KB)
    │   ├── ADR-0004-persistencia-mysql-seaorm-docker.md (35 LoC | 4.38KB)
    │   ├── ADR-0005-migraciones-seeding.md (73 LoC | 6.22KB)
    │   ├── ADR-0006-rbac-sessions-audit.md (44 LoC | 6.06KB)
    │   ├── ADR-0007-manejo-errores.md (80 LoC | 7.77KB)
    │   ├── ADR-0008-seguridad-auth-paseto.md (38 LoC | 4.83KB)
    │   ├── ADR-0009-rate-limiting.md (111 LoC | 6.39KB)
    │   ├── ADR-0010-testing-calidad.md (102 LoC | 9.86KB)
    │   ├── ADR-0011-estandares-desarrollo.md (103 LoC | 10.73KB)
    │   ├── ADR-0012-herramientas-desarrollo.md (88 LoC | 7.08KB)
    │   ├── ADR-0013-infraestructura-docker-compose.md (83 LoC | 6.85KB)
    │   ├── ADR-0014-monitoreo-tareas-criticas.md (67 LoC | 6.25KB)
    │   ├── ADR-0015-tokio-jobs.md (99 LoC | 8.03KB)
    │   ├── ADR-0016-documentacion-openapi-utoipa.md (70 LoC | 5.45KB)
    │   ├── ADR-0017-frontend-sveltekit-svelte5.md (108 LoC | 9.50KB)
    │   ├── ADR-0018-sintonia-cli.md (88 LoC | 8.89KB)
    │   ├── ADR-0019-coolify-deploy.md (112 LoC | 8.46KB)
    │   ├── ADR-0020-monitoreo-infraestructura-regional.md (116 LoC | 8.40KB)
    │   ├── ADR-0021-local-first-sync-offline.md (140 LoC | 7.80KB)
    │   ├── ADR-0022-agentes-monitoreo-distribuidos.md (148 LoC | 10.08KB)
    │   └── justifile.md (112 LoC | 5.62KB)
    └── roadmap/ [114.67KB]
        ├── ROADMAP_MODULO_0_SETUP.md (373 LoC | 14.09KB)
        ├── ROADMAP_MODULO_10_DESPLIEGUE.md (103 LoC | 10.81KB)
        ├── ROADMAP_MODULO_1_AUTH.md (128 LoC | 8.84KB)
        ├── ROADMAP_MODULO_2_CONFI_SISTEMA.md (248 LoC | 12.57KB)
        ├── ROADMAP_MODULO_3_DASHBOARD.md (173 LoC | 8.73KB)
        ├── ROADMAP_MODULO_4_NOTIFICACIONES.md (119 LoC | 10.64KB)
        ├── ROADMAP_MODULO_5_ARCHIVO_INFRA _TOPOLOGIA.md (117 LoC | 10.00KB)
        ├── ROADMAP_MODULO_6_AUDITORIA_DINAMICA_INMUTABLE.md (117 LoC | 10.11KB)
        ├── ROADMAP_MODULO_7_API_TELEMATRIA.md (108 LoC | 9.86KB)
        ├── ROADMAP_MODULO_8_TAREA SEGUNDO_PLANO_AUTOMATIZACION.md (102 LoC | 9.31KB)
        └── ROADMAP_MODULO_9_REPORTES.md (106 LoC | 9.70KB)
```
