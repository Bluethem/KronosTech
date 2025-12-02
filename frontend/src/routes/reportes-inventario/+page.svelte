<script lang="ts">
  import { onMount } from 'svelte';
  
  // State
  let tipoReporte = $state('General');
  let formatoSalida = $state('Ver en Pantalla');
  let fechaInicio = $state('');
  let fechaFin = $state('');
  let loading = $state(false);
  let showResults = $state(false);
  
  // Report data
  let reportData = $state(null);
  let valorizacionData = $state(null);
  
  async function generarReporte() {
    loading = true;
    showResults = false;
    
    try {
      if (tipoReporte === 'General') {
        const response = await fetch('http://localhost:3000/api/inventario/reportes/general');
        if (!response.ok) throw new Error('Error al generar reporte');
        reportData = await response.json();
        
        if (formatoSalida === 'Exportar a PDF') {
          exportarPDF();
        } else if (formatoSalida === 'Exportar a Excel') {
          exportarExcel();
        } else {
          showResults = true;
        }
      } else if (tipoReporte === 'Reporte de Valorización') {
        const response = await fetch('http://localhost:3000/api/inventario/reportes/valorizacion');
        if (!response.ok) throw new Error('Error al generar reporte');
        valorizacionData = await response.json();
        
        if (formatoSalida === 'Exportar a PDF') {
          exportarValorizacionPDF();
        } else if (formatoSalida === 'Exportar a Excel') {
          exportarValorizacionExcel();
        } else {
          showResults = true;
        }
      }
    } catch (e) {
      console.error(e);
      alert('Error al generar el reporte');
    } finally {
      loading = false;
    }
  }
  
  function exportarPDF() {
    if (!reportData) return;
    
    // Crear contenido HTML para el PDF
    let htmlContent = `
      <html>
      <head>
        <style>
          body { font-family: Arial, sans-serif; padding: 20px; }
          h1 { color: #333; margin-bottom: 20px; }
          .summary { display: flex; gap: 20px; margin-bottom: 30px; }
          .summary-card { flex: 1; padding: 15px; background: #f5f5f5; border-radius: 8px; }
          .summary-card h3 { margin: 0 0 10px 0; color: #666; font-size: 14px; }
          .summary-card p { margin: 0; font-size: 24px; font-weight: bold; }
          table { width: 100%; border-collapse: collapse; margin-top: 20px; }
          th, td { border: 1px solid #ddd; padding: 12px; text-align: left; }
          th { background-color: #4CAF50; color: white; }
          tr:nth-child(even) { background-color: #f2f2f2; }
          .text-right { text-align: right; }
          tfoot { font-weight: bold; background-color: #e8e8e8; }
        </style>
      </head>
      <body>
        <h1>Reporte General de Inventario</h1>
        <div class="summary">
          <div class="summary-card">
            <h3>Total Productos</h3>
            <p>${reportData.total_productos}</p>
          </div>
          <div class="summary-card">
            <h3>Total Stock</h3>
            <p>${reportData.total_stock}</p>
          </div>
          <div class="summary-card">
            <h3>Valor Total</h3>
            <p>${formatCurrency(reportData.valor_total_inventario)}</p>
          </div>
        </div>
        <table>
          <thead>
            <tr>
              <th>SKU</th>
              <th>Producto</th>
              <th>Categoría</th>
              <th>Marca</th>
              <th class="text-right">Stock</th>
              <th class="text-right">Precio</th>
              <th class="text-right">Valor Total</th>
              <th>Último Movimiento</th>
            </tr>
          </thead>
          <tbody>
            ${reportData.items.map(item => `
              <tr>
                <td>${item.sku}</td>
                <td>${item.nombre}</td>
                <td>${item.categoria}</td>
                <td>${item.marca}</td>
                <td class="text-right">${item.stock_actual}</td>
                <td class="text-right">${formatCurrency(item.precio_venta)}</td>
                <td class="text-right">${formatCurrency(item.valor_total)}</td>
                <td>${formatDate(item.ultimo_movimiento)}</td>
              </tr>
            `).join('')}
          </tbody>
          <tfoot>
            <tr>
              <td colspan="4">TOTALES</td>
              <td class="text-right">${reportData.total_stock}</td>
              <td></td>
              <td class="text-right">${formatCurrency(reportData.valor_total_inventario)}</td>
              <td></td>
            </tr>
          </tfoot>
        </table>
      </body>
      </html>
    `;
    
    // Crear ventana para imprimir
    const printWindow = window.open('', '', 'height=600,width=800');
    printWindow.document.write(htmlContent);
    printWindow.document.close();
    printWindow.focus();
    
    // Esperar a que cargue y luego imprimir
    setTimeout(() => {
      printWindow.print();
      printWindow.close();
    }, 250);
  }
  
  function exportarExcel() {
    if (!reportData) return;
    
    // Crear datos para Excel
    const excelData = [
      ['Reporte General de Inventario'],
      [],
      ['Total Productos:', reportData.total_productos],
      ['Total Stock:', reportData.total_stock],
      ['Valor Total:', reportData.valor_total_inventario],
      [],
      ['SKU', 'Producto', 'Categoría', 'Marca', 'Stock Actual', 'Precio', 'Valor Total', 'Último Movimiento'],
      ...reportData.items.map(item => [
        item.sku,
        item.nombre,
        item.categoria,
        item.marca,
        item.stock_actual,
        parseFloat(item.precio_venta),
        parseFloat(item.valor_total),
        formatDate(item.ultimo_movimiento)
      ]),
      [],
      ['TOTALES', '', '', '', reportData.total_stock, '', parseFloat(reportData.valor_total_inventario), '']
    ];
    
    // Convertir a CSV
    const csv = excelData.map(row => 
      row.map(cell => {
        const cellStr = String(cell ?? '');
        // Escapar comillas y envolver en comillas si contiene coma o comilla
        if (cellStr.includes(',') || cellStr.includes('"') || cellStr.includes('\n')) {
          return '"' + cellStr.replace(/"/g, '""') + '"';
        }
        return cellStr;
      }).join(',')
    ).join('\n');
    
    // Crear blob y descargar
    const blob = new Blob(['\ufeff' + csv], { type: 'text/csv;charset=utf-8;' });
    const link = document.createElement('a');
    const url = URL.createObjectURL(blob);
    link.setAttribute('href', url);
    link.setAttribute('download', `reporte_inventario_${new Date().toISOString().split('T')[0]}.csv`);
    link.style.visibility = 'hidden';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  }
  
  function exportarValorizacionPDF() {
    if (!valorizacionData) return;
    
    let htmlContent = `
      <html>
      <head>
        <style>
          body { font-family: Arial, sans-serif; padding: 20px; }
          h1 { color: #333; margin-bottom: 20px; }
          .summary-card { padding: 20px; background: #f5f5f5; border-radius: 8px; margin-bottom: 30px; }
          .summary-card h3 { margin: 0 0 10px 0; color: #666; font-size: 14px; }
          .summary-card p { margin: 0; font-size: 32px; font-weight: bold; }
          table { width: 100%; border-collapse: collapse; margin-top: 20px; }
          th, td { border: 1px solid #ddd; padding: 12px; text-align: left; }
          th { background-color: #4CAF50; color: white; }
          tr:nth-child(even) { background-color: #f2f2f2; }
          .text-right { text-align: right; }
          tfoot { font-weight: bold; background-color: #e8e8e8; }
        </style>
      </head>
      <body>
        <h1>Reporte de Valorización de Inventario</h1>
        <div class="summary-card">
          <h3>Valor Total del Inventario</h3>
          <p>${formatCurrency(valorizacionData.valor_total_general)}</p>
        </div>
        <table>
          <thead>
            <tr>
              <th>Categoría</th>
              <th class="text-right">Total Productos</th>
              <th class="text-right">Total Stock</th>
              <th class="text-right">Valor Total</th>
              <th class="text-right">% del Total</th>
            </tr>
          </thead>
          <tbody>
            ${valorizacionData.por_categoria.map(cat => {
              const porcentaje = (cat.valor_total / valorizacionData.valor_total_general) * 100;
              return `
                <tr>
                  <td>${cat.categoria}</td>
                  <td class="text-right">${cat.total_productos}</td>
                  <td class="text-right">${cat.total_stock}</td>
                  <td class="text-right">${formatCurrency(cat.valor_total)}</td>
                  <td class="text-right">${porcentaje.toFixed(2)}%</td>
                </tr>
              `;
            }).join('')}
          </tbody>
          <tfoot>
            <tr>
              <td>Total General</td>
              <td class="text-right">${valorizacionData.por_categoria.reduce((sum, cat) => sum + Number(cat.total_productos), 0)}</td>
              <td class="text-right">${valorizacionData.por_categoria.reduce((sum, cat) => sum + Number(cat.total_stock), 0)}</td>
              <td class="text-right">${formatCurrency(valorizacionData.valor_total_general)}</td>
              <td class="text-right">100.00%</td>
            </tr>
          </tfoot>
        </table>
      </body>
      </html>
    `;
    
    const printWindow = window.open('', '', 'height=600,width=800');
    printWindow.document.write(htmlContent);
    printWindow.document.close();
    printWindow.focus();
    
    setTimeout(() => {
      printWindow.print();
      printWindow.close();
    }, 250);
  }
  
  function exportarValorizacionExcel() {
    if (!valorizacionData) return;
    
    const excelData = [
      ['Reporte de Valorización de Inventario'],
      [],
      ['Valor Total del Inventario:', parseFloat(valorizacionData.valor_total_general)],
      [],
      ['Categoría', 'Total Productos', 'Total Stock', 'Valor Total', '% del Total'],
      ...valorizacionData.por_categoria.map(cat => {
        const porcentaje = (cat.valor_total / valorizacionData.valor_total_general) * 100;
        return [
          cat.categoria,
          cat.total_productos,
          cat.total_stock,
          parseFloat(cat.valor_total),
          porcentaje.toFixed(2) + '%'
        ];
      }),
      [],
      [
        'Total General',
        valorizacionData.por_categoria.reduce((sum, cat) => sum + Number(cat.total_productos), 0),
        valorizacionData.por_categoria.reduce((sum, cat) => sum + Number(cat.total_stock), 0),
        parseFloat(valorizacionData.valor_total_general),
        '100.00%'
      ]
    ];
    
    const csv = excelData.map(row => 
      row.map(cell => {
        const cellStr = String(cell ?? '');
        if (cellStr.includes(',') || cellStr.includes('"') || cellStr.includes('\n')) {
          return '"' + cellStr.replace(/"/g, '""') + '"';
        }
        return cellStr;
      }).join(',')
    ).join('\n');
    
    const blob = new Blob(['\ufeff' + csv], { type: 'text/csv;charset=utf-8;' });
    const link = document.createElement('a');
    const url = URL.createObjectURL(blob);
    link.setAttribute('href', url);
    link.setAttribute('download', `reporte_valorizacion_${new Date().toISOString().split('T')[0]}.csv`);
    link.style.visibility = 'hidden';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  }
  
  function formatCurrency(amount: number) {
    return new Intl.NumberFormat('es-PE', { style: 'currency', currency: 'PEN' }).format(amount);
  }
  
  function formatDate(dateString: string | null) {
    if (!dateString) return 'N/A';
    return new Date(dateString).toLocaleDateString('es-PE');
  }
</script>

<div class="relative flex h-auto min-h-screen w-full flex-col group/design-root overflow-x-hidden bg-background-light dark:bg-background-dark font-display">
<div class="layout-container flex h-full grow flex-col">
<main class="flex-1 p-8 overflow-y-auto">
<div class="max-w-7xl mx-auto">
<!-- PageHeading -->
<div class="flex flex-wrap justify-between items-center gap-3 mb-8">
<div class="flex min-w-72 flex-col gap-2">
<h1 class="text-gray-900 dark:text-white text-4xl font-black leading-tight tracking-[-0.033em]">Reportes de Inventario</h1>
<p class="text-gray-500 dark:text-gray-400 text-base font-normal leading-normal">Genera y visualiza reportes detallados de tu inventario.</p>
</div>
</div>
<!-- Section: Configuration -->
<section class="mb-8">
<h2 class="text-gray-900 dark:text-white text-[22px] font-bold leading-tight tracking-[-0.015em] pb-4">Configuración</h2>
<div class="bg-white dark:bg-gray-900/50 p-6 rounded-xl border border-gray-200 dark:border-gray-800 shadow-sm">
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-6 items-end">
<!-- TextField: Tipo Reporte -->
<label class="flex flex-col min-w-40 flex-1">
<p class="text-gray-800 dark:text-gray-200 text-sm font-medium leading-normal pb-2">Tipo Reporte</p>
<select bind:value={tipoReporte} class="form-select w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 focus:border-primary h-12 placeholder:text-gray-400 px-4 text-base font-normal leading-normal">
<option>General</option>
<option>Reporte de Valorización</option>
</select>
</label>
<!-- Date Range Picker (Conditional) -->
<div class="flex flex-col flex-1">
<p class="text-gray-800 dark:text-gray-200 text-sm font-medium leading-normal pb-2">Fecha Inicio (Opcional)</p>
<div class="relative">
<div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
<span class="material-symbols-outlined text-gray-500">calendar_month</span>
</div>
<input bind:value={fechaInicio} class="form-input w-full rounded-lg text-gray-900 dark:text-white focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 focus:border-primary h-12 placeholder:text-gray-400 pl-10 pr-4 text-base font-normal leading-normal" type="date"/>
</div>
</div>
<div class="flex flex-col flex-1">
<p class="text-gray-800 dark:text-gray-200 text-sm font-medium leading-normal pb-2">Fecha Fin (Opcional)</p>
<div class="relative">
<div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
<span class="material-symbols-outlined text-gray-500">calendar_month</span>
</div>
<input bind:value={fechaFin} class="form-input w-full rounded-lg text-gray-900 dark:text-white focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 focus:border-primary h-12 placeholder:text-gray-400 pl-10 pr-4 text-base font-normal leading-normal" type="date"/>
</div>
</div>
<!-- TextField: Formato -->
<label class="flex flex-col min-w-40 flex-1">
<p class="text-gray-800 dark:text-gray-200 text-sm font-medium leading-normal pb-2">Formato de Salida</p>
<select bind:value={formatoSalida} class="form-select w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 focus:border-primary h-12 placeholder:text-gray-400 px-4 text-base font-normal leading-normal">
<option>Ver en Pantalla</option>
<option>Exportar a PDF</option>
<option>Exportar a Excel</option>
</select>
</label>
<!-- Generate Button -->
<button on:click={generarReporte} disabled={loading} class="flex min-w-[84px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-12 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em] hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
<span class="material-symbols-outlined mr-2 text-lg">play_arrow</span>
<span class="truncate">{loading ? 'Generando...' : 'Generar'}</span>
</button>
</div>
</div>
</section>
<!-- Section: Results -->
<section>
<h2 class="text-gray-900 dark:text-white text-[22px] font-bold leading-tight tracking-[-0.015em] pb-4">Resultados</h2>
<!-- Empty State -->
{#if !showResults}
<div class="text-center bg-white dark:bg-gray-900/50 p-12 rounded-xl border-2 border-dashed border-gray-300 dark:border-gray-700">
<span class="material-symbols-outlined text-5xl text-gray-400 dark:text-gray-500 mx-auto">monitoring</span>
<p class="mt-4 text-lg font-semibold text-gray-800 dark:text-gray-200">Aún no has generado un reporte</p>
<p class="mt-1 text-gray-500 dark:text-gray-400">Selecciona los filtros y presiona 'Generar' para ver el reporte.</p>
</div>
{/if}

<!-- Results Content - General Report -->
{#if showResults && tipoReporte === 'General' && reportData}
<div class="space-y-6">
<!-- Summary Cards -->
<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
<div class="bg-white dark:bg-gray-900/50 p-6 rounded-xl border border-gray-200 dark:border-gray-800 shadow-sm">
<p class="text-gray-500 dark:text-gray-400 text-sm font-medium">Total Productos</p>
<p class="text-gray-900 dark:text-white text-3xl font-bold mt-2">{reportData.total_productos}</p>
</div>
<div class="bg-white dark:bg-gray-900/50 p-6 rounded-xl border border-gray-200 dark:border-gray-800 shadow-sm">
<p class="text-gray-500 dark:text-gray-400 text-sm font-medium">Total Stock</p>
<p class="text-gray-900 dark:text-white text-3xl font-bold mt-2">{reportData.total_stock}</p>
</div>
<div class="bg-white dark:bg-gray-900/50 p-6 rounded-xl border border-gray-200 dark:border-gray-800 shadow-sm">
<p class="text-gray-500 dark:text-gray-400 text-sm font-medium">Valor Total</p>
<p class="text-gray-900 dark:text-white text-3xl font-bold mt-2">{formatCurrency(reportData.valor_total_inventario)}</p>
</div>
</div>

<!-- Data Table -->
<div class="bg-white dark:bg-gray-900/50 rounded-xl border border-gray-200 dark:border-gray-800 shadow-sm overflow-hidden">
<div class="overflow-x-auto">
<table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
<thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-800 dark:text-gray-400">
<tr>
<th class="px-6 py-3" scope="col">SKU</th>
<th class="px-6 py-3" scope="col">Producto</th>
<th class="px-6 py-3" scope="col">Categoría</th>
<th class="px-6 py-3" scope="col">Marca</th>
<th class="px-6 py-3 text-right" scope="col">Stock Actual</th>
<th class="px-6 py-3 text-right" scope="col">Precio</th>
<th class="px-6 py-3 text-right" scope="col">Valor Total</th>
<th class="px-6 py-3" scope="col">Último Movimiento</th>
</tr>
</thead>
<tbody>
{#each reportData.items as item}
<tr class="bg-white dark:bg-gray-900/50 border-b dark:border-gray-800">
<th class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white" scope="row">{item.sku}</th>
<td class="px-6 py-4">{item.nombre}</td>
<td class="px-6 py-4">{item.categoria}</td>
<td class="px-6 py-4">{item.marca}</td>
<td class="px-6 py-4 text-right">{item.stock_actual}</td>
<td class="px-6 py-4 text-right">{formatCurrency(item.precio_venta)}</td>
<td class="px-6 py-4 text-right">{formatCurrency(item.valor_total)}</td>
<td class="px-6 py-4">{formatDate(item.ultimo_movimiento)}</td>
</tr>
{/each}
</tbody>
<tfoot>
<tr class="font-semibold text-gray-900 dark:text-white bg-gray-50 dark:bg-gray-800">
<th class="px-6 py-3 text-base" colspan="4" scope="row">Totales</th>
<td class="px-6 py-3 text-right">{reportData.total_stock}</td>
<td class="px-6 py-3"></td>
<td class="px-6 py-3 text-right">{formatCurrency(reportData.valor_total_inventario)}</td>
<td class="px-6 py-3"></td>
</tr>
</tfoot>
</table>
</div>
</div>
</div>
{/if}

<!-- Results Content - Valorización Report -->
{#if showResults && tipoReporte === 'Reporte de Valorización' && valorizacionData}
<div class="space-y-6">
<!-- Summary Card -->
<div class="bg-white dark:bg-gray-900/50 p-6 rounded-xl border border-gray-200 dark:border-gray-800 shadow-sm">
<p class="text-gray-500 dark:text-gray-400 text-sm font-medium">Valor Total del Inventario</p>
<p class="text-gray-900 dark:text-white text-4xl font-bold mt-2">{formatCurrency(valorizacionData.valor_total_general)}</p>
</div>

<!-- Data Table -->
<div class="bg-white dark:bg-gray-900/50 rounded-xl border border-gray-200 dark:border-gray-800 shadow-sm overflow-hidden">
<div class="overflow-x-auto">
<table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
<thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-800 dark:text-gray-400">
<tr>
<th class="px-6 py-3" scope="col">Categoría</th>
<th class="px-6 py-3 text-right" scope="col">Total Productos</th>
<th class="px-6 py-3 text-right" scope="col">Total Stock</th>
<th class="px-6 py-3 text-right" scope="col">Valor Total</th>
<th class="px-6 py-3 text-right" scope="col">% del Total</th>
</tr>
</thead>
<tbody>
{#each valorizacionData.por_categoria as cat}
{@const porcentaje = (cat.valor_total / valorizacionData.valor_total_general) * 100}
<tr class="bg-white dark:bg-gray-900/50 border-b dark:border-gray-800">
<th class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white" scope="row">{cat.categoria}</th>
<td class="px-6 py-4 text-right">{cat.total_productos}</td>
<td class="px-6 py-4 text-right">{cat.total_stock}</td>
<td class="px-6 py-4 text-right">{formatCurrency(cat.valor_total)}</td>
<td class="px-6 py-4 text-right">{porcentaje.toFixed(2)}%</td>
</tr>
{/each}
</tbody>
<tfoot>
<tr class="font-semibold text-gray-900 dark:text-white bg-gray-50 dark:bg-gray-800">
<th class="px-6 py-3 text-base" scope="row">Total General</th>
<td class="px-6 py-3 text-right">{valorizacionData.por_categoria.reduce((sum, cat) => sum + Number(cat.total_productos), 0)}</td>
<td class="px-6 py-3 text-right">{valorizacionData.por_categoria.reduce((sum, cat) => sum + Number(cat.total_stock), 0)}</td>
<td class="px-6 py-3 text-right">{formatCurrency(valorizacionData.valor_total_general)}</td>
<td class="px-6 py-3 text-right">100.00%</td>
</tr>
</tfoot>
</table>
</div>
</div>
</div>
{/if}
</section>
</div>
</main>
</div>
</div>

