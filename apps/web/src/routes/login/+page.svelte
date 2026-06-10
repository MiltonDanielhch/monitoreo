<script lang="ts">
    import { auth } from '$lib/auth.svelte';
    import { goto } from '$app/navigation';

    let email = $state('');
    let password = $state('');
    let isLoading = $state(false);
    let errorMessage = $state('');

    // CÓDIGO 3026: Apuntamos directamente al contenedor de Axum en producción
    const API_URL = 'http://q5q91n0vgnt82ofr4alpioip.190.129.54.198.sslip.io';

    async function handleLogin(event: Event) {
        event.preventDefault();
        isLoading = true;
        errorMessage = '';

        console.log('Intentando login con email:', email);
        // Ahora el log reflejará la URL absoluta del backend productivo
        console.log('URL del endpoint:', `${API_URL}/api/auth/login`);

        try {
            // Reemplazada la ruta relativa por la URL absoluta del backend
            const response = await fetch(`${API_URL}/api/auth/login`, {
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