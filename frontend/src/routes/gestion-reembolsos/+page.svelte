<script lang="ts">
  import { onMount } from 'svelte';

  interface Reembolso {
    id_reembolso: number;
    numero_pedido: string;
    id_venta: number;
    nombre_cliente: string | null;
    email_cliente: string | null;
    tipo_reembolso: string;
    monto_reembolsado: number;
    estado: string | null;
    fecha_solicitado: string | null;
  }

  interface Stats {
    pendientes: number;
    completados_mes: number;
    monto_total_mes: number;
    cambio_pendientes: number;
    cambio_completados: number;
    cambio_monto: number;
  }

  let reembolsos: Reembolso[] = [];
  let stats: Stats = {
    pendientes: 0,
    completados_mes: 0,
    monto_total_mes: 0,
    cambio_pendientes: 0,
    cambio_completados: 0,
    cambio_monto: 0
  };
  let loading = true;
  let error = '';

  // Filters
  let searchQuery = '';
  let estadoFilter = 'Todos';
  let tipoFilter = 'Todos';
  let currentPage = 1;
  let itemsPerPage = 10;
  let totalItems = 0;

  // Modals
  let isReviewModalOpen = false;
  let isDetailsModalOpen = false;
  let selectedReembolso: Reembolso | null = null;
  let reembolsoDetalle: any = null;
  let loadingDetalle = false;
  let processingDecision = false;
  let decision = '';
  let adminNotes = '';

  const API_BASE = 'http://localhost:3000';

  async function fetchStats() {
    try {
      const response = await fetch(`${API_BASE}/api/reembolsos/stats`);
      if (!response.ok) throw new Error('Error al cargar estadísticas');
      stats = await response.json();
    } catch (e) {
      console.error('Error fetching stats:', e);
      error = 'Error al cargar estadísticas';
    }
  }

  async function fetchReembolsos() {
    loading = true;
    error = '';
    try {
      const params = new URLSearchParams({
        page: currentPage.toString(),
        limit: itemsPerPage.toString(),
      });

      if (estadoFilter !== 'Todos') {
        params.append('estado', estadoFilter);
      }
      if (tipoFilter !== 'Todos') {
        params.append('tipo', tipoFilter);
      }
      if (searchQuery.trim()) {
        params.append('search', searchQuery.trim());
      }

      const response = await fetch(`${API_BASE}/api/reembolsos?${params}`);
      if (!response.ok) throw new Error('Error al cargar reembolsos');
      
      reembolsos = await response.json();
      // Note: Backend doesn't return total count yet, so we estimate
      totalItems = reembolsos.length === itemsPerPage ? currentPage * itemsPerPage + 1 : (currentPage - 1) * itemsPerPage + reembolsos.length;
    } catch (e) {
      console.error('Error fetching reembolsos:', e);
      error = 'Error al cargar reembolsos';
      reembolsos = [];
    } finally {
      loading = false;
    }
  }

  function formatDate(dateString: string | null): string {
    if (!dateString) return '-';
    const date = new Date(dateString);
    return date.toLocaleDateString('es-PE', { year: 'numeric', month: '2-digit', day: '2-digit' });
  }

  function formatCurrency(amount: number): string {
    return new Intl.NumberFormat('es-PE', {
      style: 'currency',
      currency: 'PEN'
    }).format(amount);
  }

  function isProcesado(estado: string | null): boolean {
    const normalizado = estado?.toLowerCase();
    return normalizado === 'completado' || normalizado === 'rechazado';
  }

  function getEstadoBadgeClass(estado: string | null): string {
    switch (estado?.toLowerCase()) {
      case 'solicitado':
        return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300';
      case 'procesando':
        return 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300';
      case 'completado':
        return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300';
      case 'rechazado':
        return 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300';
      default:
        return 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
    }
  }

  function getTipoBadgeClass(tipo: string): string {
    return 'bg-gray-200 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
  }

  async function openReviewModal(reembolso: Reembolso) {
    selectedReembolso = reembolso;
    loadingDetalle = true;
    isReviewModalOpen = true;
    
    try {
      const response = await fetch(`${API_BASE}/api/reembolsos/${reembolso.id_reembolso}`);
      if (!response.ok) throw new Error('Error al cargar detalles');
      reembolsoDetalle = await response.json();
    } catch (e) {
      console.error('Error fetching detalle:', e);
    } finally {
      loadingDetalle = false;
    }
  }

  function closeReviewModal() {
    isReviewModalOpen = false;
    selectedReembolso = null;
    reembolsoDetalle = null;
    decision = '';
    adminNotes = '';
  }

  async function procesarReembolso() {
    if (!selectedReembolso || !decision) {
      alert('Por favor selecciona una decisión');
      return;
    }

    processingDecision = true;
    try {
      const response = await fetch(`${API_BASE}/api/reembolsos/${selectedReembolso.id_reembolso}/procesar`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          decision: decision,
          notas_admin: adminNotes || null,
        }),
      });

      if (!response.ok) throw new Error('Error al procesar reembolso');

      // Refresh data
      await fetchReembolsos();
      await fetchStats();
      closeReviewModal();
      alert('Reembolso procesado exitosamente');
    } catch (e) {
      console.error('Error processing reembolso:', e);
      alert('Error al procesar el reembolso');
    } finally {
      processingDecision = false;
    }
  }

  async function openDetailsModal(reembolso: Reembolso) {
    selectedReembolso = reembolso;
    loadingDetalle = true;
    isDetailsModalOpen = true;
    
    try {
      const response = await fetch(`${API_BASE}/api/reembolsos/${reembolso.id_reembolso}`);
      if (!response.ok) throw new Error('Error al cargar detalles');
      reembolsoDetalle = await response.json();
    } catch (e) {
      console.error('Error fetching detalle:', e);
    } finally {
      loadingDetalle = false;
    }
  }

  function closeDetailsModal() {
    isDetailsModalOpen = false;
    selectedReembolso = null;
    reembolsoDetalle = null;
  }

  function nextPage() {
    if (reembolsos.length === itemsPerPage) {
      currentPage++;
      fetchReembolsos();
    }
  }

  function prevPage() {
    if (currentPage > 1) {
      currentPage--;
      fetchReembolsos();
    }
  }

  function goToPage(page: number) {
    currentPage = page;
    fetchReembolsos();
  }

  function handleSearchChange() {
    currentPage = 1;
    fetchReembolsos();
  }

  function handleEstadoChange() {
    currentPage = 1;
    fetchReembolsos();
  }

  function handleTipoChange() {
    currentPage = 1;
    fetchReembolsos();
  }

  onMount(() => {
    fetchStats();
    fetchReembolsos();
  });
</script>

<div class="relative flex h-auto min-h-screen w-full flex-col group/design-root overflow-x-hidden bg-background-light dark:bg-background-dark font-display text-gray-800 dark:text-gray-200">
  <div class="layout-container flex h-full grow flex-col">
    <div class="px-4 sm:px-8 md:px-12 lg:px-20 xl:px-40 flex flex-1 justify-center py-5">
      <div class="layout-content-container flex flex-col max-w-[1280px] flex-1">
        <!-- Page Header -->
        <div class="flex flex-wrap justify-between items-center gap-4 p-4">
          <div class="flex flex-col gap-1">
            <p class="text-[#111418] dark:text-white text-4xl font-black leading-tight tracking-[-0.033em]">Gestión de Reembolsos</p>
            <p class="text-[#617589] dark:text-gray-400 text-base font-normal leading-normal">Administra y procesa las solicitudes de reembolso de los clientes.</p>
          </div>
        </div>

        <!-- Stats Cards -->
        <div class="flex flex-wrap gap-4 p-4">
          <div class="flex min-w-[200px] flex-1 flex-col gap-2 rounded-lg p-6 border border-gray-200 dark:border-gray-700 bg-white dark:bg-background-dark">
            <div class="flex items-center justify-between">
              <p class="text-[#111418] dark:text-gray-300 text-base font-medium leading-normal">Reembolsos pendientes</p>
              <span class="material-symbols-outlined text-gray-400">hourglass_top</span>
            </div>
            <div class="flex items-center gap-2">
              <p class="text-[#111418] dark:text-white tracking-light text-3xl font-bold leading-tight">{stats.pendientes}</p>
              {#if stats.pendientes > 0}
                <div class="relative flex items-center justify-center w-6 h-6 bg-red-500 rounded-full text-white text-xs font-bold">!</div>
              {/if}
            </div>
            <p class="text-gray-500 text-sm font-medium leading-normal">{stats.cambio_pendientes >= 0 ? '+' : ''}{stats.cambio_pendientes} esta semana</p>
          </div>
          <div class="flex min-w-[200px] flex-1 flex-col gap-2 rounded-lg p-6 border border-gray-200 dark:border-gray-700 bg-white dark:bg-background-dark">
            <div class="flex items-center justify-between">
              <p class="text-[#111418] dark:text-gray-300 text-base font-medium leading-normal">Reembolsos completados (mes)</p>
              <span class="material-symbols-outlined text-gray-400">task_alt</span>
            </div>
            <p class="text-[#111418] dark:text-white tracking-light text-3xl font-bold leading-tight">{stats.completados_mes}</p>
            <p class="{stats.cambio_completados >= 0 ? 'text-green-600' : 'text-red-500'} text-sm font-medium leading-normal">{stats.cambio_completados >= 0 ? '+' : ''}{stats.cambio_completados.toFixed(1)}%</p>
          </div>
          <div class="flex min-w-[200px] flex-1 flex-col gap-2 rounded-lg p-6 border border-gray-200 dark:border-gray-700 bg-white dark:bg-background-dark">
            <div class="flex items-center justify-between">
              <p class="text-[#111418] dark:text-gray-300 text-base font-medium leading-normal">Monto total reembolsado (mes)</p>
              <span class="material-symbols-outlined text-gray-400">payments</span>
            </div>
            <p class="text-[#111418] dark:text-white tracking-light text-3xl font-bold leading-tight">{formatCurrency(stats.monto_total_mes)}</p>
            <p class="{stats.cambio_monto >= 0 ? 'text-green-600' : 'text-red-500'} text-sm font-medium leading-normal">{stats.cambio_monto >= 0 ? '+' : ''}{stats.cambio_monto.toFixed(1)}%</p>
          </div>
        </div>

        <!-- Filters -->
        <div class="flex flex-wrap justify-between items-center gap-4 p-4">
          <div class="flex items-center gap-2">
            <div class="relative">
              <span class="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-gray-400">search</span>
              <input bind:value={searchQuery} on:input={handleSearchChange} class="pl-10 pr-4 py-2 w-64 border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800 focus:ring-2 focus:ring-primary focus:outline-none dark:text-white" placeholder="Buscar por pedido o email..." type="text"/>
            </div>
            <select bind:value={estadoFilter} on:change={handleEstadoChange} class="border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800 focus:ring-2 focus:ring-primary focus:outline-none dark:text-white px-4 py-2">
              <option value="Todos">Estado: Todos</option>
              <option value="Solicitado">Solicitado</option>
              <option value="Procesando">Procesando</option>
              <option value="Completado">Completado</option>
              <option value="Rechazado">Rechazado</option>
            </select>
            <select bind:value={tipoFilter} on:change={handleTipoChange} class="border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800 focus:ring-2 focus:ring-primary focus:outline-none dark:text-white px-4 py-2">
              <option value="Todos">Tipo: Todos</option>
              <option value="Total">Total</option>
              <option value="Parcial">Parcial</option>
            </select>
          </div>
        </div>

        <!-- Table -->
        <div class="px-4 py-3 @container">
          <div class="overflow-x-auto rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-background-dark">
            <table class="w-full text-left">
              <thead class="bg-gray-50 dark:bg-gray-800">
                <tr>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">ID Reembolso</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Nº de pedido</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Cliente</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Tipo</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Monto</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Estado</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Fecha solicitado</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider" style="width: 210px;">Acciones</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
                {#if loading}
                  <tr>
                    <td colspan="8" class="px-4 py-8 text-center text-gray-500 dark:text-gray-400">
                      <div class="flex items-center justify-center gap-2">
                        <span class="material-symbols-outlined animate-spin">progress_activity</span>
                        Cargando reembolsos...
                      </div>
                    </td>
                  </tr>
                {:else if error}
                  <tr>
                    <td colspan="8" class="px-4 py-8 text-center text-red-500">
                      {error}
                    </td>
                  </tr>
                {:else if reembolsos.length === 0}
                  <tr>
                    <td colspan="8" class="px-4 py-8 text-center text-gray-500 dark:text-gray-400">
                      No se encontraron reembolsos
                    </td>
                  </tr>
                {:else}
                  {#each reembolsos as reembolso}
                    <tr class="hover:bg-gray-50 dark:hover:bg-gray-800/50 {reembolso.estado?.toLowerCase() === 'solicitado' ? 'bg-yellow-50 dark:bg-yellow-900/20' : ''}">
                      <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">RF{reembolso.id_reembolso}</td>
                      <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">{reembolso.numero_pedido}</td>
                      <td class="px-4 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">
                        {reembolso.nombre_cliente || 'N/A'}
                        <br/>
                        <span class="font-normal text-gray-500 dark:text-gray-400">{reembolso.email_cliente || 'N/A'}</span>
                      </td>
                      <td class="px-4 py-4 whitespace-nowrap">
                        <span class="px-2.5 py-1 text-xs font-semibold rounded-full capitalize {getTipoBadgeClass(reembolso.tipo_reembolso)}">
                          {reembolso.tipo_reembolso}
                        </span>
                      </td>
                      <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">{formatCurrency(reembolso.monto_reembolsado)}</td>
                      <td class="px-4 py-4 whitespace-nowrap">
                        <span class="px-2.5 py-1 text-xs font-semibold rounded-full capitalize {getEstadoBadgeClass(reembolso.estado)}">
                          {reembolso.estado || 'Desconocido'}
                        </span>
                      </td>
                      <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">{formatDate(reembolso.fecha_solicitado)}</td>
                      <td class="px-4 py-4 text-sm font-medium">
                        <div class="flex items-center gap-2">
                          {#if !isProcesado(reembolso.estado)}
                            <button class="px-3 py-1.5 text-xs font-semibold text-white bg-primary rounded-md hover:bg-blue-600 whitespace-nowrap" on:click={() => openReviewModal(reembolso)}>Procesar</button>
                          {:else}
                            <span class="px-3 py-1.5 text-xs font-semibold text-gray-400 bg-gray-100 dark:bg-gray-800 rounded-md whitespace-nowrap cursor-not-allowed">Procesado</span>
                          {/if}
                          <button class="px-3 py-1.5 text-xs font-semibold text-gray-700 bg-gray-200 rounded-md hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 whitespace-nowrap" on:click={() => openDetailsModal(reembolso)}>Ver detalle</button>
                        </div>
                      </td>
                    </tr>
                  {/each}
                {/if}
              </tbody>
            </table>
          </div>
        </div>

        <!-- Pagination -->
        <div class="flex items-center justify-between p-4 border-t border-gray-200 dark:border-gray-700">
          <p class="text-sm text-gray-600 dark:text-gray-400">
            Mostrando 
            <span class="font-semibold">{(currentPage - 1) * itemsPerPage + 1}</span>-<span class="font-semibold">{Math.min(currentPage * itemsPerPage, (currentPage - 1) * itemsPerPage + reembolsos.length)}</span>
            {#if totalItems > 0}
              de <span class="font-semibold">{totalItems}</span>
            {/if}
            resultados
          </p>
          <div class="flex items-center justify-center">
            <button on:click={prevPage} disabled={currentPage === 1} class="flex w-10 h-10 items-center justify-center rounded-md border border-gray-200 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-800 disabled:opacity-50 disabled:cursor-not-allowed">
              <span class="material-symbols-outlined text-lg">chevron_left</span>
            </button>
            <button on:click={() => goToPage(currentPage)} class="text-sm font-bold flex w-10 h-10 items-center justify-center text-white rounded-md bg-primary mx-1">{currentPage}</button>
            {#if reembolsos.length === itemsPerPage}
              <button on:click={() => goToPage(currentPage + 1)} class="text-sm font-normal flex w-10 h-10 items-center justify-center text-[#111418] dark:text-gray-300 rounded-md hover:bg-gray-100 dark:hover:bg-gray-800">{currentPage + 1}</button>
            {/if}
            <button on:click={nextPage} disabled={reembolsos.length < itemsPerPage} class="flex w-10 h-10 items-center justify-center rounded-md border border-gray-200 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-800 ml-1 disabled:opacity-50 disabled:cursor-not-allowed">
              <span class="material-symbols-outlined text-lg">chevron_right</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Refund Review Modal -->
{#if isReviewModalOpen}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-gray-900/50 dark:bg-gray-900/70 backdrop-blur-sm" on:click={closeReviewModal}>
    <div class="w-full max-w-4xl rounded-xl bg-white dark:bg-background-dark shadow-2xl" on:click|stopPropagation>
      <div class="flex flex-col">
        <!-- Modal Header -->
        <div class="flex items-center justify-between gap-3 p-6 border-b border-gray-200 dark:border-gray-800">
          <p class="text-[#111418] dark:text-gray-100 tracking-light text-[24px] font-bold leading-tight">Revisar Solicitud de Reembolso #{selectedReembolso?.id_reembolso || ''}</p>
          <button class="flex h-8 w-8 items-center justify-center rounded-full text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-800 dark:text-gray-400" on:click={closeReviewModal}>
            <span aria-hidden="true" class="material-symbols-outlined">close</span>
          </button>
        </div>
        <!-- Modal Body -->
        <div class="flex-1 p-6 space-y-8">
          {#if loadingDetalle}
            <div class="flex items-center justify-center py-12">
              <span class="material-symbols-outlined animate-spin text-4xl text-primary">progress_activity</span>
            </div>
          {:else if reembolsoDetalle}
          <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
            <!-- Left Column: Info -->
            <div class="space-y-6">
              <!-- Order Info -->
              <div>
                <h3 class="text-[#111418] dark:text-gray-100 text-lg font-bold leading-tight tracking-[-0.015em] pb-2">Información del pedido</h3>
                <div class="grid grid-cols-2 gap-x-4 gap-y-1 border-t border-gray-200 dark:border-gray-800 pt-4">
                  <div class="flex flex-col gap-1 py-2 pr-2 col-span-2 sm:col-span-1">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Número de pedido:</p>
                    <p class="text-[#111418] dark:text-gray-200 text-sm font-medium leading-normal">{reembolsoDetalle.numero_pedido}</p>
                  </div>
                  <div class="flex flex-col gap-1 py-2 pl-0 sm:pl-2 col-span-2 sm:col-span-1">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Cliente:</p>
                    <p class="text-[#111418] dark:text-gray-200 text-sm font-medium leading-normal">{reembolsoDetalle.nombre_cliente || 'N/A'}</p>
                  </div>
                  <div class="flex flex-col gap-1 py-2 pr-2 col-span-2 sm:col-span-1">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Email:</p>
                    <p class="text-[#111418] dark:text-gray-200 text-sm font-medium leading-normal">{reembolsoDetalle.email_cliente || 'N/A'}</p>
                  </div>
                  <div class="flex flex-col gap-1 py-2 pl-0 sm:pl-2 col-span-2 sm:col-span-1">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Fecha del pedido:</p>
                    <p class="text-[#111418] dark:text-gray-200 text-sm font-medium leading-normal">{formatDate(reembolsoDetalle.fecha_pedido)}</p>
                  </div>
                  <div class="flex flex-col gap-1 py-2 pr-2 col-span-2 sm:col-span-1">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Monto original pagado:</p>
                    <p class="text-[#111418] dark:text-gray-200 text-sm font-medium leading-normal">{formatCurrency(reembolsoDetalle.total_venta)}</p>
                  </div>
                  <div class="flex flex-col gap-1 py-2 pl-0 sm:pl-2 col-span-2 sm:col-span-1">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Método de pago:</p>
                    <p class="text-[#111418] dark:text-gray-200 text-sm font-medium leading-normal">{reembolsoDetalle.metodo_pago || 'N/A'}</p>
                  </div>
                </div>
              </div>
              <!-- Refund Request Info -->
              <div>
                <h3 class="text-[#111418] dark:text-gray-100 text-lg font-bold leading-tight tracking-[-0.015em] pb-2">Solicitud de reembolso</h3>
                <div class="space-y-4 border-t border-gray-200 dark:border-gray-800 pt-4">
                  <div class="flex items-center gap-4">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal w-32 shrink-0">Tipo:</p>
                    <span class="inline-flex items-center rounded-full bg-primary/20 dark:bg-primary/30 px-3 py-1 text-sm font-medium text-primary dark:text-blue-200 capitalize">{reembolsoDetalle.tipo_reembolso}</span>
                  </div>
                  <div class="flex items-center gap-4">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal w-32 shrink-0">Monto solicitado:</p>
                    <p class="text-primary dark:text-blue-300 text-base font-bold leading-normal">{formatCurrency(reembolsoDetalle.monto_reembolsado)}</p>
                  </div>
                  <div class="flex items-center gap-4">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal w-32 shrink-0">Fecha de solicitud:</p>
                    <p class="text-[#111418] dark:text-gray-200 text-sm font-medium leading-normal">{formatDate(reembolsoDetalle.fecha_solicitado)}</p>
                  </div>
                  <div class="flex flex-col gap-2">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Motivo del cliente:</p>
                    <div class="w-full rounded-lg border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 p-3">
                      <p class="text-[#111418] dark:text-gray-200 text-sm font-normal leading-relaxed">{reembolsoDetalle.motivo}</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <!-- Right Column: Decision Form -->
            <div class="bg-background-light dark:bg-gray-900/50 rounded-lg p-6 space-y-6">
              <h3 class="text-[#111418] dark:text-gray-100 text-lg font-bold leading-tight tracking-[-0.015em]">Decisión</h3>
              <div class="space-y-4">
                <!-- Radio Buttons -->
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                  <label class="flex cursor-pointer items-center gap-3 rounded-lg border-2 border-green-500 bg-green-50 dark:bg-green-900/30 p-4 ring-2 ring-transparent has-[:checked]:ring-green-500">
                    <input bind:group={decision} class="form-radio h-5 w-5 border-gray-300 text-green-600 focus:ring-green-500" name="decision" type="radio" value="aprobar"/>
                    <span class="font-semibold text-green-800 dark:text-green-300">Aprobar reembolso</span>
                  </label>
                  <label class="flex cursor-pointer items-center gap-3 rounded-lg border-2 border-red-500 bg-red-50 dark:bg-red-900/30 p-4 ring-2 ring-transparent has-[:checked]:ring-red-500">
                    <input bind:group={decision} class="form-radio h-5 w-5 border-gray-300 text-red-600 focus:ring-red-500" name="decision" type="radio" value="rechazar"/>
                    <span class="font-semibold text-red-800 dark:text-red-300">Rechazar reembolso</span>
                  </label>
                </div>
                <!-- Conditional Fields -->
                <div>
                  <!-- Approval Fields -->
                  <div class="space-y-4">
                    <div>
                      <label class="text-sm font-medium text-gray-700 dark:text-gray-300" for="refund_amount">Monto final a reembolsar</label>
                      <input class="mt-1 block w-full rounded border-gray-300 shadow-sm focus:border-primary focus:ring-primary dark:bg-gray-700 dark:border-gray-600 dark:text-white sm:text-sm" id="refund_amount" type="text" value="S/ 250.00"/>
                    </div>
                    <div>
                      <label class="text-sm font-medium text-gray-700 dark:text-gray-300" for="admin_notes">Notas administrativas (opcional)</label>
                      <textarea bind:value={adminNotes} class="mt-1 block w-full rounded border-gray-300 shadow-sm focus:border-primary focus:ring-primary dark:bg-gray-700 dark:border-gray-600 dark:text-white sm:text-sm" id="admin_notes" placeholder="Notas internas sobre este reembolso..." rows="3"></textarea>
                    </div>
                    <p class="text-xs text-gray-500 dark:text-gray-400">Al aprobar, el reembolso se marcará como completado y se iniciará el proceso de devolución.</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        {/if}
        </div>
        <!-- Modal Footer -->
        <div class="flex items-center justify-end gap-4 p-6 bg-background-light dark:bg-gray-900/50 rounded-b-xl border-t border-gray-200 dark:border-gray-800">
          <button class="rounded px-4 py-2 text-sm font-semibold text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 border border-gray-300 dark:border-gray-600 shadow-sm" on:click={closeReviewModal}>Cancelar</button>
          <button on:click={procesarReembolso} disabled={processingDecision || !decision} class="flex items-center gap-2 rounded bg-green-600 px-4 py-2 text-sm font-semibold text-green-50 shadow-sm hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed">
            {#if processingDecision}
              <span class="material-symbols-outlined animate-spin text-green-50" style="font-size: 18px;">progress_activity</span>
            {:else}
              <span aria-hidden="true" class="material-symbols-outlined text-green-50" style="font-size: 18px;">check_circle</span>
            {/if}
            Confirmar Decisión
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<!-- Refund Details Modal -->
{#if isDetailsModalOpen}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-gray-900/50 dark:bg-black/60 backdrop-blur-sm" on:click={closeDetailsModal}>
    <div class="relative w-full max-w-2xl bg-white dark:bg-gray-900 rounded-xl shadow-lg flex flex-col" on:click|stopPropagation>
      <!-- Modal Header -->
      <div class="flex items-start justify-between p-6 border-b border-gray-200 dark:border-gray-700">
        <div class="flex flex-col gap-2">
          <p class="text-[#111418] dark:text-gray-100 tracking-light text-2xl font-bold leading-tight">Detalle del Reembolso #{selectedReembolso?.id_reembolso || ''}</p>
          {#if reembolsoDetalle}
            <div class="flex items-center gap-x-2">
              <div class="flex h-7 shrink-0 items-center justify-center gap-x-2 rounded-full {getEstadoBadgeClass(reembolsoDetalle.estado)} px-3 py-1">
                <p class="text-sm font-medium leading-normal capitalize">{reembolsoDetalle.estado || 'Desconocido'}</p>
              </div>
            </div>
          {/if}
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:text-gray-500 dark:hover:text-gray-300" on:click={closeDetailsModal}>
          <span class="material-symbols-outlined">close</span>
        </button>
      </div>
      <!-- Modal Body -->
      <div class="p-6 space-y-8 overflow-y-auto max-h-[70vh] scrollbar-thin scrollbar-thumb-gray-400 dark:scrollbar-thumb-gray-600 scrollbar-track-gray-200 dark:scrollbar-track-gray-800">
        {#if loadingDetalle}
          <div class="flex items-center justify-center py-12">
            <span class="material-symbols-outlined animate-spin text-4xl text-primary">progress_activity</span>
          </div>
        {:else if reembolsoDetalle}
        <!-- Sección Información del pedido -->
        <div class="space-y-4">
          <h3 class="text-[#111418] dark:text-gray-200 text-lg font-bold leading-tight tracking-[-0.015em]">Información del pedido</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-x-6 gap-y-4 text-sm">
            <div>
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Número de pedido</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">{reembolsoDetalle.numero_pedido}</p>
            </div>
            <div>
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Cliente</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">{reembolsoDetalle.nombre_cliente || 'N/A'}</p>
            </div>
            <div>
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Email</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">{reembolsoDetalle.email_cliente || 'N/A'}</p>
            </div>
            <div>
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Monto original</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">{formatCurrency(reembolsoDetalle.total_venta)}</p>
            </div>
            <div>
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Fecha del pedido</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">{formatDate(reembolsoDetalle.fecha_pedido)}</p>
            </div>
          </div>
        </div>
        <hr class="border-t border-gray-200 dark:border-gray-700"/>
        <!-- Sección Detalles del reembolso -->
        <div class="space-y-4">
          <h3 class="text-[#111418] dark:text-gray-200 text-lg font-bold leading-tight tracking-[-0.015em]">Detalles del reembolso</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-x-6 gap-y-4 text-sm">
            <div>
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Tipo</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal capitalize">{reembolsoDetalle.tipo_reembolso}</p>
            </div>
            <div>
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Fecha solicitado</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">{formatDate(reembolsoDetalle.fecha_solicitado)}</p>
            </div>
            <div class="col-span-1 md:col-span-2">
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Monto reembolsado</p>
              <p class="text-primary dark:text-primary-300 text-2xl font-bold leading-normal">{formatCurrency(reembolsoDetalle.monto_reembolsado)}</p>
            </div>
            <div class="col-span-1 md:col-span-2">
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Motivo del cliente</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">{reembolsoDetalle.motivo}</p>
            </div>
            {#if reembolsoDetalle.fecha_completado}
              <div>
                <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Fecha completado</p>
                <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">{formatDate(reembolsoDetalle.fecha_completado)}</p>
              </div>
            {/if}
          </div>
        </div>
        {#if reembolsoDetalle.notas_admin || reembolsoDetalle.nombre_aprobador}
          <hr class="border-t border-gray-200 dark:border-gray-700"/>
          <!-- Sección Decisión administrativa -->
          <div class="space-y-4">
            <h3 class="text-[#111418] dark:text-gray-200 text-lg font-bold leading-tight tracking-[-0.015em]">Decisión administrativa</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-x-6 gap-y-4 text-sm">
              {#if reembolsoDetalle.nombre_aprobador}
                <div>
                  <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Aprobado por</p>
                  <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">{reembolsoDetalle.nombre_aprobador}</p>
                </div>
              {/if}
              {#if reembolsoDetalle.notas_admin}
                <div class="col-span-1 md:col-span-2">
                  <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Notas del administrador</p>
                  <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">{reembolsoDetalle.notas_admin}</p>
                </div>
              {/if}
            </div>
          </div>
        {/if}
        <!-- Timeline -->
        <div class="pt-2">
          <div class="relative pl-8">
            <div class="absolute left-3 top-1 bottom-1 w-0.5 bg-gray-200 dark:bg-gray-700"></div>
            <div class="relative mb-6">
              <div class="absolute -left-[30px] top-1/2 -translate-y-1/2 h-5 w-5 rounded-full bg-primary flex items-center justify-center">
                <div class="h-2 w-2 rounded-full bg-white"></div>
              </div>
              <p class="text-[#111418] dark:text-gray-100 text-sm font-medium">Solicitado</p>
              <p class="text-[#617589] dark:text-gray-400 text-xs">{formatDate(reembolsoDetalle.fecha_solicitado)}</p>
            </div>
            {#if reembolsoDetalle.fecha_completado}
              <div class="relative">
                <div class="absolute -left-[30px] top-1/2 -translate-y-1/2 h-5 w-5 rounded-full bg-green-500 flex items-center justify-center">
                  <div class="h-2 w-2 rounded-full bg-white"></div>
                </div>
                <p class="text-[#111418] dark:text-gray-100 text-sm font-medium">Completado</p>
                <p class="text-[#617589] dark:text-gray-400 text-xs">{formatDate(reembolsoDetalle.fecha_completado)}</p>
              </div>
            {/if}
          </div>
        </div>
        {/if}
      </div>
      <!-- Modal Footer -->
      <div class="flex items-center justify-end p-6 border-t border-gray-200 dark:border-gray-700">
        <button class="bg-gray-100 dark:bg-gray-700 text-[#111418] dark:text-gray-100 hover:bg-gray-200 dark:hover:bg-gray-600 font-medium py-2 px-4 rounded-lg transition-colors" on:click={closeDetailsModal}>Cerrar</button>
      </div>
    </div>
  </div>
{/if}
