<!-- apps/web/src/routes/dashboard/dispositivos/+page.svelte -->
<!-- Página de Dispositivos del panel de monitoreo -->

<script lang="ts">
    import { onMount } from 'svelte';
    import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '$lib/components/ui/card';
    import { Badge } from '$lib/components/ui/badge';
    import { Skeleton } from '$lib/components/ui/skeleton';
    import { Monitor, Wifi, Cpu, Clock3 } from 'lucide-svelte';

    interface Device {
        id: string;
        name: string;
        device_type: string;
        location_id: string | null;
        ip_address: string | null;
        bandwidth_gbps: number;
        status: string;
        is_active: boolean;
        last_seen: string | null;
    }

    let devices = $state<Device[]>([]);
    let isLoading = $state(true);
    let error = $state<string | null>(null);

    function getStatusVariant(status: string) {
        return status === 'online' ? 'secondary' : status === 'maintenance' ? 'outline' : 'destructive';
    }

    function getStatusLabel(status: string) {
        return status === 'online' ? 'Online' : status === 'maintenance' ? 'Mantenimiento' : 'Offline';
    }

    async function fetchDevices() {
        try {
            const token = typeof sessionStorage !== 'undefined' ? sessionStorage.getItem('access_token') : null;
            const response = await fetch('/api/devices', {
                headers: { Authorization: `Bearer ${token}` }
            });

            if (response.ok) {
                const payload = await response.json();
                devices = payload.devices ?? [];
            } else {
                error = 'Error cargando dispositivos';
            }
        } catch (e) {
            error = 'Error de conexión';
            console.error(e);
        } finally {
            isLoading = false;
        }
    }

    onMount(() => {
        fetchDevices();
    });
</script>

<div class="p-6 space-y-6">
    <div class="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
        <div>
            <h1 class="text-3xl font-bold text-zinc-900">Dispositivos</h1>
            <p class="text-zinc-500 mt-1">Inventario de dispositivos activos y estado de conectividad.</p>
        </div>
        <Badge variant="secondary" class="self-start md:self-auto">
            <Monitor class="h-4 w-4 mr-1" /> {devices.length} dispositivos
        </Badge>
    </div>

    {#if error}
        <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg">
            {error}
        </div>
    {/if}

    {#if isLoading}
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
            {#each [1, 2, 3, 4] as _}
                <Card class="bg-white">
                    <CardHeader>
                        <Skeleton class="h-6 w-32" />
                    </CardHeader>
                    <CardContent>
                        <div class="space-y-3">
                            <Skeleton class="h-4 w-3/4" />
                            <Skeleton class="h-4 w-2/4" />
                            <Skeleton class="h-4 w-full" />
                        </div>
                    </CardContent>
                </Card>
            {/each}
        </div>
    {:else if devices.length === 0}
        <div class="bg-yellow-50 border border-yellow-200 text-yellow-800 px-4 py-4 rounded-lg">
            No hay dispositivos registrados en el sistema.
        </div>
    {:else}
        <div class="grid grid-cols-1 xl:grid-cols-2 gap-6">
            {#each devices as device}
                <Card class="bg-white">
                    <CardHeader>
                        <div class="flex items-center justify-between gap-3">
                            <div>
                                <CardTitle class="text-lg flex items-center gap-2">
                                    <Wifi class="h-5 w-5 text-sky-600" />
                                    {device.name}
                                </CardTitle>
                                <CardDescription>{device.device_type}</CardDescription>
                            </div>
                            <Badge variant={getStatusVariant(device.status)}>
                                {getStatusLabel(device.status)}
                            </Badge>
                        </div>
                    </CardHeader>
                    <CardContent>
                        <div class="space-y-3 text-sm text-zinc-600">
                            <div class="flex items-center gap-2">
                                <Cpu class="h-4 w-4 text-zinc-400" />
                                <span>IP: {device.ip_address ?? 'Sin asignar'}</span>
                            </div>
                            <div class="flex items-center gap-2">
                                <Monitor class="h-4 w-4 text-zinc-400" />
                                <span>Ancho de banda: {device.bandwidth_gbps?.toFixed(2) ?? '0.00'} Gbps</span>
                            </div>
                            <div class="flex items-center gap-2">
                                <Clock3 class="h-4 w-4 text-zinc-400" />
                                <span>Última vez visto: {device.last_seen ?? 'No disponible'}</span>
                            </div>
                        </div>
                    </CardContent>
                </Card>
            {/each}
        </div>
    {/if}
</div>
