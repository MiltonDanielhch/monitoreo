<!-- apps/web/src/routes/dashboard/sedes/+page.svelte -->
<!-- Página de Sedes del Beni con jerarquía de áreas -->
<!-- Vinculado con ADR-0017-frontend-sveltekit-svelte5.md -->

<script lang="ts">
	import { onMount } from 'svelte';
	import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { Building2, MapPin, Globe, ChevronRight, Server } from 'lucide-svelte';

	interface Location {
		id: string;
		name: string;
		code: string;
		region: string;
		parent_id: string | null;
		latitude: number | null;
		longitude: number | null;
		is_active: boolean;
	}

	let locations = $state<Location[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	async function fetchLocations() {
		try {
			const token = typeof sessionStorage !== 'undefined' ? sessionStorage.getItem('access_token') : null;
			const response = await fetch('/api/locations', {
				headers: { Authorization: `Bearer ${token}` }
			});

			if (response.ok) {
				locations = await response.json();
			} else {
				error = 'Error cargando sedes';
			}
		} catch (e) {
			error = 'Error de conexión';
			console.error(e);
		} finally {
			isLoading = false;
		}
	}

	onMount(() => {
		fetchLocations();
	});

	function getSedes(): Location[] {
		return locations.filter(loc => loc.parent_id === null);
	}

	function getAreas(parentId: string): Location[] {
		return locations.filter(loc => loc.parent_id === parentId);
	}
</script>

<div class="p-6 space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold text-zinc-900">Sedes y Áreas</h1>
			<p class="text-zinc-500 mt-1">Red de monitoreo regional del Beni</p>
		</div>
		<Badge variant="secondary">
			<Building2 class="h-4 w-4 mr-1" />
			{getSedes().length} sedes · {locations.filter(l => l.parent_id !== null).length} áreas
		</Badge>
	</div>

	{#if error}
		<div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg">
			{error}
		</div>
	{/if}

	{#if isLoading}
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			{#each [1, 2] as _}
				<Card class="bg-white">
					<CardHeader>
						<Skeleton class="h-6 w-32" />
					</CardHeader>
					<CardContent>
						<div class="space-y-2">
							<Skeleton class="h-12 w-full" />
							<Skeleton class="h-12 w-full" />
						</div>
					</CardContent>
				</Card>
			{/each}
		</div>
	{:else}
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			{#each getSedes() as sede}
				{@const areas = getAreas(sede.id)}
				<Card class="bg-white">
					<CardHeader>
						<div class="flex items-start justify-between">
							<div>
								<CardTitle class="text-lg flex items-center gap-2">
									<Building2 class="h-5 w-5 text-blue-600" />
									{sede.name}
								</CardTitle>
								<Badge variant="outline" class="mt-1">{sede.code}</Badge>
							</div>
							<div class="flex items-center gap-2">
								<Globe class="h-4 w-4 text-zinc-400" />
								<span class="text-sm text-zinc-500">{sede.region}</span>
							</div>
						</div>
						<CardDescription class="flex items-center gap-1 mt-2">
							<MapPin class="h-3 w-3" />
							{#if sede.latitude && sede.longitude}
								{sede.latitude.toFixed(4)}, {sede.longitude.toFixed(4)}
							{/if}
						</CardDescription>
					</CardHeader>
					<CardContent>
						{#if areas.length > 0}
							<div class="border-t border-zinc-100 pt-4">
								<p class="text-xs font-medium text-zinc-500 uppercase tracking-wide mb-3">
									Áreas ({areas.length})
								</p>
								<div class="space-y-2">
									{#each areas as area}
										<div class="flex items-center gap-3 p-2 rounded-lg bg-zinc-50 hover:bg-zinc-100 transition-colors">
											<Server class="h-4 w-4 text-zinc-400" />
											<div class="flex-1">
												<p class="text-sm font-medium text-zinc-700">{area.name}</p>
												<p class="text-xs text-zinc-400">{area.code}</p>
											</div>
											<ChevronRight class="h-4 w-4 text-zinc-300" />
										</div>
									{/each}
								</div>
							</div>
						{:else}
							<div class="border-t border-zinc-100 pt-4">
								<p class="text-sm text-zinc-400 italic">Sin áreas registradas</p>
							</div>
						{/if}
					</CardContent>
				</Card>
			{/each}
		</div>
	{/if}
</div>