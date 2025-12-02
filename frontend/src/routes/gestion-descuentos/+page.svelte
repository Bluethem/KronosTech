<script lang="ts">
  import { onMount } from 'svelte';
  
  // State
  let isSidebarOpen = false;
  let descuentos = [];
  let stats = {
    total_activos: 0,
    vigentes_hoy: 0,
    usos_mes_actual: 0,
    ahorro_generado_mes: 0
  };
  let selectedDescuento = null;
  let loading = false;
  
  // Filters
  let searchQuery = '';
  let tipoFilter = '';
  let estadoFilter = '';
  
  onMount(async () => {
    await Promise.all([
      fetchDescuentos(),
      fetchStats()
    ]);
  });
  
  async function fetchDescuentos() {
    loading = true;
    try {
      let url = 'http://localhost:3000/api/descuentos?';
      if (searchQuery) url += `search=${encodeURIComponent(searchQuery)}&`;
      if (tipoFilter) url += `tipo=${encodeURIComponent(tipoFilter)}&`;
      if (estadoFilter) url += `estado=${encodeURIComponent(estadoFilter)}&`;
      
      const response = await fetch(url);
      if (response.ok) {
        descuentos = await response.json();
      }
    } catch (e) {
      console.error('Error fetching descuentos:', e);
    } finally {
      loading = false;
    }
  }
  
  async function fetchStats() {
    try {
      const response = await fetch('http://localhost:3000/api/descuentos/stats');
      if (response.ok) {
        stats = await response.json();
      }
    } catch (e) {
      console.error('Error fetching stats:', e);
    }
  }
  
  async function openSidebar(descuento) {
    try {
      const response = await fetch(`http://localhost:3000/api/descuentos/${descuento.id_descuento}`);
      if (response.ok) {
        selectedDescuento = await response.json();
        isSidebarOpen = true;
      }
    } catch (e) {
      console.error('Error fetching descuento detail:', e);
    }
  }

  function closeSidebar() {
    isSidebarOpen = false;
    selectedDescuento = null;
  }
  
  function formatCurrency(amount) {
    return new Intl.NumberFormat('es-PE', { style: 'currency', currency: 'PEN' }).format(amount);
  }
  
  function formatDate(dateString) {
    if (!dateString) return 'N/A';
    return new Date(dateString).toLocaleDateString('es-PE');
  }
  
  function getProgressWidth(actual, max) {
    if (!max) return '100%';
    const percentage = (actual / max) * 100;
    return `${Math.min(percentage, 100)}%`;
  }
  
  function getEstadoBadgeClass(descuento) {
    const now = new Date();
    const start = new Date(descuento.fecha_inicio);
    const end = new Date(descuento.fecha_fin);
    
    if (!descuento.activo) return 'bg-gray-100 text-gray-800';
    if (now > end) return 'bg-red-100 text-red-800';
    if (now < start) return 'bg-yellow-100 text-yellow-800';
    return 'bg-green-100 text-green-800';
  }
  
  function getEstadoText(descuento) {
    const now = new Date();
    const start = new Date(descuento.fecha_inicio);
    const end = new Date(descuento.fecha_fin);
    
    if (!descuento.activo) return 'Inactivo';
    if (now > end) return 'Vencido';
    if (now < start) return 'Próximo';
    return 'Vigente';
  }

  // Debounce search
  let searchTimeout;
  function handleSearch() {
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(() => {
      fetchDescuentos();
    }, 300);
  }
</script>

<div class="relative flex h-auto min-h-screen w-full flex-col group/design-root bg-background-light dark:bg-background-dark font-display text-[#111418] dark:text-gray-200">
<div class="layout-container flex h-full grow flex-col">
<main class="flex-1 p-4 sm:p-6 lg:p-8">
<div class="mx-auto max-w-7xl">
<div class="flex flex-col sm:flex-row flex-wrap justify-between items-start gap-4 mb-6">
<div class="flex items-center gap-4">
<h1 class="text-3xl lg:text-4xl font-black tracking-tight text-[#111418] dark:text-white">Gestión de Descuentos</h1>
<div class="flex h-8 shrink-0 items-center justify-center gap-x-2 rounded-full bg-primary/10 dark:bg-primary/20 px-4">
<span class="material-symbols-outlined text-primary dark:text-primary-400 !text-base">sell</span>
<p class="text-primary dark:text-primary-400 text-sm font-medium">{stats.total_activos} Descuentos Activos</p>
</div>
</div>
<a href="/gestion-descuentos/nuevo" class="flex items-center justify-center gap-2 min-w-[84px] cursor-pointer rounded-lg h-10 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em]">
<span class="material-symbols-outlined">add_circle</span>
<span class="truncate">Crear Descuento</span>
</a>
</div>

      <div class="mb-6">
        <h3 class="text-sm font-semibold uppercase text-gray-500 dark:text-gray-400 mb-3 tracking-wider">Estadísticas Superiores</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-6">
          <div class="bg-white dark:bg-background-dark rounded-xl p-6 shadow-sm flex items-start gap-4 border border-gray-100 dark:border-slate-800">
            <div class="flex-shrink-0 size-12 rounded-lg bg-primary/10 flex items-center justify-center">
              <span class="material-symbols-outlined text-primary text-3xl">sell</span>
            </div>
            <div class="flex flex-col">
              <p class="text-gray-500 dark:text-gray-400 text-sm">Total de descuentos activos</p>
              <p class="text-gray-900 dark:text-white text-2xl font-bold">{stats.total_activos}</p>
            </div>
          </div>
          <div class="bg-white dark:bg-background-dark rounded-xl p-6 shadow-sm flex items-start gap-4 border border-gray-100 dark:border-slate-800">
            <div class="flex-shrink-0 size-12 rounded-lg bg-primary/10 flex items-center justify-center">
              <span class="material-symbols-outlined text-primary text-3xl">today</span>
            </div>
            <div class="flex flex-col">
              <p class="text-gray-500 dark:text-gray-400 text-sm">Descuentos vigentes hoy</p>
              <p class="text-gray-900 dark:text-white text-2xl font-bold">{stats.vigentes_hoy}</p>
            </div>
          </div>
          <div class="bg-white dark:bg-background-dark rounded-xl p-6 shadow-sm flex items-start gap-4 border border-gray-100 dark:border-slate-800">
            <div class="flex-shrink-0 size-12 rounded-lg bg-primary/10 flex items-center justify-center">
              <span class="material-symbols-outlined text-primary text-3xl">monitoring</span>
            </div>
            <div class="flex flex-col">
              <p class="text-gray-500 dark:text-gray-400 text-sm">Total de uso en el mes actual</p>
              <p class="text-gray-900 dark:text-white text-2xl font-bold">{stats.usos_mes_actual}</p>
            </div>
          </div>
          <div class="bg-white dark:bg-background-dark rounded-xl p-6 shadow-sm flex items-start gap-4 border border-gray-100 dark:border-slate-800">
            <div class="flex-shrink-0 size-12 rounded-lg bg-primary/10 flex items-center justify-center">
              <span class="material-symbols-outlined text-primary text-3xl">savings</span>
            </div>
            <div class="flex flex-col">
              <p class="text-gray-500 dark:text-gray-400 text-sm">Ahorro generado este mes</p>
              <p class="text-gray-900 dark:text-white text-2xl font-bold">{formatCurrency(stats.ahorro_generado_mes)}</p>
            </div>
          </div>
        </div>
      </div>

<div class="mb-6 rounded-xl border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900/50 p-4">
        <div class="flex flex-col md:flex-row gap-4 mb-4">
          <div class="flex-grow">
            <label class="flex flex-col min-w-40 h-12 w-full">
              <div class="flex w-full flex-1 items-stretch rounded-lg h-full border border-gray-200 dark:border-slate-700">
                <div class="text-gray-500 dark:text-gray-400 flex bg-background-light dark:bg-slate-800 items-center justify-center pl-4 rounded-l-lg border-r-0">
                  <span class="material-symbols-outlined">search</span>
                </div>
                <input 
                  bind:value={searchQuery}
                  on:input={handleSearch}
                  class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-r-lg text-gray-900 dark:text-white focus:outline-0 focus:ring-2 focus:ring-primary/50 border-none bg-background-light dark:bg-slate-800 focus:border-none h-full placeholder:text-gray-500 dark:placeholder:text-gray-400 px-4 pl-2 text-sm font-normal leading-normal" 
                  placeholder="Buscar por nombre de descuento..." 
                />
              </div>
            </label>
          </div>
          <div class="flex gap-3 overflow-x-auto pb-2">
            <select bind:value={tipoFilter} on:change={fetchDescuentos} class="flex h-10 shrink-0 items-center justify-center gap-x-2 rounded-lg bg-background-light dark:bg-slate-800 px-4 text-gray-800 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-slate-700 transition-colors border-none outline-none cursor-pointer">
              <option value="">Todos los tipos</option>
              <option value="Porcentaje">Porcentaje</option>
              <option value="Monto Fijo">Monto Fijo</option>
              <option value="Envío Gratis">Envío Gratis</option>
            </select>
            <select bind:value={estadoFilter} on:change={fetchDescuentos} class="flex h-10 shrink-0 items-center justify-center gap-x-2 rounded-lg bg-background-light dark:bg-slate-800 px-4 text-gray-800 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-slate-700 transition-colors border-none outline-none cursor-pointer">
              <option value="">Todos los estados</option>
              <option value="vigente">Vigente</option>
              <option value="proximo">Próximo</option>
              <option value="vencido">Vencido</option>
              <option value="inactivo">Inactivo</option>
            </select>
          </div>
        </div>
        <div class="overflow-x-auto">
          <div class="border border-gray-200 dark:border-slate-800 rounded-lg">
            <table class="min-w-full divide-y divide-gray-200 dark:divide-slate-800">
              <thead class="bg-background-light dark:bg-slate-800">
                <tr>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider" scope="col">Nombre</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider" scope="col">Tipo</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider" scope="col">Valor</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider" scope="col">Aplica a</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider" scope="col">Vigencia</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider" scope="col">Usos</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider" scope="col">Estado</th>
                  <th class="relative px-6 py-3" scope="col"><span class="sr-only">Acciones</span></th>
                </tr>
              </thead>
              <tbody class="bg-white dark:bg-background-dark divide-y divide-gray-200 dark:divide-slate-800">
                {#if loading}
                  <tr>
                    <td colspan="8" class="px-6 py-4 text-center text-gray-500">Cargando descuentos...</td>
                  </tr>
                {:else if descuentos.length === 0}
                  <tr>
                    <td colspan="8" class="px-6 py-4 text-center text-gray-500">No se encontraron descuentos</td>
                  </tr>
                {:else}
                  {#each descuentos as descuento}
                    <tr>
                      <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">{descuento.nombre}</td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">{descuento.tipo_descuento}</td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                        {descuento.tipo_descuento === 'Porcentaje' ? `${descuento.valor}%` : formatCurrency(descuento.valor)}
                      </td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                        {descuento.aplica_a}
                        {#if descuento.referencia_nombre}
                          : {descuento.referencia_nombre}
                        {/if}
                      </td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                        {formatDate(descuento.fecha_inicio)} - {formatDate(descuento.fecha_fin)}
                      </td>
                      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                        <div class="flex items-center gap-2">
                          <div class="w-24 bg-gray-200 dark:bg-slate-700 rounded-full h-1.5">
                            <div class="bg-primary h-1.5 rounded-full" style="width: {getProgressWidth(descuento.usos_actuales, descuento.usos_maximos)}"></div>
                          </div>
                          <span class="text-gray-800 dark:text-gray-200">
                            {descuento.usos_actuales || 0}/{descuento.usos_maximos || '∞'}
                          </span>
                        </div>
                      </td>
                      <td class="px-6 py-4 whitespace-nowrap">
                        <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getEstadoBadgeClass(descuento)}">
                          {getEstadoText(descuento)}
                        </span>
                      </td>
                      <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                        <div class="flex items-center justify-end gap-4">
                          <button class="text-primary hover:text-primary/80" on:click={() => openSidebar(descuento)}>Edit</button>
                          <button class="text-red-500 hover:text-red-700">
                            <span class="material-symbols-outlined text-xl">delete</span>
                          </button>
                        </div>
                      </td>
                    </tr>
                  {/each}
                {/if}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </main>
</div>
</div>

<!-- Side Panel Overlay -->
{#if isSidebarOpen}
  <div class="fixed inset-0 bg-black/50 z-40 transition-opacity" on:click={closeSidebar} on:keydown={(e) => e.key === 'Escape' && closeSidebar()} role="button" tabindex="0"></div>
{/if}

<!-- Side Panel -->
<div class="fixed top-0 right-0 h-full w-full max-w-2xl bg-white dark:bg-gray-900 shadow-2xl flex flex-col transform transition-transform duration-300 ease-in-out z-50 {isSidebarOpen ? 'translate-x-0' : 'translate-x-full'}">
  {#if selectedDescuento}
    <div class="flex-shrink-0 flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
      <div class="flex flex-col">
        <h1 class="text-lg font-bold text-gray-900 dark:text-white">Detalle del Descuento</h1>
        <p class="text-sm font-medium text-primary">{selectedDescuento.nombre}</p>
      </div>
      <button class="p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-500 dark:text-gray-400" on:click={closeSidebar}>
        <span class="material-symbols-outlined">close</span>
      </button>
    </div>
    <div class="flex-grow overflow-y-auto p-6 space-y-6 bg-background-light dark:bg-background-dark">
      <!-- General Information Card -->
      <div class="bg-white dark:bg-gray-900/50 rounded-lg border border-gray-200 dark:border-gray-700/50">
        <div class="flex justify-between items-center px-4 pt-4 pb-2">
          <h3 class="text-lg font-bold leading-tight tracking-[-0.015em] text-[#111418] dark:text-white">Información General</h3>
          <div class="flex h-8 shrink-0 items-center justify-center gap-x-2 rounded-lg {selectedDescuento.activo ? 'bg-green-100 dark:bg-green-900/50' : 'bg-red-100 dark:bg-red-900/50'} px-4">
            <p class="{selectedDescuento.activo ? 'text-green-800 dark:text-green-300' : 'text-red-800 dark:text-red-300'} text-sm font-medium leading-normal">
              {selectedDescuento.activo ? 'Activo' : 'Inactivo'}
            </p>
          </div>
        </div>
        <div class="p-4 grid grid-cols-[160px_1fr] gap-x-6">
          <div class="col-span-2 grid grid-cols-subgrid border-t border-t-[#dbe0e6] dark:border-t-gray-700 py-4">
            <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Descripción</p>
            <p class="text-[#111418] dark:text-gray-200 text-sm font-normal leading-normal">{selectedDescuento.descripcion || 'Sin descripción'}</p>
          </div>
          <div class="col-span-2 grid grid-cols-subgrid border-t border-t-[#dbe0e6] dark:border-t-gray-700 py-4">
            <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Tipo</p>
            <p class="text-[#111418] dark:text-gray-200 text-sm font-normal leading-normal">{selectedDescuento.tipo_descuento}</p>
          </div>
          <div class="col-span-2 grid grid-cols-subgrid border-t border-t-[#dbe0e6] dark:border-t-gray-700 py-4">
            <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Valor</p>
            <p class="text-[#111418] dark:text-gray-200 text-sm font-normal leading-normal">
              {selectedDescuento.tipo_descuento === 'Porcentaje' ? `${selectedDescuento.valor}%` : formatCurrency(selectedDescuento.valor)}
            </p>
          </div>
          <div class="col-span-2 grid grid-cols-subgrid border-t border-t-[#dbe0e6] dark:border-t-gray-700 py-4">
            <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Aplica a</p>
            <p class="text-[#111418] dark:text-gray-200 text-sm font-normal leading-normal">{selectedDescuento.aplica_a}</p>
          </div>
          <div class="col-span-2 grid grid-cols-subgrid border-t border-t-[#dbe0e6] dark:border-t-gray-700 py-4">
            <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Fechas de Validez</p>
            <p class="text-[#111418] dark:text-gray-200 text-sm font-normal leading-normal">
              {formatDate(selectedDescuento.fecha_inicio)} - {formatDate(selectedDescuento.fecha_fin)}
            </p>
          </div>
          <div class="col-span-2 grid grid-cols-subgrid border-t border-t-[#dbe0e6] dark:border-t-gray-700 py-4">
            <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Condiciones</p>
            <p class="text-[#111418] dark:text-gray-200 text-sm font-normal leading-normal">
              {#if selectedDescuento.compra_minima}
                Compra mínima: {formatCurrency(selectedDescuento.compra_minima)}
              {:else if selectedDescuento.cantidad_minima}
                Cantidad mínima: {selectedDescuento.cantidad_minima} unidades
              {:else}
                Ninguna
              {/if}
            </p>
          </div>
        </div>
      </div>
      
      <!-- Statistics Card -->
      <div class="bg-white dark:bg-gray-900/50 rounded-lg border border-gray-200 dark:border-gray-700/50">
        <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4">Estadísticas de Uso</h3>
        <div class="grid grid-cols-2 p-4">
          <div class="p-4">
            <p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Usos Actuales</p>
            <p class="text-3xl font-bold text-gray-900 dark:text-white">{selectedDescuento.usos_actuales || 0}</p>
          </div>
          <div class="p-4 border-l border-gray-200 dark:border-gray-700">
            <p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Límite de Usos</p>
            <p class="text-3xl font-bold text-gray-900 dark:text-white">{selectedDescuento.usos_maximos || '∞'}</p>
          </div>
        </div>
      </div>
    </div>
    <div class="flex-shrink-0 flex items-center justify-end gap-4 p-4 border-t border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900">
      <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200 text-sm font-bold leading-normal tracking-[0.015em] hover:bg-gray-300 dark:hover:bg-gray-600">
        <span class="truncate">Archivar</span>
      </button>
      <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em] hover:bg-blue-600 dark:hover:bg-blue-500">
        <span class="truncate">Editar Descuento</span>
      </button>
    </div>
  {/if}
</div>
