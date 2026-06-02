<!-- apps/web/src/routes/dashboard/discovery/[id]/+page.svelte -->
<!-- Página de Detalle de Dispositivo Descubierto - Módulo 12 -->
<!-- Vinculado con ADR-0017 (Frontend SvelteKit/Svelte 5) -->

<script lang="ts">
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import {
		ArrowLeft,
		Shield,
		ShieldOff,
		Monitor,
		Wifi,
		Server,
		Router,
		Printer,
		Smartphone,
		HelpCircle,
		Network,
		Clock,
		Hash,
		Globe,
		Cpu,
	} from 'lucide-svelte';

	// ID del dispositivo desde la URL
	const deviceId = $derived(page.params.id);

	// Tipo para dispositivo
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

	// Estado
	let device = $state<DiscoveredDevice | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let isAuthorized = $state(false);

	// Cargar dispositivo
	async function loadDevice() {
		loading = true;
		error = null;

		try {
			const response = await fetch(`/api/v1/discovery/devices`, {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
				},
			});

			if (!response.ok) {
				throw new Error('Error al cargar dispositivo');
			}

			const data = await response.json();
			const found = (data.devices || []).find((d: DiscoveredDevice) => d.id === deviceId);

			if (!found) {
				throw new Error('Dispositivo no encontrado');
			}

			device = found;
			isAuthorized = found.is_authorized;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Error desconocido';
		} finally {
			loading = false;
		}
	}

	// Autorizar/Desautorizar
	async function toggleAuthorization() {
		if (!device) return;

		const endpoint = isAuthorized
			? `/api/v1/discovery/devices/${device.id}/unauthorize`
			: `/api/v1/discovery/devices/${device.id}/authorize`;

		try {
			const response = await fetch(endpoint, {
				method: 'PUT',
				headers: {
					'Content-Type': 'application/json',
				},
			});

			if (!response.ok) {
				throw new Error('Error al actualizar autorización');
			}

			isAuthorized = !isAuthorized;
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
				return Network;
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
				dateStyle: 'medium',
				timeStyle: 'medium',
			});
		} catch {
			return dateStr;
		}
	}

	function formatJson(obj: any): string {
		try {
			return JSON.stringify(obj, null, 2);
		} catch {
			return String(obj);
		}
	}

	onMount(() => {
		loadDevice();
	});
</script>

<div class="p-6 space-y-6">
	<!-- Encabezado -->
	<div class="flex items-center gap-4">
		<Button variant="ghost" href="/dashboard/discovery">
			<ArrowLeft class="h-4 w-4 mr-1" />
			Volver
		</Button>
		<div class="flex-1">
			<h1 class="text-2xl font-bold text-zinc-900">Detalle del Dispositivo</h1>
		</div>
		{#if device && !loading}
			<Button
				variant={isAuthorized ? 'destructive' : 'default'}
				onclick={toggleAuthorization}
			>
				{#if isAuthorized}
					<ShieldOff class="h-4 w-4 mr-1" />
					Desautorizar
				{:else}
					<Shield class="h-4 w-4 mr-1" />
					Autorizar
				{/if}
			</Button>
		{/if}
	</div>

	<!-- Error -->
	{#if error}
		<div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg">
			{error}
		</div>
	{/if}

	<!-- Loading -->
	{#if loading}
		<div class="space-y-4">
			<Skeleton class="h-32 w-full" />
			<Skeleton class="h-64 w-full" />
		</div>

		<!-- Contenido cargado -->
	{:else if device}
		<!-- Información Principal -->
		<Card>
			<CardHeader>
				<div class="flex items-center gap-3">
					<div class="p-2 bg-zinc-100 rounded-lg">
						<svelte:component this={getDeviceTypeIcon(device.device_type)} class="h-8 w-8" />
					</div>
					<div>
						<CardTitle class="text-xl">{device.hostname || device.ip_address}</CardTitle>
						<CardDescription>
							{device.manufacturer || 'Fabricante desconocido'} - {device.device_type}
						</CardDescription>
					</div>
					<div class="ml-auto flex gap-2">
						<Badge variant={getStatusVariant(device.status)} class="text-sm">
							<Wifi class="h-3 w-3 mr-1" />
							{device.status}
						</Badge>
						{#if isAuthorized}
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
					</div>
				</div>
			</CardHeader>
			<CardContent>
				<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
					<div class="space-y-1">
						<div class="text-xs text-zinc-500">Dirección IP</div>
						<div class="font-mono text-sm font-medium">{device.ip_address}</div>
					</div>
					<div class="space-y-1">
						<div class="text-xs text-zinc-500">Dirección MAC</div>
						<div class="font-mono text-sm">{device.mac_address || '-'}</div>
					</div>
					<div class="space-y-1">
						<div class="text-xs text-zinc-500">Tipo</div>
						<div class="text-sm font-medium capitalize">{device.device_type}</div>
					</div>
					<div class="space-y-1">
						<div class="text-xs text-zinc-500">Fabricante</div>
						<div class="text-sm">{device.manufacturer || '-'}</div>
					</div>
				</div>
			</CardContent>
		</Card>

		<!-- Puertos y Servicios -->
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<Card>
				<CardHeader>
					<CardTitle class="text-lg">
						<Hash class="h-5 w-5 inline mr-2" />
						Puertos Abiertos
					</CardTitle>
				</CardHeader>
				<CardContent>
					{#if device.open_ports.length > 0}
						<div class="flex flex-wrap gap-2">
							{#each device.open_ports as port}
								<Badge variant="secondary" class="font-mono">
									{port}
								</Badge>
							{/each}
						</div>
					{:else}
						<p class="text-sm text-zinc-500">No se detectaron puertos abiertos.</p>
					{/if}
				</CardContent>
			</Card>

			<Card>
				<CardHeader>
					<CardTitle class="text-lg">
						<Globe class="h-5 w-5 inline mr-2" />
						Servicios Detectados
					</CardTitle>
				</CardHeader>
				<CardContent>
					{#if device.services.length > 0}
						<div class="flex flex-wrap gap-2">
							{#each device.services as service}
								<Badge variant="outline">
									{service}
								</Badge>
							{/each}
						</div>
					{:else}
						<p class="text-sm text-zinc-500">No se detectaron servicios.</p>
					{/if}
				</CardContent>
			</Card>
		</div>

		<!-- Información Técnica -->
		<Card>
			<CardHeader>
				<CardTitle class="text-lg">
					<Cpu class="h-5 w-5 inline mr-2" />
					Información Técnica
				</CardTitle>
			</CardHeader>
			<CardContent>
				<div class="grid grid-cols-2 gap-4">
					<div class="space-y-1">
						<div class="text-xs text-zinc-500">ID del Dispositivo</div>
						<div class="font-mono text-xs bg-zinc-100 px-2 py-1 rounded">
							{device.id}
						</div>
					</div>
					<div class="space-y-1">
						<div class="text-xs text-zinc-500">ID del Escaneo</div>
						<div class="font-mono text-xs bg-zinc-100 px-2 py-1 rounded">
							{device.scan_id}
						</div>
					</div>
					<div class="space-y-1">
						<div class="text-xs text-zinc-500">OS Fingerprint</div>
						<div class="text-sm">{device.os_fingerprint || '-'}</div>
					</div>
					<div class="space-y-1">
						<div class="text-xs text-zinc-500">Sede</div>
						<div class="text-sm">{device.sede_id || '-'}</div>
					</div>
					<div class="space-y-1">
						<div class="text-xs text-zinc-500">Primera Detección</div>
						<div class="text-sm flex items-center gap-1">
							<Clock class="h-3 w-3" />
							{formatDate(device.first_seen)}
						</div>
					</div>
					<div class="space-y-1">
						<div class="text-xs text-zinc-500">Última Detección</div>
						<div class="text-sm flex items-center gap-1">
							<Clock class="h-3 w-3" />
							{formatDate(device.last_seen)}
						</div>
					</div>
				</div>
			</CardContent>
		</Card>

		<!-- Metadata JSON -->
		{#if device.metadata}
			<Card>
				<CardHeader>
					<CardTitle class="text-lg">Metadata</CardTitle>
					<CardDescription>Datos adicionales del dispositivo en formato JSON.</CardDescription>
				</CardHeader>
				<CardContent>
					<pre class="bg-zinc-900 text-zinc-100 p-4 rounded-lg overflow-auto text-xs font-mono">{formatJson(device.metadata)}</pre>
				</CardContent>
			</Card>
		{/if}
	{/if}
</div>
