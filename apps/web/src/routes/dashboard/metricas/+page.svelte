<!-- apps/web/src/routes/dashboard/metricas/+page.svelte -->
<!-- Dashboard de Métricas - Módulo 3 -->
<!-- Indicadores de rendimiento de la red -->

<script lang="ts">
    import { createQuery } from '@tanstack/svelte-query';
    import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
    import { Button } from '$lib/components/ui/button';
    import { Badge } from '$lib/components/ui/badge';
    import { BarChart3, Activity, Cpu, HardDrive, Signal, Download, TrendingUp, TrendingDown } from 'lucide-svelte';

    let selectedTimeRange = $state('1h'); // 1h, 6h, 24h
    let selectedSede = $state<string | null>(null);

    interface MetricData {
        timestamp: string;
        value: number;
    }

    interface SedeMetrics {
        sede_id: string;
        sede_name: string;
        bandwidth_mbps: number;
        latency_ms: number;
        packet_loss_percent: number;
        cpu_usage_percent: number;
        memory_usage_percent: number;
    }

    // Query para obtener métricas (simulado por ahora)
    const metricsQuery = createQuery(() => ({
        queryKey: ['metrics', selectedTimeRange, selectedSede],
        queryFn: async () => {
            // Simulación de datos de métricas
            const now = new Date();
            const metrics: MetricData[] = [];
            const points = 60; // 60 puntos de datos

            for (let i = 0; i < points; i++) {
                const timestamp = new Date(now.getTime() - (points - i) * 60000); // Cada minuto
                metrics.push({
                    timestamp: timestamp.toISOString(),
                    value: 800 + Math.random() * 200
                });
            }

            return metrics;
        },
        refetchInterval: 10000 // Refrescar cada 10 segundos
    }));

    // Query para obtener métricas por sede (simulado por ahora)
    const sedesMetricsQuery = createQuery(() => ({
        queryKey: ['sedes-metrics'],
        queryFn: async () => {
            return [
                {
                    sede_id: 'trinidad',
                    sede_name: 'Trinidad',
                    bandwidth_mbps: 950.5,
                    latency_ms: 12,
                    packet_loss_percent: 0.1,
                    cpu_usage_percent: 45.2,
                    memory_usage_percent: 62.8
                },
                {
                    sede_id: 'beni',
                    sede_name: 'Beni',
                    bandwidth_mbps: 820.3,
                    latency_ms: 18,
                    packet_loss_percent: 0.3,
                    cpu_usage_percent: 52.1,
                    memory_usage_percent: 58.4
                },
                {
                    sede_id: 'riberalta',
                    sede_name: 'Riberalta',
                    bandwidth_mbps: 780.7,
                    latency_ms: 25,
                    packet_loss_percent: 0.5,
                    cpu_usage_percent: 38.9,
                    memory_usage_percent: 55.2
                }
            ] as SedeMetrics[];
        },
        refetchInterval: 30000 // Refrescar cada 30 segundos
    }));

    function formatTime(timestamp: string): string {
        const date = new Date(timestamp);
        return date.toLocaleTimeString('es-BO', { hour: '2-digit', minute: '2-digit' });
    }

    function formatValue(value: number): string {
        if (value >= 1000) {
            return `${(value / 1000).toFixed(1)} Gbps`;
        }
        return `${value.toFixed(1)} Mbps`;
    }

    function getTrendIcon(current: number, previous: number) {
        if (current > previous) {
            return TrendingUp;
        } else {
            return TrendingDown;
        }
    }

    function getTrendColor(current: number, previous: number): string {
        if (current > previous) {
            return 'text-green-400';
        } else {
            return 'text-red-400';
        }
    }

    // Generar SVG del gráfico
    function generateChart(data: MetricData[], color: string): string {
        if (data.length === 0) return '';

        const width = 800;
        const height = 200;
        const padding = 20;
        const max = Math.max(...data.map(d => d.value), 1);
        const min = Math.min(...data.map(d => d.value), 0);
        const range = max - min || 1;

        let path = '';
        let area = '';

        data.forEach((d, i) => {
            const x = padding + (i / (data.length - 1)) * (width - 2 * padding);
            const y = height - padding - ((d.value - min) / range) * (height - 2 * padding);

            if (i === 0) {
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
        <div>
            <h1 class="text-3xl font-bold text-zinc-100">Métricas</h1>
            <p class="text-zinc-400 mt-1">Indicadores de rendimiento de la red</p>
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

    <!-- Estadísticas generales -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card class="bg-zinc-900 border-zinc-800">
            <CardHeader class="pb-2">
                <CardTitle class="text-sm font-medium text-zinc-400">Ancho de Banda</CardTitle>
            </CardHeader>
            <CardContent>
                <div class="flex items-center justify-between">
                    <div class="text-2xl font-bold text-zinc-100">950.5 Mbps</div>
                    <Download class="h-5 w-5 text-blue-400" />
                </div>
                <div class="text-sm text-green-400 mt-1">+2.5% vs hora anterior</div>
            </CardContent>
        </Card>

        <Card class="bg-zinc-900 border-zinc-800">
            <CardHeader class="pb-2">
                <CardTitle class="text-sm font-medium text-zinc-400">Latencia</CardTitle>
            </CardHeader>
            <CardContent>
                <div class="flex items-center justify-between">
                    <div class="text-2xl font-bold text-zinc-100">12 ms</div>
                    <Signal class="h-5 w-5 text-green-400" />
                </div>
                <div class="text-sm text-green-400 mt-1">-1.2ms vs hora anterior</div>
            </CardContent>
        </Card>

        <Card class="bg-zinc-900 border-zinc-800">
            <CardHeader class="pb-2">
                <CardTitle class="text-sm font-medium text-zinc-400">Pérdida de Paquetes</CardTitle>
            </CardHeader>
            <CardContent>
                <div class="flex items-center justify-between">
                    <div class="text-2xl font-bold text-zinc-100">0.1%</div>
                    <Activity class="h-5 w-5 text-yellow-400" />
                </div>
                <div class="text-sm text-red-400 mt-1">+0.05% vs hora anterior</div>
            </CardContent>
        </Card>

        <Card class="bg-zinc-900 border-zinc-800">
            <CardHeader class="pb-2">
                <CardTitle class="text-sm font-medium text-zinc-400">CPU Promedio</CardTitle>
            </CardHeader>
            <CardContent>
                <div class="flex items-center justify-between">
                    <div class="text-2xl font-bold text-zinc-100">45.2%</div>
                    <Cpu class="h-5 w-5 text-purple-400" />
                </div>
                <div class="text-sm text-green-400 mt-1">-3.1% vs hora anterior</div>
            </CardContent>
        </Card>
    </div>

    <!-- Gráfico de ancho de banda -->
    <Card class="bg-zinc-900 border-zinc-800">
        <CardHeader>
            <div class="flex items-center gap-3">
                <Download class="h-5 w-5 text-zinc-400" />
                <div>
                    <CardTitle class="text-zinc-100">Ancho de Banda</CardTitle>
                    <CardDescription class="text-zinc-500">Últimos {selectedTimeRange}</CardDescription>
                </div>
            </div>
        </CardHeader>
        <CardContent>
            {#if metricsQuery.isLoading}
                <p class="text-zinc-400 text-center py-8">Cargando métricas...</p>
            {:else if metricsQuery.isError}
                <p class="text-red-400 text-center py-8">Error al cargar métricas</p>
            {:else if metricsQuery.data && metricsQuery.data.length > 0}
                <div class="h-64">
                    {@html generateChart(metricsQuery.data, '#3b82f6')}
                </div>
                <div class="flex justify-between mt-4 text-sm text-zinc-500">
                    <span>{formatTime(metricsQuery.data[0].timestamp)}</span>
                    <span>{formatTime(metricsQuery.data[metricsQuery.data.length - 1].timestamp)}</span>
                </div>
            {:else}
                <p class="text-zinc-500 text-center py-8">No hay datos disponibles</p>
            {/if}
        </CardContent>
    </Card>

    <!-- Métricas por sede -->
    <div>
        <h2 class="text-xl font-bold text-zinc-100 mb-4">Métricas por Sede</h2>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            {#if sedesMetricsQuery.isLoading}
                <p class="text-zinc-400 col-span-3 text-center py-8">Cargando métricas por sede...</p>
            {:else if sedesMetricsQuery.isError}
                <p class="text-red-400 col-span-3 text-center py-8">Error al cargar métricas por sede</p>
            {:else if sedesMetricsQuery.data && sedesMetricsQuery.data.length > 0}
                {#each sedesMetricsQuery.data as sede (sede.sede_id)}
                    <Card class="bg-zinc-900 border-zinc-800 hover:border-zinc-700 transition-colors">
                        <CardHeader>
                            <CardTitle class="text-zinc-100">{sede.sede_name}</CardTitle>
                        </CardHeader>
                        <CardContent class="space-y-3">
                            <div class="flex items-center justify-between text-sm">
                                <div class="flex items-center gap-2 text-zinc-400">
                                    <Download class="h-4 w-4" />
                                    <span>Ancho de Banda</span>
                                </div>
                                <span class="text-zinc-300">{sede.bandwidth_mbps.toFixed(1)} Mbps</span>
                            </div>
                            <div class="flex items-center justify-between text-sm">
                                <div class="flex items-center gap-2 text-zinc-400">
                                    <Signal class="h-4 w-4" />
                                    <span>Latencia</span>
                                </div>
                                <span class="text-zinc-300">{sede.latency_ms} ms</span>
                            </div>
                            <div class="flex items-center justify-between text-sm">
                                <div class="flex items-center gap-2 text-zinc-400">
                                    <Activity class="h-4 w-4" />
                                    <span>Pérdida de Paquetes</span>
                                </div>
                                <span class="text-zinc-300">{sede.packet_loss_percent}%</span>
                            </div>
                            <div class="flex items-center justify-between text-sm">
                                <div class="flex items-center gap-2 text-zinc-400">
                                    <Cpu class="h-4 w-4" />
                                    <span>CPU</span>
                                </div>
                                <span class="text-zinc-300">{sede.cpu_usage_percent}%</span>
                            </div>
                            <div class="flex items-center justify-between text-sm">
                                <div class="flex items-center gap-2 text-zinc-400">
                                    <HardDrive class="h-4 w-4" />
                                    <span>Memoria</span>
                                </div>
                                <span class="text-zinc-300">{sede.memory_usage_percent}%</span>
                            </div>
                        </CardContent>
                    </Card>
                {/each}
            {:else}
                <p class="text-zinc-500 col-span-3 text-center py-8">No hay datos por sede</p>
            {/if}
        </div>
    </div>
</div>
