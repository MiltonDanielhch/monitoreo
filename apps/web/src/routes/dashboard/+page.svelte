<!-- apps/web/src/routes/dashboard/+page.svelte -->
<!-- Dashboard principal con KPIs y métricas -->
<!-- Vinculado con ADR-0017-frontend-sveltekit-svelte5.md -->

<script lang="ts">
	import { onMount } from 'svelte';
	import { auth } from '$lib/auth.svelte';
	import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { Building2, Monitor, AlertTriangle, Wifi, Activity } from 'lucide-svelte';

	interface DashboardStats {
		active_locations: number;
		online_devices: number;
		total_devices: number;
		pending_alerts: number;
		critical_alerts: number;
		total_bandwidth_gbps: number;
	}

	interface Alert {
		id: string;
		severity: string;
		title: string;
		description: string | null;
		metric_name: string | null;
		metric_value: number | null;
		created_at: string;
	}

	let stats = $state<DashboardStats | null>(null);
	let recentAlerts = $state<Alert[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	async function fetchDashboardData() {
		try {
			const token = typeof sessionStorage !== 'undefined' ? sessionStorage.getItem('access_token') : null;

			const [statsRes, alertsRes] = await Promise.all([
				fetch('/api/dashboard/stats', {
					headers: { Authorization: `Bearer ${token}` }
				}),
				fetch('/api/dashboard/alerts', {
					headers: { Authorization: `Bearer ${token}` }
				})
			]);

			if (statsRes.ok) {
				stats = await statsRes.json();
			}

			if (alertsRes.ok) {
				const data = await alertsRes.json();
				recentAlerts = data.alerts || [];
			}
		} catch (e) {
			error = 'Error cargando datos del dashboard';
			console.error(e);
		} finally {
			isLoading = false;
		}
	}

	onMount(() => {
		fetchDashboardData();
		const interval = setInterval(fetchDashboardData, 30000);
		return () => clearInterval(interval);
	});

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleString('es-BO');
	}
</script>

<div class="p-6 space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold text-zinc-900">Panel de Monitoreo</h1>
			<p class="text-zinc-500 mt-1">Estado operativo en tiempo real del Beni</p>
		</div>
		{#if auth.email}
			<Badge variant="secondary">{auth.email}</Badge>
		{/if}
	</div>

	{#if error}
		<div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg">
			{error}
		</div>
	{/if}

	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
		<Card class="bg-white">
			<CardHeader class="flex flex-row items-center justify-between pb-2">
				<CardTitle class="text-sm font-medium text-zinc-500">Sedes Activas</CardTitle>
				<Building2 class="h-4 w-4 text-zinc-400" />
			</CardHeader>
			<CardContent>
				{#if isLoading}
					<Skeleton class="h-8 w-16" />
				{:else}
					<div class="text-3xl font-bold text-zinc-900">{stats?.active_locations ?? 0}</div>
					<p class="text-xs text-zinc-500 mt-1">de {5} sedes registradas</p>
				{/if}
			</CardContent>
		</Card>

		<Card class="bg-white">
			<CardHeader class="flex flex-row items-center justify-between pb-2">
				<CardTitle class="text-sm font-medium text-zinc-500">Dispositivos Online</CardTitle>
				<Monitor class="h-4 w-4 text-green-500" />
			</CardHeader>
			<CardContent>
				{#if isLoading}
					<Skeleton class="h-8 w-16" />
				{:else}
					<div class="text-3xl font-bold text-zinc-900">{stats?.online_devices ?? 0}</div>
					<p class="text-xs text-zinc-500 mt-1">de {stats?.total_devices ?? 0} dispositivos</p>
				{/if}
			</CardContent>
		</Card>

		<Card class="bg-white">
			<CardHeader class="flex flex-row items-center justify-between pb-2">
				<CardTitle class="text-sm font-medium text-zinc-500">Alertas Pendientes</CardTitle>
				<AlertTriangle class="h-4 w-4 text-yellow-500" />
			</CardHeader>
			<CardContent>
				{#if isLoading}
					<Skeleton class="h-8 w-16" />
				{:else}
					<div class="text-3xl font-bold text-zinc-900">{stats?.pending_alerts ?? 0}</div>
					<p class="text-xs text-zinc-500 mt-1">
						{stats?.critical_alerts ?? 0} críticas
					</p>
				{/if}
			</CardContent>
		</Card>

		<Card class="bg-white">
			<CardHeader class="flex flex-row items-center justify-between pb-2">
				<CardTitle class="text-sm font-medium text-zinc-500">Ancho de Banda</CardTitle>
				<Wifi class="h-4 w-4 text-blue-500" />
			</CardHeader>
			<CardContent>
				{#if isLoading}
					<Skeleton class="h-8 w-16" />
				{:else}
					<div class="text-3xl font-bold text-zinc-900">
						{stats?.total_bandwidth_gbps?.toFixed(2) ?? '0.00'}
					</div>
					<p class="text-xs text-zinc-500 mt-1">Gbps throughput total</p>
				{/if}
			</CardContent>
		</Card>
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
		<Card class="bg-white">
			<CardHeader>
				<CardTitle class="flex items-center gap-2">
					<Activity class="h-5 w-5 text-zinc-700" />
					Métricas de Red
				</CardTitle>
				<CardDescription>Resumen de rendimiento en las últimas 24 horas</CardDescription>
			</CardHeader>
			<CardContent>
				{#if isLoading}
					<div class="space-y-3">
						<Skeleton class="h-12 w-full" />
						<Skeleton class="h-12 w-full" />
						<Skeleton class="h-12 w-full" />
					</div>
				{:else}
					<div class="space-y-4">
						<div class="flex items-center justify-between">
							<span class="text-sm text-zinc-600">Latencia Promedio</span>
							<span class="font-semibold">45 ms</span>
						</div>
						<div class="w-full bg-zinc-100 rounded-full h-2">
							<div class="bg-green-500 h-2 rounded-full" style="width: 45%"></div>
						</div>

						<div class="flex items-center justify-between">
							<span class="text-sm text-zinc-600">Pérdida de Paquetes</span>
							<span class="font-semibold">0.3%</span>
						</div>
						<div class="w-full bg-zinc-100 rounded-full h-2">
							<div class="bg-green-500 h-2 rounded-full" style="width: 3%"></div>
						</div>

						<div class="flex items-center justify-between">
							<span class="text-sm text-zinc-600">Uptime Global</span>
							<span class="font-semibold">99.7%</span>
						</div>
						<div class="w-full bg-zinc-100 rounded-full h-2">
							<div class="bg-green-500 h-2 rounded-full" style="width: 99.7%"></div>
						</div>
					</div>
				{/if}
			</CardContent>
		</Card>

		<Card class="bg-white">
			<CardHeader>
				<CardTitle class="flex items-center gap-2">
					<AlertTriangle class="h-5 w-5 text-red-500" />
					Alertas Recientes
				</CardTitle>
				<CardDescription>Últimos incidentes detectados en la red</CardDescription>
			</CardHeader>
			<CardContent>
				{#if isLoading}
					<div class="space-y-3">
						<Skeleton class="h-16 w-full" />
						<Skeleton class="h-16 w-full" />
					</div>
				{:else if recentAlerts.length === 0}
					<div class="text-center py-8 text-zinc-500">
						<p>No hay alertas activas</p>
					</div>
				{:else}
					<div class="space-y-3">
						{#each recentAlerts as alert}
							<div class="flex items-start gap-3 p-3 rounded-lg bg-zinc-50">
								<div class="flex-shrink-0 mt-0.5">
									{#if alert.severity === 'critical'}
										<div class="w-2 h-2 rounded-full bg-red-500"></div>
									{:else if alert.severity === 'warning'}
										<div class="w-2 h-2 rounded-full bg-yellow-500"></div>
									{:else}
										<div class="w-2 h-2 rounded-full bg-blue-500"></div>
									{/if}
								</div>
								<div class="flex-1 min-w-0">
									<p class="text-sm font-medium text-zinc-900">{alert.title}</p>
									{#if alert.description}
										<p class="text-xs text-zinc-500 mt-1">{alert.description}</p>
									{/if}
									<p class="text-xs text-zinc-400 mt-1">{formatDate(alert.created_at)}</p>
								</div>
								<Badge variant={alert.severity === 'critical' ? 'destructive' : alert.severity === 'warning' ? 'secondary' : 'outline'}>
									{alert.severity}
								</Badge>
							</div>
						{/each}
					</div>
				{/if}
			</CardContent>
		</Card>
	</div>
</div>