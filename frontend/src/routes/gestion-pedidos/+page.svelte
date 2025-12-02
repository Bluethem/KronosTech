<script lang="ts">
	import type { PageData } from './$types';
	
	export let data: PageData;
	
	$: orders = data.orders || [];
	
	function formatDate(dateString: string | null): string {
		if (!dateString) return 'N/A';
		const date = new Date(dateString);
		return date.toLocaleString('es-ES', {
			year: 'numeric',
			month: '2-digit',
			day: '2-digit',
			hour: '2-digit',
			minute: '2-digit'
		});
	}
	
	function formatCurrency(amount: number | null): string {
		if (amount === null) return 'S/ 0.00';
		return `S/ ${amount.toFixed(2)}`;
	}
	
	function getStatusBadgeClass(status: string | null): string {
		if (!status) return 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
		
		const normalizedStatus = status.toLowerCase();
		const statusMap: Record<string, string> = {
			'pendiente': 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/40 dark:text-yellow-300 border border-yellow-200 dark:border-yellow-800',
			'confirmado': 'bg-blue-100 text-blue-800 dark:bg-blue-900/40 dark:text-blue-300 border border-blue-200 dark:border-blue-800',
			'procesando': 'bg-purple-100 text-purple-800 dark:bg-purple-900/40 dark:text-purple-300 border border-purple-200 dark:border-purple-800',
			'en proceso': 'bg-purple-100 text-purple-800 dark:bg-purple-900/40 dark:text-purple-300 border border-purple-200 dark:border-purple-800',
			'enviado': 'bg-indigo-100 text-indigo-800 dark:bg-indigo-900/40 dark:text-indigo-300 border border-indigo-200 dark:border-indigo-800',
			'entregado': 'bg-emerald-100 text-emerald-800 dark:bg-emerald-900/40 dark:text-emerald-300 border border-emerald-200 dark:border-emerald-800',
			'cancelado': 'bg-red-100 text-red-800 dark:bg-red-900/40 dark:text-red-300 border border-red-200 dark:border-red-800',
			'devuelto': 'bg-orange-100 text-orange-800 dark:bg-orange-900/40 dark:text-orange-300 border border-orange-200 dark:border-orange-800'
		};
		
		return statusMap[normalizedStatus] || 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
	}
	
	function getPaymentStatusBadgeClass(status: string | null): string {
		if (!status) return 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
		
		const normalizedStatus = status.toLowerCase();
		const statusMap: Record<string, string> = {
			'pendiente': 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/40 dark:text-yellow-300 border border-yellow-200 dark:border-yellow-800',
			'procesando': 'bg-blue-100 text-blue-800 dark:bg-blue-900/40 dark:text-blue-300 border border-blue-200 dark:border-blue-800',
			'completado': 'bg-green-100 text-green-800 dark:bg-green-900/40 dark:text-green-300 border border-green-200 dark:border-green-800',
			'fallido': 'bg-red-100 text-red-800 dark:bg-red-900/40 dark:text-red-300 border border-red-200 dark:border-red-800',
			'rechazado': 'bg-red-100 text-red-800 dark:bg-red-900/40 dark:text-red-300 border border-red-200 dark:border-red-800',
			'cancelado': 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300 border border-gray-200 dark:border-gray-600',
			'reembolsado': 'bg-pink-100 text-pink-800 dark:bg-pink-900/40 dark:text-pink-300 border border-pink-200 dark:border-pink-800',
			'parcialmente_reembolsado': 'bg-orange-100 text-orange-800 dark:bg-orange-900/40 dark:text-orange-300 border border-orange-200 dark:border-orange-800'
		};
		
		return statusMap[normalizedStatus] || 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
	}
	
	function isPendingOver24h(fechaPedido: string | null, estado: string | null): boolean {
		if (!fechaPedido || estado !== 'Pendiente') return false;
		const orderDate = new Date(fechaPedido);
		const now = new Date();
		const diffHours = (now.getTime() - orderDate.getTime()) / (1000 * 60 * 60);
		return diffHours > 24;
	}
</script>

<div class="relative flex h-auto min-h-screen w-full flex-col group/design-root bg-background-light dark:bg-background-dark font-display text-[#111418] dark:text-gray-200">
<div class="layout-container flex h-full grow flex-col">
<main class="flex-1 p-4 sm:p-6 lg:p-8">
<div class="mx-auto max-w-7xl">
<div class="flex flex-col sm:flex-row flex-wrap justify-between items-start gap-4 mb-6">
<div class="flex items-center gap-4">
<h1 class="text-3xl lg:text-4xl font-black tracking-tight text-[#111418] dark:text-white">Gestión de Pedidos</h1>
<div class="flex h-8 shrink-0 items-center justify-center gap-x-2 rounded-full bg-red-100 dark:bg-red-500/20 px-4">
<span class="material-symbols-outlined text-red-600 dark:text-red-400 !text-base">notifications</span>
<p class="text-red-600 dark:text-red-400 text-sm font-medium">Pedidos Pendientes &gt; 24h</p>
</div>
</div>
<button class="flex items-center justify-center gap-2 min-w-[84px] cursor-pointer rounded-lg h-10 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em]">
<span class="material-symbols-outlined">download</span>
<span class="truncate">Generar Reporte</span>
</button>
</div>
<div class="mb-6 rounded-xl border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900/50 p-4">
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
<div class="lg:col-span-4">
<label class="flex flex-col min-w-40 h-12 w-full">
<div class="flex w-full flex-1 items-stretch rounded-lg h-full">
<div class="text-[#617589] dark:text-gray-400 flex border-none bg-background-light dark:bg-background-dark items-center justify-center pl-4 rounded-l-lg border-r-0">
<span class="material-symbols-outlined">search</span>
</div>
<input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-gray-200 focus:outline-0 focus:ring-2 focus:ring-primary/50 border-none bg-background-light dark:bg-background-dark h-full placeholder:text-[#617589] dark:placeholder:text-gray-500 px-4 rounded-l-none border-l-0 pl-2 text-sm font-normal" placeholder="Buscar por número de pedido, cliente, email..."/>
</div>
</label>
</div>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2 dark:text-gray-300">Estado del Pedido</p>
<select class="form-select flex w-full min-w-0 flex-1 overflow-hidden rounded-lg text-[#111418] dark:text-gray-200 focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 focus:border-primary/50 h-12 placeholder:text-[#617589] px-3 text-sm font-normal">
<option>Todos</option>
<option>Pendiente</option>
<option>Confirmado</option>
<option>En Proceso</option>
<option>Enviado</option>
<option>Entregado</option>
<option>Cancelado</option>
</select>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2 dark:text-gray-300">Estado de Pago</p>
<select class="form-select flex w-full min-w-0 flex-1 overflow-hidden rounded-lg text-[#111418] dark:text-gray-200 focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 focus:border-primary/50 h-12 placeholder:text-[#617589] px-3 text-sm font-normal">
<option>Todos</option>
<option>Pendiente</option>
<option>Completado</option>
<option>Fallido</option>
<option>Reembolsado</option>
</select>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2 dark:text-gray-300">Fecha desde</p>
<div class="relative">
<input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-gray-200 focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 focus:border-primary/50 h-12 placeholder:text-[#617589] dark:placeholder:text-gray-500 p-3 pl-10 text-sm font-normal" placeholder="DD/MM/AAAA" type="text"/>
<span class="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-gray-400">calendar_today</span>
</div>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2 dark:text-gray-300">Fecha hasta</p>
<div class="relative">
<input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-gray-200 focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 focus:border-primary/50 h-12 placeholder:text-[#617589] dark:placeholder:text-gray-500 p-3 pl-10 text-sm font-normal" placeholder="DD/MM/AAAA" type="text"/>
<span class="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-gray-400">calendar_today</span>
</div>
</label>
</div>
<div class="flex justify-end gap-3 mt-4 pt-4 border-t border-gray-200 dark:border-gray-800">
<button class="flex min-w-[84px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-transparent text-gray-600 dark:text-gray-300 text-sm font-bold hover:bg-gray-100 dark:hover:bg-gray-800">
<span class="truncate">Limpiar filtros</span>
</button>
<button class="flex min-w-[84px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em]">
<span class="truncate">Aplicar filtros</span>
</button>
</div>
</div>
<div class="overflow-x-auto rounded-xl border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900/50">
<table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
<thead class="text-xs text-gray-700 dark:text-gray-300 uppercase bg-gray-50 dark:bg-gray-800">
<tr>
<th class="px-6 py-3" scope="col">Número de Pedido</th>
<th class="px-6 py-3" scope="col">Cliente</th>
<th class="px-6 py-3" scope="col">Fecha/Hora</th>
<th class="px-6 py-3" scope="col">Estado Pedido</th>
<th class="px-6 py-3" scope="col">Estado Pago</th>
<th class="px-6 py-3 text-right" scope="col">Total</th>
<th class="px-6 py-3 text-center" scope="col">Acciones</th>
</tr>
</thead>
<tbody>
{#each orders as order}
<tr class="{isPendingOver24h(order.fecha_pedido, order.estado) ? 'bg-red-50 dark:bg-red-500/10' : 'bg-white dark:bg-gray-900/50'} border-b dark:border-gray-800 hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors">
<th class="px-6 py-4 font-medium text-gray-900 dark:text-white whitespace-nowrap" scope="row">
<div class="flex items-center gap-2">
<span>{order.numero_pedido}</span>
{#if isPendingOver24h(order.fecha_pedido, order.estado)}
<span class="material-symbols-outlined text-red-500" style="font-size: 18px;" title="Pendiente > 24h">warning</span>
{/if}
</div>
</th>
<td class="px-6 py-4">
<div class="font-semibold text-gray-800 dark:text-gray-100">{order.nombre_usuario || 'N/A'}</div>
<div class="text-xs text-gray-500 dark:text-gray-400">{order.email_usuario || 'N/A'}</div>
</td>
<td class="px-6 py-4">{formatDate(order.fecha_pedido)}</td>
<td class="px-6 py-4">
<span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getStatusBadgeClass(order.estado)}">{order.estado || 'N/A'}</span>
</td>
<td class="px-6 py-4">
<span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getPaymentStatusBadgeClass(order.estado_pago)}">{order.estado_pago || 'N/A'}</span>
</td>
<td class="px-6 py-4 text-right font-medium text-gray-800 dark:text-gray-100">{formatCurrency(order.total)}</td>
<td class="px-6 py-4 text-center">
<div class="flex items-center justify-center gap-2">
<a href="/gestion-pedidos/{order.id_venta}" class="p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors flex items-center justify-center"><span class="material-symbols-outlined">visibility</span></a>
<button class="p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"><span class="material-symbols-outlined">edit</span></button>
<button class="p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"><span class="material-symbols-outlined">local_shipping</span></button>
</div>
</td>
</tr>
{:else}
<tr>
<td colspan="7" class="px-6 py-8 text-center text-gray-500 dark:text-gray-400">
No se encontraron pedidos
</td>
</tr>
{/each}
</tbody>
</table>
</div>
<nav aria-label="Table navigation" class="flex flex-col md:flex-row justify-between items-start md:items-center space-y-3 md:space-y-0 p-4">
<span class="text-sm font-normal text-gray-500 dark:text-gray-400">
                            Mostrando <span class="font-semibold text-gray-900 dark:text-white">1-10</span> de <span class="font-semibold text-gray-900 dark:text-white">1000</span> resultados
                        </span>
<div class="flex items-center gap-4">
<div class="flex items-center gap-2 text-sm">
<span class="text-gray-500 dark:text-gray-400">Items por página:</span>
<select class="form-select bg-gray-50 dark:bg-gray-800 border-gray-300 dark:border-gray-600 rounded-lg text-sm p-1 pr-7 focus:ring-primary focus:border-primary">
<option>10</option>
<option>25</option>
<option>50</option>
<option>100</option>
</select>
</div>
<ul class="inline-flex items-stretch -space-x-px">
<li>
<a class="flex items-center justify-center h-full py-1.5 px-3 ml-0 text-gray-500 bg-white rounded-l-lg border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white" href="#">
<span class="sr-only">Previous</span>
<span class="material-symbols-outlined" style="font-size: 18px;">chevron_left</span>
</a>
</li>
<li>
<a aria-current="page" class="flex items-center justify-center text-sm z-10 py-2 px-3 leading-tight text-primary bg-blue-50 border border-primary hover:bg-blue-100 hover:text-blue-700 dark:border-gray-700 dark:bg-gray-700 dark:text-white" href="#">1</a>
</li>
<li>
<a class="flex items-center justify-center text-sm py-2 px-3 leading-tight text-gray-500 bg-white border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white" href="#">2</a>
</li>
<li>
<a class="flex items-center justify-center text-sm py-2 px-3 leading-tight text-gray-500 bg-white border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white" href="#">3</a>
</li>
<li>
<a class="flex items-center justify-center h-full py-1.5 px-3 leading-tight text-gray-500 bg-white rounded-r-lg border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white" href="#">
<span class="sr-only">Next</span>
<span class="material-symbols-outlined" style="font-size: 18px;">chevron_right</span>
</a>
</li>
</ul>
</div>
</nav>
</div>
</main>
</div>
</div>
