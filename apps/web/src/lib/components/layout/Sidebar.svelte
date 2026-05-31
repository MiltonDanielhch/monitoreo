<!-- apps/web/src/lib/components/layout/Sidebar.svelte -->
<!-- Sidebar de navegación principal con items de menú colapsables -->
<!-- Vinculado con ADR-0017 (Frontend SvelteKit/Svelte 5) -->
<script lang="ts">
	import { page } from '$app/state';
	import { Building2, Monitor, BarChart3, AlertTriangle, ShieldAlert, Settings, ChevronLeft, ChevronRight, Home, Mail } from 'lucide-svelte';

	let collapsed = $state(false);

	const navItems = [
		{ href: '/dashboard', label: 'Inicio', icon: Home, description: 'Panel principal de monitoreo' },
		{ href: '/dashboard/sedes', label: 'Sedes', icon: Building2, description: 'Gestión de sedes regionales' },
		{ href: '/dashboard/dispositivos', label: 'Dispositivos', icon: Monitor, description: 'Monitoreo de dispositivos' },
		{ href: '/dashboard/metricas', label: 'Métricas', icon: BarChart3, description: 'Indicadores de rendimiento' },
		{ href: '/dashboard/alertas', label: 'Alertas', icon: AlertTriangle, badge: 0, description: 'Incidentes críticos' },
		{ href: '/dashboard/notifications', label: 'Notificaciones', icon: Mail, description: 'Historial de alertas enviadas' },
		{ href: '/dashboard/infrastructure', label: 'Infraestructura', icon: Settings, description: 'Archivos técnicos y topologías' },
		{ href: '/dashboard/audit', label: 'Auditoría', icon: ShieldAlert, description: 'Historial inmutable de acciones' },
	];

	const bottomItems = [
		{ href: '/dashboard/notifications/config', label: 'Config. SMTP', icon: Settings, description: 'Configuración de correo' },
		{ href: '/dashboard/settings', label: 'Configuración', icon: Settings, description: 'Ajustes del sistema' },
	];

	function isActive(href: string): boolean {
		if (href === '/dashboard') {
			return page.url.pathname === '/dashboard';
		}
		return page.url.pathname.startsWith(href);
	}
</script>

<aside class="flex flex-col border-r border-zinc-800 bg-zinc-950 text-zinc-50 transition-all duration-300 {collapsed ? 'w-16' : 'w-64'}">
	<div class="flex h-14 items-center border-b border-zinc-800 px-4">
		{#if !collapsed}
			<div class="flex items-center gap-2">
				<div class="flex h-8 w-8 items-center justify-center rounded-lg bg-blue-600 text-white">
					<ShieldAlert class="h-5 w-5" />
				</div>
				<div class="flex flex-col">
					<span class="text-sm font-semibold">Lab 3030</span>
					<span class="text-xs text-zinc-400">Monitoreo</span>
				</div>
			</div>
		{:else}
			<div class="flex h-8 w-8 items-center justify-center rounded-lg bg-blue-600 text-white">
				<ShieldAlert class="h-5 w-5" />
			</div>
		{/if}
	</div>

	<nav class="flex-1 overflow-y-auto p-2">
		<ul class="space-y-1">
			{#each navItems as item}
				{@const active = isActive(item.href)}
				<li>
					<a
						href={item.href}
						class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors {active
							? 'bg-blue-600 text-white'
							: 'text-zinc-300 hover:bg-zinc-800 hover:text-white'} {collapsed ? 'justify-center px-2' : ''}"
						title={item.description ?? item.label}
					>
						<item.icon class="h-5 w-5 shrink-0" />
						{#if !collapsed}
							<span class="flex-1">{item.label}</span>
							{#if item.badge && item.badge > 0}
								<span class="flex h-5 min-w-5 items-center justify-center rounded-md bg-red-600 px-1 text-xs font-medium">
									{item.badge}
								</span>
							{/if}
						{/if}
					</a>
				</li>
			{/each}
		</ul>
	</nav>

	<div class="border-t border-zinc-800 p-2">
		<ul class="space-y-1">
			{#each bottomItems as item}
				{@const active = isActive(item.href)}
				<li>
					<a
						href={item.href}
						class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors {active
							? 'bg-zinc-800 text-white'
							: 'text-zinc-300 hover:bg-zinc-800 hover:text-white'} {collapsed ? 'justify-center px-2' : ''}"
						title={item.description ?? item.label}
					>
						<item.icon class="h-5 w-5 shrink-0" />
						{#if !collapsed}
							<span>{item.label}</span>
						{/if}
					</a>
				</li>
			{/each}
		</ul>

		<button
			onclick={() => collapsed = !collapsed}
			class="mt-2 flex w-full items-center gap-2 rounded-lg px-3 py-2 text-sm text-zinc-400 transition-colors hover:bg-zinc-800 hover:text-white"
		>
			{#if collapsed}
				<ChevronRight class="h-4 w-4" />
			{:else}
				<ChevronLeft class="h-4 w-4" />
				<span>Colapsar</span>
			{/if}
		</button>
	</div>
</aside>