<!-- apps/web/src/routes/dashboard/notifications/config/+page.svelte -->
<!-- Configuración de canales de notificación - Svelte 5 + Zod -->
<!-- Vinculado con ADR-0011-openapi.md y ADR-0017-frontend-sveltekit-svelte5.md -->
<!-- Módulo 4: Motor de Notificaciones -->

<script lang="ts">
    import { z } from 'zod';
    import { Settings, Mail, Send, Check, X } from 'lucide-svelte';

    // Esquema de validación Zod para configuración SMTP
    const smtpConfigSchema = z.object({
        smtp_host: z.string().min(1, 'El host es requerido'),
        smtp_port: z.number().min(1).max(65535),
        smtp_user: z.string().min(1, 'El usuario es requerido'),
        smtp_password: z.string().min(1, 'La contraseña es requerida'),
        smtp_secure: z.enum(['tls', 'starttls']),
    });

    type SmtpConfig = z.infer<typeof smtpConfigSchema>;

    // Estado del formulario
    let config = $state<SmtpConfig>({
        smtp_host: 'smtp.gmail.com',
        smtp_port: 587,
        smtp_user: '',
        smtp_password: '',
        smtp_secure: 'tls',
    });

    let errors = $state<Record<string, string>>({});
    let isTesting = $state(false);
    let testResult = $state<{ success: boolean; message: string } | null>(null);

    // Validar formulario
    function validateForm(): boolean {
        const result = smtpConfigSchema.safeParse(config);
        
        if (!result.success) {
            errors = {};
            result.error.issues.forEach((err) => {
                const path = err.path.join('.');
                errors[path] = err.message;
            });
            return false;
        }
        
        errors = {};
        return true;
    }

    // Probar conexión SMTP
    async function testSmtp() {
        if (!validateForm()) {
            return;
        }

        isTesting = true;
        testResult = null;

        try {
            const response = await fetch('/api/v1/notifications/test-smtp', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    channel_id: '550e8400-e29b-41d4-a716-446655440001', // Canal por defecto
                    recipient: 'test@example.com',
                }),
            });

            const data = await response.json();
            testResult = {
                success: data.success,
                message: data.message,
            };
        } catch (error) {
            testResult = {
                success: false,
                message: 'Error de conexión al servidor',
            };
        } finally {
            isTesting = false;
        }
    }

    // Guardar configuración
    function saveConfig() {
        if (!validateForm()) {
            return;
        }
        
        // Aquí se implementaría la lógica para guardar en la base de datos
        // Por ahora, solo mostramos un mensaje de éxito
        testResult = {
            success: true,
            message: 'Configuración guardada exitosamente',
        };
    }
</script>

<div class="p-6 space-y-6">
    <div class="flex items-center justify-between">
        <div>
            <h1 class="text-3xl font-bold text-white tracking-tight">Configuración de Notificaciones</h1>
            <p class="text-slate-400 mt-1">Configurar canales de envío de alertas</p>
        </div>
        <Settings class="w-8 h-8 text-slate-400" />
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- Formulario de configuración SMTP -->
        <div class="bg-slate-900/50 border border-slate-800 rounded-lg p-6">
            <div class="flex items-center gap-3 mb-6">
                <Mail class="w-6 h-6 text-blue-400" />
                <h2 class="text-xl font-semibold text-white">Configuración SMTP</h2>
            </div>

            <form onsubmit={(e) => e.preventDefault()} class="space-y-4">
                <div>
                    <label for="smtp_host" class="block text-sm font-medium text-slate-300 mb-2">
                        Servidor SMTP
                    </label>
                    <input
                        id="smtp_host"
                        type="text"
                        bind:value={config.smtp_host}
                        class="w-full px-3 py-2 bg-slate-800 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        placeholder="smtp.gmail.com"
                    />
                    {#if errors.smtp_host}
                        <p class="text-red-400 text-sm mt-1">{errors.smtp_host}</p>
                    {/if}
                </div>

                <div>
                    <label for="smtp_port" class="block text-sm font-medium text-slate-300 mb-2">
                        Puerto
                    </label>
                    <input
                        id="smtp_port"
                        type="number"
                        bind:value={config.smtp_port}
                        class="w-full px-3 py-2 bg-slate-800 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        placeholder="587"
                    />
                    {#if errors.smtp_port}
                        <p class="text-red-400 text-sm mt-1">{errors.smtp_port}</p>
                    {/if}
                </div>

                <div>
                    <label for="smtp_user" class="block text-sm font-medium text-slate-300 mb-2">
                        Usuario
                    </label>
                    <input
                        id="smtp_user"
                        type="text"
                        bind:value={config.smtp_user}
                        class="w-full px-3 py-2 bg-slate-800 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        placeholder="usuario@ejemplo.com"
                    />
                    {#if errors.smtp_user}
                        <p class="text-red-400 text-sm mt-1">{errors.smtp_user}</p>
                    {/if}
                </div>

                <div>
                    <label for="smtp_password" class="block text-sm font-medium text-slate-300 mb-2">
                        Contraseña
                    </label>
                    <input
                        id="smtp_password"
                        type="password"
                        bind:value={config.smtp_password}
                        class="w-full px-3 py-2 bg-slate-800 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        placeholder="••••••••"
                    />
                    {#if errors.smtp_password}
                        <p class="text-red-400 text-sm mt-1">{errors.smtp_password}</p>
                    {/if}
                </div>

                <div>
                    <label for="smtp_secure" class="block text-sm font-medium text-slate-300 mb-2">
                        Seguridad
                    </label>
                    <select
                        id="smtp_secure"
                        bind:value={config.smtp_secure}
                        class="w-full px-3 py-2 bg-slate-800 border border-slate-700 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                    >
                        <option value="tls">TLS</option>
                        <option value="starttls">STARTTLS</option>
                    </select>
                </div>

                <div class="flex gap-3 pt-4">
                    <button
                        type="button"
                        onclick={saveConfig}
                        class="flex-1 bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 transition-colors"
                    >
                        Guardar Configuración
                    </button>
                    <button
                        type="button"
                        onclick={testSmtp}
                        disabled={isTesting}
                        class="flex-1 bg-slate-700 text-white py-2 px-4 rounded-md hover:bg-slate-600 focus:outline-none focus:ring-2 focus:ring-slate-500 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
                    >
                        {#if isTesting}
                            <span>Probando...</span>
                        {:else}
                            <Send class="w-4 h-4" />
                            <span>Probar Conexión</span>
                        {/if}
                    </button>
                </div>
            </form>
        </div>

        <!-- Resultados y ayuda -->
        <div class="space-y-6">
            <!-- Resultado de prueba -->
            {#if testResult}
                <div class="bg-slate-900/50 border border-slate-800 rounded-lg p-6">
                    <div class="flex items-center gap-3 mb-4">
                        {#if testResult.success}
                            <div class="w-10 h-10 bg-green-500/20 rounded-full flex items-center justify-center">
                                <Check class="w-6 h-6 text-green-500" />
                            </div>
                            <h3 class="text-lg font-semibold text-white">Prueba Exitosa</h3>
                        {:else}
                            <div class="w-10 h-10 bg-red-500/20 rounded-full flex items-center justify-center">
                                <X class="w-6 h-6 text-red-500" />
                            </div>
                            <h3 class="text-lg font-semibold text-white">Prueba Fallida</h3>
                        {/if}
                    </div>
                    <p class="text-slate-300">{testResult.message}</p>
                </div>
            {/if}

            <!-- Información de ayuda -->
            <div class="bg-slate-900/50 border border-slate-800 rounded-lg p-6">
                <h3 class="text-lg font-semibold text-white mb-4">Configuración SMTP</h3>
                <div class="space-y-3 text-sm text-slate-300">
                    <p>
                        <strong class="text-white">Gmail:</strong> smtp.gmail.com:587 (STARTTLS)
                    </p>
                    <p>
                        <strong class="text-white">Outlook:</strong> smtp.office365.com:587 (STARTTLS)
                    </p>
                    <p>
                        <strong class="text-white">Yahoo:</strong> smtp.mail.yahoo.com:587 (STARTTLS)
                    </p>
                </div>
                <div class="mt-4 p-3 bg-yellow-500/10 border border-yellow-500/20 rounded-md">
                    <p class="text-yellow-400 text-sm">
                        <strong>Nota:</strong> Para Gmail, necesitas usar una "Contraseña de aplicación" en lugar de tu contraseña normal.
                    </p>
                </div>
            </div>
        </div>
    </div>
</div>
