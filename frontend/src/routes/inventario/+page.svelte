<script lang="ts">
  import { onMount } from 'svelte';
  
  let showStockModal = $state(false);
  let showHistorySidebar = $state(false);
  let showAdjustModal = $state(false);
  
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
  
  // Stock entry modal state
  let productSearchQuery = $state('');
  let searchResults = $state([]);
  let selectedProduct = $state(null);
  let formData = $state({
    cantidad: '',
    ubicacion_fisica: '',
    lote: '',
    fecha_vencimiento: ''
  });
  let submitting = $state(false);
  let successMessage = $state('');
  
  // History sidebar state
  let selectedProductHistory = $state(null);
  let historyMovements = $state([]);
  let loadingHistory = $state(false);
  
  // Adjustment modal state
  let selectedProductAdjust = $state(null);
  let adjustFormData = $state({
    cantidad_disponible: '',
    cantidad_minima: '',
    cantidad_maxima: '',
    ubicacion_fisica: '',
    motivo: ''
  });
  let adjustSubmitting = $state(false);
  let adjustSuccessMessage = $state('');
  
  // Image URL for adjustment modal
  let imageUrlAdjust = $state('');
  
  // PDF Generation
  let jsPDF: any;
  let autoTable: any;
  
  onMount(async () => {
    await fetchData();
    // Dynamically import jsPDF and autoTable
    const jsPDFModule = await import('jspdf');
    jsPDF = jsPDFModule.default;
    const autoTableModule = await import('jspdf-autotable');
    autoTable = autoTableModule.default;
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
  
  // Stock entry functions
  async function searchProducts() {
    if (productSearchQuery.length < 2) {
      searchResults = [];
      return;
    }
    
    try {
      const response = await fetch(`http://localhost:3000/api/inventario/search?search=${encodeURIComponent(productSearchQuery)}`);
      if (!response.ok) throw new Error('Error al buscar productos');
      searchResults = await response.json();
    } catch (e) {
      console.error(e);
      searchResults = [];
    }
  }
  
  function selectProduct(product) {
    selectedProduct = product;
    productSearchQuery = product.nombre;
    searchResults = [];
  }
  
  async function submitStockEntry() {
    if (!selectedProduct || !formData.cantidad) {
      alert('Por favor complete los campos requeridos');
      return;
    }
    
    submitting = true;
    successMessage = '';
    
    try {
      const response = await fetch('http://localhost:3000/api/inventario/entrada', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          id_producto_detalle: selectedProduct.id_producto_detalle,
          cantidad: parseInt(formData.cantidad),
          ubicacion_fisica: formData.ubicacion_fisica || null,
          lote: formData.lote || null,
          fecha_vencimiento: formData.fecha_vencimiento || null
        })
      });
      
      if (!response.ok) throw new Error('Error al registrar entrada');
      
      successMessage = 'Entrada de stock registrada exitosamente';
      setTimeout(() => {
        resetModal();
        fetchData(); // Refresh inventory list
      }, 1500);
    } catch (e) {
      console.error(e);
      alert('Error al registrar entrada de stock');
    } finally {
      submitting = false;
    }
  }
  
  function resetModal() {
    showStockModal = false;
    productSearchQuery = '';
    searchResults = [];
    selectedProduct = null;
    formData = {
      cantidad: '',
      ubicacion_fisica: '',
      lote: '',
      fecha_vencimiento: ''
    };
    successMessage = '';
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

  async function openHistorySidebar(product) {
    selectedProductHistory = product;
    showHistorySidebar = true;
    await fetchProductHistory(product.id_producto_detalle);
  }
  
  async function fetchProductHistory(idProductoDetalle) {
    loadingHistory = true;
    try {
      const response = await fetch(`http://localhost:3000/api/inventario/${idProductoDetalle}/historial`);
      if (!response.ok) throw new Error('Error al cargar historial');
      historyMovements = await response.json();
    } catch (e) {
      console.error('Error fetching history:', e);
      historyMovements = [];
    } finally {
      loadingHistory = false;
    }
  }

  function getMovementIcon(tipoMovimiento: string) {
    switch(tipoMovimiento.toLowerCase()) {
      case 'entrada': return 'add_shopping_cart';
      case 'salida':
      case 'venta': return 'shopping_cart';
      case 'ajuste': return 'tune';
      case 'devolucion': return 'keyboard_return';
      default: return 'swap_horiz';
    }
  }
  
  function getMovementColor(tipoMovimiento: string) {
    switch(tipoMovimiento.toLowerCase()) {
      case 'entrada': return 'green';
      case 'salida':
      case 'venta': return 'red';
      case 'ajuste': return 'blue';
      case 'devolucion': return 'yellow';
      default: return 'gray';
    }
  }
  
  function openAdjustModal(product) {
    selectedProductAdjust = product;
    adjustFormData = {
      cantidad_disponible: product.cantidad_disponible.toString(),
      cantidad_minima: product.cantidad_minima.toString(),
      cantidad_maxima: product.cantidad_maxima?.toString() || '',
      ubicacion_fisica: product.ubicacion_fisica || '',
      motivo: ''
    };
    // Reset image URL
    imageUrlAdjust = product.imagen_principal || '';
    showAdjustModal = true;
  }
  

  
  async function submitAdjustment() {
    if (!selectedProductAdjust || !adjustFormData.cantidad_disponible || !adjustFormData.motivo) {
      alert('Por favor complete los campos requeridos (cantidad disponible y motivo)');
      return;
    }
    
    adjustSubmitting = true;
    adjustSuccessMessage = '';
    
    try {
      // Prepare payload
      const payload = {
        cantidad_disponible: parseInt(adjustFormData.cantidad_disponible),
        cantidad_minima: parseInt(adjustFormData.cantidad_minima),
        cantidad_maxima: adjustFormData.cantidad_maxima ? parseInt(adjustFormData.cantidad_maxima) : null,
        ubicacion_fisica: adjustFormData.ubicacion_fisica || null,
        motivo: adjustFormData.motivo
      };
      
      // Add image URL if provided and different from current
      if (imageUrlAdjust && imageUrlAdjust !== selectedProductAdjust.imagen_principal) {
        payload.imagen_principal = imageUrlAdjust;
      }
      
      const response = await fetch(`http://localhost:3000/api/inventario/${selectedProductAdjust.id_producto_detalle}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });
      
      if (!response.ok) throw new Error('Error al actualizar inventario');
      
      adjustSuccessMessage = 'Inventario actualizado exitosamente';
      setTimeout(() => {
        resetAdjustModal();
        fetchData(); // Refresh inventory list
      }, 1500);
    } catch (e) {
      console.error(e);
      alert('Error al actualizar inventario');
    } finally {
      adjustSubmitting = false;
    }
  }
  
  function resetAdjustModal() {
    showAdjustModal = false;
    selectedProductAdjust = null;
    adjustFormData = {
      cantidad_disponible: '',
      cantidad_minima: '',
      cantidad_maxima: '',
      ubicacion_fisica: '',
      motivo: ''
    };
    adjustSuccessMessage = '';
    imageUrlAdjust = '';
  }
  
  async function deleteProduct(product) {
    const confirmDelete = confirm(`¿Está seguro de que desea eliminar "${product.nombre}" del inventario?\n\nEsta acción eliminará el registro de inventario y todo su historial de movimientos.`);
    
    if (!confirmDelete) return;
    
    try {
      const response = await fetch(`http://localhost:3000/api/inventario/${product.id_producto_detalle}`, {
        method: 'DELETE'
      });
      
      if (!response.ok) throw new Error('Error al eliminar producto');
      
      alert('Producto eliminado exitosamente');
      fetchData(); // Refresh inventory list
    } catch (e) {
      console.error(e);
      alert('Error al eliminar producto del inventario');
    }
  }
  

  
  function generateInventoryReport() {
    if (!jsPDF || !autoTable) {
      alert('Cargando librería PDF...');
      return;
    }
    
    const doc = new jsPDF();
    const pageWidth = doc.internal.pageSize.getWidth();
    const pageHeight = doc.internal.pageSize.getHeight();
    
    // Header with gradient background
    doc.setFillColor(59, 130, 246); // Primary blue color
    doc.rect(0, 0, pageWidth, 40, 'F');
    
    // Company name
    doc.setTextColor(255, 255, 255);
    doc.setFontSize(24);
    doc.setFont('helvetica', 'bold');
    doc.text('KronosTech', 15, 20);
    
    // Report title
    doc.setFontSize(14);
    doc.setFont('helvetica', 'normal');
    doc.text('Reporte de Inventario', 15, 30);
    
    // Date
    doc.setFontSize(10);
    const currentDate = new Date().toLocaleDateString('es-ES', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
    doc.text(`Generado: ${currentDate}`, pageWidth - 15, 30, { align: 'right' });
    
    // Reset text color
    doc.setTextColor(0, 0, 0);
    
    let yPosition = 50;
    
    // Summary box
    doc.setFillColor(249, 250, 251);
    doc.roundedRect(15, yPosition, pageWidth - 30, 40, 3, 3, 'F');
    
    doc.setFontSize(11);
    doc.setFont('helvetica', 'bold');
    doc.text('Resumen del Inventario', 20, yPosition + 8);
    
    doc.setFont('helvetica', 'normal');
    doc.setFontSize(9);
    doc.text(`Total de Productos: ${stats.total_productos}`, 20, yPosition + 16);
    doc.text(`Valor Total: ${formatCurrency(stats.valor_total)}`, 20, yPosition + 23);
    doc.text(`Stock Bajo: ${stats.stock_bajo}`, 20, yPosition + 30);
    doc.text(`Sin Stock: ${stats.sin_stock}`, 20, yPosition + 37);
    
    doc.text(`Productos con Stock OK: ${stats.total_productos - stats.stock_bajo - stats.sin_stock}`, 100, yPosition + 16);
    doc.text(`% Stock Bajo: ${((stats.stock_bajo / stats.total_productos) * 100).toFixed(1)}%`, 100, yPosition + 23);
    doc.text(`% Sin Stock: ${((stats.sin_stock / stats.total_productos) * 100).toFixed(1)}%`, 100, yPosition + 30);
    
    yPosition += 50;
    
    // Table title
    doc.setFontSize(12);
    doc.setFont('helvetica', 'bold');
    doc.text('Detalle de Productos', 15, yPosition);
    
    yPosition += 5;
    
    // Prepare table data
    const tableData = inventarioItems.map(item => [
      item.sku || 'N/A',
      item.nombre || 'N/A',
      item.cantidad_disponible?.toString() || '0',
      item.cantidad_minima?.toString() || '0',
      item.stock_estado || 'N/A',
      item.ubicacion_fisica || 'Sin ubicación',
      formatCurrency(item.precio_venta || 0)
    ]);
    
    // Generate table
    autoTable(doc, {
      startY: yPosition,
      head: [['SKU', 'Producto', 'Stock', 'Mín', 'Estado', 'Ubicación', 'Precio']],
      body: tableData,
      theme: 'striped',
      headStyles: {
        fillColor: [59, 130, 246],
        textColor: [255, 255, 255],
        fontStyle: 'bold',
        fontSize: 9
      },
      bodyStyles: {
        fontSize: 8,
        textColor: [50, 50, 50]
      },
      alternateRowStyles: {
        fillColor: [249, 250, 251]
      },
      columnStyles: {
        0: { cellWidth: 20 },
        1: { cellWidth: 50 },
        2: { cellWidth: 15, halign: 'center' },
        3: { cellWidth: 15, halign: 'center' },
        4: { cellWidth: 20, halign: 'center' },
        5: { cellWidth: 30 },
        6: { cellWidth: 25, halign: 'right' }
      },
      margin: { left: 15, right: 15 },
      didDrawCell: function(data) {
        // Color code stock status
        if (data.section === 'body' && data.column.index === 4) {
          const stockEstado = data.cell.raw;
          if (stockEstado === 'agotado') {
            doc.setFillColor(254, 202, 202);
          } else if (stockEstado === 'bajo') {
            doc.setFillColor(254, 243, 199);
          } else if (stockEstado === 'ok') {
            doc.setFillColor(209, 250, 229);
          }
        }
      },
      didDrawPage: function(data) {
        // Footer
        const footerY = pageHeight - 15;
        doc.setFontSize(8);
        doc.setTextColor(128, 128, 128);
        doc.text(
          `Página ${doc.internal.getCurrentPageInfo().pageNumber}`,
          pageWidth / 2,
          footerY,
          { align: 'center' }
        );
        doc.text('KronosTech © 2024', 15, footerY);
      }
    });
    
    // Save the PDF
    doc.save(`Reporte_Inventario_${new Date().toISOString().split('T')[0]}.pdf`);
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
<button on:click={generateInventoryReport} class="flex items-center justify-center gap-2 h-10 px-4 text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-900/50 border border-gray-300 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg w-full sm:w-auto">
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
      <img class="w-10 h-10 rounded-md object-cover" alt={item.nombre} src={item.imagen_principal || '/placeholder-product.svg'}/>
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
        <button on:click={() => openAdjustModal(item)} class="p-2 text-gray-500 hover:text-primary dark:hover:text-primary-400 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800" title="Ajustar inventario"><span class="material-symbols-outlined text-xl">tune</span></button>
        <button on:click={() => openHistorySidebar(item)} class="p-2 text-gray-500 hover:text-primary dark:hover:text-primary-400 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800" title="Ver historial"><span class="material-symbols-outlined text-xl">history</span></button>
        <button on:click={() => deleteProduct(item)} class="p-2 text-gray-500 hover:text-red-600 dark:hover:text-red-400 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800" title="Eliminar del inventario"><span class="material-symbols-outlined text-xl">delete</span></button>
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
<!-- Success Message -->
{#if successMessage}
<div class="p-4 bg-green-100 dark:bg-green-900/30 border border-green-300 dark:border-green-700 rounded-lg">
  <p class="text-green-800 dark:text-green-300 text-sm font-medium">{successMessage}</p>
</div>
{/if}

<!-- Search Section -->
<div class="space-y-4 relative">
<p class="text-gray-800 dark:text-gray-200 text-base font-medium leading-normal">Buscar Producto por nombre o SKU</p>
<label class="flex flex-col w-full">
<div class="flex w-full flex-1 items-stretch rounded-lg h-12">
<div class="text-gray-500 dark:text-gray-400 flex bg-gray-100 dark:bg-gray-700 items-center justify-center pl-4 rounded-l-lg border-r-0">
<span class="material-symbols-outlined">search</span>
</div>
<input 
  bind:value={productSearchQuery}
  on:input={searchProducts}
  class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-0 focus:ring-2 focus:ring-primary/50 border-gray-300 dark:border-gray-600 bg-gray-100 dark:bg-gray-700 h-full placeholder:text-gray-500 dark:placeholder:text-gray-400 px-4 rounded-l-none border-l-0 pl-2 text-base font-normal leading-normal" 
  placeholder="Ej: Intel Core i5" 
/>
</div>
</label>

<!-- Search Results Dropdown -->
{#if searchResults.length > 0}
<div class="absolute z-10 w-full bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-lg shadow-lg max-h-60 overflow-y-auto">
  {#each searchResults as result}
  <button
    on:click={() => selectProduct(result)}
    class="w-full p-3 hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3 text-left border-b border-gray-200 dark:border-gray-700 last:border-b-0"
  >
    <img src={result.imagen_principal || '/placeholder-product.svg'} alt={result.nombre} class="w-10 h-10 rounded object-cover" />
    <div class="flex-1">
      <p class="text-sm font-medium text-gray-900 dark:text-white">{result.nombre}</p>
      <p class="text-xs text-gray-500 dark:text-gray-400">SKU: {result.sku} | Stock: {result.stock_actual}</p>
    </div>
  </button>
  {/each}
</div>
{/if}
</div>

<!-- Selected Product Card -->
{#if selectedProduct}
<div class="p-4 border rounded-lg border-gray-200 dark:border-gray-700 bg-background-light dark:bg-background-dark">
<div class="flex items-center justify-between gap-4">
<div class="flex items-center gap-4 flex-[2_2_0px]">
<div class="w-24 h-24 bg-center bg-no-repeat aspect-square bg-cover rounded-lg flex-shrink-0" style="background-image: url('{selectedProduct.imagen_principal || '/placeholder-product.svg'}');"></div>
<div class="flex flex-col gap-1">
<p class="text-gray-900 dark:text-white text-lg font-bold leading-tight">{selectedProduct.nombre}</p>
<p class="text-gray-500 dark:text-gray-400 text-sm font-normal leading-normal">SKU: {selectedProduct.sku}</p>
<p class="text-gray-500 dark:text-gray-400 text-sm font-normal leading-normal">Stock Actual: <span class="font-semibold text-gray-700 dark:text-gray-300">{selectedProduct.stock_actual}</span></p>
</div>
</div>
</div>
</div>
{:else}
<div class="p-4 border border-dashed rounded-lg border-gray-300 dark:border-gray-700 bg-gray-50 dark:bg-gray-900/50 text-center">
<p class="text-gray-500 dark:text-gray-400 text-sm">Busca y selecciona un producto para continuar</p>
</div>
{/if}
<!-- Form Section -->
<div class="grid grid-cols-1 md:grid-cols-2 gap-x-6 gap-y-5">
<!-- Cantidad -->
<div class="flex flex-col md:col-span-1">
<label class="text-gray-800 dark:text-gray-200 text-base font-medium leading-normal pb-2" for="quantity">Cantidad</label>
<input bind:value={formData.cantidad} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 h-14 placeholder:text-gray-500 dark:placeholder:text-gray-400 p-[15px] text-lg font-normal leading-normal" id="quantity" placeholder="0" type="number"/>
</div>
<!-- Ubicación Física -->
<div class="flex flex-col md:col-span-1">
<label class="text-gray-800 dark:text-gray-200 text-base font-medium leading-normal pb-2" for="location">Ubicación Física</label>
<input bind:value={formData.ubicacion_fisica} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 h-14 placeholder:text-gray-500 dark:placeholder:text-gray-400 p-[15px] text-base font-normal leading-normal" id="location" placeholder="Estantería A-3" type="text"/>
</div>
<!-- Lote -->
<div class="flex flex-col md:col-span-1">
<label class="text-gray-800 dark:text-gray-200 text-base font-medium leading-normal pb-2" for="batch">Lote (Opcional)</label>
<input bind:value={formData.lote} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 h-14 placeholder:text-gray-500 dark:placeholder:text-gray-400 p-[15px] text-base font-normal leading-normal" id="batch" placeholder="Lote-001" type="text"/>
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
<input bind:value={formData.fecha_vencimiento} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 h-14 placeholder:text-gray-500 dark:placeholder:text-gray-400 p-[15px] text-base font-normal leading-normal" id="expiry" placeholder="YYYY-MM-DD" type="date"/>
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
{#if selectedProduct && formData.cantidad}
<div class="bg-primary/10 dark:bg-primary/20 p-4 rounded-lg flex items-center justify-center space-x-4">
<div class="text-center">
<p class="text-sm text-gray-600 dark:text-gray-300">Stock Actual</p>
<p class="text-2xl font-bold text-gray-800 dark:text-white">{selectedProduct.stock_actual}</p>
</div>
<span class="material-symbols-outlined text-primary dark:text-primary/80 text-3xl">arrow_forward</span>
<div class="text-center">
<p class="text-sm text-primary dark:text-primary/80">Nuevo Stock (calculado)</p>
<p class="text-2xl font-bold text-primary dark:text-primary/90">{selectedProduct.stock_actual + parseInt(formData.cantidad || 0)}</p>
</div>
</div>
{/if}
</div>
<!-- Modal Footer -->
<div class="p-6 bg-gray-50 dark:bg-gray-900/50 rounded-b-xl flex justify-end items-center gap-4">
<button on:click={resetModal} class="px-6 py-3 rounded-lg text-base font-medium text-gray-700 dark:text-gray-300 bg-transparent hover:bg-gray-200 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-400 dark:focus:ring-offset-background-dark">
Cancelar
</button>
<button 
  on:click={submitStockEntry}
  disabled={!selectedProduct || !formData.cantidad || submitting}
  class="px-6 py-3 rounded-lg text-base font-medium text-white bg-primary hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary dark:focus:ring-offset-background-dark disabled:opacity-50 disabled:cursor-not-allowed">
{submitting ? 'Guardando...' : 'Registrar Entrada'}
</button>
</div>
</div>
</div>
{/if}

<!-- Adjustment Modal -->
{#if showAdjustModal}
<div class="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6 md:p-8" style="background-color: rgba(0, 0, 0, 0.5);">
<!-- Modal Container -->
<div class="w-full max-w-3xl rounded-xl bg-white dark:bg-gray-800 shadow-2xl flex flex-col">
<!-- Modal Header -->
<div class="p-6 border-b border-gray-200 dark:border-gray-700">
<div class="flex flex-wrap justify-between items-center gap-3">
<div class="flex flex-col gap-1">
<p class="text-gray-900 dark:text-white text-2xl font-bold leading-tight">Ajustar Inventario</p>
<p class="text-gray-500 dark:text-gray-400 text-sm font-normal leading-normal">Modifica las cantidades y configuración del inventario.</p>
</div>
</div>
</div>
<!-- Modal Body -->
<div class="p-6 space-y-6">
<!-- Success Message -->
{#if adjustSuccessMessage}
<div class="p-4 bg-green-100 dark:bg-green-900/30 border border-green-300 dark:border-green-700 rounded-lg">
  <p class="text-green-800 dark:text-green-300 text-sm font-medium">{adjustSuccessMessage}</p>
</div>
{/if}

<!-- Product Info Card -->
{#if selectedProductAdjust}
<div class="p-4 border rounded-lg border-gray-200 dark:border-gray-700 bg-background-light dark:bg-background-dark">
<div class="flex items-center gap-4">
<img src={selectedProductAdjust.imagen_principal || '/placeholder-product.svg'} alt={selectedProductAdjust.nombre} class="w-16 h-16 rounded object-cover" />
<div class="flex-1">
  <p class="text-gray-900 dark:text-white text-lg font-bold leading-tight">{selectedProductAdjust.nombre}</p>
  <p class="text-gray-500 dark:text-gray-400 text-sm font-normal leading-normal">SKU: {selectedProductAdjust.sku}</p>
  <p class="text-gray-500 dark:text-gray-400 text-sm font-normal leading-normal">Stock Actual: <span class="font-semibold text-gray-700 dark:text-gray-300">{selectedProductAdjust.cantidad_disponible}</span></p>
</div>
</div>
</div>
{/if}

<!-- Form Section -->
<div class="grid grid-cols-1 md:grid-cols-2 gap-x-6 gap-y-5">
<!-- Cantidad Disponible -->
<div class="flex flex-col md:col-span-1">
<label class="text-gray-800 dark:text-gray-200 text-base font-medium leading-normal pb-2" for="adjust-quantity">Cantidad Disponible *</label>
<input bind:value={adjustFormData.cantidad_disponible} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 h-14 placeholder:text-gray-500 dark:placeholder:text-gray-400 p-[15px] text-lg font-normal leading-normal" id="adjust-quantity" placeholder="0" type="number"/>
</div>
<!-- Cantidad Mínima -->
<div class="flex flex-col md:col-span-1">
<label class="text-gray-800 dark:text-gray-200 text-base font-medium leading-normal pb-2" for="adjust-min">Cantidad Mínima *</label>
<input bind:value={adjustFormData.cantidad_minima} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 h-14 placeholder:text-gray-500 dark:placeholder:text-gray-400 p-[15px] text-base font-normal leading-normal" id="adjust-min" placeholder="0" type="number"/>
</div>
<!-- Cantidad Máxima -->
<div class="flex flex-col md:col-span-1">
<label class="text-gray-800 dark:text-gray-200 text-base font-medium leading-normal pb-2" for="adjust-max">Cantidad Máxima (Opcional)</label>
<input bind:value={adjustFormData.cantidad_maxima} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 h-14 placeholder:text-gray-500 dark:placeholder:text-gray-400 p-[15px] text-base font-normal leading-normal" id="adjust-max" placeholder="0" type="number"/>
</div>
<!-- Ubicación Física -->
<div class="flex flex-col md:col-span-1">
<label class="text-gray-800 dark:text-gray-200 text-base font-medium leading-normal pb-2" for="adjust-location">Ubicación Física</label>
<input bind:value={adjustFormData.ubicacion_fisica} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 h-14 placeholder:text-gray-500 dark:placeholder:text-gray-400 p-[15px] text-base font-normal leading-normal" id="adjust-location" placeholder="Estantería A-3" type="text"/>
</div>
<!-- URL de Imagen -->
<div class="flex flex-col md:col-span-2">
<label class="text-gray-800 dark:text-gray-200 text-base font-medium leading-normal pb-2" for="adjust-image-url">URL de Imagen del Producto</label>
<input bind:value={imageUrlAdjust} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 h-14 placeholder:text-gray-500 dark:placeholder:text-gray-400 p-[15px] text-base font-normal leading-normal" id="adjust-image-url" placeholder="https://ejemplo.com/imagen.jpg" type="url"/>
<p class="text-xs text-gray-500 dark:text-gray-400 mt-1">Ingresa la URL de la imagen del producto (ej: desde Unsplash, Imgur, etc.)</p>
</div>
<!-- Motivo -->
<div class="flex flex-col md:col-span-2">
<label class="text-gray-800 dark:text-gray-200 text-base font-medium leading-normal pb-2" for="adjust-motivo">Motivo del Ajuste *</label>
<textarea bind:value={adjustFormData.motivo} class="form-textarea w-full min-w-0 flex-1 resize-y overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 h-24 placeholder:text-gray-500 dark:placeholder:text-gray-400 p-[15px] text-base font-normal leading-normal" id="adjust-motivo" placeholder="Ej: Corrección por inventario físico, ajuste por merma, etc."></textarea>
</div>
</div>
</div>
<!-- Modal Footer -->
<div class="p-6 bg-gray-50 dark:bg-gray-900/50 rounded-b-xl flex justify-end items-center gap-4">
<button on:click={resetAdjustModal} class="px-6 py-3 rounded-lg text-base font-medium text-gray-700 dark:text-gray-300 bg-transparent hover:bg-gray-200 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-400 dark:focus:ring-offset-background-dark">
Cancelar
</button>
<button 
  on:click={submitAdjustment}
  disabled={!selectedProductAdjust || !adjustFormData.cantidad_disponible || !adjustFormData.motivo || adjustSubmitting}
  class="px-6 py-3 rounded-lg text-base font-medium text-white bg-primary hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary dark:focus:ring-offset-background-dark disabled:opacity-50 disabled:cursor-not-allowed">
{adjustSubmitting ? 'Guardando...' : 'Guardar Ajuste'}
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
<img class="w-16 h-16 rounded-lg object-cover" alt={selectedProductHistory?.nombre || 'Producto'} src={selectedProductHistory?.imagen_principal || '/placeholder-product.svg'}/>
<div class="flex flex-col gap-1">
<h2 class="text-lg font-semibold text-gray-900 dark:text-white">Historial de Movimientos</h2>
<p class="text-sm text-gray-700 dark:text-gray-300">{selectedProductHistory?.nombre || 'Producto'}</p>
<div class="flex items-center gap-4 text-xs text-gray-500 dark:text-gray-400">
<span>SKU: {selectedProductHistory?.sku || 'N/A'}</span>
<span class="flex items-center gap-1">
<span class="font-medium text-gray-600 dark:text-gray-300">Stock Actual:</span>
<span class="font-bold text-green-600 dark:text-green-400">{selectedProductHistory?.cantidad_disponible || 0}</span>
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
{#if loadingHistory}
  <div class="flex items-center justify-center py-12">
    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
  </div>
{:else if historyMovements.length === 0}
  <div class="flex flex-col items-center justify-center py-12 text-gray-500 dark:text-gray-400">
    <span class="material-symbols-outlined text-6xl mb-4">history</span>
    <p>No hay movimientos registrados</p>
  </div>
{:else}
  <ul class="space-y-4">
  {#each historyMovements as movement}
    {@const color = getMovementColor(movement.tipo_movimiento)}
    {@const icon = getMovementIcon(movement.tipo_movimiento)}
    {@const tipoLower = movement.tipo_movimiento.toLowerCase()}
    {@const isPositive = tipoLower === 'entrada' || tipoLower === 'devolucion'}
    {@const displayCantidad = (tipoLower === 'salida' || tipoLower === 'venta') && movement.cantidad > 0 ? -movement.cantidad : movement.cantidad}
    <li class="flex items-start gap-4">
      <div class="flex flex-col items-center">
        <span class="flex items-center justify-center size-8 rounded-full bg-{color}-100 dark:bg-{color}-900/50 text-{color}-600 dark:text-{color}-400">
          <span class="material-symbols-outlined text-lg">{icon}</span>
        </span>
      </div>
      <div class="flex-1">
        <div class="flex justify-between items-center">
          <p class="text-sm font-medium text-gray-800 dark:text-gray-200">
            <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-{color}-100 text-{color}-800 dark:bg-{color}-900/80 dark:text-{color}-300">
              {movement.tipo_movimiento}
            </span>
          </p>
          <p class="text-xs text-gray-500 dark:text-gray-400">{formatDate(movement.fecha_movimiento)}</p>
        </div>
        <div class="mt-1 flex justify-between items-center">
          <p class="text-sm text-gray-500 dark:text-gray-400">
            Cantidad: <span class="font-semibold text-{isPositive ? 'green' : 'red'}-600 dark:text-{isPositive ? 'green' : 'red'}-400">{isPositive ? '+' : ''}{displayCantidad}</span>
          </p>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            {movement.cantidad_anterior} <span class="material-symbols-outlined text-sm align-middle">arrow_right_alt</span> {movement.cantidad_nueva}
          </p>
        </div>
        {#if movement.motivo || movement.usuario}
          <p class="mt-1 text-xs text-gray-500 dark:text-gray-400">
            {#if movement.motivo}Motivo: {movement.motivo}{/if}
            {#if movement.motivo && movement.usuario} | {/if}
            {#if movement.usuario}Usuario: {movement.usuario}{/if}
          </p>
        {/if}
        {#if movement.notas}
          <p class="mt-1 text-xs italic text-gray-400 dark:text-gray-500">{movement.notas}</p>
        {/if}
      </div>
    </li>
  {/each}
  </ul>
{/if}
</div>
</aside>
{/if}
