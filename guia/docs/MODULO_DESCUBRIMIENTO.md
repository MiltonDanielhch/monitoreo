# 🔍 Módulo de Descubrimiento de Red

## 🎯 Descripción General

El **Módulo de Descubrimiento** proporciona capacidades avanzadas de escaneo automático de red para detectar dispositivos, obtener sus características y mantener un inventario actualizado de la infraestructura. Este módulo utiliza técnicas de escaneo de red, clasificación inteligente de dispositivos y lookup de fabricantes para mantener un inventario preciso y actualizado.

---

## 🏛️ Componentes del Módulo

### 1. Dispositivos

**Propósito**: Visualización y gestión de dispositivos descubiertos en la red.

**Funcionalidades**:
- Listado de todos los dispositivos descubiertos
- Filtros por tipo, estado, autorización, fabricante
- Clasificación automática de dispositivos
- Gestión de autorización de dispositivos
- Historial de escaneos por dispositivo

**Información de Dispositivo**:
- Dirección IP
- Dirección MAC
- Hostname (si disponible)
- Tipo de dispositivo (Router, Switch, Server, PC, Mobile, IoT, Printer, Unknown)
- Fabricante (OUI lookup)
- Sistema operativo detectado
- Puertos abiertos
- Servicios detectados
- Estado (Online, Offline, Unknown)
- Autorización (Autorizado/No autorizado)
- Última vez visto
- Primera vez visto

**Tipos de Dispositivos**:
- **Router**: Enrutadores de red (puertos 22, 23, 80, 443, 161)
- **Switch**: Conmutadores de red (puertos 22, 23, 161)
- **Server**: Servidores (puertos 22, 80, 443, 3306, 5432)
- **PC**: Computadoras personales (puertos 135, 139, 445)
- **Mobile**: Dispositivos móviles
- **IoT**: Dispositivos IoT (puertos 80, 443, 1883)
- **Printer**: Impresoras (puertos 9100, 515, 631)
- **Unknown**: Dispositivo no clasificado

**Estados de Dispositivo**:
- 🟢 **Online**: Dispositivo respondiendo a pings
- 🔴 **Offline**: Dispositivo no respondiendo
- ⚪ **Unknown**: Estado desconocido

**Autorización**:
- ✅ **Autorizado**: Dispositivo aprobado en la red
- ❌ **No autorizado**: Dispositivo no reconocido o sospechoso

**Flujo de Uso**:
1. Usuario accede a sección de Dispositivos
2. Sistema muestra lista de dispositivos descubiertos
3. Usuario puede filtrar por criterios
4. Al hacer clic, ve detalles completos del dispositivo
5. Puede marcar dispositivo como autorizado/no autorizado
6. Sistema actualiza estado en base de datos

---

### 2. Nuevo Escaneo

**Propósito**: Iniciar escaneos de red para descubrir nuevos dispositivos o actualizar inventario existente.

**Funcionalidades**:
- Configuración de rango de IPs a escanear
- Selección de tipo de escaneo
- Configuración de puertos a escanear
- Visualización de progreso en tiempo real
- Cancelación de escaneos en progreso
- Historial de escaneos realizados

**Tipos de Escaneo**:
- **Full**: Escaneo completo de todos los puertos y servicios
- **Partial**: Escaneo de puertos comunes
- **Targeted**: Escaneo de IPs específicas

**Parámetros de Escaneo**:
- **Rango de IPs**: CIDR (ej: 192.168.1.0/24)
- **Puertos a escanear**: Lista de puertos TCP/UDP
- **Timeout**: Tiempo de espera por host (ms)
- **Concurrencia**: Número máximo de hosts simultáneos

**Proceso de Escaneo**:
1. Usuario configura parámetros del escaneo
2. Sistema valida rango de IPs
3. Inicia motor de escaneo con configuración
4. Escanea IPs en paralelo usando rayon
5. Realiza ping ICMP para detectar hosts activos
6. Realiza ARP scan para obtener direcciones MAC
7. Realiza port scanning TCP/UDP
8. Realiza OS fingerprinting
9. Realiza DNS reverse lookup para hostname
10. Realiza OUI lookup para fabricante
11. Clasifica dispositivo según puertos y servicios
12. Guarda dispositivos en base de datos
13. Actualiza progreso en tiempo real

**Visualización de Progreso**:
- Porcentaje completado
- IPs escaneadas / total
- Dispositivos encontrados
- Tiempo transcurrido
- Tiempo estimado restante

**Flujo de Uso**:
1. Usuario accede a sección de Nuevo Escaneo
2. Configura rango de IPs y parámetros
3. Hace clic en "Iniciar Escaneo"
4. Sistema muestra progreso en tiempo real
5. Puede cancelar escaneo si es necesario
6. Al completar, muestra resumen de dispositivos encontrados
7. Usuario puede navegar a lista de dispositivos

---

## 🔬 Motor de Escaneo de Red

### Arquitectura

```
┌─────────────────────────────────────────────────────────┐
│              Configuración de Escaneo                     │
│  Rango IPs | Puertos | Timeout | Concurrencia           │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────▼────────────┐
        │  Motor de Escaneo        │
        │  - Ping ICMP             │
        │  - ARP Scan              │
        │  - Port Scanning         │
        │  - OS Fingerprinting     │
        │  - DNS Reverse Lookup    │
        │  - OUI Lookup            │
        └────────────┬────────────┘
                     │
        ┌────────────▼────────────┐
        │  Clasificación de       │
        │  Dispositivos           │
        └────────────┬────────────┘
                     │
        ┌────────────▼────────────┐
        │  Repositorio de         │
        │  Descubrimiento         │
        └─────────────────────────┘
```

### Técnicas de Escaneo

**Ping ICMP**:
- Envía paquetes ICMP Echo Request
- Detecta hosts activos en la red
- Mide tiempo de respuesta

**ARP Scan**:
- Envía paquetes ARP Request
- Obtiene direcciones MAC de hosts
- Identifica fabricante vía OUI

**Port Scanning**:
- Escanea puertos TCP/UDP específicos
- Identifica servicios en ejecución
- Detecta puertos abiertos/cerrados/filtrados

**OS Fingerprinting**:
- Analiza respuestas de red
- Identifica sistema operativo
- Determina versión aproximada

**DNS Reverse Lookup**:
- Consulta DNS para obtener hostname
- Resuelve IP a nombre de dominio
- Facilita identificación de dispositivos

**OUI Lookup**:
- Consulta base de datos OUI IEEE
- Identifica fabricante por dirección MAC
- Usa primeros 24 bits (6 caracteres hex)

---

## 🏷️ Clasificación de Dispositivos

### Reglas de Clasificación

**Router**:
- Puertos: 22 (SSH), 23 (Telnet), 80 (HTTP), 443 (HTTPS), 161 (SNMP)
- Características: Múltiples interfaces de red, NAT

**Switch**:
- Puertos: 22 (SSH), 23 (Telnet), 161 (SNMP)
- Características: Múltiples puertos Ethernet, gestión VLAN

**Server**:
- Puertos: 22 (SSH), 80 (HTTP), 443 (HTTPS), 3306 (MySQL), 5432 (PostgreSQL)
- Características: Servicios de base de datos, web

**PC**:
- Puertos: 135 (RPC), 139 (NetBIOS), 445 (SMB)
- Características: Protocolos de Windows, compartición de archivos

**IoT**:
- Puertos: 80 (HTTP), 443 (HTTPS), 1883 (MQTT)
- Características: Protocolos IoT, firmware específico

**Printer**:
- Puertos: 9100 (RAW), 515 (LPD), 631 (IPP)
- Características: Servicios de impresión

---

## 📊 Base de Datos

### Tablas

**discovered_devices**:
- Almacena información de dispositivos descubiertos
- Índices en IP, MAC, estado, tipo, sede
- Histórico de primera y última vez visto

**network_scans**:
- Registra escaneos de red realizados
- Información de configuración y resultados
- Métricas de duración y dispositivos encontrados

---

## 🔗 Integraciones

### Dependencias del Sistema

- **Módulo de Monitoreo**: Integra dispositivos descubiertos en monitoreo
- **Módulo de Seguridad**: Detecta dispositivos no autorizados como amenazas
- **Módulo de Auditoría**: Registra acciones de descubrimiento
- **Módulo de Sistema**: Workers procesan escaneos en segundo plano

### Servicios Externos

- **DNS Servers**: Resolución de nombres
- **OUI Database**: Identificación de fabricantes
- **NTP**: Sincronización de tiempo para timestamps

---

## 📈 Casos de Uso

### Caso 1: Escaneo de Nueva Sede

1. Administrador configura nueva sede en sistema
2. Accede a sección de Nuevo Escaneo
3. Configura rango de IPs de la sede (ej: 192.168.10.0/24)
4. Selecciona tipo de escaneo "Full"
5. Inicia escaneo
6. Sistema descubre 25 dispositivos
7. Clasifica automáticamente: 3 routers, 5 switches, 10 PCs, 5 IoT, 2 printers
8. Administrador revisa lista y marca dispositivos autorizados
9. Sistema integra dispositivos en monitoreo

### Caso 2: Detección de Dispositivo No Autorizado

1. Escaneo rutinario detecta nuevo dispositivo
2. Sistema clasifica como "No autorizado"
3. Módulo de Seguridad genera alerta
4. Administrador investiga dispositivo
5. Verifica que es dispositivo personal de empleado
6. Marca dispositivo como autorizado
7. Sistema actualiza estado y cierra alerta

### Caso 3: Actualización de Inventario

1. Administrador necesita actualizar inventario
2. Inicia escaneo completo de todas las sedes
3. Sistema compara con inventario existente
4. Identifica dispositivos nuevos y removidos
5. Actualiza base de datos con cambios
6. Genera reporte de diferencias
7. Administrador aprueba cambios

---

## 🚨 Procedimientos de Emergencia

### Dispositivo Sospechoso Detectado

1. Sistema detecta dispositivo no autorizado
2. Genera alerta de seguridad
3. Administrador investiga inmediatamente
4. Si es intrusión, bloquea acceso a red
5. Notifica a equipo de seguridad
6. Documenta incidente en auditoría
7. Implementa medidas preventivas

### Escaneo Fallido

1. Escaneo se detiene o falla
2. Verificar configuración de red
3. Revisar logs del motor de escaneo
4. Si es error de red, corregir conectividad
5. Si es error de configuración, ajustar parámetros
6. Reintentar escaneo
7. Documentar solución

---

## 📚 Referencias

- [Roadmap del Módulo de Descubrimiento](../roadmap/ROADMAP_MODULO_12_DESCUBRIMIENTO_RED.md)
- [Documentación del Motor de Escaneo](../discovery/scan-engine.md)
- [Guía de Clasificación de Dispositivos](../discovery/classification.md)

---

**Última actualización**: Junio 2026
**Versión**: 1.0.0
