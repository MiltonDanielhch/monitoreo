# Configuración de Proxy Inverso en Coolify para API

## Problema
Error 405 (Method Not Allowed) en producción porque Nginx estático intenta resolver `/api/*` como archivos físicos en lugar de proxyarlos al backend Axum.

## Solución
Configurar proxy inverso en Nginx para redirigir `/api/*` al contenedor Axum.

## Pasos en Coolify

### 1. Obtener el hostname del backend Axum
En el panel de Coolify:
- Ve a tu aplicación del backend (Axum)
- En la sección de "Network" o "Domains"
- Copia el **Internal Hostname** o **Service Name** del contenedor
- Ejemplo: `axum-app-1` o similar

### 2. Configurar el archivo nginx.conf
El archivo `apps/web/nginx.conf` ya está actualizado con la configuración del proxy inverso.

**IMPORTANTE:** Reemplaza `BACKEND_HOST` en la línea 14 con el hostname real del backend:

```nginx
location /api/ {
    proxy_pass http://axum-app-1:8000;  # Reemplazar BACKEND_HOST
    # ... resto de configuración
}
```

### 3. Aplicar la configuración en Coolify

#### Opción A: Usar configuración personalizada de Nginx
1. En tu aplicación del frontend (SvelteKit) en Coolify
2. Ve a "Settings" → "Build"
3. En "Nginx Configuration" o "Custom Nginx Config"
4. Pega el contenido de `apps/web/nginx.conf`
5. **IMPORTANTE:** Reemplaza `BACKEND_HOST` con el hostname real
6. Guarda y redeploy

#### Opción B: Usar variables de entorno
1. En tu aplicación del frontend en Coolify
2. Ve a "Settings" → "Environment Variables"
3. Agrega: `BACKEND_HOST=axum-app-1` (o el hostname real)
4. Modifica el nginx.conf para usar la variable:
```nginx
location /api/ {
    proxy_pass http://${BACKEND_HOST}:8000;
    # ... resto de configuración
}
```

### 4. Verificar conexión entre contenedores
En Coolify, los contenedores deben estar en la misma red Docker para comunicarse. Verifica:
- Ambas aplicaciones (frontend y backend) están en el mismo proyecto
- O están en redes Docker que pueden comunicarse

### 5. Redeploy
- Haz redeploy del frontend para aplicar la nueva configuración de Nginx
- Verifica que el backend esté corriendo

## Verificación

### En local (desarrollo)
El archivo `vite.config.ts` ya tiene el proxy configurado:
```typescript
server: {
    proxy: {
        '/api': {
            target: 'http://localhost:3000',
            changeOrigin: true,
            secure: false
        }
    }
}
```

### En producción
El archivo `apps/web/nginx.conf` maneja el proxy inverso.

## Archivo de login
El archivo `apps/web/src/routes/login/+page.svelte` ya está correcto usando rutas relativas:
```javascript
const response = await fetch('/api/auth/login', {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json',
    },
    body: JSON.stringify({ email, password })
});
```

No requiere cambios - funciona tanto en local como en producción.

## Troubleshooting

### Error 502 Bad Gateway
- Verifica que el hostname del backend sea correcto
- Verifica que el backend esté corriendo en el puerto 8000
- Verifica que los contenedores puedan comunicarse (misma red Docker)

### Error 404 en /api/*
- Verifica que la configuración de nginx.conf esté aplicada
- Verifica que el archivo nginx.conf se esté usando en el deploy

### Error de CORS
- La configuración de nginx ya incluye los headers necesarios
- Verifica que el backend no tenga restricciones de CORS adicionales
