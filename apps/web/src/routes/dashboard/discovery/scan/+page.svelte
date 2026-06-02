<!-- apps/web/src/routes/dashboard/discovery/scan/+page.svelte -->
<!-- Página de Escaneo de Red - Módulo 12 -->
<!-- Vinculado con ADR-0017 (Frontend SvelteKit/Svelte 5) -->

<script lang="ts">
	import { goto } from '$app/navigation';
	import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { AlertCircle, Play, RotateCcw, Search, Wifi, Loader2 } from 'lucide-svelte';

	// Estado del formulario
	let ipRange = $state('192.168.1.0/24');
	let scanType = $state('partial');
	let customPorts = $state('');
	let timeoutMs = $state(1000);
	let maxConcurrent = $state(100);
	let sedeId = $state('');

	// Estado de envío
	let isSubmitting = $state(false);
	let error = $state<string | null>(null);
	let successMessage = $state<string | null>(null);

	// Estado del escaneo
	let scanId = $state<string | null>(null);
	let scanStatus = $state<string | null>(null);
	let progress = $state(0);
	let devicesFound = $state(0);

	// Tipos de escaneo
	const scanTypes = [
		{ value: 'full', label: 'Completo', description: 'Escanea todos los puertos comunes (25+ puertos)' },
		{ value: 'partial', label: 'Parcial', description: 'Escanea puertos principales (8 puertos)' },
		{ value: 'targeted', label: 'Dirigido', description: 'Escanea solo SSH, HTTP, HTTPS' },
	];

	// Iniciar escaneo
	async function startScan() {
		if (!ipRange) {
			error = 'El rango de IPs es requerido';
			return;
		}

		// Validar formato CIDR básico
		const cidrPattern = /^(\d{1,3}\.){3}\d{1,3}\/\d{1,2}$/;
		if (!cidrPattern.test(ipRange)) {
			error = 'Formato de rango IP inválido. Use formato CIDR (ej: 192.168.1.0/24)';
			return;
		}

		isSubmitting = true;
		error = null;
		successMessage = null;

		try {
			const portsArray = customPorts
				? customPorts.split(',').map((p) => parseInt(p.trim(), 10)).filter((p) => !isNaN(p))
				: [];

			const response = await fetch('/api/v1/discovery/scan', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify({
					ip_range: ipRange,
					scan_type: scanType,
					ports_to_scan: portsArray,
					timeout_ms: timeoutMs,
					max_concurrent: maxConcurrent,
					sede_id: sedeId || null,
				}),
			});

			if (!response.ok) {
				const data = await response.json();
				throw new Error(data.error || 'Error al iniciar escaneo');
			}

			const data = await response.json();
			scanId = data.scan_id;
			scanStatus = 'pending';
			successMessage = `Escaneo enqueued exitosamente. ID: ${scanId}`;

			// Redirigir a la página principal después de un momento
			setTimeout(() => {
				goto('/dashboard/discovery');
			}, 2000);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Error desconocido';
		} finally {
			isSubmitting = false;
		}
	}

	// Verificar progreso (placeholder - se conectará al ScanEngine)
	async function checkProgress() {
		if (!scanId) return;

		try {
			const response = await fetch(`/api/v1/discovery/scan/${scanId}/progress`, {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
				},
			});

			if (response.ok) {
				const data = await response.json();
				progress = data.percentage || 0;
				devicesFound = data.devices_found || 0;
				scanStatus = data.status || scanStatus;
			}
		} catch (e) {
			console.error('Error verificando progreso:', e);
		}
	}

	function resetForm() {
		ipRange = '192.168.1.0/24';
		scanType = 'partial';
		customPorts = '';
		timeoutMs = 1000;
		maxConcurrent = 100;
		sedeId = '';
		scanId = null;
		scanStatus = null;
		progress = 0;
		devicesFound = 0;
		error = null;
		successMessage = null;
	}
</script>

<div class="p-6 space-y-6 max-w-3xl">
	<!-- Encabezado -->
	<div>
		<h1 class="text-3xl font-bold text-zinc-900">Nuevo Escaneo de Red</h1>
		<p class="text-zinc-500 mt-1">
			Configura y ejecuta un escaneo de descubrimiento de dispositivos en el rango de red
			especificado.
		</p>
	</div>

	<!-- Alertas -->
	{#if error}
		<div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg flex items-start gap-2">
			<AlertCircle class="h-5 w-5 mt-0.5 flex-shrink-0" />
			<div>
				<strong>Error:</strong> {error}
			</div>
		</div>
	{/if}

	{#if successMessage}
		<div class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded-lg flex items-start gap-2">
			<Wifi class="h-5 w-5 mt-0.5 flex-shrink-0" />
			<div>
				<strong>Éxito:</strong> {successMessage}
			</div>
		</div>
	{/if}

	<!-- Formulario -->
	<Card>
		<CardHeader>
			<CardTitle class="text-lg">Configuración del Escaneo</CardTitle>
			<CardDescription>
				Define el rango de IPs y el tipo de escaneo a realizar.
			</CardDescription>
		</CardHeader>
		<CardContent class="space-y-6">
			<!-- Rango de IPs -->
			<div class="space-y-2">
				<label for="ipRange" class="text-sm font-medium">
					Rango de IPs (CIDR) <span class="text-red-500">*</span>
				</label>
				<input
					id="ipRange"
					type="text"
					bind:value={ipRange}
					placeholder="192.168.1.0/24"
					class="w-full border rounded-md px-3 py-2 text-sm font-mono"
				/>
				<p class="text-xs text-zinc-500">
					Formato: dirección de red seguida de máscara CIDR (ej: 192.168.1.0/24)
				</p>
			</div>

			<!-- Tipo de Escaneo -->
			<div class="space-y-2">
				<label for="scanType" class="text-sm font-medium"> Tipo de Escaneo </label>
				<div class="grid grid-cols-3 gap-3">
					{#each scanTypes as type}
						<button
							type="button"
							onclick={() => (scanType = type.value)}
							class="p-3 border rounded-lg text-left transition-colors {scanType === type.value
								? 'border-blue-500 bg-blue-50'
								: 'border-zinc-200 hover:border-zinc-300'}"
						>
							<div class="font-medium text-sm">{type.label}</div>
							<div class="text-xs text-zinc-500 mt-1">{type.description}</div>
						</button>
					{/each}
				</div>
			</div>

			<!-- Puertos Personalizados -->
			<div class="space-y-2">
				<label for="customPorts" class="text-sm font-medium">
					Puertos Personalizados
					<span class="text-zinc-400 font-normal">(opcional)</span>
				</label>
				<input
					id="customPorts"
					type="text"
					bind:value={customPorts}
					placeholder="22, 80, 443, 3306, 5432"
					disabled={scanType !== 'targeted'}
					class="w-full border rounded-md px-3 py-2 text-sm font-mono disabled:opacity-50"
				/>
				<p class="text-xs text-zinc-500">
					Separados por coma. Si está vacío, se usan los puertos predeterminados del tipo de
					escaneo.
				</p>
			</div>

			<!-- Timeout y Concurrencia -->
			<div class="grid grid-cols-2 gap-4">
				<div class="space-y-2">
					<label for="timeout" class="text-sm font-medium">
						Timeout (ms)
					</label>
					<input
						id="timeout"
						type="number"
						bind:value={timeoutMs}
						min="100"
						max="10000"
						class="w-full border rounded-md px-3 py-2 text-sm"
					/>
					<p class="text-xs text-zinc-500">Tiempo de espera por conexión (100-10000ms)</p>
				</div>

				<div class="space-y-2">
					<label for="maxConcurrent" class="text-sm font-medium">
						Máx. Conexiones Concurrentes
					</label>
					<input
						id="maxConcurrent"
						type="number"
						bind:value={maxConcurrent}
						min="10"
						max="500"
						class="w-full border rounded-md px-3 py-2 text-sm"
					/>
					<p class="text-xs text-zinc-500">Cantidad máxima de escaneos paralelos</p>
				</div>
			</div>

			<!-- Sede -->
			<div class="space-y-2">
				<label for="sedeId" class="text-sm font-medium">
					Sede
					<span class="text-zinc-400 font-normal">(opcional)</span>
				</label>
				<input
					id="sedeId"
					type="text"
					bind:value={sedeId}
					placeholder="sede-trinidad"
					class="w-full border rounded-md px-3 py-2 text-sm"
				/>
				<p class="text-xs text-zinc-500">
					Identificador de la sede donde se ejecuta el escaneo.
				</p>
			</div>

			<!-- Botones -->
			<div class="flex gap-3 pt-4 border-t">
				<Button
					variant="default"
					onclick={startScan}
					disabled={isSubmitting}
				>
					{#if isSubmitting}
						<Loader2 class="h-4 w-4 mr-1 animate-spin" />
						Iniciando...
					{:else}
						<Play class="h-4 w-4 mr-1" />
						Iniciar Escaneo
					{/if}
				</Button>

				<Button variant="outline" onclick={resetForm}>
					<RotateCcw class="h-4 w-4 mr-1" />
					Limpiar
				</Button>
			</div>
		</CardContent>
	</Card>

	<!-- Progreso (si hay un escaneo activo) -->
	{#if scanId && scanStatus}
		<Card>
			<CardHeader>
				<CardTitle class="text-lg">Estado del Escaneo</CardTitle>
			</CardHeader>
			<CardContent class="space-y-4">
				<div class="flex items-center justify-between">
					<div>
						<span class="text-sm font-medium">ID:</span>
						<code class="ml-2 text-xs bg-zinc-100 px-2 py-1 rounded">{scanId}</code>
					</div>
					<Badge
						variant={scanStatus === 'completed'
							? 'default'
							: scanStatus === 'running'
								? 'secondary'
								: 'outline'}
					>
						{scanStatus}
					</Badge>
				</div>

				{#if scanStatus === 'running'}
					<div class="space-y-2">
						<div class="flex justify-between text-sm">
							<span>Progreso</span>
							<span>{progress.toFixed(1)}%</span>
						</div>
						<div class="w-full bg-zinc-200 rounded-full h-2">
							<div
								class="bg-blue-600 h-2 rounded-full transition-all"
								style="width: {progress}%"
							></div>
						</div>
						<div class="text-sm text-zinc-500">
							{devicesFound} dispositivos encontrados
						</div>
					</div>

					<Button variant="outline" onclick={checkProgress}>
						<Search class="h-4 w-4 mr-1" />
						Verificar Progreso
					</Button>
				{/if}
			</CardContent>
		</Card>
	{/if}

	<!-- Información -->
	<Card>
		<CardHeader>
			<CardTitle class="text-lg">Información</CardTitle>
		</CardHeader>
		<CardContent class="text-sm text-zinc-600 space-y-2">
			<p>
				El escaneo de red descubre dispositivos activos en el rango especificado mediante
				conexiones TCP a puertos comunes.
			</p>
			<ul class="list-disc list-inside space-y-1 text-zinc-500">
				<li>
					<strong>Escaneo Completo:</strong> SSH, Telnet, HTTP, HTTPS, SMB, MySQL, RDP, SNMP,
					MQTT, etc.
				</li>
				<li>
					<strong>Escaneo Parcial:</strong> SSH, Telnet, HTTP, HTTPS, SMB, RDP
				</li>
				<li><strong>Escaneo Dirigido:</strong> Solo SSH (22), HTTP (80), HTTPS (443)</li>
			</ul>
		</CardContent>
	</Card>
</div>
