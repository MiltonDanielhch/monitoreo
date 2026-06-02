<!-- apps/web/src/routes/dashboard/reports/+page.svelte -->
<!-- UI de Selección de Periodos y Previsualización de Reportes SLA -->
<!-- Vinculado con ADR-0017 (Frontend Svelte 5 + TanStack Query) -->

<script lang="ts">
  import { onMount } from 'svelte';
  import { Download, FileText, AlertTriangle, CheckCircle, Clock } from 'lucide-svelte';

  // Estado reactivo
  let selectedMonth = $state(new Date().toISOString().slice(5, 7));
  let selectedYear = $state(new Date().getFullYear());
  let slaData = $state<any[]>([]);
  let loading = $state(false);
  let error = $state('');

  // Meses disponibles
  const months = [
    { value: '01', label: 'Enero' },
    { value: '02', label: 'Febrero' },
    { value: '03', label: 'Marzo' },
    { value: '04', label: 'Abril' },
    { value: '05', label: 'Mayo' },
    { value: '06', label: 'Junio' },
    { value: '07', label: 'Julio' },
    { value: '08', label: 'Agosto' },
    { value: '09', label: 'Septiembre' },
    { value: '10', label: 'Octubre' },
    { value: '11', label: 'Noviembre' },
    { value: '12', label: 'Diciembre' },
  ];

  // Años disponibles (últimos 5 años)
  const years = Array.from({ length: 5 }, (_, i) => new Date().getFullYear() - i);

  // Cargar datos de SLA
  async function loadSlaData() {
    loading = true;
    error = '';
    try {
      const response = await fetch(
        `/api/v1/reports/sla/summary?sede_id=all&month=${selectedMonth}&year=${selectedYear}`
      );
      if (!response.ok) throw new Error('Error al obtener resumen SLA');
      slaData = await response.json();
    } catch (e) {
      error = 'Error al cargar datos de SLA';
      console.error(e);
    } finally {
      loading = false;
    }
  }

  // Generar reporte PDF
  async function generateReport(sedeId: string) {
    try {
      const response = await fetch(
        `/api/v1/reports/sla/generate?sede_id=${sedeId}&month=${selectedMonth}&year=${selectedYear}`
      );
      if (!response.ok) throw new Error('Error al generar reporte');
      const data = await response.json();
      alert(`Reporte generado: ${data.report_id}`);
    } catch (e) {
      alert('Error al generar reporte');
      console.error(e);
    }
  }

  // Descargar reporte PDF
  async function downloadReport(sedeId: string) {
    try {
      const response = await fetch(
        `/api/v1/reports/sla/download?sede_id=${sedeId}&month=${selectedMonth}&year=${selectedYear}`
      );
      if (!response.ok) throw new Error('Error al descargar reporte');
      const blob = await response.blob();
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `sla_report_${sedeId}_${selectedMonth}_${selectedYear}.pdf`;
      a.click();
      window.URL.revokeObjectURL(url);
    } catch (e) {
      alert('Error al descargar reporte');
      console.error(e);
    }
  }

  // Obtener color de estado
  function getStatusColor(status: string) {
    switch (status) {
      case 'Compliant': return 'bg-green-500';
      case 'Breached': return 'bg-red-500';
      case 'AtRisk': return 'bg-yellow-500';
      default: return 'bg-gray-500';
    }
  }

  // Obtener icono de estado
  function getStatusIcon(status: string) {
    switch (status) {
      case 'Compliant': return CheckCircle;
      case 'Breached': return AlertTriangle;
      case 'AtRisk': return Clock;
      default: return FileText;
    }
  }

  // Cargar datos al cambiar selección
  $effect(() => {
    loadSlaData();
  });

  // Cargar datos al montar
  onMount(() => {
    loadSlaData();
  });
</script>

<div class="p-6 space-y-6">
  <!-- Header -->
  <div class="flex justify-between items-center">
    <div>
      <h1 class="text-3xl font-bold text-white">Reportes SLA</h1>
      <p class="text-gray-400">Generación y descarga de reportes de Service Level Agreement</p>
    </div>
  </div>

  <!-- Filtros de Periodo -->
  <div class="bg-zinc-900 border border-zinc-800 rounded-lg p-6">
    <h2 class="text-xl font-semibold text-white mb-4">Selección de Periodo</h2>
    <p class="text-gray-400 mb-4">Selecciona el mes y año para generar los reportes</p>
    <div class="grid grid-cols-2 gap-4">
      <div class="space-y-2">
        <label class="text-gray-300 block">Mes</label>
        <select 
          bind:value={selectedMonth} 
          class="w-full bg-zinc-800 border border-zinc-700 text-white rounded px-3 py-2"
        >
          {#each months as month}
            <option value={month.value}>{month.label}</option>
          {/each}
        </select>
      </div>
      <div class="space-y-2">
        <label class="text-gray-300 block">Año</label>
        <select 
          bind:value={selectedYear} 
          class="w-full bg-zinc-800 border border-zinc-700 text-white rounded px-3 py-2"
        >
          {#each years as year}
            <option value={year}>{year}</option>
          {/each}
        </select>
      </div>
    </div>
  </div>

  <!-- Estado de carga -->
  {#if loading}
    <div class="text-center py-8 text-gray-400">Cargando reportes...</div>
  {:else if error}
    <div class="text-center py-8 text-red-400">{error}</div>
  {/if}

  <!-- Lista de Reportes -->
  {#if !loading && !error && slaData.length > 0}
    <div class="bg-zinc-900 border border-zinc-800 rounded-lg p-6">
      <h2 class="text-xl font-semibold text-white mb-4">Reportes por Sede</h2>
      <p class="text-gray-400 mb-4">Resumen de SLA para todas las sedes del periodo seleccionado</p>
      
      <div class="space-y-3">
        {#each slaData as report}
          <div class="flex items-center justify-between p-4 bg-zinc-800 rounded-lg border border-zinc-700 hover:border-zinc-600 transition-colors">
            <div class="flex items-center gap-4">
              <div class="p-2 rounded-lg {getStatusColor(report.status)} bg-opacity-20">
                <svelte:component this={getStatusIcon(report.status)} class="text-white" />
              </div>
              <div>
                <h4 class="text-white font-semibold">{report.sede_name}</h4>
                <p class="text-gray-400 text-sm">SLA: {report.availability_achieved.toFixed(2)}%</p>
              </div>
            </div>
            <div class="flex items-center gap-3">
              <span class="px-2 py-1 rounded text-sm font-medium {report.status === 'Compliant' ? 'bg-green-500 text-white' : report.status === 'Breached' ? 'bg-red-500 text-white' : 'bg-yellow-500 text-white'}">
                {report.status}
              </span>
              <button 
                onclick={() => generateReport(report.sede_id)}
                class="px-3 py-1 border border-zinc-700 text-white rounded hover:bg-zinc-700 flex items-center gap-2"
              >
                <FileText class="w-4 h-4" />
                Generar
              </button>
              <button 
                onclick={() => downloadReport(report.sede_id)}
                class="px-3 py-1 border border-zinc-700 text-white rounded hover:bg-zinc-700 flex items-center gap-2"
              >
                <Download class="w-4 h-4" />
                Descargar
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>
  {:else if !loading && !error && slaData.length === 0}
    <div class="text-center py-8 text-gray-400">No hay datos de SLA para el periodo seleccionado</div>
  {/if}
</div>
