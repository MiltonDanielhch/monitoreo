<!-- apps/web/src/routes/dashboard/discovery/+page.svelte -->
<!-- Dashboard de Descubrimiento de Red - Módulo 12 -->
<!-- Vinculado con ADR-0017 (Frontend SvelteKit/Svelte 5) -->

<script lang="ts">
	import { onMount } from 'svelte';
	import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import {
		Table,
		TableBody,
		TableCaption,
		TableCell,
		TableHead,
		TableHeader,
		TableRow,
	} from '$lib/components/ui/table';
	import {
		Monitor,
		Wifi,
		Server,
		Router,
		Printer,
		Smartphone,
		HelpCircle,
		Search,
		Plus,
		Shield,
		ShieldOff,
		Clock,
		Layers,
		Activity
	} from 'lucide-svelte';

	// Tipos para dispositivos descubiertos
	interface DiscoveredDevice {
		id: string;
		ip_address: string;
		mac_address: string | null;
		hostname: string | null;
		device_type: string;
		os_fingerprint: string | null;
		manufacturer: string | null;
		open_ports: number[];
		services: string[];
		status: string;
		is_authorized: boolean;
		last_seen: string;
		first_seen: string;
		scan_id: string;
		sede_id: string | null;
		metadata: any;
	}

	// Tipos para escaneos
	interface NetworkScan {
		id: string;
		scan_type: string;
		ip_range: string;
		status: string;
		devices_found: number;
		started_at: string;
		completed_at: string | null;
		duration_seconds: number | null;
		sede_id: string | null;
		created_by: string;
	}

	// Estado
	let devices = $state<DiscoveredDevice[]>([]);
	let scans = $state<NetworkScan[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	// Filtros
	let deviceTypeFilter = $state('');
	let statusFilter = $state('');
	let authorizedFilter = $state('');
	let manufacturerFilter = $state('');

	// Cargar dispositivos descubiertos
	async function loadDevices() {
		loading = true;
		error = null;

		try {
			const params = new URLSearchParams();
			if (deviceTypeFilter) params.append('device_type', deviceTypeFilter);
			if (statusFilter) params.append('status', statusFilter);
			if (authorizedFilter) params.append('is_authorized', authorizedFilter);
			if (manufacturerFilter) params.append('manufacturer', manufacturerFilter);

			const response = await fetch(`/api/v1/discovery/devices?${params.toString()}`, {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
				},
			});

			if (!response.ok) {
				throw new Error('Error al cargar dispositivos descubiertos');
			}

			const data = await response.json();
			devices = data.devices || [];
		} catch (e) {
			error = e instanceof Error ? e.message : 'Error desconocido';
		} finally {
			loading = false;
		}
	}

	// Cargar escaneos recientes
	async function loadScans() {
		try {
			const response = await fetch('/api/v1/discovery/scans', {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
				},
			});

			if (response.ok) {
				const data = await response.json();
				scans = (data.scans || []).slice(0, 5); // Solo últimos 5
			}
		} catch (e) {
			console.error('Error cargando escaneos:', e);
		}
	}

	// Autorizar dispositivo
	async function authorizeDevice(deviceId: string) {
		try {
			const response = await fetch(`/api/v1/discovery/devices/${deviceId}/authorize`, {
				method: 'PUT',
				headers: {
					'Content-Type': 'application/json',
				},
			});

			if (!response.ok) {
				throw new Error('Error al autorizar dispositivo');
			}

			await loadDevices();
		} catch (e) {
			alert(e instanceof Error ? e.message : 'Error desconocido');
		}
	}

	// Desautorizar dispositivo
	async function unAuthorizeDevice(deviceId: string) {
		try {
			const response = await fetch(`/api/v1/discovery/devices/${deviceId}/unauthorize`, {
				method: 'PUT',
				headers: {
					'Content-Type': 'application/json',
				},
			});

			if (!response.ok) {
				throw new Error('Error al desautorizar dispositivo');
			}

			await loadDevices();
		} catch (e) {
			alert(e instanceof Error ? e.message : 'Error desconocido');
		}
	}

	// Helpers de UI
	function getDeviceTypeIcon(type_: string) {
		switch (type_.toLowerCase()) {
			case 'router':
				return Router;
			case 'switch':
				return Layers;
			case 'server':
				return Server;
			case 'pc':
				return Monitor;
			case 'mobile':
				return Smartphone;
			case 'printer':
				return Printer;
			default:
				return HelpCircle;
		}
	}

	function getDeviceTypeVariant(type_: string): 'default' | 'secondary' | 'outline' {
		switch (type_.toLowerCase()) {
			case 'router':
			case 'switch':
				return 'default';
			case 'server':
				return 'secondary';
			default:
				return 'outline';
		}
	}

	function getStatusVariant(status: string): 'default' | 'secondary' | 'destructive' | 'outline' {
		switch (status.toLowerCase()) {
			case 'online':
				return 'default';
			case 'offline':
				return 'destructive';
			default:
				return 'outline';
		}
	}

	function formatDate(dateStr: string): string {
		try {
			return new Date(dateStr).toLocaleString('es-BO', {
				dateStyle: 'short',
				timeStyle: 'short',
			});
		} catch {
			return dateStr;
		}
	}

	function formatDuration(seconds: number | null): string {
		if (seconds === null) return '-';
		const mins = Math.floor(seconds / 60);
		const secs = seconds % 60;
		return `${mins}m ${secs}s`;
	}

	onMount(() => {
		loadDevices();
		loadScans();
		// Refrescar cada 30 segundos
		const interval = setInterval(() => {
			loadDevices();
			loadScans();
		}, 30000);
		return () => clearInterval(interval);
	});
</script>

<div class="p-6 space-y-6">
	<!-- Encabezado -->
	<div class="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
		<div>
			<h1 class="text-3xl font-bold text-zinc-900">Descubrimiento de Red</h1>
			<p class="text-zinc-500 mt-1">Dispositivos detectados en la red mediante escaneo automático.</p>
		</div>
		<div class="flex gap-2">
			<Button variant="default" href="/dashboard/discovery/scan">
				<Plus class="h-4 w-4 mr-1" />
				Nuevo Escaneo
			</Button>
			<Badge variant="secondary" class="self-center">
				<Activity class="h-4 w-4 mr-1" />
				{devices.length} dispositivos
			</Badge>
		</div>
	</div>

	<!-- Estadísticas -->
	<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
		<Card>
			<CardHeader class="pb-2">
				<CardTitle class="text-sm font-medium text-zinc-500">Total</CardTitle>
			</CardHeader>
			<CardContent>
				<div class="text-2xl font-bold">{devices.length}</div>
			</CardContent>
		</Card>
		<Card>
			<CardHeader class="pb-2">
				<CardTitle class="text-sm font-medium text-zinc-500">Online</CardTitle>
			</CardHeader>
			<CardContent>
				<div class="text-2xl font-bold text-green-600">
					{devices.filter((d) => d.status === 'online').length}
				</div>
			</CardContent>
		</Card>
		<Card>
			<CardHeader class="pb-2">
				<CardTitle class="text-sm font-medium text-zinc-500">Autorizados</CardTitle>
			</CardHeader>
			<CardContent>
				<div class="text-2xl font-bold text-blue-600">
					{devices.filter((d) => d.is_authorized).length}
				</div>
			</CardContent>
		</Card>
		<Card>
			<CardHeader class="pb-2">
				<CardTitle class="text-sm font-medium text-zinc-500">No Autorizados</CardTitle>
			</CardHeader>
			<CardContent>
				<div class="text-2xl font-bold text-red-600">
					{devices.filter((d) => !d.is_authorized && d.status === 'online').length}
				</div>
			</CardContent>
		</Card>
	</div>

	<!-- Filtros -->
	<Card>
		<CardHeader>
			<CardTitle class="text-lg">Filtros</CardTitle>
		</CardHeader>
		<CardContent class="flex flex-wrap gap-3">
			<select
				bind:value={deviceTypeFilter}
				onchange={() => loadDevices()}
				class="border rounded-md px-3 py-2 text-sm bg-white"
			>
				<option value="">Todos los tipos</option>
				<option value="router">Router</option>
				<option value="switch">Switch</option>
				<option value="server">Server</option>
				<option value="pc">PC</option>
				<option value="mobile">Mobile</option>
				<option value="iot">IoT</option>
				<option value="printer">Printer</option>
				<option value="unknown">Unknown</option>
			</select>

			<select
				bind:value={statusFilter}
				onchange={() => loadDevices()}
				class="border rounded-md px-3 py-2 text-sm bg-white"
			>
				<option value="">Todos los estados</option>
				<option value="online">Online</option>
				<option value="offline">Offline</option>
				<option value="unknown">Unknown</option>
			</select>

			<select
				bind:value={authorizedFilter}
				onchange={() => loadDevices()}
				class="border rounded-md px-3 py-2 text-sm bg-white"
			>
				<option value="">Todos</option>
				<option value="true">Autorizados</option>
				<option value="false">No Autorizados</option>
			</select>

			<Button variant="outline" onclick={() => loadDevices()}>
				<Search class="h-4 w-4 mr-1" />
				Buscar
			</Button>
		</CardContent>
	</Card>

	<!-- Errores -->
	{#if error}
		<div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg">
			{error}
		</div>
	{/if}

	<!-- Tabla de Dispositivos -->
	{#if loading}
		<Card>
			<CardContent class="p-6 space-y-4">
				{#each [1, 2, 3, 4, 5] as _}
					<Skeleton class="h-12 w-full" />
				{/each}
			</CardContent>
		</Card>
	{:else if devices.length === 0}
		<Card>
			<CardContent class="p-6 text-center text-zinc-500">
				<Search class="h-12 w-12 mx-auto mb-3 opacity-50" />
				<p>No se encontraron dispositivos.</p>
				<p class="text-sm">Inicia un escaneo para descubrir dispositivos en la red.</p>
				<Button variant="default" href="/dashboard/discovery/scan" class="mt-4">
					<Plus class="h-4 w-4 mr-1" />
					Iniciar Escaneo
				</Button>
			</CardContent>
		</Card>
	{:else}
		<Card>
			<CardContent class="p-0">
				<Table>
					<TableCaption>Lista de dispositivos descubiertos en la red.</TableCaption>
					<TableHeader>
						<TableRow>
							<TableHead>IP</TableHead>
							<TableHead>MAC</TableHead>
							<TableHead>Hostname</TableHead>
							<TableHead>Tipo</TableHead>
							<TableHead>Fabricante</TableHead>
							<TableHead>Estado</TableHead>
							<TableHead>Autorización</TableHead>
							<TableHead>Última vez visto</TableHead>
							<TableHead>Acciones</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						{#each devices as device}
							<TableRow>
								<TableCell class="font-mono text-sm">{device.ip_address}</TableCell>
								<TableCell class="font-mono text-xs text-zinc-500">
									{device.mac_address || '-'}
								</TableCell>
								<TableCell class="text-sm">
									{device.hostname || '-'}
								</TableCell>
								<TableCell>
									<div class="flex items-center gap-1">
										<svelte:component
											this={getDeviceTypeIcon(device.device_type)}
											class="h-4 w-4"
										/>
										<Badge variant={getDeviceTypeVariant(device.device_type)} class="text-xs">
											{device.device_type}
										</Badge>
									</div>
								</TableCell>
								<TableCell class="text-sm text-zinc-500">
									{device.manufacturer || '-'}
								</TableCell>
								<TableCell>
									<Badge variant={getStatusVariant(device.status)} class="text-xs">
										{device.status}
									</Badge>
								</TableCell>
								<TableCell>
									{#if device.is_authorized}
										<Badge variant="default" class="bg-green-600">
											<Shield class="h-3 w-3 mr-1" />
											Autorizado
										</Badge>
									{:else}
										<Badge variant="destructive" class="bg-red-600">
											<ShieldOff class="h-3 w-3 mr-1" />
											No Autorizado
										</Badge>
									{/if}
								</TableCell>
								<TableCell class="text-xs text-zinc-500">
									{formatDate(device.last_seen)}
								</TableCell>
								<TableCell>
									<div class="flex gap-1">
										<Button variant="ghost" size="sm" href="/dashboard/discovery/{device.id}">
											Ver
										</Button>
										{#if device.is_authorized}
											<Button
												variant="ghost"
												size="sm"
												onclick={() => unAuthorizeDevice(device.id)}
											>
												<ShieldOff class="h-4 w-4" />
											</Button>
										{:else}
											<Button
												variant="ghost"
												size="sm"
												onclick={() => authorizeDevice(device.id)}
											>
												<Shield class="h-4 w-4" />
											</Button>
										{/if}
									</div>
								</TableCell>
							</TableRow>
						{/each}
					</TableBody>
				</Table>
			</CardContent>
		</Card>
	{/if}

	<!-- Escaneos Recientes -->
	{#if scans.length > 0}
		<Card>
			<CardHeader>
				<CardTitle class="text-lg">Escaneos Recientes</CardTitle>
				<CardDescription>Últimos escaneos ejecutados en la red.</CardDescription>
			</CardHeader>
			<CardContent>
				<div class="space-y-3">
					{#each scans as scan}
						<div class="flex items-center justify-between border-b pb-3 last:border-0">
							<div>
								<div class="font-medium">{scan.ip_range}</div>
								<div class="text-xs text-zinc-500">
									{scan.scan_type} - {formatDate(scan.started_at)}
								</div>
							</div>
							<div class="flex items-center gap-3">
								<Badge
									variant={scan.status === 'completed'
										? 'default'
										: scan.status === 'running'
											? 'secondary'
											: 'outline'}
								>
									{scan.status}
								</Badge>
								<span class="text-sm text-zinc-500">
									{scan.devices_found} dispositivos
								</span>
								<span class="text-xs text-zinc-400">
									{formatDuration(scan.duration_seconds)}
								</span>
							</div>
						</div>
					{/each}
				</div>
			</CardContent>
		</Card>
	{/if}
</div>
