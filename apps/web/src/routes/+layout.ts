// apps/web/src/routes/+layout.ts
// Filtros de navegación - Protección de rutas en cliente
// Vinculado con ADR-0017-frontend-sveltekit-svelte5.md

import { redirect } from '@sveltejs/kit';
import { browser } from '$app/environment';
import type { LayoutLoad } from './$types';

// CONFIGURACIÓN VITAL PARA COMPILAR COMO SPA EN COOLIFY/NGINX
export const prerender = false;
export const ssr = false;

const PROTECTED_PATHS = ['/dashboard', '/infraestructura', '/admin', '/operaciones'];

export const load: LayoutLoad = ({ url }) => {
    if (browser) {
        const isProtectedPath = PROTECTED_PATHS.some(path => url.pathname.startsWith(path));

        if (isProtectedPath) {
            const accessToken = sessionStorage.getItem('access_token');

            if (!accessToken) {
                throw redirect(307, '/login');
            }
        }
    }

    return {};
};