<div class="relative flex h-auto min-h-screen w-full flex-col group/design-root overflow-x-hidden bg-background-light dark:bg-background-dark font-display">
<div class="layout-container flex h-full grow flex-col">
<div class="px-4 sm:px-8 md:px-12 lg:px-16 xl:px-20 flex flex-1 justify-center py-5">
<div class="layout-content-container flex flex-col w-full max-w-7xl flex-1 gap-8">
<!-- Page Heading -->
<div class="flex flex-wrap justify-between items-center gap-4">
<div class="flex flex-col gap-2">
<p class="text-slate-900 dark:text-white text-3xl md:text-4xl font-black leading-tight tracking-[-0.033em]">Reportes de Ventas</p>
<p class="text-slate-500 dark:text-slate-400 text-base font-normal leading-normal">Seleccione los filtros para visualizar y exportar los datos de ventas.</p>
</div>
</div>
<!-- Filters Section -->
<div class="flex flex-col gap-6 rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 p-6">
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 items-end">
<div class="flex flex-col col-span-1 md:col-span-2">
<p class="text-slate-900 dark:text-white text-base font-medium leading-normal pb-2">Rango de Fechas</p>
<div class="flex items-center border border-slate-200 dark:border-slate-700 rounded-lg">
<input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden text-slate-900 dark:text-white focus:outline-0 focus:ring-0 bg-transparent h-14 placeholder:text-slate-500 p-3.5 text-base font-normal leading-normal border-0" readonly type="text" value="01 Oct, 2023"/>
<span class="text-slate-400 dark:text-slate-500">-</span>
<input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden text-slate-900 dark:text-white focus:outline-0 focus:ring-0 bg-transparent h-14 placeholder:text-slate-500 p-3.5 text-base font-normal leading-normal border-0 text-right" readonly type="text" value="07 Nov, 2023"/>
<div class="px-3">
<span class="material-symbols-outlined text-slate-500 dark:text-slate-400">calendar_today</span>
</div>
</div>
</div>
<label class="flex flex-col min-w-40 flex-1">
<p class="text-slate-900 dark:text-white text-base font-medium leading-normal pb-2">Tipo de Reporte</p>
<div class="relative">
<select class="form-select appearance-none w-full min-w-0 flex-1 rounded-lg text-slate-900 dark:text-white focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-900 h-14 placeholder:text-slate-500 p-3.5 pr-10 text-base font-normal leading-normal">
<option>Ventas por producto</option>
<option>Ventas por día</option>
<option>Ventas por categoría</option>
<option>Ventas por estado</option>
<option>Productos más vendidos</option>
</select>
<span class="material-symbols-outlined text-slate-500 dark:text-slate-400 absolute right-3 top-1/2 -translate-y-1/2 pointer-events-none">expand_more</span>
</div>
</label>
<label class="flex flex-col min-w-40 flex-1">
<p class="text-slate-900 dark:text-white text-base font-medium leading-normal pb-2">Formato de exportación</p>
<div class="relative">
<select class="form-select appearance-none w-full min-w-0 flex-1 rounded-lg text-slate-900 dark:text-white focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-900 h-14 placeholder:text-slate-500 p-3.5 pr-10 text-base font-normal leading-normal">
<option>Excel</option>
<option>PDF</option>
<option>CSV</option>
</select>
<span class="material-symbols-outlined text-slate-500 dark:text-slate-400 absolute right-3 top-1/2 -translate-y-1/2 pointer-events-none">expand_more</span>
</div>
</label>
</div>
<div class="flex justify-start">
<button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-12 px-5 bg-primary text-white gap-2 text-base font-bold leading-normal tracking-[0.015em] hover:bg-primary/90 transition-colors">
<span class="material-symbols-outlined text-white" style="font-size: 20px;">download</span>
<span class="truncate">Generar Reporte</span>
</button>
</div>
</div>
<!-- Results Section -->
<div class="flex flex-col gap-8">
<!-- Stats Cards -->
<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
<div class="flex flex-col gap-2 rounded-xl p-6 border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900">
<p class="text-slate-900 dark:text-white text-base font-medium leading-normal">Total de Pedidos</p>
<p class="text-slate-900 dark:text-white tracking-light text-3xl font-bold leading-tight">1,234</p>
<p class="text-green-600 dark:text-green-500 text-base font-medium leading-normal">+5.2%</p>
</div>
<div class="flex flex-col gap-2 rounded-xl p-6 border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900">
<p class="text-slate-900 dark:text-white text-base font-medium leading-normal">Ingresos Totales</p>
<p class="text-slate-900 dark:text-white tracking-light text-3xl font-bold leading-tight">$85,940.50</p>
<p class="text-green-600 dark:text-green-500 text-base font-medium leading-normal">+8.1%</p>
</div>
<div class="flex flex-col gap-2 rounded-xl p-6 border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900">
<p class="text-slate-900 dark:text-white text-base font-medium leading-normal">Ticket Promedio</p>
<p class="text-slate-900 dark:text-white tracking-light text-3xl font-bold leading-tight">$69.64</p>
<p class="text-red-600 dark:text-red-500 text-base font-medium leading-normal">-1.3%</p>
</div>
<div class="flex flex-col gap-2 rounded-xl p-6 border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900">
<p class="text-slate-900 dark:text-white text-base font-medium leading-normal">Tasa de Conversión</p>
<p class="text-slate-900 dark:text-white tracking-light text-3xl font-bold leading-tight">4.5%</p>
<p class="text-green-600 dark:text-green-500 text-base font-medium leading-normal">+0.5%</p>
</div>
</div>
<!-- Charts Section -->
<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
<div class="lg:col-span-2 flex flex-col gap-4 rounded-xl p-6 border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900">
<p class="text-slate-900 dark:text-white text-lg font-bold">Evolución de Ventas por Día</p>
<div class="w-full h-80 bg-slate-100 dark:bg-slate-800/50 rounded-lg flex items-center justify-center">
<span class="material-symbols-outlined text-slate-400 dark:text-slate-600" style="font-size: 48px;">show_chart</span>
</div>
</div>
<div class="flex flex-col gap-4 rounded-xl p-6 border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900">
<p class="text-slate-900 dark:text-white text-lg font-bold">Distribución por Estado de Pedido</p>
<div class="w-full h-80 bg-slate-100 dark:bg-slate-800/50 rounded-lg flex items-center justify-center">
<span class="material-symbols-outlined text-slate-400 dark:text-slate-600" style="font-size: 48px;">donut_small</span>
</div>
</div>
<div class="lg:col-span-3 flex flex-col gap-4 rounded-xl p-6 border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900">
<p class="text-slate-900 dark:text-white text-lg font-bold">Top 10 Productos Más Vendidos</p>
<div class="w-full h-80 bg-slate-100 dark:bg-slate-800/50 rounded-lg flex items-center justify-center">
<span class="material-symbols-outlined text-slate-400 dark:text-slate-600" style="font-size: 48px;">bar_chart</span>
</div>
</div>
</div>
<!-- Data Table -->
<div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 overflow-hidden">
<div class="p-6">
<h3 class="text-lg font-bold text-slate-900 dark:text-white">Detalle de Ventas por Producto</h3>
</div>
<div class="overflow-x-auto">
<table class="w-full text-left">
<thead class="border-b border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-slate-800/50">
<tr>
<th class="p-4 text-sm font-semibold text-slate-600 dark:text-slate-300">
<div class="flex items-center gap-1 cursor-pointer">Producto <span class="material-symbols-outlined text-base">unfold_more</span></div>
</th>
<th class="p-4 text-sm font-semibold text-slate-600 dark:text-slate-300">
<div class="flex items-center gap-1 cursor-pointer">ID Producto <span class="material-symbols-outlined text-base">unfold_more</span></div>
</th>
<th class="p-4 text-sm font-semibold text-slate-600 dark:text-slate-300">
<div class="flex items-center gap-1 cursor-pointer">Categoría <span class="material-symbols-outlined text-base">unfold_more</span></div>
</th>
<th class="p-4 text-sm font-semibold text-slate-600 dark:text-slate-300">
<div class="flex items-center gap-1 cursor-pointer">Unidades Vendidas <span class="material-symbols-outlined text-base">unfold_more</span></div>
</th>
<th class="p-4 text-sm font-semibold text-slate-600 dark:text-slate-300">
<div class="flex items-center gap-1 cursor-pointer">Monto Total <span class="material-symbols-outlined text-base">unfold_more</span></div>
</th>
</tr>
</thead>
<tbody class="divide-y divide-slate-200 dark:divide-slate-800">
<tr class="hover:bg-primary/5 dark:hover:bg-primary/10">
<td class="p-4 text-sm text-slate-800 dark:text-slate-200">Camiseta Clásica Blanca</td>
<td class="p-4 text-sm text-slate-500 dark:text-slate-400">P001</td>
<td class="p-4 text-sm text-slate-500 dark:text-slate-400">Ropa</td>
<td class="p-4 text-sm text-slate-800 dark:text-slate-200">150</td>
<td class="p-4 text-sm text-slate-800 dark:text-slate-200 font-medium">$2,250.00</td>
</tr>
<tr class="hover:bg-primary/5 dark:hover:bg-primary/10">
<td class="p-4 text-sm text-slate-800 dark:text-slate-200">Zapatillas Deportivas Runner</td>
<td class="p-4 text-sm text-slate-500 dark:text-slate-400">P012</td>
<td class="p-4 text-sm text-slate-500 dark:text-slate-400">Calzado</td>
<td class="p-4 text-sm text-slate-800 dark:text-slate-200">95</td>
<td class="p-4 text-sm text-slate-800 dark:text-slate-200 font-medium">$7,125.00</td>
</tr>
<tr class="hover:bg-primary/5 dark:hover:bg-primary/10">
<td class="p-4 text-sm text-slate-800 dark:text-slate-200">Gorra de Béisbol Negra</td>
<td class="p-4 text-sm text-slate-500 dark:text-slate-400">P005</td>
<td class="p-4 text-sm text-slate-500 dark:text-slate-400">Accesorios</td>
<td class="p-4 text-sm text-slate-800 dark:text-slate-200">120</td>
<td class="p-4 text-sm text-slate-800 dark:text-slate-200 font-medium">$1,800.00</td>
</tr>
<tr class="hover:bg-primary/5 dark:hover:bg-primary/10">
<td class="p-4 text-sm text-slate-800 dark:text-slate-200">Mochila Urbana Resistente</td>
<td class="p-4 text-sm text-slate-500 dark:text-slate-400">P023</td>
<td class="p-4 text-sm text-slate-500 dark:text-slate-400">Accesorios</td>
<td class="p-4 text-sm text-slate-800 dark:text-slate-200">80</td>
<td class="p-4 text-sm text-slate-800 dark:text-slate-200 font-medium">$4,000.00</td>
</tr>
<tr class="hover:bg-primary/5 dark:hover:bg-primary/10">
<td class="p-4 text-sm text-slate-800 dark:text-slate-200">Jeans Slim Fit Azul</td>
<td class="p-4 text-sm text-slate-500 dark:text-slate-400">P002</td>
<td class="p-4 text-sm text-slate-500 dark:text-slate-400">Ropa</td>
<td class="p-4 text-sm text-slate-800 dark:text-slate-200">110</td>
<td class="p-4 text-sm text-slate-800 dark:text-slate-200 font-medium">$5,490.00</td>
</tr>
</tbody>
</table>
</div>
<div class="p-4 border-t border-slate-200 dark:border-slate-800 flex justify-between items-center text-sm text-slate-600 dark:text-slate-400">
<div>Mostrando 1-5 de 25 resultados</div>
<div class="flex gap-2">
<button class="px-3 py-1.5 rounded-md border border-slate-300 dark:border-slate-700 hover:bg-slate-100 dark:hover:bg-slate-800">Anterior</button>
<button class="px-3 py-1.5 rounded-md border border-slate-300 dark:border-slate-700 hover:bg-slate-100 dark:hover:bg-slate-800">Siguiente</button>
</div>
</div>
</div>
</div>
</div>
</div>
</div>
</div>
