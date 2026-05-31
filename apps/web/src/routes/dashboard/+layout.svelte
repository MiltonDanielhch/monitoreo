<!-- apps/web/src/routes/dashboard/+layout.svelte -->
<!-- Layout principal del dashboard con sidebar -->
<!-- Vinculado con ADR-0017-frontend-sveltekit-svelte5.md -->

<script lang="ts">
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
</script>

<div class="flex h-screen bg-zinc-100">
	<Sidebar />
	<main class="flex-1 overflow-auto">
		<slot />
	</main>
</div>