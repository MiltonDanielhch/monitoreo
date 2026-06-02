<!-- apps/web/src/routes/dashboard/security/+page.svelte -->
<!-- Dashboard de Intrusiones - Módulo 11 -->
<!-- Vinculado con ADR-0003 (Stack Backend Rust Axum) -->

<script lang="ts">
  import { onMount } from 'svelte';
  import { browser } from '$app/environment';

  // Tipos para eventos de seguridad
  interface SecurityEvent {
    id: string;
    event_type: string;
    severity: string;
    status: string;
    source_ip: string;
    source_mac: string | null;
    target_device_id: string | null;
    target_sede_id: string | null;
    description: string;
    metadata: any;
    detected_at: string;
    resolved_at: string | null;
    resolved_by: string | null;
  }

  // Estado
  let events: SecurityEvent[] = [];
  let loading = true;
  let error: string | null = null;

  // Filtros
  let severityFilter = '';
  let statusFilter = '';
  let dateFrom = '';
  let dateTo = '';

  // Evento seleccionado
  let selectedEvent: SecurityEvent | null = null;

  // Cargar eventos de seguridad
  async function loadEvents() {
    loading = true;
    error = null;

    try {
      const params = new URLSearchParams();
      if (severityFilter) params.append('severity', severityFilter);
      if (statusFilter) params.append('status', statusFilter);
      if (dateFrom) params.append('date_from', dateFrom);
      if (dateTo) params.append('date_to', dateTo);

      const response = await fetch(`/api/v1/security/events?${params.toString()}`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        throw new Error('Error al cargar eventos de seguridad');
      }

      const data = await response.json();
      events = data.events || [];
    } catch (e) {
      error = e instanceof Error ? e.message : 'Error desconocido';
    } finally {
      loading = false;
    }
  }

  // Resolver evento
  async function resolveEvent(eventId: string) {
    try {
      const response = await fetch(`/api/v1/security/events/${eventId}/resolve`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ resolved_by: 'admin' }),
      });

      if (!response.ok) {
        throw new Error('Error al resolver evento');
      }

      await loadEvents();
    } catch (e) {
      alert(e instanceof Error ? e.message : 'Error desconocido');
    }
  }

  // Marcar como falso positivo
  async function markFalsePositive(eventId: string) {
    try {
      const response = await fetch(`/api/v1/security/events/${eventId}/false-positive`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        throw new Error('Error al marcar falso positivo');
      }

      await loadEvents();
    } catch (e) {
      alert(e instanceof Error ? e.message : 'Error desconocido');
    }
  }

  // Obtener clase de severidad
  function getSeverityClass(severity: string): string {
    switch (severity) {
      case 'Critical':
        return 'bg-red-100 text-red-800 border-red-200';
      case 'High':
        return 'bg-orange-100 text-orange-800 border-orange-200';
      case 'Medium':
        return 'bg-yellow-100 text-yellow-800 border-yellow-200';
      case 'Low':
        return 'bg-green-100 text-green-800 border-green-200';
      default:
        return 'bg-gray-100 text-gray-800 border-gray-200';
    }
  }

  // Obtener clase de estado
  function getStatusClass(status: string): string {
    switch (status) {
      case 'Detected':
        return 'bg-blue-100 text-blue-800 border-blue-200';
      case 'Investigating':
        return 'bg-purple-100 text-purple-800 border-purple-200';
      case 'Resolved':
        return 'bg-green-100 text-green-800 border-green-200';
      case 'FalsePositive':
        return 'bg-gray-100 text-gray-800 border-gray-200';
      default:
        return 'bg-gray-100 text-gray-800 border-gray-200';
    }
  }

  // Formatear fecha
  function formatDate(dateStr: string): string {
    return new Date(dateStr).toLocaleString('es-ES');
  }

  // Cargar al montar
  onMount(() => {
    if (browser) {
      loadEvents();
    }
  });
</script>

<div class="p-6">
  <div class="mb-6">
    <h1 class="text-3xl font-bold text-gray-900 mb-2">Dashboard de Intrusiones</h1>
    <p class="text-gray-600">Monitoreo y gestión de eventos de seguridad</p>
  </div>

  <!-- Filtros -->
  <div class="bg-white rounded-lg shadow p-4 mb-6">
    <h2 class="text-lg font-semibold mb-4">Filtros</h2>
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Severidad</label>
        <select
          bind:value={severityFilter}
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="">Todas</option>
          <option value="Critical">Crítica</option>
          <option value="High">Alta</option>
          <option value="Medium">Media</option>
          <option value="Low">Baja</option>
        </select>
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Estado</label>
        <select
          bind:value={statusFilter}
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="">Todos</option>
          <option value="Detected">Detectado</option>
          <option value="Investigating">Investigando</option>
          <option value="Resolved">Resuelto</option>
          <option value="FalsePositive">Falso Positivo</option>
        </select>
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Fecha Desde</label>
        <input
          type="datetime-local"
          bind:value={dateFrom}
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Fecha Hasta</label>
        <input
          type="datetime-local"
          bind:value={dateTo}
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
    </div>
    <div class="mt-4">
      <button
        onclick={loadEvents}
        class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        Aplicar Filtros
      </button>
      <button
        onclick={() => {
          severityFilter = '';
          statusFilter = '';
          dateFrom = '';
          dateTo = '';
          loadEvents();
        }}
        class="ml-2 px-4 py-2 bg-gray-200 text-gray-800 rounded-md hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-gray-500"
      >
        Limpiar Filtros
      </button>
    </div>
  </div>

  <!-- Estado de carga -->
  {#if loading}
    <div class="text-center py-8">
      <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
      <p class="mt-4 text-gray-600">Cargando eventos de seguridad...</p>
    </div>
  {:else if error}
    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
      {error}
    </div>
  {:else}
    <!-- Lista de eventos -->
    <div class="bg-white rounded-lg shadow overflow-hidden">
      <div class="px-6 py-4 border-b border-gray-200">
        <h2 class="text-lg font-semibold">
          Eventos de Seguridad ({events.length})
        </h2>
      </div>
      {#if events.length === 0}
        <div class="p-6 text-center text-gray-500">
          No hay eventos de seguridad que coincidan con los filtros
        </div>
      {:else}
        <div class="overflow-x-auto">
          <table class="min-w-full divide-y divide-gray-200">
            <thead class="bg-gray-50">
              <tr>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  ID
                </th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Tipo
                </th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Severidad
                </th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Estado
                </th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  IP Origen
                </th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Detectado
                </th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Acciones
                </th>
              </tr>
            </thead>
            <tbody class="bg-white divide-y divide-gray-200">
              {#each events as event}
                <tr class="hover:bg-gray-50">
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                    {event.id}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                    {event.event_type}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span
                      class="px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full border {getSeverityClass(event.severity)}"
                    >
                      {event.severity}
                    </span>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span
                      class="px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full border {getStatusClass(event.status)}"
                    >
                      {event.status}
                    </span>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                    {event.source_ip}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                    {formatDate(event.detected_at)}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
                    <button
                      onclick={() => (selectedEvent = event)}
                      class="text-blue-600 hover:text-blue-900 mr-3"
                    >
                      Ver Detalles
                    </button>
                    {#if event.status === 'Detected' || event.status === 'Investigating'}
                      <button
                        onclick={() => resolveEvent(event.id)}
                        class="text-green-600 hover:text-green-900 mr-3"
                      >
                        Resolver
                      </button>
                      <button
                        onclick={() => markFalsePositive(event.id)}
                        class="text-gray-600 hover:text-gray-900"
                      >
                        Falso Positivo
                      </button>
                    {/if}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    </div>
  {/if}

  <!-- Modal de detalles -->
  {#if selectedEvent}
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-screen overflow-y-auto">
        <div class="px-6 py-4 border-b border-gray-200 flex justify-between items-center">
          <h3 class="text-lg font-semibold">Detalles del Evento</h3>
          <button
            onclick={() => (selectedEvent = null)}
            class="text-gray-400 hover:text-gray-600"
          >
            ✕
          </button>
        </div>
        <div class="px-6 py-4">
          <div class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700">ID</label>
              <p class="mt-1 text-sm text-gray-900">{selectedEvent.id}</p>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700">Tipo de Intrusión</label>
              <p class="mt-1 text-sm text-gray-900">{selectedEvent.event_type}</p>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700">Severidad</label>
              <p class="mt-1 text-sm text-gray-900">{selectedEvent.severity}</p>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700">Estado</label>
              <p class="mt-1 text-sm text-gray-900">{selectedEvent.status}</p>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700">IP Origen</label>
              <p class="mt-1 text-sm text-gray-900">{selectedEvent.source_ip}</p>
            </div>
            {#if selectedEvent.source_mac}
              <div>
                <label class="block text-sm font-medium text-gray-700">MAC Origen</label>
                <p class="mt-1 text-sm text-gray-900">{selectedEvent.source_mac}</p>
              </div>
            {/if}
            {#if selectedEvent.target_device_id}
              <div>
                <label class="block text-sm font-medium text-gray-700">Dispositivo Objetivo</label>
                <p class="mt-1 text-sm text-gray-900">{selectedEvent.target_device_id}</p>
              </div>
            {/if}
            {#if selectedEvent.target_sede_id}
              <div>
                <label class="block text-sm font-medium text-gray-700">Sede Objetivo</label>
                <p class="mt-1 text-sm text-gray-900">{selectedEvent.target_sede_id}</p>
              </div>
            {/if}
            <div>
              <label class="block text-sm font-medium text-gray-700">Descripción</label>
              <p class="mt-1 text-sm text-gray-900">{selectedEvent.description}</p>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700">Detectado</label>
              <p class="mt-1 text-sm text-gray-900">{formatDate(selectedEvent.detected_at)}</p>
            </div>
            {#if selectedEvent.resolved_at}
              <div>
                <label class="block text-sm font-medium text-gray-700">Resuelto</label>
                <p class="mt-1 text-sm text-gray-900">{formatDate(selectedEvent.resolved_at)}</p>
              </div>
            {/if}
            {#if selectedEvent.resolved_by}
              <div>
                <label class="block text-sm font-medium text-gray-700">Resuelto Por</label>
                <p class="mt-1 text-sm text-gray-900">{selectedEvent.resolved_by}</p>
              </div>
            {/if}
          </div>
        </div>
        <div class="px-6 py-4 border-t border-gray-200 flex justify-end space-x-3">
          <button
            onclick={() => (selectedEvent = null)}
            class="px-4 py-2 bg-gray-200 text-gray-800 rounded-md hover:bg-gray-300"
          >
            Cerrar
          </button>
          {#if selectedEvent.status === 'Detected' || selectedEvent.status === 'Investigating'}
            <button
              onclick={() => {
                resolveEvent(selectedEvent.id);
                selectedEvent = null;
              }}
              class="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700"
            >
              Resolver
            </button>
            <button
              onclick={() => {
                markFalsePositive(selectedEvent.id);
                selectedEvent = null;
              }}
              class="px-4 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-700"
            >
              Falso Positivo
            </button>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>
