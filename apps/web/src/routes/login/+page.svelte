<!-- apps/web/src/routes/login/+page.svelte -->
<!-- Vista de login con Tailwind v4 y Svelte 5 Runes -->
<!-- Vinculado con ADR-0017-frontend-sveltekit-svelte5.md -->

<script lang="ts">
    import { auth } from '$lib/auth.svelte';
    import { goto } from '$app/navigation';

    let email = $state('');
    let password = $state('');
    let isLoading = $state(false);
    let errorMessage = $state('');

    async function handleLogin(event: Event) {
        event.preventDefault();
        isLoading = true;
        errorMessage = '';

        console.log('Intentando login con email:', email);
        console.log('URL del endpoint:', '/api/auth/login');

        try {
            const response = await fetch('/api/auth/login', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    email,
                    password
                })
            });

            console.log('Respuesta del servidor:', response.status, response.statusText);

            if (response.ok) {
                const data = await response.json();
                console.log('Datos recibidos:', data);
                auth.setTokens(
                    data.access_token,
                    data.refresh_token,
                    data.user_info.id,
                    data.user_info.role,
                    data.user_info.email
                );
                
                // Redirigir al dashboard
                await goto('/dashboard');
            } else {
                const errorData = await response.json();
                console.error('Error del servidor:', errorData);
                errorMessage = errorData.message || 'Error al iniciar sesión';
            }
        } catch (error) {
            console.error('Error de conexión:', error);
            errorMessage = 'Error de conexión. Por favor, intente nuevamente.';
        } finally {
            isLoading = false;
        }
    }
</script>

<div class="min-h-screen flex items-center justify-center bg-gray-100">
    <div class="max-w-md w-full bg-white rounded-lg shadow-md p-8">
        <div class="text-center mb-8">
            <h1 class="text-3xl font-bold text-gray-900">Iniciar Sesión</h1>
            <p class="text-gray-600 mt-2">Sistema de Monitoreo Regional - Lab 3030</p>
        </div>

        {#if errorMessage}
            <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded mb-4">
                {errorMessage}
            </div>
        {/if}

        <form onsubmit={handleLogin} class="space-y-6">
            <div>
                <label for="email" class="block text-sm font-medium text-gray-700 mb-2">
                    Correo Electrónico
                </label>
                <input
                    id="email"
                    type="email"
                    bind:value={email}
                    required
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                    placeholder="usuario@ejemplo.com"
                />
            </div>

            <div>
                <label for="password" class="block text-sm font-medium text-gray-700 mb-2">
                    Contraseña
                </label>
                <input
                    id="password"
                    type="password"
                    bind:value={password}
                    required
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                    placeholder="••••••••"
                />
            </div>

            <button
                type="submit"
                disabled={isLoading}
                class="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
                {#if isLoading}
                    <span class="flex items-center justify-center">
                        <svg class="animate-spin h-5 w-5 mr-2" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"></circle>
                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                        Cargando...
                    </span>
                {:else}
                    Iniciar Sesión
                {/if}
            </button>
        </form>

        <div class="mt-6 text-center text-sm text-gray-600">
            <p>¿Olvidó su contraseña? <a href="/forgot-password" class="text-blue-600 hover:text-blue-800">Recuperar</a></p>
        </div>
    </div>
</div>
