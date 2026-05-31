<!-- apps/web/src/routes/dashboard/agents/+page.svelte -->
<!-- Dashboard de Conectividad de Agentes - Módulo 7 -->
<!-- Telemetría de Alta Velocidad y Conectividad Provincial -->

<script lang="ts">
    import { createQuery } from '@tanstack/svelte-query';
    import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
    import { Button } from '$lib/components/ui/button';
    import { Badge } from '$lib/components/ui/badge';
    import { Activity, Server, Cpu, HardDrive, Thermometer, Clock, Signal, Plus } from 'lucide-svelte';

    // Estado de agentes
    let showRegisterDialog = $state(false);

    interface Agent {
        id: string;
        name: string;
        sede_id: string;
        agent_type: string;
        ip_address: string;
        last_seen: string | null;
        status: string;
    }

    interface AgentMetrics {
        id: string;
        agent_id: string;
        cpu_usage_percent: number | null;
        memory_usage_percent: number | null;
        latency_ms: number | null;
        packet_loss_percent: number | null;
        bandwidth_mbps: number | null;
        disk_usage_percent: number | null;
        temperature_celsius: number | null;
        uptime_seconds: number | null;
        created_at: string;
    }

    // Query para obtener agentes (simulado por ahora)
    const agentsQuery = createQuery(() => ({
        queryKey: ['agents'],
        queryFn: async () => {
            // Simulación de datos de agentes
            return [
                {
                    id: 'agent-1',
                    name: 'Router Trinidad',
                    sede_id: 'trinidad',
                    agent_type: 'ROUTER',
                    ip_address: '192.168.1.1',
                    last_seen: new Date().toISOString(),
                    status: 'ACTIVE'
                },
                {
                    id: 'agent-2',
                    name: 'Switch Beni',
                    sede_id: 'beni',
                    agent_type: 'SWITCH',
                    ip_address: '192.168.1.2',
                    last_seen: new Date(Date.now() - 60000).toISOString(),
                    status: 'ACTIVE'
                },
                {
                    id: 'agent-3',
                    name: 'Server Riberalta',
                    sede_id: 'riberalta',
                    agent_type: 'SERVER',
                    ip_address: '192.168.1.3',
                    last_seen: new Date(Date.now() - 300000).toISOString(),
                    status: 'ACTIVE'
                }
            ] as Agent[];
        },
        refetchInterval: 5000 // Refrescar cada 5 segundos
    }));

    // Query para obtener métricas recientes (simulado por ahora)
    const metricsQuery = createQuery(() => ({
        queryKey: ['agent-metrics'],
        queryFn: async () => {
            // Simulación de métricas
            return [
                {
                    id: 'metrics-1',
                    agent_id: 'agent-1',
                    cpu_usage_percent: 45.2,
                    memory_usage_percent: 62.8,
                    latency_ms: 12,
                    packet_loss_percent: 0.1,
                    bandwidth_mbps: 950.5,
                    disk_usage_percent: 78.3,
                    temperature_celsius: 42.5,
                    uptime_seconds: 86400,
                    created_at: new Date().toISOString()
                }
            ] as AgentMetrics[];
        },
        refetchInterval: 5000 // Refrescar cada 5 segundos
    }));

    // Función para obtener métricas de un agente
    function getAgentMetrics(agentId: string): AgentMetrics | undefined {
        return metricsQuery.data?.find(m => m.agent_id === agentId);
    }

    // Función para obtener el estado de un agente
    function getStatusBadge(status: string): { class: string, label: string } {
        switch (status) {
            case 'ACTIVE':
                return { class: 'bg-green-500', label: 'Online' };
            case 'INACTIVE':
                return { class: 'bg-gray-500', label: 'Offline' };
            case 'ERROR':
                return { class: 'bg-red-500', label: 'Error' };
            default:
                return { class: 'bg-yellow-500', label: 'Unknown' };
        }
    }

    // Función para formatear fecha
    function formatDate(timestamp: string | null): string {
        if (!timestamp) return 'Nunca';
        const date = new Date(timestamp);
        const now = new Date();
        const diff = now.getTime() - date.getTime();
        const minutes = Math.floor(diff / 60000);

        if (minutes < 1) return 'Ahora mismo';
        if (minutes < 60) return `Hace ${minutes} min`;
        const hours = Math.floor(minutes / 60);
        if (hours < 24) return `Hace ${hours} h`;
        return date.toLocaleDateString('es-BO');
    }

    // Función para formatear uptime
    function formatUptime(seconds: number | null): string {
        if (!seconds) return 'N/A';
        const days = Math.floor(seconds / 86400);
        const hours = Math.floor((seconds % 86400) / 3600);
        const minutes = Math.floor((seconds % 3600) / 60);

        if (days > 0) return `${days}d ${hours}h`;
        if (hours > 0) return `${hours}h ${minutes}m`;
        return `${minutes}m`;
    }
</script>

<div class="p-6 space-y-6">
    <div class="flex items-center justify-between">
        <div>
            <h1 class="text-3xl font-bold text-zinc-100">Agentes</h1>
            <p class="text-zinc-400 mt-1">Monitoreo de conectividad provincial</p>
        </div>
        <Button class="bg-blue-600">
            <Plus class="h-4 w-4 mr-2" />
            Registrar Agente
        </Button>
    </div>

    <!-- Estadísticas generales -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card class="bg-zinc-900 border-zinc-800">
            <CardHeader class="pb-2">
                <CardTitle class="text-sm font-medium text-zinc-400">Total Agentes</CardTitle>
            </CardHeader>
            <CardContent>
                <div class="text-2xl font-bold text-zinc-100">{agentsQuery.data?.length || 0}</div>
            </CardContent>
        </Card>

        <Card class="bg-zinc-900 border-zinc-800">
            <CardHeader class="pb-2">
                <CardTitle class="text-sm font-medium text-zinc-400">Online</CardTitle>
            </CardHeader>
            <CardContent>
                <div class="text-2xl font-bold text-green-400">
                    {agentsQuery.data?.filter(a => a.status === 'ACTIVE').length || 0}
                </div>
            </CardContent>
        </Card>

        <Card class="bg-zinc-900 border-zinc-800">
            <CardHeader class="pb-2">
                <CardTitle class="text-sm font-medium text-zinc-400">Offline</CardTitle>
            </CardHeader>
            <CardContent>
                <div class="text-2xl font-bold text-red-400">
                    {agentsQuery.data?.filter(a => a.status !== 'ACTIVE').length || 0}
                </div>
            </CardContent>
        </Card>

        <Card class="bg-zinc-900 border-zinc-800">
            <CardHeader class="pb-2">
                <CardTitle class="text-sm font-medium text-zinc-400">Última Actualización</CardTitle>
            </CardHeader>
            <CardContent>
                <div class="text-2xl font-bold text-zinc-100">
                    {formatDate(new Date().toISOString())}
                </div>
            </CardContent>
        </Card>
    </div>

    <!-- Lista de agentes -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#if agentsQuery.isLoading}
            <p class="text-zinc-400 col-span-3 text-center py-8">Cargando agentes...</p>
        {:else if agentsQuery.isError}
            <p class="text-red-400 col-span-3 text-center py-8">Error al cargar agentes</p>
        {:else if agentsQuery.data && agentsQuery.data.length > 0}
            {#each agentsQuery.data as agent (agent.id)}
                {@const status = getStatusBadge(agent.status)}
                {@const metrics = getAgentMetrics(agent.id)}
                <Card class="bg-zinc-900 border-zinc-800 hover:border-zinc-700 transition-colors">
                    <CardHeader>
                        <div class="flex items-start justify-between">
                            <div class="flex items-center gap-3">
                                <Server class="h-5 w-5 text-zinc-400" />
                                <div>
                                    <CardTitle class="text-zinc-100">{agent.name}</CardTitle>
                                    <CardDescription class="text-zinc-500">{agent.ip_address}</CardDescription>
                                </div>
                            </div>
                            <Badge class={status.class}>{status.label}</Badge>
                        </div>
                    </CardHeader>
                    <CardContent class="space-y-3">
                        <div class="flex items-center gap-2 text-sm text-zinc-400">
                            <Clock class="h-4 w-4" />
                            <span>Última conexión: {formatDate(agent.last_seen)}</span>
                        </div>

                        {#if metrics}
                            <div class="space-y-2 pt-2 border-t border-zinc-800">
                                <div class="flex items-center justify-between text-sm">
                                    <div class="flex items-center gap-2 text-zinc-400">
                                        <Cpu class="h-4 w-4" />
                                        <span>CPU</span>
                                    </div>
                                    <span class="text-zinc-300">{metrics.cpu_usage_percent}%</span>
                                </div>
                                <div class="flex items-center justify-between text-sm">
                                    <div class="flex items-center gap-2 text-zinc-400">
                                        <HardDrive class="h-4 w-4" />
                                        <span>Memoria</span>
                                    </div>
                                    <span class="text-zinc-300">{metrics.memory_usage_percent}%</span>
                                </div>
                                <div class="flex items-center justify-between text-sm">
                                    <div class="flex items-center gap-2 text-zinc-400">
                                        <Signal class="h-4 w-4" />
                                        <span>Latencia</span>
                                    </div>
                                    <span class="text-zinc-300">{metrics.latency_ms}ms</span>
                                </div>
                                <div class="flex items-center justify-between text-sm">
                                    <div class="flex items-center gap-2 text-zinc-400">
                                        <Activity class="h-4 w-4" />
                                        <span>Uptime</span>
                                    </div>
                                    <span class="text-zinc-300">{formatUptime(metrics.uptime_seconds)}</span>
                                </div>
                                {#if metrics.temperature_celsius}
                                    <div class="flex items-center justify-between text-sm">
                                        <div class="flex items-center gap-2 text-zinc-400">
                                            <Thermometer class="h-4 w-4" />
                                            <span>Temperatura</span>
                                        </div>
                                        <span class="text-zinc-300">{metrics.temperature_celsius}°C</span>
                                    </div>
                                {/if}
                            </div>
                        {:else}
                            <div class="pt-2 border-t border-zinc-800 text-sm text-zinc-500">
                                Sin métricas disponibles
                            </div>
                        {/if}
                    </CardContent>
                </Card>
            {/each}
        {:else}
            <p class="text-zinc-500 col-span-3 text-center py-8">No hay agentes registrados</p>
        {/if}
    </div>
</div>
