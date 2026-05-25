<!-- apps/web/src/routes/dashboard/settings/+page.svelte -->
<!-- Configuración de Umbrales del Sistema -->
<!-- Vinculado con ADR-0017-frontend-sveltekit-svelte5.md -->

<script lang="ts">
	import { z } from 'zod';

	const thresholdSchema = z.object({
		ping_ms: z.object({
			warning: z.number().min(1).max(5000),
			critical: z.number().min(1).max(10000)
		}),
		latency_ms: z.object({
			warning: z.number().min(1).max(5000),
			critical: z.number().min(1).max(10000)
		}),
		packet_loss_percent: z.object({
			warning: z.number().min(0.1).max(100),
			critical: z.number().min(0.1).max(100)
		})
	}).refine(data => data.ping_ms.critical > data.ping_ms.warning, {
		message: 'El umbral crítico de ping debe ser mayor al de advertencia',
		path: ['ping_ms', 'critical']
	}).refine(data => data.latency_ms.critical > data.latency_ms.warning, {
		message: 'El umbral crítico de latencia debe ser mayor al de advertencia',
		path: ['latency_ms', 'critical']
	}).refine(data => data.packet_loss_percent.critical > data.packet_loss_percent.warning, {
		message: 'El umbral crítico de pérdida de paquetes debe ser mayor al de advertencia',
		path: ['packet_loss_percent', 'critical']
	});

	type ThresholdForm = z.infer<typeof thresholdSchema>;

	let formData = $state<ThresholdForm>({
		ping_ms: { warning: 100, critical: 500 },
		latency_ms: { warning: 150, critical: 800 },
		packet_loss_percent: { warning: 5, critical: 15 }
	});

	let errors = $state<Record<string, string>>({});
	let isLoading = $state(false);
	let isSaving = $state(false);
	let saveSuccess = $state(false);

	async function fetchThresholds() {
		isLoading = true;
		try {
			const response = await fetch('/api/settings/thresholds');
			if (response.ok) {
				const data = await response.json();
				formData = {
					ping_ms: { warning: data.ping_ms.warning, critical: data.ping_ms.critical },
					latency_ms: { warning: data.latency_ms.warning, critical: data.latency_ms.critical },
					packet_loss_percent: { warning: data.packet_loss_percent.warning, critical: data.packet_loss_percent.critical }
				};
			}
		} catch (error) {
			console.error('Error cargando thresholds:', error);
		} finally {
			isLoading = false;
		}
	}

	function validateForm(): boolean {
		errors = {};
		const result = thresholdSchema.safeParse(formData);
		if (!result.success) {
			for (const issue of result.error.issues) {
				const path = issue.path.join('_');
				errors[path] = issue.message;
			}
			return false;
		}
		return true;
	}

	async function handleSubmit(event: Event) {
		event.preventDefault();
		saveSuccess = false;

		if (!validateForm()) {
			return;
		}

		isSaving = true;
		try {
			const response = await fetch('/api/settings/thresholds', {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(formData)
			});

			if (response.ok) {
				saveSuccess = true;
				setTimeout(() => saveSuccess = false, 3000);
			} else {
				const errorData = await response.json();
				errors.general = errorData.message || 'Error al guardar';
			}
		} catch (error) {
			console.error('Error guardando thresholds:', error);
			errors.general = 'Error de conexión';
		} finally {
			isSaving = false;
		}
	}

	$effect(() => {
		fetchThresholds();
	});
</script>

<div class="min-h-screen bg-gray-100 p-8">
	<div class="max-w-2xl mx-auto">
		<div class="mb-8">
			<h1 class="text-3xl font-bold text-gray-900">Configuración de Umbrales</h1>
			<p class="text-gray-600 mt-2">Configure los límites de alerta para el monitoreo de red</p>
		</div>

		{#if saveSuccess}
			<div class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded mb-6">
				✓ Cambios guardados exitosamente
			</div>
		{/if}

		{#if errors.general}
			<div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded mb-6">
				{errors.general}
			</div>
		{/if}

		{#if isLoading}
			<div class="bg-white rounded-lg shadow-md p-8 text-center">
				<div class="animate-pulse">Cargando configuración...</div>
			</div>
		{:else}
			<form onsubmit={handleSubmit} class="bg-white rounded-lg shadow-md p-6 space-y-8">
				<div class="border-b border-gray-200 pb-6">
					<h2 class="text-xl font-semibold text-gray-800 mb-4">Ping (ms)</h2>
					<div class="grid grid-cols-2 gap-6">
						<div>
							<label for="ping_warning" class="block text-sm font-medium text-gray-700 mb-2">
								Umbral de Advertencia
							</label>
							<input
								id="ping_warning"
								type="number"
								bind:value={formData.ping_ms.warning}
								min="1"
								max="5000"
								class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 {errors.ping_ms_warning ? 'border-red-500' : 'border-gray-300'}"
							/>
							{#if errors.ping_ms_warning}
								<p class="text-red-500 text-sm mt-1">{errors.ping_ms_warning}</p>
							{/if}
						</div>
						<div>
							<label for="ping_critical" class="block text-sm font-medium text-gray-700 mb-2">
								Umbral Crítico
							</label>
							<input
								id="ping_critical"
								type="number"
								bind:value={formData.ping_ms.critical}
								min="1"
								max="10000"
								class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 {errors.ping_ms_critical ? 'border-red-500' : 'border-gray-300'}"
							/>
							{#if errors.ping_ms_critical}
								<p class="text-red-500 text-sm mt-1">{errors.ping_ms_critical}</p>
							{/if}
						</div>
					</div>
				</div>

				<div class="border-b border-gray-200 pb-6">
					<h2 class="text-xl font-semibold text-gray-800 mb-4">Latencia (ms)</h2>
					<div class="grid grid-cols-2 gap-6">
						<div>
							<label for="latency_warning" class="block text-sm font-medium text-gray-700 mb-2">
								Umbral de Advertencia
							</label>
							<input
								id="latency_warning"
								type="number"
								bind:value={formData.latency_ms.warning}
								min="1"
								max="5000"
								class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 {errors.latency_ms_warning ? 'border-red-500' : 'border-gray-300'}"
							/>
						</div>
						<div>
							<label for="latency_critical" class="block text-sm font-medium text-gray-700 mb-2">
								Umbral Crítico
							</label>
							<input
								id="latency_critical"
								type="number"
								bind:value={formData.latency_ms.critical}
								min="1"
								max="10000"
								class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 {errors.latency_ms_critical ? 'border-red-500' : 'border-gray-300'}"
							/>
						</div>
					</div>
				</div>

				<div>
					<h2 class="text-xl font-semibold text-gray-800 mb-4">Pérdida de Paquetes (%)</h2>
					<div class="grid grid-cols-2 gap-6">
						<div>
							<label for="packet_loss_warning" class="block text-sm font-medium text-gray-700 mb-2">
								Umbral de Advertencia
							</label>
							<input
								id="packet_loss_warning"
								type="number"
								step="0.1"
								bind:value={formData.packet_loss_percent.warning}
								min="0.1"
								max="100"
								class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 {errors.packet_loss_percent_warning ? 'border-red-500' : 'border-gray-300'}"
							/>
						</div>
						<div>
							<label for="packet_loss_critical" class="block text-sm font-medium text-gray-700 mb-2">
								Umbral Crítico
							</label>
							<input
								id="packet_loss_critical"
								type="number"
								step="0.1"
								bind:value={formData.packet_loss_percent.critical}
								min="0.1"
								max="100"
								class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 {errors.packet_loss_percent_critical ? 'border-red-500' : 'border-gray-300'}"
							/>
						</div>
					</div>
				</div>

				<div class="flex justify-end">
					<button
						type="submit"
						disabled={isSaving}
						class="bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-6 rounded-md transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
					>
						{isSaving ? 'Guardando...' : 'Guardar Cambios'}
					</button>
				</div>
			</form>
		{/if}
	</div>
</div>