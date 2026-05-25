// apps/web/src/lib/auth.svelte.ts
// Gestión de estado de autenticación con Svelte 5 Runes
// Vinculado con ADR-0017-frontend-sveltekit-svelte5.md

class AuthStore {
    // Estado reactivo puro encapsulado con $state
    #accessToken = $state<string | null>(null);
    #refreshToken = $state<string | null>(null);
    #userId = $state<string | null>(null);
    #role = $state<string | null>(null);
    #email = $state<string | null>(null);
    #refreshTimer: ReturnType<typeof setInterval> | null = null;

    constructor() {
        // Cargar estado desde cookies al inicializar
        this.#loadFromCookies();
    }

    // Valor computado reactivo para verificación de autenticación
    get isAuthenticated() {
        return this.#accessToken !== null;
    }

    get accessToken() {
        return this.#accessToken;
    }

    get refreshToken() {
        return this.#refreshToken;
    }

    get userId() {
        return this.#userId;
    }

    get role() {
        return this.#role;
    }

    get email() {
        return this.#email;
    }

    // Establecer tokens después de login exitoso
    setTokens(accessToken: string, refreshToken: string, userId: string, role: string, email: string) {
        this.#accessToken = accessToken;
        this.#refreshToken = refreshToken;
        this.#userId = userId;
        this.#role = role;
        this.#email = email;
        this.#saveToCookies();
        this.#startAutoRefresh();
    }

    // Limpiar sesión (logout)
    clearSession() {
        this.#accessToken = null;
        this.#refreshToken = null;
        this.#userId = null;
        this.#role = null;
        this.#email = null;
        this.#clearCookies();
        this.#stopAutoRefresh();
    }

    // Actualizar tokens después de refresh
    updateTokens(newAccessToken: string, newRefreshToken: string) {
        this.#accessToken = newAccessToken;
        this.#refreshToken = newRefreshToken;
        this.#saveToCookies();
    }

    // Iniciar refresh automático de tokens
    #startAutoRefresh() {
        this.#stopAutoRefresh();
        
        // Verificar cada 5 minutos si el token necesita refresh
        this.#refreshTimer = setInterval(() => {
            this.#checkAndRefresh();
        }, 5 * 60 * 1000); // 5 minutos
    }

    // Detener refresh automático
    #stopAutoRefresh() {
        if (this.#refreshTimer) {
            clearInterval(this.#refreshTimer);
            this.#refreshTimer = null;
        }
    }

    // Verificar y refrescar token si es necesario
    async #checkAndRefresh() {
        if (!this.#refreshToken) {
            return;
        }

        try {
            const response = await fetch('/api/auth/refresh', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    refresh_token: this.#refreshToken
                })
            });

            if (response.ok) {
                const data = await response.json();
                this.updateTokens(data.access_token, data.refresh_token);
            } else {
                // Si el refresh falla, limpiar la sesión
                this.clearSession();
            }
        } catch (error) {
            console.error('Error al refrescar token:', error);
            // En caso de error de red, no limpiamos la sesión inmediatamente
            // El siguiente intento lo hará
        }
    }

    // Guardar en cookies (httpOnly, Secure, SameSite=Strict)
    #saveToCookies() {
        if (typeof document !== 'undefined') {
            // Access token en cookie con httpOnly (debe hacerse desde el servidor)
            // Por ahora usamos sessionStorage como fallback para el cliente
            if (this.#accessToken) {
                sessionStorage.setItem('access_token', this.#accessToken);
            }
            if (this.#refreshToken) {
                sessionStorage.setItem('refresh_token', this.#refreshToken);
            }
            if (this.#userId) {
                sessionStorage.setItem('user_id', this.#userId);
            }
            if (this.#role) {
                sessionStorage.setItem('user_role', this.#role);
            }
            if (this.#email) {
                sessionStorage.setItem('user_email', this.#email);
            }
        }
    }

    // Cargar desde cookies
    #loadFromCookies() {
        if (typeof document !== 'undefined') {
            this.#accessToken = sessionStorage.getItem('access_token');
            this.#refreshToken = sessionStorage.getItem('refresh_token');
            this.#userId = sessionStorage.getItem('user_id');
            this.#role = sessionStorage.getItem('user_role');
            this.#email = sessionStorage.getItem('user_email');
            
            // Si hay tokens, iniciar refresh automático
            if (this.#accessToken && this.#refreshToken) {
                this.#startAutoRefresh();
            }
        }
    }

    // Limpiar cookies
    #clearCookies() {
        if (typeof document !== 'undefined') {
            sessionStorage.removeItem('access_token');
            sessionStorage.removeItem('refresh_token');
            sessionStorage.removeItem('user_id');
            sessionStorage.removeItem('user_role');
            sessionStorage.removeItem('user_email');
        }
    }
}

// Instancia singleton del store de autenticación
export const auth = new AuthStore();
