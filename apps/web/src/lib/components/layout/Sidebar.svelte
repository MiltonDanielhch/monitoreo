<!-- apps/web/src/lib/components/layout/Sidebar.svelte -->
<!-- Sidebar de navegación principal con items de menú colapsables -->
<!-- Vinculado con ADR-0017 (Frontend SvelteKit/Svelte 5) -->
<script lang="ts">
	import { page } from '$app/state';
	import { Building2, Monitor, BarChart3, AlertTriangle, ShieldAlert, Settings, ChevronLeft, ChevronRight, Home, Mail, Server, Cpu, FileText, ChevronDown, ChevronUp, Layers, Activity, Lock, FileText as ReportIcon, Search } from 'lucide-svelte';

	let collapsed = $state(false);

	// Grupos de navegación
	const navGroups = [
		{
			label: 'Monitoreo',
			icon: Activity,
			items: [
				{ href: '/dashboard', label: 'Inicio', icon: Home, description: 'Panel principal de monitoreo' },
				{ href: '/dashboard/sedes', label: 'Sedes', icon: Building2, description: 'Gestión de sedes regionales' },
				{ href: '/dashboard/dispositivos', label: 'Dispositivos', icon: Monitor, description: 'Monitoreo de dispositivos' },
				{ href: '/dashboard/metricas', label: 'Métricas', icon: BarChart3, description: 'Indicadores de rendimiento' },
				{ href: '/dashboard/alertas', label: 'Alertas', icon: AlertTriangle, badge: 0, description: 'Incidentes críticos' },
				{ href: '/dashboard/agents', label: 'Agentes', icon: Server, description: 'Conectividad de agentes remotos' },
			]
		},
		{
			label: 'Descubrimiento',
			icon: Search,
			items: [
				{ href: '/dashboard/discovery', label: 'Dispositivos', icon: Monitor, description: 'Dispositivos descubiertos en la red' },
				{ href: '/dashboard/discovery/scan', label: 'Nuevo Escaneo', icon: Search, description: 'Iniciar escaneo de red' },
			]
		},
		{
			label: 'Sistema',
			icon: Layers,
			items: [
				{ href: '/dashboard/workers', label: 'Workers', icon: Cpu, description: 'Monitoreo de tareas en segundo plano' },
				{ href: '/dashboard/jobs/settings', label: 'Config. Workers', icon: Settings, description: 'Configuración de workers' },
				{ href: '/dashboard/infrastructure', label: 'Infraestructura', icon: Settings, description: 'Archivos técnicos y topologías' },
				{ href: '/dashboard/notifications', label: 'Notificaciones', icon: Mail, description: 'Historial de alertas enviadas' },
			]
		},
		{
			label: 'Seguridad',
			icon: Lock,
			items: [
				{ href: '/dashboard/security', label: 'Seguridad', icon: ShieldAlert, description: 'Detección de intrusiones' },
				{ href: '/dashboard/audit', label: 'Auditoría', icon: ShieldAlert, description: 'Historial inmutable de acciones' },
			]
		},
		{
			label: 'Reportes',
			icon: ReportIcon,
			items: [
				{ href: '/dashboard/reports', label: 'Reportes', icon: FileText, description: 'Generación de reportes SLA' },
			]
		}
	];

	// Estado de grupos colapsados
	let groupCollapsed = $state<Record<string, boolean>>({});

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
			{#each navGroups as group}
				<li>
					{#if !collapsed}
						<button
							onclick={() => groupCollapsed[group.label] = !groupCollapsed[group.label]}
							class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium text-zinc-400 transition-colors hover:bg-zinc-800 hover:text-white"
						>
							<group.icon class="h-4 w-4" />
							<span class="flex-1">{group.label}</span>
							{#if groupCollapsed[group.label]}
								<ChevronDown class="h-4 w-4" />
							{:else}
								<ChevronUp class="h-4 w-4" />
							{/if}
						</button>
						{#if !groupCollapsed[group.label]}
							<ul class="mt-1 space-y-1 pl-4">
								{#each group.items as item}
									{@const active = isActive(item.href)}
									<li>
										<a
											href={item.href}
											class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors {active
												? 'bg-blue-600 text-white'
												: 'text-zinc-300 hover:bg-zinc-800 hover:text-white'}"
											title={item.description ?? item.label}
										>
											<item.icon class="h-4 w-4 shrink-0" />
											<span class="flex-1">{item.label}</span>
											{#if item.badge && item.badge > 0}
												<span class="flex h-5 min-w-5 items-center justify-center rounded-md bg-red-600 px-1 text-xs font-medium">
													{item.badge}
												</span>
											{/if}
										</a>
									</li>
								{/each}
							</ul>
						{/if}
					{:else}
						<!-- Modo colapsado: mostrar solo iconos de grupos -->
						<div class="flex flex-col gap-1 py-2">
							{#each group.items as item}
								{@const active = isActive(item.href)}
								<a
									href={item.href}
									class="flex items-center justify-center rounded-lg px-2 py-2 text-sm transition-colors {active
										? 'bg-blue-600 text-white'
										: 'text-zinc-300 hover:bg-zinc-800 hover:text-white'}"
									title={item.label}
								>
									<item.icon class="h-5 w-5 shrink-0" />
								</a>
							{/each}
						</div>
					{/if}
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