<script lang="ts">
	let backendStatus = $state<'loading' | 'connected' | 'disconnected'>('loading');
	let dbStatus = $state<'loading' | 'connected' | 'disconnected'>('loading');

	async function checkHealth() {
		try {
			const response = await fetch('http://localhost:3000/api/health');
			const data = await response.json();
			
			if (data.status === 'OK') {
				backendStatus = 'connected';
			} else {
				backendStatus = 'disconnected';
			}
			
			if (data.database === 'Conectada') {
				dbStatus = 'connected';
			} else {
				dbStatus = 'disconnected';
			}
		} catch (error) {
			backendStatus = 'disconnected';
			dbStatus = 'disconnected';
		}
	}

	checkHealth();
	setInterval(checkHealth, 5000);
</script>

<div class="min-h-screen bg-gray-100 flex items-center justify-center p-4">
	<div class="max-w-4xl w-full">
		<h1 class="text-3xl font-bold text-center mb-8">Sistema de Monitoreo Regional</h1>
		
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<!-- Backend Status Card -->
			<div class="bg-white rounded-lg shadow-md p-6">
				<h2 class="text-xl font-semibold mb-4">Backend API</h2>
				<div class="flex items-center">
					{#if backendStatus === 'loading'}
						<div class="w-4 h-4 bg-yellow-400 rounded-full animate-pulse mr-3"></div>
						<span class="text-gray-600">Verificando...</span>
					{:else if backendStatus === 'connected'}
						<div class="w-4 h-4 bg-green-500 rounded-full mr-3"></div>
						<span class="text-green-600 font-semibold">Conectado</span>
					{:else}
						<div class="w-4 h-4 bg-red-500 rounded-full mr-3"></div>
						<span class="text-red-600 font-semibold">Desconectado</span>
					{/if}
				</div>
			</div>

			<!-- Database Status Card -->
			<div class="bg-white rounded-lg shadow-md p-6">
				<h2 class="text-xl font-semibold mb-4">Base de Datos MySQL</h2>
				<div class="flex items-center">
					{#if dbStatus === 'loading'}
						<div class="w-4 h-4 bg-yellow-400 rounded-full animate-pulse mr-3"></div>
						<span class="text-gray-600">Verificando...</span>
					{:else if dbStatus === 'connected'}
						<div class="w-4 h-4 bg-green-500 rounded-full mr-3"></div>
						<span class="text-green-600 font-semibold">Conectada</span>
					{:else}
						<div class="w-4 h-4 bg-red-500 rounded-full mr-3"></div>
						<span class="text-red-600 font-semibold">Desconectada</span>
					{/if}
				</div>
			</div>
		</div>

		<p class="text-center text-gray-500 mt-8 text-sm">
			Actualizando cada 5 segundos...
		</p>
	</div>
</div>
