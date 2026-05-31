<!-- apps/web/src/routes/dashboard/alertas/+page.svelte -->
<!-- Dashboard de Alertas - Módulo 3 -->
<!-- Gestión de alertas críticas de la red -->

<script lang="ts">
    import { createQuery } from '@tanstack/svelte-query';
    import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
    import { Button } from '$lib/components/ui/button';
    import { Badge } from '$lib/components/ui/badge';
    import { AlertTriangle, CheckCircle, Clock, Filter } from 'lucide-svelte';

    let statusFilter = $state<string | null>(null);
    let severityFilter = $state<string | null>(null);

    interface Alert {
        id: string;
        title: string;
        description: string;
        severity: string;
        status: string;
        device_id: string | null;
        sede_id: string | null;
        created_at: string;
        resolved_at: string | null;
    }

    // Query para obtener alertas (simulado por ahora)
    const alertsQuery = createQuery(() => ({
        queryKey: ['alerts', statusFilter, severityFilter],
        queryFn: async () => {
            // Simulación de datos de alertas
            return [
                {
                    id: 'alert-1',
                    title: 'Alta latencia en Router Trinidad',
                    description: 'Latencia superior a 200ms detectada',
                    severity: 'high',
                    status: 'active',
                    device_id: 'device-1',
                    sede_id: 'trinidad',
                    created_at: new Date().toISOString(),
                    resolved_at: null
                },
                {
                    id: 'alert-2',
                    title: 'Pérdida de paquetes en Beni',
                    description: 'Pérdida de paquetes del 5% detectada',
                    severity: 'medium',
                    status: 'active',
                    device_id: 'device-2',
                    sede_id: 'beni',
                    created_at: new Date(Date.now() - 3600000).toISOString(),
                    resolved_at: null
                },
                {
                    id: 'alert-3',
                    title: 'CPU crítica en Server Riberalta',
                    description: 'Uso de CPU superior al 90%',
                    severity: 'critical',
                    status: 'resolved',
                    device_id: 'device-3',
                    sede_id: 'riberalta',
                    created_at: new Date(Date.now() - 7200000).toISOString(),
                    resolved_at: new Date(Date.now() - 3600000).toISOString()
                }
            ] as Alert[];
        },
        refetchInterval: 30000 // Refrescar cada 30 segundos
    }));

    function getSeverityColor(severity: string): string {
        switch (severity.toLowerCase()) {
            case 'critical':
                return 'bg-red-500/10 text-red-500 border-red-500/20';
            case 'high':
                return 'bg-orange-500/10 text-orange-500 border-orange-500/20';
            case 'medium':
                return 'bg-yellow-500/10 text-yellow-500 border-yellow-500/20';
            case 'low':
                return 'bg-blue-500/10 text-blue-500 border-blue-500/20';
            default:
                return 'bg-gray-500/10 text-gray-500 border-gray-500/20';
        }
    }

    function getStatusBadge(status: string): { class: string, label: string } {
        switch (status.toLowerCase()) {
            case 'active':
                return { class: 'bg-red-500', label: 'Activa' };
            case 'resolved':
                return { class: 'bg-green-500', label: 'Resuelta' };
            case 'acknowledged':
                return { class: 'bg-yellow-500', label: 'Reconocida' };
            default:
                return { class: 'bg-gray-500', label: status };
        }
    }

    function formatDate(timestamp: string): string {
        const date = new Date(timestamp);
        return date.toLocaleString('es-BO');
    }

    async function handleResolve(id: string) {
        console.log('Resolving alert:', id);
        // Implementar lógica de resolución
    }

    async function handleAcknowledge(id: string) {
        console.log('Acknowledging alert:', id);
        // Implementar lógica de reconocimiento
    }
</script>

<div class="p-6 space-y-6">
    <div>
        <h1 class="text-3xl font-bold text-zinc-100">Alertas</h1>
        <p class="text-zinc-400 mt-1">Gestión de alertas críticas de la red</p>
    </div>

    <!-- Filtros -->
    <div class="flex gap-4">
        <select
            bind:value={statusFilter}
            class="flex h-10 rounded-md border border-zinc-700 bg-zinc-900 px-3 py-2 text-sm text-zinc-300"
        >
            <option value={null}>Todos los estados</option>
            <option value="active">Activa</option>
            <option value="resolved">Resuelta</option>
            <option value="acknowledged">Reconocida</option>
        </select>

        <select
            bind:value={severityFilter}
            class="flex h-10 rounded-md border border-zinc-700 bg-zinc-900 px-3 py-2 text-sm text-zinc-300"
        >
            <option value={null}>Todas las severidades</option>
            <option value="critical">Crítica</option>
            <option value="high">Alta</option>
            <option value="medium">Media</option>
            <option value="low">Baja</option>
        </select>
    </div>

    <!-- Estadísticas -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card class="bg-zinc-900 border-zinc-800">
            <CardHeader class="pb-2">
                <CardTitle class="text-sm font-medium text-zinc-400">Total Alertas</CardTitle>
            </CardHeader>
            <CardContent>
                <div class="text-2xl font-bold text-zinc-100">{alertsQuery.data?.length || 0}</div>
            </CardContent>
        </Card>

        <Card class="bg-zinc-900 border-zinc-800">
            <CardHeader class="pb-2">
                <CardTitle class="text-sm font-medium text-zinc-400">Activas</CardTitle>
            </CardHeader>
            <CardContent>
                <div class="text-2xl font-bold text-red-400">
                    {alertsQuery.data?.filter(a => a.status === 'active').length || 0}
                </div>
            </CardContent>
        </Card>

        <Card class="bg-zinc-900 border-zinc-800">
            <CardHeader class="pb-2">
                <CardTitle class="text-sm font-medium text-zinc-400">Críticas</CardTitle>
            </CardHeader>
            <CardContent>
                <div class="text-2xl font-bold text-red-500">
                    {alertsQuery.data?.filter(a => a.severity === 'critical').length || 0}
                </div>
            </CardContent>
        </Card>

        <Card class="bg-zinc-900 border-zinc-800">
            <CardHeader class="pb-2">
                <CardTitle class="text-sm font-medium text-zinc-400">Resueltas</CardTitle>
            </CardHeader>
            <CardContent>
                <div class="text-2xl font-bold text-green-400">
                    {alertsQuery.data?.filter(a => a.status === 'resolved').length || 0}
                </div>
            </CardContent>
        </Card>
    </div>

    <!-- Lista de alertas -->
    <div class="space-y-4">
        {#if alertsQuery.isLoading}
            <p class="text-zinc-400 text-center py-8">Cargando alertas...</p>
        {:else if alertsQuery.isError}
            <p class="text-red-400 text-center py-8">Error al cargar alertas</p>
        {:else if alertsQuery.data && alertsQuery.data.length > 0}
            {#each alertsQuery.data as alert (alert.id)}
                {@const status = getStatusBadge(alert.status)}
                <Card class="bg-zinc-900 border-zinc-800 hover:border-zinc-700 transition-colors">
                    <CardContent class="p-4">
                        <div class="flex items-start justify-between">
                            <div class="flex items-start gap-4">
                                <div class="flex h-10 w-10 items-center justify-center rounded-full {getSeverityColor(alert.severity)}">
                                    <AlertTriangle class="h-5 w-5" />
                                </div>

                                <div class="flex-1 space-y-2">
                                    <div class="flex items-center gap-2">
                                        <h3 class="font-medium text-zinc-100">{alert.title}</h3>
                                        <Badge class={status.class}>{status.label}</Badge>
                                        <Badge class={getSeverityColor(alert.severity)}>{alert.severity.toUpperCase()}</Badge>
                                    </div>

                                    <p class="text-sm text-zinc-400">{alert.description}</p>

                                    <div class="flex items-center gap-4 text-sm text-zinc-500">
                                        <div class="flex items-center gap-1">
                                            <Clock class="h-4 w-4" />
                                            <span>{formatDate(alert.created_at)}</span>
                                        </div>
                                        {#if alert.device_id}
                                            <span>Dispositivo: {alert.device_id}</span>
                                        {/if}
                                        {#if alert.sede_id}
                                            <span>Sede: {alert.sede_id}</span>
                                        {/if}
                                    </div>
                                </div>
                            </div>

                            <div class="flex gap-2">
                                {#if alert.status === 'active'}
                                    <Button
                                        variant="outline"
                                        size="sm"
                                        onclick={() => handleAcknowledge(alert.id)}
                                        class="border-zinc-700 text-zinc-300 hover:bg-zinc-800"
                                    >
                                        <CheckCircle class="mr-1 h-4 w-4" />
                                        Reconocer
                                    </Button>
                                    <Button
                                        variant="outline"
                                        size="sm"
                                        onclick={() => handleResolve(alert.id)}
                                        class="border-zinc-700 text-zinc-300 hover:bg-zinc-800"
                                    >
                                        Resolver
                                    </Button>
                                {/if}
                            </div>
                        </div>
                    </CardContent>
                </Card>
            {/each}
        {:else}
            <p class="text-zinc-500 text-center py-8">No hay alertas</p>
        {/if}
    </div>
</div>
