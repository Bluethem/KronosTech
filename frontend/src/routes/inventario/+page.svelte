<script lang="ts">
  import { onMount } from 'svelte';
  
  let showStockModal = $state(false);
  let showHistorySidebar = $state(false);
  
  // Data state
  let inventarioItems = $state([]);
  let stats = $state({
    total_productos: 0,
    stock_bajo: 0,
    sin_stock: 0,
    valor_total: 0
  });
  
  // Filter state
  let searchQuery = $state('');
  let stockFilter = $state('todos');
  let marcaFilter = $state('Todas');
  let loading = $state(true);
  let error = $state(null);
  
  // Pagination
  let currentPage = $state(1);
  let itemsPerPage = 10;
  let totalItems = $state(0);
  let totalPages = $state(0);
  
  onMount(async () => {
    await fetchData();
  });
  
  async function fetchData() {
    loading = true;
    error = null;
    
    try {
      // Fetch stats
      const statsResponse = await fetch('http://localhost:3000/api/inventario/stats');
      if (!statsResponse.ok) throw new Error('Error al cargar estadísticas');
      stats = await statsResponse.json();
      
      // Fetch inventory items
      const params = new URLSearchParams({
        page: currentPage.toString(),
        limit: itemsPerPage.toString()
      });
      if (searchQuery) params.append('search', searchQuery);
      if (stockFilter && stockFilter !== 'todos') params.append('stock_estado', stockFilter);
      if (marcaFilter && marcaFilter !== 'Todas') params.append('marca', marcaFilter);
      
      const itemsResponse = await fetch(`http://localhost:3000/api/inventario?${params}`);
      if (!itemsResponse.ok) throw new Error('Error al cargar inventario');
      const data = await itemsResponse.json();
      
      inventarioItems = data.items;
      totalItems = data.total_count;
      totalPages = Math.ceil(totalItems / itemsPerPage);
      
    } catch (e) {
      console.error(e);
      error = e.message;
    } finally {
      loading = false;
    }
  }
  
  function handleSearch() {
    currentPage = 1;
    fetchData();
  }
  
  function handleFilterChange() {
    currentPage = 1;
    fetchData();
  }
  
  function clearFilters() {
    searchQuery = '';
    stockFilter = 'todos';
    marcaFilter = 'Todas';
    currentPage = 1;
    fetchData();
  }
  
  function goToPage(page: number) {
    if (page >= 1 && page <= totalPages) {
      currentPage = page;
      fetchData();
    }
  }
  
  function nextPage() {
    if (currentPage < totalPages) {
      currentPage++;
      fetchData();
    }
  }
  
  function prevPage() {
    if (currentPage > 1) {
      currentPage--;
      fetchData();
    }
  }
  
  function getStockBadgeClass(stockEstado: string) {
    switch(stockEstado) {
      case 'ok':
        return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300';
      case 'bajo':
        return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300';
      case 'agotado':
        return 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300';
      default:
        return 'bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-300';
    }
  }
  
  function formatCurrency(amount: number) {
    return new Intl.NumberFormat('es-PE', { style: 'currency', currency: 'PEN' }).format(amount);
  }
  
  function formatDate(dateString: string | null) {
    if (!dateString) return 'N/A';
    return new Date(dateString).toLocaleString('es-PE');
  }
</script>

<div class="relative flex h-auto min-h-screen w-full flex-col group/design-root overflow-x-hidden bg-background-light dark:bg-background-dark font-display">
<div class="layout-container flex h-full grow flex-col">
<main class="flex-1 flex flex-col p-6 md:p-8">
<header class="flex flex-wrap items-center justify-between gap-4 mb-8">
<h1 class="text-gray-900 dark:text-white text-3xl font-bold leading-tight tracking-tight">Dashboard de Inventario</h1>
</header>
<section class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 mb-6">
<div class="flex flex-col gap-2 rounded-xl p-6 border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900/50">
<p class="text-gray-500 dark:text-gray-400 text-sm font-medium leading-normal">Total Productos Activos</p>
<p class="text-gray-900 dark:text-white tracking-tight text-3xl font-bold leading-tight">{stats.total_productos}</p>
</div>
<a class="flex flex-col gap-2 rounded-xl p-6 border border-yellow-300 dark:border-yellow-900 bg-white dark:bg-gray-900/50 hover:bg-yellow-50 dark:hover:bg-yellow-500/10 transition-colors" href="#">
<p class="text-yellow-600 dark:text-yellow-400 text-sm font-medium leading-normal">Stock Bajo</p>
<p class="text-yellow-800 dark:text-yellow-300 tracking-tight text-3xl font-bold leading-tight">{stats.stock_bajo}</p>
</a>
<a class="flex flex-col gap-2 rounded-xl p-6 border border-red-300 dark:border-red-900 bg-white dark:bg-gray-900/50 hover:bg-red-50 dark:hover:bg-red-500/10 transition-colors" href="#">
<p class="text-red-600 dark:text-red-400 text-sm font-medium leading-normal">Sin Stock</p>
<p class="text-red-700 dark:text-red-300 tracking-tight text-3xl font-bold leading-tight">{stats.sin_stock}</p>
</a>
<div class="flex flex-col gap-2 rounded-xl p-6 border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900/50">
<p class="text-gray-500 dark:text-gray-400 text-sm font-medium leading-normal">Valor Total Inventario</p>
<p class="text-gray-900 dark:text-white tracking-tight text-3xl font-bold leading-tight">{formatCurrency(stats.valor_total)}</p>
</div>
</section>
<section class="mb-6">
<div class="flex flex-col md:flex-row items-center gap-4">
<div class="relative flex-grow w-full md:w-auto">
<span class="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 dark:text-gray-500">search</span>
<input bind:value={searchQuery} on:keydown={(e) => e.key === 'Enter' && handleSearch()} class="w-full h-10 pl-10 pr-4 text-sm bg-white dark:bg-gray-900/50 border-gray-300 dark:border-gray-700 rounded-lg focus:ring-primary focus:border-primary" placeholder="Buscar por nombre o SKU..." type="text"/>
</div>
<div class="flex flex-wrap items-center gap-2 w-full md:w-auto">
<select bind:value={stockFilter} on:change={handleFilterChange} class="h-10 text-sm bg-white dark:bg-gray-900/50 border-gray-300 dark:border-gray-700 rounded-lg focus:ring-primary focus:border-primary w-full sm:w-auto">
<option value="todos">Estado Stock: Todos</option>
<option value="ok">Stock OK</option>
<option value="bajo">Bajo</option>
<option value="agotado">Sin Stock</option>
</select>
<select bind:value={marcaFilter} on:change={handleFilterChange} class="h-10 text-sm bg-white dark:bg-gray-900/50 border-gray-300 dark:border-gray-700 rounded-lg focus:ring-primary focus:border-primary w-full sm:w-auto">
<option value="Todas">Marca: Todas</option>
<option value="Marca A">Marca A</option>
<option value="Marca B">Marca B</option>
</select>
<button on:click={clearFilters} class="h-10 px-4 text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-900/50 border border-gray-300 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg w-full sm:w-auto">Limpiar</button>
<button class="flex items-center justify-center gap-2 h-10 px-4 text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-900/50 border border-gray-300 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg w-full sm:w-auto">
<span class="material-symbols-outlined text-base">download</span>
<span>Generar Reporte</span>
</button>
</div>
</section>
<section class="mb-6 flex flex-wrap items-center gap-3">
<button on:click={() => showStockModal = true} class="flex items-center justify-center gap-2 h-10 px-4 text-sm font-medium text-white bg-primary rounded-lg hover:bg-primary/90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 dark:focus-visible:ring-offset-background-dark">
<span class="material-symbols-outlined text-base">add</span>
<span>Entrada de Stock</span>
</button>
<a href="/nuevo-producto" class="flex items-center justify-center gap-2 h-10 px-4 text-sm font-medium text-white bg-primary rounded-lg hover:bg-primary/90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 dark:focus-visible:ring-offset-background-dark">
<span class="material-symbols-outlined text-base">add_circle</span>
<span>Nuevo Producto</span>
</a>
</section>
<div class="overflow-x-auto bg-white dark:bg-gray-900/50 border border-gray-200 dark:border-gray-800 rounded-xl flex-grow flex flex-col">
<div class="flex-grow">
<table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
<thead class="text-xs text-gray-700 dark:text-gray-300 uppercase bg-gray-50 dark:bg-gray-900/80">
<tr>
<th class="p-4 w-16" scope="col">Imagen</th>
<th class="px-6 py-3" scope="col">Producto</th>
<th class="px-6 py-3" scope="col">Stock Actual</th>
<th class="px-6 py-3" scope="col">Mínimo</th>
<th class="px-6 py-3" scope="col">Ubicación</th>
<th class="px-6 py-3" scope="col">Precio</th>
<th class="px-6 py-3" scope="col">Última Actualización</th>
<th class="px-6 py-3 text-center" scope="col">Acciones</th>
</tr>
</thead>
<tbody>
{#if loading}
  <tr>
    <td colspan="8" class="p-4 text-center text-gray-500 dark:text-gray-400">Cargando inventario...</td>
  </tr>
{:else if inventarioItems.length === 0}
  <tr>
    <td colspan="8" class="p-4 text-center text-gray-500 dark:text-gray-400">No se encontraron productos</td>
  </tr>
{:else}
  {#each inventarioItems as item}
  <tr class="border-b dark:border-gray-800 hover:bg-gray-50 dark:hover:bg-gray-900/60">
    <td class="p-4">
      <img class="w-10 h-10 rounded-md object-cover" alt={item.nombre} src={item.imagen_principal || 'https://via.placeholder.com/40'}/>
    </td>
    <th class="px-6 py-4 font-medium text-gray-900 dark:text-white whitespace-nowrap" scope="row">
      {item.nombre}<br/>
      <span class="font-normal text-gray-500 dark:text-gray-400">SKU: {item.sku}</span>
    </th>
    <td class="px-6 py-4">
      <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getStockBadgeClass(item.stock_estado)}">
        {item.cantidad_disponible}
      </span>
    </td>
    <td class="px-6 py-4">{item.cantidad_minima}</td>
    <td class="px-6 py-4">{item.ubicacion_fisica || 'Sin ubicación'}</td>
    <td class="px-6 py-4">{formatCurrency(item.precio_venta)}</td>
    <td class="px-6 py-4">{formatDate(item.fecha_actualizacion)}</td>
    <td class="px-6 py-4">
      <div class="flex items-center justify-center gap-2">
        <button class="p-2 text-gray-500 hover:text-primary dark:hover:text-primary-400 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800"><span class="material-symbols-outlined text-xl">tune</span></button>
        <button on:click={() => showHistorySidebar = true} class="p-2 text-gray-500 hover:text-primary dark:hover:text-primary-400 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800"><span class="material-symbols-outlined text-xl">history</span></button>
        <button class="p-2 text-gray-500 hover:text-red-600 dark:hover:text-red-400 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800"><span class="material-symbols-outlined text-xl">delete</span></button>
      </div>
    </td>
  </tr>
  {/each}
{/if}
</tbody>
</table>
</div>
<nav aria-label="Table navigation" class="flex flex-col md:flex-row justify-between items-start md:items-center space-y-3 md:space-y-0 p-4 border-t border-gray-200 dark:border-gray-800">
<span class="text-sm font-normal text-gray-500 dark:text-gray-400">
Mostrando <span class="font-semibold text-gray-900 dark:text-white">{((currentPage - 1) * itemsPerPage) + 1}-{Math.min(currentPage * itemsPerPage, totalItems)}</span> de <span class="font-semibold text-gray-900 dark:text-white">{totalItems}</span> productos
</span>
<ul class="inline-flex items-stretch -space-x-px">
<li>
<button 
  on:click={prevPage}
  disabled={currentPage === 1}
  class="flex items-center justify-center h-full py-1.5 px-3 ml-0 text-gray-500 bg-white rounded-l-lg border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white disabled:opacity-50 disabled:cursor-not-allowed">
<span class="sr-only">Anterior</span>
<span class="material-symbols-outlined text-xl">chevron_left</span>
</button>
</li>
{#if totalPages > 0}
  {#if currentPage > 2}
  <li>
    <button on:click={() => goToPage(1)} class="flex items-center justify-center text-sm py-2 px-3 leading-tight text-gray-500 bg-white border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white">1</button>
  </li>
  {/if}
  {#if currentPage > 3}
  <li>
    <span class="flex items-center justify-center text-sm py-2 px-3 leading-tight text-gray-500 bg-white border border-gray-300 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400">...</span>
  </li>
  {/if}
  {#if currentPage > 1}
  <li>
    <button on:click={() => goToPage(currentPage - 1)} class="flex items-center justify-center text-sm py-2 px-3 leading-tight text-gray-500 bg-white border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white">{currentPage - 1}</button>
  </li>
  {/if}
  <li>
    <button aria-current="page" class="flex items-center justify-center text-sm py-2 px-3 leading-tight text-primary bg-primary/10 border border-primary hover:bg-primary/20 hover:text-primary-700 dark:border-primary dark:bg-primary/20 dark:text-white">{currentPage}</button>
  </li>
  {#if currentPage < totalPages}
  <li>
    <button on:click={() => goToPage(currentPage + 1)} class="flex items-center justify-center text-sm py-2 px-3 leading-tight text-gray-500 bg-white border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white">{currentPage + 1}</button>
  </li>
  {/if}
  {#if currentPage < totalPages - 2}
  <li>
    <span class="flex items-center justify-center text-sm py-2 px-3 leading-tight text-gray-500 bg-white border border-gray-300 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400">...</span>
  </li>
  {/if}
  {#if currentPage < totalPages - 1}
  <li>
    <button on:click={() => goToPage(totalPages)} class="flex items-center justify-center text-sm py-2 px-3 leading-tight text-gray-500 bg-white border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white">{totalPages}</button>
  </li>
  {/if}
{/if}
<li>
<button 
  on:click={nextPage}
  disabled={currentPage === totalPages || totalPages === 0}
  class="flex items-center justify-center h-full py-1.5 px-3 leading-tight text-gray-500 bg-white rounded-r-lg border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white disabled:opacity-50 disabled:cursor-not-allowed">
<span class="sr-only">Siguiente</span>
<span class="material-symbols-outlined text-xl">chevron_right</span>
</button>
</li>
</ul>
</nav>
</div>
</main>
</div>
</div>

<!-- Stock Entry Modal -->
{#if showStockModal}
<div class="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6 md:p-8" style="background-color: rgba(0, 0, 0, 0.5);">
<!-- Modal Container -->
<div class="w-full max-w-4xl rounded-xl bg-white dark:bg-gray-800 shadow-2xl flex flex-col">
<!-- Modal Header -->
<div class="p-6 border-b border-gray-200 dark:border-gray-700">
<div class="flex flex-wrap justify-between items-center gap-3">
<div class="flex flex-col gap-1">
<p class="text-gray-900 dark:text-white text-2xl font-bold leading-tight">Registrar Entrada de Stock</p>
<p class="text-gray-500 dark:text-gray-400 text-sm font-normal leading-normal">Busca un producto y completa la información para añadir nuevo stock.</p>
</div>
</div>
</div>
<!-- Modal Body -->
<div class="p-6 space-y-6">
<!-- Search Section -->
<div class="space-y-4">
<p class="text-gray-800 dark:text-gray-200 text-base font-medium leading-normal">Buscar Producto por nombre o SKU</p>
<label class="flex flex-col w-full">
<div class="flex w-full flex-1 items-stretch rounded-lg h-12">
<div class="text-gray-500 dark:text-gray-400 flex bg-gray-100 dark:bg-gray-700 items-center justify-center pl-4 rounded-l-lg border-r-0">
<span class="material-symbols-outlined">search</span>
</div>
<input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-0 focus:ring-2 focus:ring-primary/50 border-gray-300 dark:border-gray-600 bg-gray-100 dark:bg-gray-700 h-full placeholder:text-gray-500 dark:placeholder:text-gray-400 px-4 rounded-l-none border-l-0 pl-2 text-base font-normal leading-normal" placeholder="Ej: Camiseta Azul M" value="Camiseta Estampada 'Classic'"/>
</div>
</label>
</div>
<!-- Selected Product Card -->
<div class="p-4 border rounded-lg border-gray-200 dark:border-gray-700 bg-background-light dark:bg-background-dark">
<div class="flex items-center justify-between gap-4">
<div class="flex items-center gap-4 flex-[2_2_0px]">
<div class="w-24 h-24 bg-center bg-no-repeat aspect-square bg-cover rounded-lg flex-shrink-0" style='background-image: url("https://lh3.googleusercontent.com/aida-public/AB6AXuCUOB0B0eHS5JDXKA96YaxTbu4zEcv_P43zV9iKJgN70oumBEOuboUyZ0KUOCoIToc20UZYt8Lu3w8oM9jj2lZoCOEsyweXG0iQvU9YBOoWRsJDFN_G3r8MKeu-ztjCtOxmG6s3hZ7CG_9COAj-gV4D45AZg4jVwpKVbMfwNrdtcRzk0G22Ehv9dhx-byiIbGUuqg30Xqg6f5SB0GsSOYbV32fuHxovVn2RiwSL3hSBhtYEQf-yzLa-4jNpOvHWjq4m5veRKfHCmks");'></div>
<div class="flex flex-col gap-1">
<p class="text-gray-900 dark:text-white text-lg font-bold leading-tight">Camiseta Estampada 'Classic'</p>
<p class="text-gray-500 dark:text-gray-400 text-sm font-normal leading-normal">SKU: 12345-ABC</p>
<p class="text-gray-500 dark:text-gray-400 text-sm font-normal leading-normal">Stock Actual: <span class="font-semibold text-gray-700 dark:text-gray-300">150</span></p>
</div>
</div>
</div>
</div>
<!-- Form Section -->
<div class="grid grid-cols-1 md:grid-cols-2 gap-x-6 gap-y-5">
<!-- Cantidad -->
<div class="flex flex-col md:col-span-1">
<label class="text-gray-800 dark:text-gray-200 text-base font-medium leading-normal pb-2" for="quantity">Cantidad</label>
<input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 h-14 placeholder:text-gray-500 dark:placeholder:text-gray-400 p-[15px] text-lg font-normal leading-normal" id="quantity" placeholder="0" type="number" value=""/>
</div>
<!-- Ubicación Física -->
<div class="flex flex-col md:col-span-1">
<label class="text-gray-800 dark:text-gray-200 text-base font-medium leading-normal pb-2" for="location">Ubicación Física</label>
<input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 h-14 placeholder:text-gray-500 dark:placeholder:text-gray-400 p-[15px] text-base font-normal leading-normal" id="location" placeholder="Estantería A-3" type="text" value=""/>
</div>
<!-- Lote -->
<div class="flex flex-col md:col-span-1">
<label class="text-gray-800 dark:text-gray-200 text-base font-medium leading-normal pb-2" for="batch">Lote (Opcional)</label>
<input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 h-14 placeholder:text-gray-500 dark:placeholder:text-gray-400 p-[15px] text-base font-normal leading-normal" id="batch" placeholder="Lote-001" type="text" value=""/>
</div>
<!-- Costo Unitario -->
<div class="flex flex-col md:col-span-1">
<label class="text-gray-800 dark:text-gray-200 text-base font-medium leading-normal pb-2" for="cost">Costo Unitario (Opcional)</label>
<div class="relative">
<span class="absolute inset-y-0 left-0 flex items-center pl-4 text-gray-500 dark:text-gray-400">$</span>
<input class="form-input pl-8 flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 h-14 placeholder:text-gray-500 dark:placeholder:text-gray-400 p-[15px] text-base font-normal leading-normal" id="cost" placeholder="0.00" type="text" value=""/>
</div>
</div>
<!-- Vencimiento -->
<div class="flex flex-col md:col-span-1">
<label class="text-gray-800 dark:text-gray-200 text-base font-medium leading-normal pb-2" for="expiry">Vencimiento (Opcional)</label>
<div class="relative">
<input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 h-14 placeholder:text-gray-500 dark:placeholder:text-gray-400 p-[15px] text-base font-normal leading-normal" id="expiry" placeholder="DD/MM/AAAA" type="text" value=""/>
<span class="material-symbols-outlined absolute inset-y-0 right-0 flex items-center pr-4 text-gray-500 dark:text-gray-400 pointer-events-none">calendar_today</span>
</div>
</div>
<!-- Documento de Referencia -->
<div class="flex flex-col md:col-span-1">
<label class="text-gray-800 dark:text-gray-200 text-base font-medium leading-normal pb-2" for="document">Documento (Opcional)</label>
<input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 h-14 placeholder:text-gray-500 dark:placeholder:text-gray-400 p-[15px] text-base font-normal leading-normal" id="document" placeholder="Factura #12345" type="text" value=""/>
</div>
<!-- Notas -->
<div class="flex flex-col md:col-span-2">
<label class="text-gray-800 dark:text-gray-200 text-base font-medium leading-normal pb-2" for="notes">Notas (Opcional)</label>
<textarea class="form-textarea w-full min-w-0 flex-1 resize-y overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 h-24 placeholder:text-gray-500 dark:placeholder:text-gray-400 p-[15px] text-base font-normal leading-normal" id="notes" placeholder="Añadir detalles sobre la entrada..."></textarea>
</div>
</div>
<!-- Summary Section -->
<div class="bg-primary/10 dark:bg-primary/20 p-4 rounded-lg flex items-center justify-center space-x-4">
<div class="text-center">
<p class="text-sm text-gray-600 dark:text-gray-300">Stock Actual</p>
<p class="text-2xl font-bold text-gray-800 dark:text-white">150</p>
</div>
<span class="material-symbols-outlined text-primary dark:text-primary/80 text-3xl">arrow_forward</span>
<div class="text-center">
<p class="text-sm text-primary dark:text-primary/80">Nuevo Stock (calculado)</p>
<p class="text-2xl font-bold text-primary dark:text-primary/90">175</p>
</div>
</div>
</div>
<!-- Modal Footer -->
<div class="p-6 bg-gray-50 dark:bg-gray-900/50 rounded-b-xl flex justify-end items-center gap-4">
<button on:click={() => showStockModal = false} class="px-6 py-3 rounded-lg text-base font-medium text-gray-700 dark:text-gray-300 bg-transparent hover:bg-gray-200 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-400 dark:focus:ring-offset-background-dark">
Cancelar
</button>
<button class="px-6 py-3 rounded-lg text-base font-medium text-white bg-primary hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary dark:focus:ring-offset-background-dark">
Registrar Entrada
</button>
</div>
</div>
</div>
{/if}

<!-- History Sidebar -->
{#if showHistorySidebar}
<div class="fixed inset-0 bg-gray-900/50 dark:bg-gray-900/80 z-40" on:click={() => showHistorySidebar = false}></div>
<aside class="fixed top-0 right-0 h-full w-full max-w-2xl bg-background-light dark:bg-background-dark z-50 flex flex-col shadow-2xl">
<header class="flex items-start justify-between p-6 border-b border-gray-200 dark:border-gray-800">
<div class="flex items-center gap-4">
<img class="w-16 h-16 rounded-lg object-cover" alt="Zapatilla deportiva roja" src="https://lh3.googleusercontent.com/aida-public/AB6AXuDc48OhfbdmqeXdjAmallc0JlVZwxSdYr1QbMjB61_p4dtImvP51MzwXHktoAB0-d8eP4ktQkFHqU-BgmNqEi7PWrF96mpRezMiBMSKt2VO6tOSc7uCSGG8ZGfxS6TbXy-kUzs8jA1IZsyhfMwGCz2wvvnbwd0F8_2BespoQGYD_6XX_C0yfHDqIEF5SbNiYD6WmwoDFADPANGIkU1onF1FfwWnBpoZceOUMJdva84NyTcbqRHstv59r4Nw7-W6oD6Q4HsBwa6N78w"/>
<div class="flex flex-col gap-1">
<h2 class="text-lg font-semibold text-gray-900 dark:text-white">Historial de Movimientos</h2>
<p class="text-sm text-gray-700 dark:text-gray-300">Zapatilla Deportiva NX-200</p>
<div class="flex items-center gap-4 text-xs text-gray-500 dark:text-gray-400">
<span>SKU: ZAP-NX-200-42</span>
<span class="flex items-center gap-1">
<span class="font-medium text-gray-600 dark:text-gray-300">Stock Actual:</span>
<span class="font-bold text-green-600 dark:text-green-400">250</span>
</span>
</div>
</div>
</div>
<div class="flex items-center gap-2">
<button class="h-9 px-3 text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg">Ajuste Rápido</button>
<button on:click={() => showHistorySidebar = false} class="p-2 text-gray-500 hover:text-gray-800 dark:text-gray-400 dark:hover:text-white rounded-full hover:bg-gray-100 dark:hover:bg-gray-800"><span class="material-symbols-outlined">close</span></button>
</div>
</header>
<div class="p-6 border-b border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900/50">
<div class="flex flex-wrap items-center gap-4">
<div class="flex-grow">
<label class="text-xs text-gray-500 dark:text-gray-400" for="move-type">Tipo de Movimiento</label>
<select class="mt-1 block w-full h-9 text-sm bg-white dark:bg-gray-800 border-gray-300 dark:border-gray-700 rounded-lg focus:ring-primary focus:border-primary" id="move-type">
<option>Todos</option>
<option>Entrada</option>
<option>Salida</option>
<option>Ajuste</option>
<option>Venta</option>
</select>
</div>
<div class="flex-grow">
<label class="text-xs text-gray-500 dark:text-gray-400" for="date-range">Rango de Fechas</label>
<input class="mt-1 block w-full h-9 text-sm bg-white dark:bg-gray-800 border-gray-300 dark:border-gray-700 rounded-lg focus:ring-primary focus:border-primary" id="date-range" placeholder="Seleccionar fechas..." type="text"/>
</div>
<div class="self-end">
<button class="flex items-center justify-center gap-2 h-9 px-4 text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg">
<span class="material-symbols-outlined text-base">download</span>
<span>Exportar CSV</span>
</button>
</div>
</div>
</div>
<div class="flex-1 overflow-y-auto p-6">
<ul class="space-y-4">
<li class="flex items-start gap-4">
<div class="flex flex-col items-center">
<span class="flex items-center justify-center size-8 rounded-full bg-green-100 dark:bg-green-900/50 text-green-600 dark:text-green-400"><span class="material-symbols-outlined text-lg">add_shopping_cart</span></span>
</div>
<div class="flex-1">
<div class="flex justify-between items-center">
<p class="text-sm font-medium text-gray-800 dark:text-gray-200"><span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-green-100 text-green-800 dark:bg-green-900/80 dark:text-green-300">Entrada</span></p>
<p class="text-xs text-gray-500 dark:text-gray-400">26 Oct 2023, 10:30 AM</p>
</div>
<div class="mt-1 flex justify-between items-center">
<p class="text-sm text-gray-500 dark:text-gray-400">Cantidad: <span class="font-semibold text-green-600 dark:text-green-400">+50</span></p>
<p class="text-sm text-gray-500 dark:text-gray-400">200 <span class="material-symbols-outlined text-sm align-middle">arrow_right_alt</span> 250</p>
</div>
<p class="mt-1 text-xs text-gray-500 dark:text-gray-400">Motivo: Recepción de proveedor | Usuario: admin</p>
</div>
</li>
<li class="flex items-start gap-4">
<div class="flex flex-col items-center">
<span class="flex items-center justify-center size-8 rounded-full bg-red-100 dark:bg-red-900/50 text-red-600 dark:text-red-400"><span class="material-symbols-outlined text-lg">shopping_cart</span></span>
</div>
<div class="flex-1">
<div class="flex justify-between items-center">
<p class="text-sm font-medium text-gray-800 dark:text-gray-200"><span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-red-100 text-red-800 dark:bg-red-900/80 dark:text-red-300">Venta</span></p>
<p class="text-xs text-gray-500 dark:text-gray-400">25 Oct 2023, 04:50 PM</p>
</div>
<div class="mt-1 flex justify-between items-center">
<p class="text-sm text-gray-500 dark:text-gray-400">Cantidad: <span class="font-semibold text-red-600 dark:text-red-400">-2</span></p>
<p class="text-sm text-gray-500 dark:text-gray-400">202 <span class="material-symbols-outlined text-sm align-middle">arrow_right_alt</span> 200</p>
</div>
<p class="mt-1 text-xs text-gray-500 dark:text-gray-400">Pedido #10524 | Usuario: Juan Pérez</p>
</div>
</li>
<li class="flex items-start gap-4">
<div class="flex flex-col items-center">
<span class="flex items-center justify-center size-8 rounded-full bg-yellow-100 dark:bg-yellow-900/50 text-yellow-600 dark:text-yellow-400"><span class="material-symbols-outlined text-lg">published_with_changes</span></span>
</div>
<div class="flex-1">
<div class="flex justify-between items-center">
<p class="text-sm font-medium text-gray-800 dark:text-gray-200"><span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-yellow-100 text-yellow-800 dark:bg-yellow-900/80 dark:text-yellow-300">Ajuste Negativo</span></p>
<p class="text-xs text-gray-500 dark:text-gray-400">24 Oct 2023, 11:00 AM</p>
</div>
<div class="mt-1 flex justify-between items-center">
<p class="text-sm text-gray-500 dark:text-gray-400">Cantidad: <span class="font-semibold text-red-600 dark:text-red-400">-3</span></p>
<p class="text-sm text-gray-500 dark:text-gray-400">205 <span class="material-symbols-outlined text-sm align-middle">arrow_right_alt</span> 202</p>
</div>
<p class="mt-1 text-xs text-gray-500 dark:text-gray-400">Motivo: Producto dañado | Usuario: admin</p>
</div>
</li>
<li class="flex items-start gap-4">
<div class="flex flex-col items-center">
<span class="flex items-center justify-center size-8 rounded-full bg-blue-100 dark:bg-blue-900/50 text-blue-600 dark:text-blue-400"><span class="material-symbols-outlined text-lg">local_shipping</span></span>
</div>
<div class="flex-1">
<div class="flex justify-between items-center">
<p class="text-sm font-medium text-gray-800 dark:text-gray-200"><span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900/80 dark:text-blue-300">Salida</span></p>
<p class="text-xs text-gray-500 dark:text-gray-400">23 Oct 2023, 09:15 AM</p>
</div>
<div class="mt-1 flex justify-between items-center">
<p class="text-sm text-gray-500 dark:text-gray-400">Cantidad: <span class="font-semibold text-red-600 dark:text-red-400">-10</span></p>
<p class="text-sm text-gray-500 dark:text-gray-400">215 <span class="material-symbols-outlined text-sm align-middle">arrow_right_alt</span> 205</p>
</div>
<p class="mt-1 text-xs text-gray-500 dark:text-gray-400">Transferencia a Tienda Central | Usuario: Marta Gómez</p>
</div>
</li>
</ul>
<div class="mt-6 text-center">
<button class="h-9 px-4 text-sm font-medium text-primary dark:text-primary-400 bg-primary/10 hover:bg-primary/20 rounded-lg">Cargar más movimientos</button>
</div>
</div>
</aside>
{/if}
