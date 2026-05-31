<!-- apps/web/src/routes/dashboard/notifications/+page.svelte -->
<!-- Historial de alertas - Svelte 5 + TanStack Query + shadcn-svelte -->
<!-- Vinculado con ADR-0017-frontend-sveltekit-svelte5.md -->
<!-- Módulo 4: Motor de Notificaciones -->

<script lang="ts">
    import { createQuery } from '@tanstack/svelte-query';
    import { onMount } from 'svelte';
    import { Badge } from '$lib/components/ui/badge';
    import { Button } from '$lib/components/ui/button';
    import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
    import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '$lib/components/ui/table';
    import { ChevronLeft, ChevronRight, Mail, AlertCircle, CheckCircle2, Clock } from 'lucide-svelte';

    // Estado de paginación
    let currentPage = $state(1);
    let perPage = $state(20);

    interface NotificationLog {
        id: string;
        channel_id: string;
        template_id: string;
        recipient: string;
        status: string;
        attempt_count: number;
        max_attempts: number;
        error_message: string | null;
        sent_at: string | null;
        created_at: string;
    }

    interface NotificationLogsResponse {
        logs: NotificationLog[];
        total: number;
        page: number;
        per_page: number;
    }

    // Query para obtener logs de notificaciones
    const logsQuery = createQuery(() => ({
        queryKey: ['notification-logs', currentPage, perPage],
        queryFn: async () => {
            const response = await fetch('/api/v1/notifications/logs', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    page: currentPage,
                    per_page: perPage,
                }),
            });

            if (!response.ok) {
                throw new Error('Error al obtener logs de notificaciones');
            }

            return response.json() as Promise<NotificationLogsResponse>;
        },
    }));

    // Función para obtener el badge según el estado
    function getStatusBadge(status: string) {
        switch (status) {
            case 'SENT':
                return {
                    variant: 'default' as const,
                    icon: CheckCircle2,
                    class: 'bg-green-500/10 text-green-500 border-green-500/20',
                    label: 'Enviado',
                };
            case 'PENDING':
                return {
                    variant: 'secondary' as const,
                    icon: Clock,
                    class: 'bg-yellow-500/10 text-yellow-500 border-yellow-500/20',
                    label: 'Pendiente',
                };
            case 'RETRYING':
                return {
                    variant: 'secondary' as const,
                    icon: Clock,
                    class: 'bg-orange-500/10 text-orange-500 border-orange-500/20',
                    label: 'Reintentando',
                };
            case 'FAILED':
                return {
                    variant: 'destructive' as const,
                    icon: AlertCircle,
                    class: 'bg-red-500/10 text-red-500 border-red-500/20',
                    label: 'Fallido',
                };
            default:
                return {
                    variant: 'secondary' as const,
                    icon: Clock,
                    class: 'bg-gray-500/10 text-gray-500 border-gray-500/20',
                    label: status,
                };
        }
    }

    // Iconos para badges
    const CheckCircle2Icon = CheckCircle2;
    const ClockIcon = Clock;
    const AlertCircleIcon = AlertCircle;

    // Función para formatear fecha
    function formatDate(dateString: string | null) {
        if (!dateString) return '-';
        const date = new Date(dateString);
        return date.toLocaleString('es-BO', {
            day: '2-digit',
            month: '2-digit',
            year: 'numeric',
            hour: '2-digit',
            minute: '2-digit',
        });
    }

    // Paginación
    function totalPages() {
        return Math.ceil((logsQuery.data?.total || 0) / perPage);
    }

    function nextPage() {
        if (currentPage < totalPages()) {
            currentPage++;
        }
    }

    function previousPage() {
        if (currentPage > 1) {
            currentPage--;
        }
    }
</script>

<div class="p-6 space-y-6">
    <div class="flex items-center justify-between">
        <div>
            <h1 class="text-3xl font-bold text-white tracking-tight">Historial de Notificaciones</h1>
            <p class="text-slate-400 mt-1">Registro de envíos y reintentos de alertas</p>
        </div>
        <Mail class="w-8 h-8 text-slate-400" />
    </div>

    <Card class="bg-slate-900/50 border-slate-800">
        <CardHeader>
            <CardTitle class="text-white">Logs de Envío</CardTitle>
            <CardDescription class="text-slate-400">
                {#if logsQuery.data}
                    Total: {logsQuery.data.total} notificaciones
                {:else}
                    Cargando...
                {/if}
            </CardDescription>
        </CardHeader>
        <CardContent>
            {#if logsQuery.isLoading}
                <div class="flex items-center justify-center py-12">
                    <div class="text-slate-400">Cargando logs...</div>
                </div>
            {:else if logsQuery.error}
                <div class="flex items-center justify-center py-12">
                    <div class="text-red-400">Error al cargar logs: {logsQuery.error.message}</div>
                </div>
            {:else if logsQuery.data && logsQuery.data.logs.length === 0}
                <div class="flex flex-col items-center justify-center py-12 space-y-4">
                    <Mail class="w-16 h-16 text-slate-600" />
                    <div class="text-slate-400">No hay notificaciones registradas</div>
                </div>
            {:else}
                <div class="rounded-md border border-slate-800">
                    <Table>
                        <TableHeader>
                            <TableRow class="border-slate-800 hover:bg-slate-800/50">
                                <TableHead class="text-slate-300">Destinatario</TableHead>
                                <TableHead class="text-slate-300">Estado</TableHead>
                                <TableHead class="text-slate-300">Intentos</TableHead>
                                <TableHead class="text-slate-300">Error</TableHead>
                                <TableHead class="text-slate-300">Enviado</TableHead>
                                <TableHead class="text-slate-300">Creado</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {#each logsQuery.data?.logs || [] as log}
                                <TableRow class="border-slate-800 hover:bg-slate-800/50">
                                    <TableCell class="text-slate-300 font-medium">{log.recipient}</TableCell>
                                    <TableCell>
                                        {@const badge = getStatusBadge(log.status)}
                                        <Badge class={badge.class} variant={badge.variant}>
                                            {#if badge.icon === CheckCircle2Icon}
                                                <CheckCircle2 class="w-3 h-3 mr-1" />
                                            {:else if badge.icon === ClockIcon}
                                                <Clock class="w-3 h-3 mr-1" />
                                            {:else if badge.icon === AlertCircleIcon}
                                                <AlertCircle class="w-3 h-3 mr-1" />
                                            {/if}
                                            {badge.label}
                                        </Badge>
                                    </TableCell>
                                    <TableCell class="text-slate-300">
                                        {log.attempt_count}/{log.max_attempts}
                                    </TableCell>
                                    <TableCell class="text-slate-400 text-sm max-w-xs truncate">
                                        {log.error_message || '-'}
                                    </TableCell>
                                    <TableCell class="text-slate-300">{formatDate(log.sent_at)}</TableCell>
                                    <TableCell class="text-slate-300">{formatDate(log.created_at)}</TableCell>
                                </TableRow>
                            {/each}
                        </TableBody>
                    </Table>
                </div>

                <!-- Paginación -->
                {#if totalPages() > 1}
                    <div class="flex items-center justify-between mt-4">
                        <div class="text-sm text-slate-400">
                            Página {currentPage} de {totalPages()}
                        </div>
                        <div class="flex items-center space-x-2">
                            <Button
                                variant="outline"
                                size="sm"
                                onclick={previousPage}
                                disabled={currentPage === 1}
                                class="bg-slate-800 border-slate-700 text-slate-300 hover:bg-slate-700 disabled:opacity-50"
                            >
                                <ChevronLeft class="w-4 h-4 mr-1" />
                                Anterior
                            </Button>
                            <Button
                                variant="outline"
                                size="sm"
                                onclick={nextPage}
                                disabled={currentPage === totalPages()}
                                class="bg-slate-800 border-slate-700 text-slate-300 hover:bg-slate-700 disabled:opacity-50"
                            >
                                Siguiente
                                <ChevronRight class="w-4 h-4 ml-1" />
                            </Button>
                        </div>
                    </div>
                {/if}
            {/if}
        </CardContent>
    </Card>
</div>
