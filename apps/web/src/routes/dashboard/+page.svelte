<!-- apps/web/src/routes/dashboard/+page.svelte -->
<!-- Dashboard del Sistema de Monitoreo -->
<!-- Vinculado con ADR-0017-frontend-sveltekit-svelte5.md -->

<script lang="ts">
    import { auth } from '$lib/auth.svelte.ts';
    import { goto } from '$app/navigation';

    async function handleLogout() {
        try {
            await fetch('/api/auth/logout', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    refresh_token: auth.refreshToken
                })
            });
        } catch (error) {
            console.error('Error en logout:', error);
        } finally {
            auth.clearSession();
            await goto('/login');
        }
    }
</script>

<div class="min-h-screen bg-gray-100 p-8">
    <div class="max-w-6xl mx-auto">
        <div class="flex justify-between items-center mb-8">
            <h1 class="text-3xl font-bold text-gray-900">Dashboard</h1>
            <div class="flex gap-4">
                <a
                    href="/dashboard/settings"
                    class="bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded transition-colors"
                >
                    Configuración
                </a>
                <button
                    onclick={handleLogout}
                    class="bg-red-500 hover:bg-red-600 text-white font-semibold py-2 px-4 rounded transition-colors"
                >
                    Cerrar Sesión
                </button>
            </div>
        </div>

        <div class="bg-white rounded-lg shadow-md p-6 mb-6">
            <h2 class="text-xl font-semibold mb-4">Bienvenido, {auth.email || 'Usuario'}</h2>
            <p class="text-gray-600">Rol: {auth.role || 'No determinado'}</p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div class="bg-white rounded-lg shadow-md p-6">
                <h3 class="text-lg font-semibold mb-2">Monitores</h3>
                <p class="text-gray-600">Gestión de agentes de monitoreo</p>
            </div>

            <div class="bg-white rounded-lg shadow-md p-6">
                <h3 class="text-lg font-semibold mb-2">Infraestructura</h3>
                <p class="text-gray-600">Estado de la red regional</p>
            </div>

            <div class="bg-white rounded-lg shadow-md p-6">
                <h3 class="text-lg font-semibold mb-2">Alertas</h3>
                <p class="text-gray-600">Notificaciones y eventos</p>
            </div>
        </div>
    </div>
</div>