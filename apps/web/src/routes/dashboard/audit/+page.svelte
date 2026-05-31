<!-- apps/web/src/routes/dashboard/audit/+page.svelte -->
<!-- UI de Línea de Tiempo Quirúrgica - Auditoría Inmutable -->
<!-- Módulo 6: Auditoría Dinámica Inmutable -->

<script lang="ts">
    import { createQuery } from '@tanstack/svelte-query';
    import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
    import { Button } from '$lib/components/ui/button';
    import { Badge } from '$lib/components/ui/badge';
    import { Clock, User, Server, FileText, Shield, Download, Upload, Search, Filter } from 'lucide-svelte';

    // Estado de filtros
    let selectedAction = $state<string | null>(null);
    let selectedEntityType = $state<string | null>(null);
    let searchQuery = $state('');
    let currentPage = $state(1);

    interface AuditLogDto {
        id: string;
        timestamp: string;
        user_id: string | null;
        action: string;
        entity_type: string;
        entity_id: string | null;
        old_value: any | null;
        new_value: any | null;
        ip_address: string | null;
        user_agent: string | null;
        metadata: any | null;
    }

    interface AuditLogsResponse {
        logs: AuditLogDto[];
        total: number;
        page: number;
        per_page: number;
    }

    // Query para obtener logs de auditoría
    const auditLogsQuery = createQuery(() => ({
        queryKey: ['audit-logs', selectedAction, selectedEntityType, currentPage],
        queryFn: async () => {
            const params = new URLSearchParams();
            if (selectedAction) params.append('action', selectedAction);
            if (selectedEntityType) params.append('entity_type', selectedEntityType);
            params.append('page', currentPage.toString());
            params.append('per_page', '50');

            const response = await fetch(`/api/v1/audit/logs?${params.toString()}`);
            if (!response.ok) throw new Error('Error fetching audit logs');
            const data: AuditLogsResponse = await response.json();
            return data;
        }
    }));

    // Función para obtener icono según acción
    function getActionIcon(action: string) {
        switch (action) {
            case 'CREATE': return Upload;
            case 'UPDATE': return FileText;
            case 'DELETE': return Shield;
            case 'LOGIN': return User;
            case 'LOGOUT': return User;
            case 'UPLOAD': return Upload;
            case 'DOWNLOAD': return Download;
            default: return Clock;
        }
    }

    // Función para obtener color según acción
    function getActionColor(action: string): string {
        switch (action) {
            case 'CREATE': return 'bg-green-500';
            case 'UPDATE': return 'bg-blue-500';
            case 'DELETE': return 'bg-red-500';
            case 'LOGIN': return 'bg-purple-500';
            case 'LOGOUT': return 'bg-orange-500';
            case 'UPLOAD': return 'bg-cyan-500';
            case 'DOWNLOAD': return 'bg-teal-500';
            default: return 'bg-gray-500';
        }
    }

    // Función para formatear fecha
    function formatDate(timestamp: string): string {
        const date = new Date(timestamp);
        return date.toLocaleString('es-BO', {
            day: '2-digit',
            month: '2-digit',
            year: 'numeric',
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit'
        });
    }

    // Función para filtrar logs
    function getFilteredLogs(): AuditLogDto[] {
        if (!auditLogsQuery.data) return [];
        
        return auditLogsQuery.data.logs.filter(log => {
            if (searchQuery && !log.entity_type.toLowerCase().includes(searchQuery.toLowerCase())) {
                return false;
            }
            return true;
        });
    }

    // Acciones disponibles
    const actions = ['CREATE', 'UPDATE', 'DELETE', 'LOGIN', 'LOGOUT', 'UPLOAD', 'DOWNLOAD'];
    
    // Tipos de entidad disponibles
    const entityTypes = ['User', 'Device', 'Alert', 'Notification', 'InfrastructureFile', 'SystemSetting'];
</script>

<div class="p-6 space-y-6">
    <div class="flex items-center justify-between">
        <div>
            <h1 class="text-3xl font-bold text-zinc-100">Auditoría</h1>
            <p class="text-zinc-400 mt-1">Historial inmutable de acciones del sistema</p>
        </div>
    </div>

    <!-- Filtros -->
    <Card class="bg-zinc-900 border-zinc-800">
        <CardHeader>
            <CardTitle class="text-zinc-100">Filtros</CardTitle>
            <CardDescription class="text-zinc-400">
                Filtra los registros de auditoría por acción y tipo de entidad
            </CardDescription>
        </CardHeader>
        <CardContent class="space-y-4">
            <div class="flex gap-4 flex-wrap">
                <!-- Búsqueda -->
                <div class="flex-1 min-w-[200px]">
                    <div class="relative">
                        <Search class="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-zinc-500" />
                        <input
                            type="text"
                            placeholder="Buscar por tipo de entidad..."
                            bind:value={searchQuery}
                            class="w-full pl-10 pr-4 py-2 bg-zinc-800 border border-zinc-700 rounded-lg text-zinc-100 placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
                        />
                    </div>
                </div>

                <!-- Filtro de acción -->
                <div class="flex gap-2">
                    <Button
                        variant={selectedAction === null ? 'default' : 'outline'}
                        onclick={() => selectedAction = null}
                        class={selectedAction === null ? 'bg-blue-600' : 'border-zinc-700 text-zinc-300'}
                    >
                        Todas
                    </Button>
                    {#each actions as action}
                        <Button
                            variant={selectedAction === action ? 'default' : 'outline'}
                            onclick={() => selectedAction = action}
                            class={selectedAction === action ? 'bg-blue-600' : 'border-zinc-700 text-zinc-300'}
                        >
                            {action}
                        </Button>
                    {/each}
                </div>
            </div>

            <!-- Filtro de tipo de entidad -->
            <div class="flex gap-2">
                <Button
                    variant={selectedEntityType === null ? 'default' : 'outline'}
                    onclick={() => selectedEntityType = null}
                    class={selectedEntityType === null ? 'bg-blue-600' : 'border-zinc-700 text-zinc-300'}
                >
                    Todos los tipos
                </Button>
                {#each entityTypes as entityType}
                    <Button
                        variant={selectedEntityType === entityType ? 'default' : 'outline'}
                        onclick={() => selectedEntityType = entityType}
                        class={selectedEntityType === entityType ? 'bg-blue-600' : 'border-zinc-700 text-zinc-300'}
                    >
                        {entityType}
                    </Button>
                {/each}
            </div>
        </CardContent>
    </Card>

    <!-- Línea de tiempo -->
    <Card class="bg-zinc-900 border-zinc-800">
        <CardHeader>
            <CardTitle class="text-zinc-100">Línea de Tiempo</CardTitle>
            <CardDescription class="text-zinc-400">
                Registro cronológico de todas las acciones del sistema
            </CardDescription>
        </CardHeader>
        <CardContent>
            {#if auditLogsQuery.isLoading}
                <p class="text-zinc-400 text-center py-8">Cargando registros...</p>
            {:else if auditLogsQuery.isError}
                <p class="text-red-400 text-center py-8">Error al cargar registros</p>
            {:else if getFilteredLogs().length > 0}
                <div class="relative">
                    <!-- Línea vertical -->
                    <div class="absolute left-4 top-0 bottom-0 w-0.5 bg-zinc-700"></div>

                    <div class="space-y-6">
                        {#each getFilteredLogs() as log (log.id)}
                            <div class="relative pl-12">
                                <!-- Punto en la línea de tiempo -->
                                <div class="absolute left-2 top-0 w-5 h-5 rounded-full bg-zinc-700 border-2 border-zinc-600 flex items-center justify-center">
                                    <div class="w-2 h-2 rounded-full {getActionColor(log.action)}"></div>
                                </div>

                                <!-- Tarjeta del evento -->
                                <div class="bg-zinc-800 rounded-lg p-4 hover:bg-zinc-750 transition-colors">
                                    <div class="flex items-start justify-between mb-2">
                                        <div class="flex items-center gap-3">
                                            <svelte:component this={getActionIcon(log.action)} class="h-5 w-5 text-zinc-400" />
                                            <Badge class={getActionColor(log.action)}>
                                                {log.action}
                                            </Badge>
                                            <span class="text-zinc-300 font-medium">{log.entity_type}</span>
                                            {#if log.entity_id}
                                                <span class="text-zinc-500 text-sm">#{log.entity_id}</span>
                                            {/if}
                                        </div>
                                        <span class="text-zinc-500 text-sm">{formatDate(log.timestamp)}</span>
                                    </div>

                                    <div class="space-y-2 text-sm">
                                        {#if log.user_id}
                                            <div class="flex items-center gap-2 text-zinc-400">
                                                <User class="h-4 w-4" />
                                                <span>Usuario: {log.user_id}</span>
                                            </div>
                                        {/if}

                                        {#if log.ip_address}
                                            <div class="flex items-center gap-2 text-zinc-400">
                                                <Server class="h-4 w-4" />
                                                <span>IP: {log.ip_address}</span>
                                            </div>
                                        {/if}

                                        {#if log.old_value || log.new_value}
                                            <div class="mt-2 p-2 bg-zinc-900 rounded">
                                                {#if log.old_value}
                                                    <div class="text-zinc-500 mb-1">Antes:</div>
                                                    <pre class="text-zinc-400 text-xs overflow-x-auto">{JSON.stringify(log.old_value, null, 2)}</pre>
                                                {/if}
                                                {#if log.new_value}
                                                    <div class="text-zinc-500 mb-1 mt-2">Después:</div>
                                                    <pre class="text-zinc-400 text-xs overflow-x-auto">{JSON.stringify(log.new_value, null, 2)}</pre>
                                                {/if}
                                            </div>
                                        {/if}
                                    </div>
                                </div>
                            </div>
                        {/each}
                    </div>
                </div>

                <!-- Paginación -->
                <div class="flex items-center justify-between mt-6">
                    <span class="text-zinc-400 text-sm">
                        Página {auditLogsQuery.data?.page || 1} de {Math.ceil((auditLogsQuery.data?.total || 0) / (auditLogsQuery.data?.per_page || 50))}
                    </span>
                    <div class="flex gap-2">
                        <Button
                            variant="outline"
                            onclick={() => currentPage = Math.max(1, currentPage - 1)}
                            disabled={currentPage === 1}
                            class="border-zinc-700 text-zinc-300"
                        >
                            Anterior
                        </Button>
                        <Button
                            variant="outline"
                            onclick={() => currentPage += 1}
                            disabled={!auditLogsQuery.data || auditLogsQuery.data.logs.length < auditLogsQuery.data.per_page}
                            class="border-zinc-700 text-zinc-300"
                        >
                            Siguiente
                        </Button>
                    </div>
                </div>
            {:else}
                <p class="text-zinc-500 text-center py-8">No hay registros de auditoría</p>
            {/if}
        </CardContent>
    </Card>
</div>
