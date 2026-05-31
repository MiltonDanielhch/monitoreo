<!-- apps/web/src/routes/dashboard/agents/[id]/+page.svelte -->
<!-- Gráficos de Rendimiento SVG en Tiempo Real - Módulo 7 -->
<!-- Telemetría de Alta Velocidad y Conectividad Provincial -->

<script lang="ts">
    import { page } from '$app/stores';
    import { createQuery } from '@tanstack/svelte-query';
    import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
    import { Button } from '$lib/components/ui/button';
    import { Badge } from '$lib/components/ui/badge';
    import { ArrowLeft, Cpu, HardDrive, Signal, Activity, Thermometer, Download } from 'lucide-svelte';

    let selectedTimeRange = $state('1h'); // 1h, 6h, 24h
    let selectedMetric = $state('cpu'); // cpu, memory, latency, bandwidth

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

    // Query para obtener métricas del agente (simulado por ahora)
    const metricsQuery = createQuery(() => ({
        queryKey: ['agent-metrics', $page.params.id, selectedTimeRange],
        queryFn: async () => {
            const agentId = $page.params.id || 'unknown';
            // Simulación de datos históricos
            const now = new Date();
            const metrics: AgentMetrics[] = [];
            const points = 60; // 60 puntos de datos

            for (let i = 0; i < points; i++) {
                const timestamp = new Date(now.getTime() - (points - i) * 60000); // Cada minuto
                metrics.push({
                    id: `metrics-${i}`,
                    agent_id: agentId,
                    cpu_usage_percent: 30 + Math.random() * 40,
                    memory_usage_percent: 40 + Math.random() * 30,
                    latency_ms: 10 + Math.random() * 20,
                    packet_loss_percent: Math.random() * 1,
                    bandwidth_mbps: 800 + Math.random() * 200,
                    disk_usage_percent: 70 + Math.random() * 10,
                    temperature_celsius: 40 + Math.random() * 10,
                    uptime_seconds: 86400 + i * 60,
                    created_at: timestamp.toISOString()
                });
            }

            return metrics;
        },
        refetchInterval: 5000 // Refrescar cada 5 segundos
    }));

    // Función para obtener datos de una métrica específica
    function getMetricData(metric: string): number[] {
        if (!metricsQuery.data) return [];
        
        switch (metric) {
            case 'cpu':
                return metricsQuery.data.map(m => m.cpu_usage_percent || 0);
            case 'memory':
                return metricsQuery.data.map(m => m.memory_usage_percent || 0);
            case 'latency':
                return metricsQuery.data.map(m => m.latency_ms || 0);
            case 'bandwidth':
                return metricsQuery.data.map(m => m.bandwidth_mbps || 0);
            case 'temperature':
                return metricsQuery.data.map(m => m.temperature_celsius || 0);
            default:
                return [];
        }
    }

    // Función para obtener el color según la métrica
    function getMetricColor(metric: string): string {
        switch (metric) {
            case 'cpu':
                return '#3b82f6'; // blue
            case 'memory':
                return '#8b5cf6'; // purple
            case 'latency':
                return '#ef4444'; // red
            case 'bandwidth':
                return '#10b981'; // green
            case 'temperature':
                return '#f59e0b'; // orange
            default:
                return '#6b7280'; // gray
        }
    }

    // Función para obtener el nombre de la métrica
    function getMetricName(metric: string): string {
        switch (metric) {
            case 'cpu':
                return 'CPU (%)';
            case 'memory':
                return 'Memoria (%)';
            case 'latency':
                return 'Latencia (ms)';
            case 'bandwidth':
                return 'Ancho de Banda (Mbps)';
            case 'temperature':
                return 'Temperatura (°C)';
            default:
                return metric;
        }
    }

    // Función para obtener el icono según la métrica
    function getMetricIcon(metric: string) {
        switch (metric) {
            case 'cpu':
                return Cpu;
            case 'memory':
                return HardDrive;
            case 'latency':
                return Signal;
            case 'bandwidth':
                return Download;
            case 'temperature':
                return Thermometer;
            default:
                return Activity;
        }
    }

    // Función para formatear fecha
    function formatDate(timestamp: string): string {
        const date = new Date(timestamp);
        return date.toLocaleTimeString('es-BO', {
            hour: '2-digit',
            minute: '2-digit'
        });
    }

    // Función para generar el SVG del gráfico
    function generateChart(data: number[], color: string): string {
        if (data.length === 0) return '';

        const width = 800;
        const height = 200;
        const padding = 20;
        const max = Math.max(...data);
        const min = Math.min(...data);
        const range = max - min || 1;

        let path = '';
        let area = '';

        data.forEach((value, index) => {
            const x = padding + (index / (data.length - 1)) * (width - 2 * padding);
            const y = height - padding - ((value - min) / range) * (height - 2 * padding);

            if (index === 0) {
                path += `M ${x} ${y}`;
                area += `M ${x} ${height - padding} L ${x} ${y}`;
            } else {
                path += ` L ${x} ${y}`;
                area += ` L ${x} ${y}`;
            }
        });

        area += ` L ${width - padding} ${height - padding} Z`;

        return `
            <svg width="${width}" height="${height}" viewBox="0 0 ${width} ${height}" class="w-full h-full">
                <defs>
                    <linearGradient id="gradient-${color}" x1="0%" y1="0%" x2="0%" y2="100%">
                        <stop offset="0%" style="stop-color:${color};stop-opacity:0.3" />
                        <stop offset="100%" style="stop-color:${color};stop-opacity:0" />
                    </linearGradient>
                </defs>
                <path d="${area}" fill="url(#gradient-${color})" />
                <path d="${path}" fill="none" stroke="${color}" stroke-width="2" />
            </svg>
        `;
    }
</script>

<div class="p-6 space-y-6">
    <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
            <Button variant="ghost" href="/dashboard/agents">
                <ArrowLeft class="h-4 w-4 mr-2" />
                Volver
            </Button>
            <div>
                <h1 class="text-3xl font-bold text-zinc-100">Gráficos de Rendimiento</h1>
                <p class="text-zinc-400 mt-1">Agente: {$page.params.id}</p>
            </div>
        </div>
        <div class="flex gap-2">
            <Button
                variant={selectedTimeRange === '1h' ? 'default' : 'outline'}
                onclick={() => selectedTimeRange = '1h'}
                class={selectedTimeRange === '1h' ? 'bg-blue-600' : 'border-zinc-700 text-zinc-300'}
            >
                1h
            </Button>
            <Button
                variant={selectedTimeRange === '6h' ? 'default' : 'outline'}
                onclick={() => selectedTimeRange = '6h'}
                class={selectedTimeRange === '6h' ? 'bg-blue-600' : 'border-zinc-700 text-zinc-300'}
            >
                6h
            </Button>
            <Button
                variant={selectedTimeRange === '24h' ? 'default' : 'outline'}
                onclick={() => selectedTimeRange = '24h'}
                class={selectedTimeRange === '24h' ? 'bg-blue-600' : 'border-zinc-700 text-zinc-300'}
            >
                24h
            </Button>
        </div>
    </div>

    <!-- Selector de métrica -->
    <div class="flex gap-2 flex-wrap">
        <Button
            variant={selectedMetric === 'cpu' ? 'default' : 'outline'}
            onclick={() => selectedMetric = 'cpu'}
            class={selectedMetric === 'cpu' ? 'bg-blue-600' : 'border-zinc-700 text-zinc-300'}
        >
            <Cpu class="h-4 w-4 mr-2" />
            CPU
        </Button>
        <Button
            variant={selectedMetric === 'memory' ? 'default' : 'outline'}
            onclick={() => selectedMetric = 'memory'}
            class={selectedMetric === 'memory' ? 'bg-blue-600' : 'border-zinc-700 text-zinc-300'}
        >
            <HardDrive class="h-4 w-4 mr-2" />
            Memoria
        </Button>
        <Button
            variant={selectedMetric === 'latency' ? 'default' : 'outline'}
            onclick={() => selectedMetric = 'latency'}
            class={selectedMetric === 'latency' ? 'bg-blue-600' : 'border-zinc-700 text-zinc-300'}
        >
            <Signal class="h-4 w-4 mr-2" />
            Latencia
        </Button>
        <Button
            variant={selectedMetric === 'bandwidth' ? 'default' : 'outline'}
            onclick={() => selectedMetric = 'bandwidth'}
            class={selectedMetric === 'bandwidth' ? 'bg-blue-600' : 'border-zinc-700 text-zinc-300'}
        >
            <Download class="h-4 w-4 mr-2" />
            Ancho de Banda
        </Button>
        <Button
            variant={selectedMetric === 'temperature' ? 'default' : 'outline'}
            onclick={() => selectedMetric = 'temperature'}
            class={selectedMetric === 'temperature' ? 'bg-blue-600' : 'border-zinc-700 text-zinc-300'}
        >
            <Thermometer class="h-4 w-4 mr-2" />
            Temperatura
        </Button>
    </div>

    <!-- Gráfico principal -->
    <Card class="bg-zinc-900 border-zinc-800">
        <CardHeader>
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                    <svelte:component this={getMetricIcon(selectedMetric)} class="h-5 w-5 text-zinc-400" />
                    <div>
                        <CardTitle class="text-zinc-100">{getMetricName(selectedMetric)}</CardTitle>
                        <CardDescription class="text-zinc-500">Últimos {selectedTimeRange}</CardDescription>
                    </div>
                </div>
                <Badge class={getMetricColor(selectedMetric)} style="background-color: {getMetricColor(selectedMetric)}">
                    {selectedMetric.toUpperCase()}
                </Badge>
            </div>
        </CardHeader>
        <CardContent>
            {#if metricsQuery.isLoading}
                <p class="text-zinc-400 text-center py-8">Cargando métricas...</p>
            {:else if metricsQuery.isError}
                <p class="text-red-400 text-center py-8">Error al cargar métricas</p>
            {:else if metricsQuery.data && metricsQuery.data.length > 0}
                {@const data = getMetricData(selectedMetric)}
                {@const color = getMetricColor(selectedMetric)}
                <div class="h-64">
                    {@html generateChart(data, color)}
                </div>
                <div class="flex justify-between mt-4 text-sm text-zinc-500">
                    <span>{formatDate(metricsQuery.data[0].created_at)}</span>
                    <span>{formatDate(metricsQuery.data[metricsQuery.data.length - 1].created_at)}</span>
                </div>
            {:else}
                <p class="text-zinc-500 text-center py-8">No hay datos disponibles</p>
            {/if}
        </CardContent>
    </Card>

    <!-- Estadísticas actuales -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        {#if metricsQuery.data && metricsQuery.data.length > 0}
            {@const latest = metricsQuery.data[metricsQuery.data.length - 1]}
            <Card class="bg-zinc-900 border-zinc-800">
                <CardHeader class="pb-2">
                    <CardTitle class="text-sm font-medium text-zinc-400">CPU</CardTitle>
                </CardHeader>
                <CardContent>
                    <div class="text-2xl font-bold text-zinc-100">{latest.cpu_usage_percent?.toFixed(1)}%</div>
                </CardContent>
            </Card>

            <Card class="bg-zinc-900 border-zinc-800">
                <CardHeader class="pb-2">
                    <CardTitle class="text-sm font-medium text-zinc-400">Memoria</CardTitle>
                </CardHeader>
                <CardContent>
                    <div class="text-2xl font-bold text-zinc-100">{latest.memory_usage_percent?.toFixed(1)}%</div>
                </CardContent>
            </Card>

            <Card class="bg-zinc-900 border-zinc-800">
                <CardHeader class="pb-2">
                    <CardTitle class="text-sm font-medium text-zinc-400">Latencia</CardTitle>
                </CardHeader>
                <CardContent>
                    <div class="text-2xl font-bold text-zinc-100">{latest.latency_ms}ms</div>
                </CardContent>
            </Card>

            <Card class="bg-zinc-900 border-zinc-800">
                <CardHeader class="pb-2">
                    <CardTitle class="text-sm font-medium text-zinc-400">Ancho de Banda</CardTitle>
                </CardHeader>
                <CardContent>
                    <div class="text-2xl font-bold text-zinc-100">{latest.bandwidth_mbps?.toFixed(1)}Mbps</div>
                </CardContent>
            </Card>
        {/if}
    </div>
</div>
