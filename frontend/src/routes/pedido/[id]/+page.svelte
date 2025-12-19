<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { checkoutService } from '$lib/services/checkout';
	import type { Venta } from '$lib/services/checkout';

	let loading = true;
	let error = '';
	let venta: Venta | null = null;

	$: idVenta = Number($page.params.id);

	onMount(async () => {
		await loadOrder();
	});

	async function loadOrder() {
		loading = true;
		error = '';

		try {
			venta = await checkoutService.getPedido(idVenta);
		} catch (err: any) {
			error = err.message ?? 'Error al cargar el pedido';
		} finally {
			loading = false;
		}
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('es-PE', {
			day: '2-digit',
			month: 'long',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit',
			timeZone: 'America/Lima'
		});
	}

	function getEstadoBadgeClass(estado: string): string {
		const classes: Record<string, string> = {
			pendiente: 'bg-yellow-100 dark:bg-yellow-900/40 text-yellow-700 dark:text-yellow-300',
			confirmado: 'bg-blue-100 dark:bg-blue-900/40 text-blue-700 dark:text-blue-300',
			procesando: 'bg-purple-100 dark:bg-purple-900/40 text-purple-700 dark:text-purple-300',
			preparando: 'bg-purple-100 dark:bg-purple-900/40 text-purple-700 dark:text-purple-300',
			enviado: 'bg-indigo-100 dark:bg-indigo-900/40 text-indigo-700 dark:text-indigo-300',
			entregado: 'bg-green-100 dark:bg-green-900/40 text-green-700 dark:text-green-300',
			cancelado: 'bg-red-100 dark:bg-red-900/40 text-red-700 dark:text-red-300',
			devuelto: 'bg-red-100 dark:bg-red-900/40 text-red-700 dark:text-red-300'
		};
		return classes[estado] || 'bg-slate-100 dark:bg-slate-700 text-slate-700 dark:text-slate-300';
	}

	function getEstadoPagoClass(estadoPago: string): string {
		const classes: Record<string, string> = {
			completado: 'bg-green-100 dark:bg-green-900/40 text-green-700 dark:text-green-300',
			procesando: 'bg-yellow-100 dark:bg-yellow-900/40 text-yellow-700 dark:text-yellow-300',
			pendiente: 'bg-yellow-100 dark:bg-yellow-900/40 text-yellow-700 dark:text-yellow-300',
			fallido: 'bg-red-100 dark:bg-red-900/40 text-red-700 dark:text-red-300',
			rechazado: 'bg-red-100 dark:bg-red-900/40 text-red-700 dark:text-red-300',
			cancelado: 'bg-red-100 dark:bg-red-900/40 text-red-700 dark:text-red-300',
			reembolsado: 'bg-orange-100 dark:bg-orange-900/40 text-orange-700 dark:text-orange-300',
			parcialmente_reembolsado: 'bg-orange-100 dark:bg-orange-900/40 text-orange-700 dark:text-orange-300'
		};
		return classes[estadoPago] || 'bg-slate-100 dark:bg-slate-700 text-slate-700 dark:text-slate-300';
	}

	function getEstadoLabel(estado: string): string {
		const labels: Record<string, string> = {
			pendiente: 'Pendiente',
			confirmado: 'Confirmado',
			procesando: 'Procesando',
			enviado: 'Enviado',
			entregado: 'Entregado',
			cancelado: 'Cancelado',
			devuelto: 'Devuelto'
		};
		return labels[estado] || estado;
	}

	function getEstadoPagoLabel(estadoPago: string): string {
		const labels: Record<string, string> = {
			pendiente: 'Pago Pendiente',
			procesando: 'Procesando Pago',
			completado: 'Pago Completado',
			fallido: 'Pago Fallido',
			rechazado: 'Pago Rechazado',
			cancelado: 'Pago Cancelado',
			reembolsado: 'Reembolsado',
			parcialmente_reembolsado: 'Parcialmente Reembolsado'
		};
		return labels[estadoPago] || estadoPago;
	}

	function goToOrders() {
		goto('/cuenta/pedidos');
	}

	function goToCatalog() {
		goto('/catalogo');
	}

	// TODO: Implementar funcionalidad de cancelación de pedido
	// Solo permitir si el estado es 'pendiente' o 'confirmado'
	function canCancelOrder(): boolean {
		return venta?.estado === 'pendiente' || venta?.estado === 'confirmado';
	}

	// TODO: Implementar API endpoint para cancelar pedido
	// async function cancelOrder() {
	//   if (!venta || !canCancelOrder()) return;
	//   try {
	//     await checkoutService.cancelarPedido(venta.id_venta);
	//     await loadOrder(); // Recargar para mostrar nuevo estado
	//   } catch (err: any) {
	//     error = err.message ?? 'Error al cancelar pedido';
	//   }
	// }
</script>

<svelte:head>
	<title>Detalle del Pedido {venta?.numero_pedido ?? ''} | KronosTech</title>
</svelte:head>

<div class="min-h-screen bg-surface-light dark:bg-surface-dark">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
		<!-- Header -->
		<div class="flex items-center justify-between gap-4 mb-6">
			<h1 class="text-xl sm:text-2xl font-bold text-text-light dark:text-text-dark">
				Detalle del Pedido
			</h1>
			<button
				type="button"
				class="flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-lg border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 hover:bg-slate-100 dark:hover:bg-slate-700 text-text-light dark:text-text-dark"
				on:click={goToOrders}
			>
				← Mis pedidos
			</button>
		</div>

		{#if loading}
			<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-8 text-center">
				<div class="animate-spin rounded-full h-10 w-10 border-b-2 border-primary mx-auto mb-4"></div>
				<p class="text-slate-600 dark:text-slate-400">Cargando pedido...</p>
			</div>
		{:else if error}
			<div class="rounded-xl border border-red-300 dark:border-red-800 bg-red-50 dark:bg-red-900/20 p-8 text-center space-y-4">
				<p class="text-red-800 dark:text-red-200 text-lg">{error}</p>
				<button
					type="button"
					class="px-6 py-3 rounded-xl text-sm font-semibold bg-primary text-white hover:bg-primary/90"
					on:click={goToOrders}
				>
					Ver todos mis pedidos
				</button>
			</div>
		{:else if venta}
			<!-- Layout de 2 columnas -->
			<div class="flex flex-col lg:flex-row gap-6">
				<!-- IZQUIERDA: Productos -->
				<div class="flex-1 space-y-4">
					<!-- Header del pedido (compacto) -->
					<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-4 flex flex-wrap items-center justify-between gap-3">
						<div class="flex items-center gap-3">
							<div class="w-10 h-10 rounded-lg bg-primary/10 dark:bg-primary/20 flex items-center justify-center">
								<svg class="w-5 h-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
								</svg>
							</div>
							<div>
								<p class="text-sm font-bold text-text-light dark:text-text-dark">{venta.numero_pedido}</p>
								<p class="text-xs text-slate-500 dark:text-slate-400">{formatDate(venta.fecha_pedido)}</p>
							</div>
						</div>
						<div class="flex flex-wrap gap-2">
							<span class={`text-xs px-3 py-1 rounded-full font-semibold ${getEstadoBadgeClass(venta.estado)}`}>
								{getEstadoLabel(venta.estado)}
							</span>
							<span class={`text-xs px-3 py-1 rounded-full font-semibold ${getEstadoPagoClass(venta.estado_pago)}`}>
								{getEstadoPagoLabel(venta.estado_pago)}
							</span>
						</div>
					</div>

					<!-- Lista de productos -->
					<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 overflow-hidden">
						<div class="px-4 py-3 border-b border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-800">
							<h2 class="text-sm font-semibold text-text-light dark:text-text-dark">
								Productos del pedido ({venta.items.length})
							</h2>
						</div>

						<div class="divide-y divide-border-light dark:divide-border-dark">
							{#each venta.items as item}
								<div class="p-4 flex gap-4 hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors">
									{#if item.imagen}
										<img
											src={item.imagen}
											alt={item.nombre_producto}
											class="w-16 h-16 object-cover rounded-lg flex-shrink-0"
										/>
									{:else}
										<div class="w-16 h-16 bg-slate-100 dark:bg-slate-700 rounded-lg flex items-center justify-center flex-shrink-0">
											<svg class="w-8 h-8 text-slate-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
												<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
											</svg>
										</div>
									{/if}

									<div class="flex-1 min-w-0">
										<p class="text-sm font-semibold text-text-light dark:text-text-dark truncate">{item.nombre_producto}</p>
										<p class="text-xs text-slate-500 dark:text-slate-400">SKU: {item.sku}</p>
										<div class="flex items-center gap-2 mt-1">
											<span class="text-xs text-slate-600 dark:text-slate-400">
												Cant: {item.cantidad}
											</span>
											{#if item.descuento_unitario > 0}
												<span class="text-[10px] px-1.5 py-0.5 rounded bg-green-100 dark:bg-green-900/40 text-green-700 dark:text-green-300">
													−S/{item.descuento_unitario.toFixed(2)}
												</span>
											{/if}
										</div>
									</div>

									<div class="text-right flex-shrink-0">
										<p class="text-sm font-bold text-primary">S/. {item.subtotal.toFixed(2)}</p>
										<p class="text-[11px] text-slate-500 dark:text-slate-400">
											S/{item.precio_final.toFixed(2)} c/u
										</p>
									</div>
								</div>
							{/each}
						</div>
					</div>

					<!-- Tracking (si existe) -->
					{#if venta.numero_tracking}
						<div class="rounded-xl border border-indigo-200 dark:border-indigo-800 bg-indigo-50 dark:bg-indigo-900/20 p-4 flex items-center gap-3">
							<div class="w-10 h-10 rounded-lg bg-indigo-100 dark:bg-indigo-800 flex items-center justify-center flex-shrink-0">
								<svg class="w-5 h-5 text-indigo-600 dark:text-indigo-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
								</svg>
							</div>
							<div class="flex-1">
								<p class="text-xs text-indigo-600 dark:text-indigo-400 font-medium">Número de seguimiento</p>
								<p class="text-sm font-mono font-bold text-indigo-700 dark:text-indigo-300">{venta.numero_tracking}</p>
							</div>
						</div>
					{/if}
				</div>

				<!-- DERECHA: Resumen y Acciones -->
				<div class="w-full lg:w-[360px] lg:flex-shrink-0 space-y-4 lg:sticky lg:top-20 lg:self-start">
					<!-- Resumen de precios -->
					<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-5 space-y-4">
						<h2 class="text-base font-semibold text-text-light dark:text-text-dark">Resumen del pedido</h2>

						<div class="space-y-2 text-sm">
							<div class="flex justify-between">
								<span class="text-slate-600 dark:text-slate-400">Subtotal</span>
								<span class="font-medium text-text-light dark:text-text-dark">S/. {venta.subtotal.toFixed(2)}</span>
							</div>

							{#if venta.descuento_total > 0}
								<div class="flex justify-between">
									<span class="text-slate-600 dark:text-slate-400">Descuento</span>
									<span class="font-medium text-green-600 dark:text-green-400">− S/. {venta.descuento_total.toFixed(2)}</span>
								</div>
							{/if}

							<div class="flex justify-between">
								<span class="text-slate-600 dark:text-slate-400">Envío</span>
								<span class="font-medium text-text-light dark:text-text-dark">
									{venta.costo_envio === 0 ? 'Gratis' : `S/. ${venta.costo_envio.toFixed(2)}`}
								</span>
							</div>

							<div class="border-t border-border-light dark:border-border-dark pt-3 mt-2 flex justify-between items-center">
								<span class="text-sm font-semibold text-text-light dark:text-text-dark">Total pagado</span>
								<span class="text-xl font-bold text-primary">
									S/. {venta.total.toFixed(2)}
								</span>
							</div>
						</div>
					</div>

					<!-- Información de envío -->
					{#if venta.direccion_envio}
						<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-4 space-y-3">
							<h3 class="text-xs font-semibold uppercase tracking-wide text-slate-500 dark:text-slate-400 flex items-center gap-2">
								<svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
								</svg>
								Dirección de envío
							</h3>
							<div class="text-sm text-text-light dark:text-text-dark space-y-1">
								<p>{venta.direccion_envio}</p>
								<p class="text-slate-600 dark:text-slate-400">
									{venta.ciudad}{venta.departamento ? `, ${venta.departamento}` : ''}
								</p>
								{#if venta.telefono_contacto}
									<p class="text-xs text-slate-500 dark:text-slate-400">Tel: {venta.telefono_contacto}</p>
								{/if}
							</div>
							{#if venta.metodo_envio}
								<div class="pt-2 border-t border-border-light dark:border-border-dark">
									<p class="text-xs text-primary font-medium">📦 {venta.metodo_envio}</p>
								</div>
							{/if}
						</div>
					{/if}

					<!-- Fechas importantes -->
					<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-4 space-y-2">
						<h3 class="text-xs font-semibold uppercase tracking-wide text-slate-500 dark:text-slate-400">Fechas</h3>
						<div class="grid grid-cols-2 gap-3 text-xs">
							<div>
								<p class="text-slate-500 dark:text-slate-400">Pedido</p>
								<p class="font-medium text-text-light dark:text-text-dark">{formatDate(venta.fecha_pedido).split(',')[0]}</p>
							</div>
							{#if venta.fecha_pago}
								<div>
									<p class="text-slate-500 dark:text-slate-400">Pago</p>
									<p class="font-medium text-text-light dark:text-text-dark">{formatDate(venta.fecha_pago).split(',')[0]}</p>
								</div>
							{/if}
							{#if venta.fecha_entrega_estimada}
								<div class="col-span-2">
									<p class="text-slate-500 dark:text-slate-400">Entrega estimada</p>
									<p class="font-medium text-primary">{formatDate(venta.fecha_entrega_estimada).split(',')[0]}</p>
								</div>
							{/if}
						</div>
					</div>

					<!-- Notas del cliente -->
					{#if venta.notas_cliente}
						<div class="rounded-xl border border-amber-200 dark:border-amber-800 bg-amber-50 dark:bg-amber-900/20 p-4">
							<h3 class="text-xs font-semibold text-amber-700 dark:text-amber-400 mb-1 flex items-center gap-1">
								<svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z" />
								</svg>
								Notas de entrega
							</h3>
							<p class="text-sm text-amber-800 dark:text-amber-300">{venta.notas_cliente}</p>
						</div>
					{/if}

					<!-- Acciones -->
					<div class="space-y-2">
						<button
							type="button"
							class="w-full px-4 py-3 rounded-xl text-sm font-semibold bg-primary text-white hover:bg-primary/90 transition-colors shadow-lg shadow-primary/25"
							on:click={goToCatalog}
						>
							Continuar comprando
						</button>

						<button
							type="button"
							class="w-full px-4 py-2.5 rounded-xl text-sm font-medium border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 text-text-light dark:text-text-dark hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
							on:click={goToOrders}
						>
							Ver todos mis pedidos
						</button>
					</div>

					<!-- Ayuda -->
					<div class="rounded-xl border border-primary/20 bg-primary/5 dark:bg-primary/10 px-4 py-3 text-center">
						<p class="text-xs text-slate-600 dark:text-slate-400">
							¿Dudas? <span class="text-primary font-medium">soporte@kronostech.com</span>
						</p>
					</div>
				</div>
			</div>
		{/if}
	</div>
</div>