<script lang="ts">
  import { onMount } from 'svelte';
  import { createQuery } from '@tanstack/svelte-query';

  // Tipos de respuesta del API
  interface WorkerStatsResponse {
    ping_success: number;
    ping_failures: number;
    ping_avg_latency: number;
    snmp_success: number;
    snmp_failures: number;
    devices_discovered: number;
    pruning_records_purged: number;
    last_pruning_run: number;
    ping_success_rate: number;
    snmp_success_rate: number;
  }

  // Estado reactivo con Svelte 5 Runes
  let stats = $state<WorkerStatsResponse | null>(null);
  let isLoading = $state(true);
  let error = $state<string | null>(null);

  // Query de TanStack para obtener estadísticas de workers
  const workerStatsQuery = createQuery({
    queryKey: ['worker-stats'],
    queryFn: async () => {
      const response = await fetch('/api/workers/stats');
      if (!response.ok) {
        throw new Error('Error al obtener estadísticas de workers');
      }
      return response.json() as Promise<WorkerStatsResponse>;
    },
    refetchInterval: 5000, // Refrescar cada 5 segundos
  });

  // Actualizar estado cuando cambia el query
  $: {
    if (workerStatsQuery.data) {
      stats = workerStatsQuery.data;
      isLoading = false;
      error = null;
    }
    if (workerStatsQuery.error) {
      error = 'Error al cargar estadísticas';
      isLoading = false;
    }
  }

  // Formatear timestamp a fecha legible
  function formatTimestamp(timestamp: number): string {
    if (timestamp === 0) return 'Nunca';
    const date = new Date(timestamp * 1000);
    return date.toLocaleString('es-BO', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
    });
  }

  // Formatear bytes a MB
  function formatBytes(bytes: number): string {
    const mb = bytes / (1024 * 1024);
    return `${mb.toFixed(2)} MB`;
  }
</script>

<div class="workers-dashboard">
  <div class="dashboard-header">
    <h1>📊 Dashboard de Monitoreo de Colas</h1>
    <p>Estado en tiempo real de los workers Tokio MPSC</p>
  </div>

  {#if isLoading}
    <div class="loading-state">
      <div class="spinner"></div>
      <p>Cargando estadísticas...</p>
    </div>
  {:else if error}
    <div class="error-state">
      <p>{error}</p>
    </div>
  {:else if stats}
    <div class="stats-grid">
      <!-- Sondeo de Red -->
      <div class="stat-card ping-card">
        <div class="stat-header">
          <h2>🌐 Sondeo de Red</h2>
          <span class="status-badge success">Activo</span>
        </div>
        <div class="stat-content">
          <p class="stat-description">
            Workers Tokio procesaron exitosamente <strong>{stats.ping_success} tareas de Ping</strong>.
          </p>
          <p class="stat-metric">
            Tiempo de respuesta promedio: <strong>{stats.ping_avg_latency}ms</strong>
          </p>
          <div class="stat-details">
            <div class="detail-item">
              <span class="detail-label">Éxitos:</span>
              <span class="detail-value success">{stats.ping_success}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">Fallos:</span>
              <span class="detail-value error">{stats.ping_failures}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">Tasa de éxito:</span>
              <span class="detail-value">{stats.ping_success_rate.toFixed(1)}%</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Ciclo SNMPv3 -->
      <div class="stat-card snmp-card">
        <div class="stat-header">
          <h2>🔌 Ciclo SNMPv3</h2>
          <span class="status-badge success">Activo</span>
        </div>
        <div class="stat-content">
          <p class="stat-description">
            Barrido completado. Descubiertos <strong>{stats.devices_discovered} nuevos dispositivos</strong>.
          </p>
          <div class="stat-details">
            <div class="detail-item">
              <span class="detail-label">Éxitos:</span>
              <span class="detail-value success">{stats.snmp_success}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">Fallos:</span>
              <span class="detail-value error">{stats.snmp_failures}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">Tasa de éxito:</span>
              <span class="detail-value">{stats.snmp_success_rate.toFixed(1)}%</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Mantenimiento de Almacenamiento -->
      <div class="stat-card pruning-card">
        <div class="stat-header">
          <h2>⚙️ Mantenimiento de Almacenamiento</h2>
          <span class="status-badge info">Programado</span>
        </div>
        <div class="stat-content">
          <p class="stat-description">
            Depuración ejecutada a las <strong>{formatTimestamp(stats.last_pruning_run)}</strong>.
          </p>
          <p class="stat-metric">
            Purgados <strong>{stats.pruning_records_purged.toLocaleString()} registros obsoletos</strong>.
          </p>
          <div class="stat-details">
            <div class="detail-item">
              <span class="detail-label">Espacio liberado:</span>
              <span class="detail-value">{formatBytes(stats.pruning_records_purged * 100)}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Gráfico de rendimiento -->
    <div class="performance-chart">
      <h3>📈 Rendimiento por Minuto</h3>
      <div class="chart-placeholder">
        <p>Gráfico de rendimiento en tiempo real</p>
        <p class="chart-note">Implementación pendiente con SVG o Chart.js</p>
      </div>
    </div>
  {/if}
</div>

<style>
  .workers-dashboard {
    padding: 2rem;
    max-width: 1400px;
    margin: 0 auto;
  }

  .dashboard-header {
    margin-bottom: 2rem;
  }

  .dashboard-header h1 {
    font-size: 2rem;
    font-weight: 700;
    margin-bottom: 0.5rem;
    color: #1a1a1a;
  }

  .dashboard-header p {
    color: #666;
    font-size: 1rem;
  }

  .loading-state,
  .error-state {
    text-align: center;
    padding: 4rem;
    color: #666;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #f3f3f3;
    border-top: 4px solid #3498db;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin: 0 auto 1rem;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
    gap: 1.5rem;
    margin-bottom: 2rem;
  }

  .stat-card {
    background: white;
    border-radius: 12px;
    padding: 1.5rem;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    border-left: 4px solid #3498db;
  }

  .ping-card {
    border-left-color: #3498db;
  }

  .snmp-card {
    border-left-color: #2ecc71;
  }

  .pruning-card {
    border-left-color: #9b59b6;
  }

  .stat-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .stat-header h2 {
    font-size: 1.25rem;
    font-weight: 600;
    margin: 0;
  }

  .status-badge {
    padding: 0.25rem 0.75rem;
    border-radius: 9999px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .status-badge.success {
    background: #d4edda;
    color: #155724;
  }

  .status-badge.info {
    background: #d1ecf1;
    color: #0c5460;
  }

  .stat-content {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .stat-description {
    color: #333;
    font-size: 0.95rem;
    line-height: 1.5;
  }

  .stat-description strong {
    color: #1a1a1a;
    font-weight: 600;
  }

  .stat-metric {
    color: #666;
    font-size: 0.9rem;
  }

  .stat-metric strong {
    color: #1a1a1a;
    font-weight: 600;
  }

  .stat-details {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding-top: 0.75rem;
    border-top: 1px solid #e0e0e0;
  }

  .detail-item {
    display: flex;
    justify-content: space-between;
    font-size: 0.875rem;
  }

  .detail-label {
    color: #666;
  }

  .detail-value {
    font-weight: 600;
    color: #1a1a1a;
  }

  .detail-value.success {
    color: #2ecc71;
  }

  .detail-value.error {
    color: #e74c3c;
  }

  .performance-chart {
    background: white;
    border-radius: 12px;
    padding: 1.5rem;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .performance-chart h3 {
    font-size: 1.25rem;
    font-weight: 600;
    margin-bottom: 1rem;
  }

  .chart-placeholder {
    height: 300px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: #f8f9fa;
    border-radius: 8px;
    border: 2px dashed #dee2e6;
  }

  .chart-placeholder p {
    color: #666;
    margin: 0.25rem 0;
  }

  .chart-note {
    font-size: 0.875rem;
    color: #999;
  }
</style>
