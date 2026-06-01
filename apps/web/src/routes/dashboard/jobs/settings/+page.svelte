<script lang="ts">
  import { onMount } from 'svelte';

  // Esquema de validación con Zod
  // Nota: Zod no está instalado, usaremos validación manual por ahora
  interface WorkerConfig {
    retention_days: number;
    ping_interval_seconds: number;
    snmp_interval_seconds: number;
    ping_max_retries: number;
    pruning_hour: number;
  }

  // Estado reactivo con Svelte 5 Runes
  let config = $state<WorkerConfig>({
    retention_days: 90,
    ping_interval_seconds: 30,
    snmp_interval_seconds: 21600,
    ping_max_retries: 3,
    pruning_hour: 2,
  });

  let isLoading = $state(false);
  let isSaving = $state(false);
  let error = $state<string | null>(null);
  let success = $state(false);

  // Función para cargar configuración
  async function loadConfig() {
    try {
      isLoading = true;
      error = null;
      const response = await fetch('/api/workers/config');
      if (!response.ok) {
        throw new Error('Error al obtener configuración');
      }
      config = await response.json();
    } catch (e) {
      error = 'Error al cargar configuración';
      console.error(e);
    } finally {
      isLoading = false;
    }
  }

  // Función para validar configuración
  function validateConfig(): string | null {
    if (config.retention_days < 1 || config.retention_days > 365) {
      return 'Días de retención deben estar entre 1 y 365';
    }
    if (config.ping_interval_seconds < 10 || config.ping_interval_seconds > 300) {
      return 'Intervalo de ping debe estar entre 10 y 300 segundos';
    }
    if (config.snmp_interval_seconds < 3600 || config.snmp_interval_seconds > 86400) {
      return 'Intervalo SNMP debe estar entre 3600 y 86400 segundos';
    }
    if (config.ping_max_retries < 1 || config.ping_max_retries > 10) {
      return 'Máximo de reintentos debe estar entre 1 y 10';
    }
    if (config.pruning_hour < 0 || config.pruning_hour > 23) {
      return 'Hora de pruning debe estar entre 0 y 23';
    }
    return null;
  }

  // Función para guardar configuración
  async function saveConfig(event: Event) {
    event.preventDefault();

    const validationError = validateConfig();
    if (validationError) {
      error = validationError;
      success = false;
      return;
    }

    try {
      isSaving = true;
      error = null;
      success = false;

      const response = await fetch('/api/workers/config', {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(config),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || 'Error al guardar configuración');
      }

      const updatedConfig = await response.json();
      config = updatedConfig;
      success = true;

      // Ocultar mensaje de éxito después de 3 segundos
      setTimeout(() => {
        success = false;
      }, 3000);
    } catch (e) {
      error = e instanceof Error ? e.message : 'Error al guardar configuración';
      console.error(e);
    } finally {
      isSaving = false;
    }
  }

  // Cargar configuración al montar
  onMount(() => {
    loadConfig();
  });
</script>

<div class="worker-settings">
  <div class="settings-header">
    <h1>⚙️ Configuración de Workers</h1>
    <p>Ajusta los parámetros de los workers en segundo plano</p>
  </div>

  {#if isLoading}
    <div class="loading-state">
      <div class="spinner"></div>
      <p>Cargando configuración...</p>
    </div>
  {:else}
    <div class="settings-container">
      {#if error}
        <div class="alert error">
          <p>{error}</p>
        </div>
      {/if}

      {#if success}
        <div class="alert success">
          <p>✓ Configuración guardada exitosamente</p>
        </div>
      {/if}

      <form onsubmit={saveConfig} class="settings-form">
        <!-- Retención de Datos -->
        <div class="form-section">
          <h2>📊 Mantenimiento de Almacenamiento</h2>
          <div class="form-field">
            <label for="retention_days">
              Días de Retención de Datos
              <span class="field-description">
                Cantidad de días que se mantienen los datos históricos antes de ser purgados
              </span>
            </label>
            <input
              id="retention_days"
              type="number"
              min="1"
              max="365"
              bind:value={config.retention_days}
              class="form-input"
            />
            <span class="field-hint">Rango: 1-365 días</span>
          </div>

          <div class="form-field">
            <label for="pruning_hour">
              Hora de Ejecución de Pruning
              <span class="field-description">
                Hora del día (0-23) en que se ejecuta la limpieza de datos
              </span>
            </label>
            <input
              id="pruning_hour"
              type="number"
              min="0"
              max="23"
              bind:value={config.pruning_hour}
              class="form-input"
            />
            <span class="field-hint">Rango: 0-23 horas (formato 24h)</span>
          </div>
        </div>

        <!-- Monitoreo ICMP/Ping -->
        <div class="form-section">
          <h2>🌐 Sondeo de Red (ICMP/Ping)</h2>
          <div class="form-field">
            <label for="ping_interval_seconds">
              Intervalo de Ping (segundos)
              <span class="field-description">
                Frecuencia con la que se ejecutan los pings a los dispositivos
              </span>
            </label>
            <input
              id="ping_interval_seconds"
              type="number"
              min="10"
              max="300"
              bind:value={config.ping_interval_seconds}
              class="form-input"
            />
            <span class="field-hint">Rango: 10-300 segundos</span>
          </div>

          <div class="form-field">
            <label for="ping_max_retries">
              Máximo de Reintentos de Ping
              <span class="field-description">
                Número máximo de reintentos antes de marcar un dispositivo como caído
              </span>
            </label>
            <input
              id="ping_max_retries"
              type="number"
              min="1"
              max="10"
              bind:value={config.ping_max_retries}
              class="form-input"
            />
            <span class="field-hint">Rango: 1-10 reintentos</span>
          </div>
        </div>

        <!-- Descubrimiento SNMP -->
        <div class="form-section">
          <h2>🔌 Descubrimiento SNMPv3</h2>
          <div class="form-field">
            <label for="snmp_interval_seconds">
              Intervalo de Escaneo SNMP (segundos)
              <span class="field-description">
                Frecuencia con la que se ejecuta el descubrimiento de dispositivos
              </span>
            </label>
            <input
              id="snmp_interval_seconds"
              type="number"
              min="3600"
              max="86400"
              bind:value={config.snmp_interval_seconds}
              class="form-input"
            />
            <span class="field-hint">Rango: 3600-86400 segundos (1-24 horas)</span>
          </div>
        </div>

        <div class="form-actions">
          <button
            type="submit"
            disabled={isSaving}
            class="btn btn-primary"
          >
            {isSaving ? 'Guardando...' : 'Guardar Configuración'}
          </button>
        </div>
      </form>
    </div>
  {/if}
</div>

<style>
  .worker-settings {
    padding: 2rem;
    max-width: 900px;
    margin: 0 auto;
  }

  .settings-header {
    margin-bottom: 2rem;
  }

  .settings-header h1 {
    font-size: 2rem;
    font-weight: 700;
    margin-bottom: 0.5rem;
    color: #1a1a1a;
  }

  .settings-header p {
    color: #666;
    font-size: 1rem;
  }

  .loading-state {
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

  .settings-container {
    background: white;
    border-radius: 12px;
    padding: 2rem;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .alert {
    padding: 1rem;
    border-radius: 8px;
    margin-bottom: 1.5rem;
  }

  .alert.error {
    background: #fee;
    color: #c33;
    border: 1px solid #fcc;
  }

  .alert.success {
    background: #efe;
    color: #3c3;
    border: 1px solid #cfc;
  }

  .settings-form {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .form-section {
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    padding: 1.5rem;
  }

  .form-section h2 {
    font-size: 1.25rem;
    font-weight: 600;
    margin-bottom: 1.5rem;
    color: #1a1a1a;
  }

  .form-field {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
  }

  .form-field:last-child {
    margin-bottom: 0;
  }

  .form-field label {
    font-weight: 500;
    color: #333;
    font-size: 0.95rem;
  }

  .field-description {
    display: block;
    font-weight: 400;
    color: #666;
    font-size: 0.85rem;
    margin-top: 0.25rem;
  }

  .form-input {
    padding: 0.75rem;
    border: 1px solid #d0d0d0;
    border-radius: 6px;
    font-size: 1rem;
    transition: border-color 0.2s;
  }

  .form-input:focus {
    outline: none;
    border-color: #3498db;
  }

  .field-hint {
    font-size: 0.8rem;
    color: #999;
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    padding-top: 1rem;
    border-top: 1px solid #e0e0e0;
  }

  .btn {
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    border: none;
  }

  .btn-primary {
    background: #3498db;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #2980b9;
  }

  .btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>
