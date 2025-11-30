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
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 items-end">
<!-- TextField: Tipo Reporte -->
<label class="flex flex-col min-w-40 flex-1">
<p class="text-gray-800 dark:text-gray-200 text-sm font-medium leading-normal pb-2">Tipo Reporte</p>
<select class="form-select w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 focus:border-primary h-12 placeholder:text-gray-400 px-4 text-base font-normal leading-normal">
<option>General</option>
<option>Movimientos de Inventario</option>
<option>Productos Sin Rotación</option>
<option>Reporte de Valorización</option>
</select>
</label>
<!-- Date Range Picker (Conditional) -->
<div class="flex flex-col flex-1">
<p class="text-gray-800 dark:text-gray-200 text-sm font-medium leading-normal pb-2">Rango de Fechas (Opcional)</p>
<div class="relative">
<div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
<span class="material-symbols-outlined text-gray-500">calendar_month</span>
</div>
<input class="form-input w-full rounded-lg text-gray-900 dark:text-white focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 focus:border-primary h-12 placeholder:text-gray-400 pl-10 pr-4 text-base font-normal leading-normal" placeholder="Seleccionar rango" type="text"/>
</div>
</div>
<!-- TextField: Formato -->
<label class="flex flex-col min-w-40 flex-1">
<p class="text-gray-800 dark:text-gray-200 text-sm font-medium leading-normal pb-2">Formato de Salida</p>
<select class="form-select w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-gray-900 dark:text-white focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 focus:border-primary h-12 placeholder:text-gray-400 px-4 text-base font-normal leading-normal">
<option>Ver en Pantalla</option>
<option>Exportar a PDF</option>
<option>Exportar a Excel</option>
</select>
</label>
<!-- Generate Button -->
<button class="flex min-w-[84px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-12 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em] hover:bg-primary/90 transition-colors">
<span class="material-symbols-outlined mr-2 text-lg">play_arrow</span>
<span class="truncate">Generar</span>
</button>
</div>
</div>
</section>
<!-- Section: Results -->
<section>
<h2 class="text-gray-900 dark:text-white text-[22px] font-bold leading-tight tracking-[-0.015em] pb-4">Resultados</h2>
<!-- Empty State -->
<div class="text-center bg-white dark:bg-gray-900/50 p-12 rounded-xl border-2 border-dashed border-gray-300 dark:border-gray-700">
<span class="material-symbols-outlined text-5xl text-gray-400 dark:text-gray-500 mx-auto">monitoring</span>
<p class="mt-4 text-lg font-semibold text-gray-800 dark:text-gray-200">Aún no has generado un reporte</p>
<p class="mt-1 text-gray-500 dark:text-gray-400">Selecciona los filtros y presiona 'Generar' para ver el reporte.</p>
</div>
<!-- Results Content (Hidden by default, shown after generating report) -->
<div class="hidden">
<div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-6">
<!-- Chart 1 -->
<div class="lg:col-span-2 bg-white dark:bg-gray-900/50 p-6 rounded-xl border border-gray-200 dark:border-gray-800 shadow-sm">
<h3 class="font-bold text-lg text-gray-800 dark:text-gray-200 mb-4">Valor Total por Categoría</h3>
<img class="w-full h-auto" alt="Bar chart showing total value per category" src="https://lh3.googleusercontent.com/aida-public/AB6AXuAnFefhImRhRO4TAbAZCCCwtYBuVZeXxqWhL5mbeNCvddL-4oyTmcryf5sXegQyR9f3PZpevJUfIaEZdK5huUxajXjvCxqYB_rEE9XN_jTwgXz-U_RPVpP2GusXFlikrgMakU31vmv2D4W64lwvk2CRpat8t-2AvD5l1cF9CZdwzLM1mnWqmiJUeG7XGFeA77c9_sChs3nppEM058sHRU6Vzi8mQ4hIdq48pQD8uepjJgSO4Y9EZbDlR95cxJnwtwfgL_tVCLkFOqo"/>
</div>
<!-- Chart 2 -->
<div class="bg-white dark:bg-gray-900/50 p-6 rounded-xl border border-gray-200 dark:border-gray-800 shadow-sm">
<h3 class="font-bold text-lg text-gray-800 dark:text-gray-200 mb-4">Distribución de Stock</h3>
<img class="w-full h-auto" alt="Doughnut chart showing stock distribution" src="https://lh3.googleusercontent.com/aida-public/AB6AXuBXRXfP8OnU8hLdGnZGb4B1bEXyqM2Qo2kN4S8y8T7zuCcXQ3q82RUvEAYRoPWLqgK2is0Unoqb03Fde15TiMJzbJfCB7wNag5ZtTrzhAwh1zGqz4OFkOItWYG7voHXI_-i9WFSlkFZwsrLoW5_r65Tjl5vqeEOq9SiURRtp4Q_XtaKA5PrxUfIKp99FWIU4frrKD80fRuTy34MK36Od5Z8AtIc37Szh-roEfABis_is4zKnNKztbBwV86koPyOpBKQNvaCznFwp5w"/>
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
<th class="px-6 py-3 text-right" scope="col">Stock Actual</th>
<th class="px-6 py-3" scope="col">Último Movimiento</th>
<th class="px-6 py-3 text-right" scope="col">Valor Unitario</th>
<th class="px-6 py-3 text-right" scope="col">Valor Total</th>
</tr>
</thead>
<tbody>
<tr class="bg-white dark:bg-gray-900/50 border-b dark:border-gray-800">
<th class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white" scope="row">ELEC-001</th>
<td class="px-6 py-4">Laptop Pro 15"</td>
<td class="px-6 py-4 text-right">50</td>
<td class="px-6 py-4">2023-10-25</td>
<td class="px-6 py-4 text-right">$1,200.00</td>
<td class="px-6 py-4 text-right">$60,000.00</td>
</tr>
<tr class="bg-white dark:bg-gray-900/50 border-b dark:border-gray-800">
<th class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white" scope="row">ROPA-052</th>
<td class="px-6 py-4">Camiseta Básica - M</td>
<td class="px-6 py-4 text-right">250</td>
<td class="px-6 py-4">2023-10-28</td>
<td class="px-6 py-4 text-right">$25.00</td>
<td class="px-6 py-4 text-right">$6,250.00</td>
</tr>
<tr class="bg-white dark:bg-gray-900/50 border-b dark:border-gray-800">
<th class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white" scope="row">HOG-110</th>
<td class="px-6 py-4">Juego de Sábanas Queen</td>
<td class="px-6 py-4 text-right">120</td>
<td class="px-6 py-4">2023-10-20</td>
<td class="px-6 py-4 text-right">$75.00</td>
<td class="px-6 py-4 text-right">$9,000.00</td>
</tr>
</tbody>
<tfoot>
<tr class="font-semibold text-gray-900 dark:text-white bg-gray-50 dark:bg-gray-800">
<th class="px-6 py-3 text-base" colspan="2" scope="row">Totales</th>
<td class="px-6 py-3 text-right">420</td>
<td class="px-6 py-3"></td>
<td class="px-6 py-3"></td>
<td class="px-6 py-3 text-right">$75,250.00</td>
</tr>
</tfoot>
</table>
</div>
</div>
</div>
</section>
</div>
</main>
</div>
</div>
