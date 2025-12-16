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
			minute: '2-digit'
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

<div class="min-h-[calc(100vh-4rem)] bg-surface-light dark:bg-surface-dark">
	<div class="max-w-5xl mx-auto px-4 lg:px-6 py-8 space-y-6">
		<!-- Header -->
		<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
			<div>
				<h1 class="text-2xl sm:text-3xl font-bold tracking-tight text-text-light dark:text-text-dark">Detalle del Pedido</h1>
				<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
					Información completa de tu pedido
				</p>
			</div>

			<button
				type="button"
				class="inline-flex items-center justify-center px-5 py-2.5 text-sm font-medium rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors text-text-light dark:text-text-dark"
				on:click={goToOrders}
			>
				← Ver todos mis pedidos
			</button>
		</div>

		{#if loading}
			<div class="rounded-2xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-8 text-center shadow-sm">
				<div class="animate-spin rounded-full h-10 w-10 border-b-2 border-primary mx-auto mb-4"></div>
				<p class="text-slate-600 dark:text-slate-400">Cargando pedido...</p>
			</div>
		{:else if error}
			<div class="rounded-2xl border border-red-300 dark:border-red-800 bg-red-50 dark:bg-red-900/20 p-8 text-center space-y-4 shadow-sm">
				<p class="text-red-800 dark:text-red-200 text-lg">{error}</p>
				<button
					type="button"
					class="px-6 py-3 rounded-xl text-sm font-semibold bg-primary text-white hover:bg-primary/90 transition-colors"
					on:click={goToOrders}
				>
					Ver todos mis pedidos
				</button>
			</div>
		{:else if venta}
			<div class="grid grid-cols-1 lg:grid-cols-[minmax(0,2fr),minmax(0,1fr)] gap-6 items-start">
				<!-- Main Content -->
				<div class="space-y-6">
					<!-- Order Header -->
					<div class="rounded-2xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm p-6 space-y-4">
						<div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
							<div>
								<p class="text-xs text-slate-500 dark:text-slate-400 uppercase tracking-wide mb-1">Número de pedido</p>
								<p class="text-2xl font-bold text-text-light dark:text-text-dark">{venta.numero_pedido}</p>
							</div>
							<div class="flex flex-wrap gap-2">
								<span class={`text-xs px-3 py-1.5 rounded-full font-semibold ${getEstadoBadgeClass(venta.estado)}`}>
									{getEstadoLabel(venta.estado)}
								</span>
								<span class={`text-xs px-3 py-1.5 rounded-full font-semibold ${getEstadoPagoClass(venta.estado_pago)}`}>
									{getEstadoPagoLabel(venta.estado_pago)}
								</span>
							</div>
						</div>

						<div class="grid grid-cols-1 sm:grid-cols-2 gap-4 pt-4 border-t border-border-light dark:border-border-dark">
							<div>
								<p class="text-xs text-slate-500 dark:text-slate-400 mb-1">Fecha del pedido</p>
								<p class="text-sm font-medium text-text-light dark:text-text-dark">{formatDate(venta.fecha_pedido)}</p>
							</div>
							{#if venta.fecha_pago}
								<div>
									<p class="text-xs text-slate-500 dark:text-slate-400 mb-1">Fecha de pago</p>
									<p class="text-sm font-medium text-text-light dark:text-text-dark">{formatDate(venta.fecha_pago)}</p>
								</div>
							{/if}
							{#if venta.fecha_entrega_estimada}
								<div>
									<p class="text-xs text-slate-500 dark:text-slate-400 mb-1">Entrega estimada</p>
									<p class="text-sm font-medium text-text-light dark:text-text-dark">{formatDate(venta.fecha_entrega_estimada)}</p>
								</div>
							{/if}
							{#if venta.numero_tracking}
								<div>
									<p class="text-xs text-slate-500 dark:text-slate-400 mb-1">Número de seguimiento</p>
									<p class="text-sm font-mono font-medium text-primary">{venta.numero_tracking}</p>
								</div>
							{/if}
						</div>
					</div>

					<!-- Items -->
					<div class="rounded-2xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm p-6 space-y-4">
						<h2 class="text-lg font-semibold text-text-light dark:text-text-dark">Artículos ({venta.items.length})</h2>

						<div class="space-y-3">
							{#each venta.items as item}
								<div class="rounded-xl border border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-800 p-4 flex gap-4">
									{#if item.imagen}
										<img
											src={item.imagen}
											alt={item.nombre_producto}
											class="w-20 h-20 object-cover rounded-lg"
										/>
									{:else}
										<div class="w-20 h-20 bg-slate-100 dark:bg-slate-700 rounded-lg flex items-center justify-center border border-border-light dark:border-border-dark">
											<svg class="w-10 h-10 text-slate-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
												<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
											</svg>
										</div>
									{/if}

									<div class="flex-1 min-w-0">
										<p class="text-sm font-semibold text-text-light dark:text-text-dark">{item.nombre_producto}</p>
										<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">SKU: {item.sku}</p>
										<div class="flex flex-wrap items-center gap-3 mt-2">
											<p class="text-xs text-slate-500 dark:text-slate-400">
												S/. {item.precio_unitario.toFixed(2)} × {item.cantidad}
											</p>
											{#if item.descuento_unitario > 0}
												<span class="text-xs px-2 py-0.5 rounded-full bg-green-100 dark:bg-green-900/40 text-green-700 dark:text-green-300">
													−S/. {item.descuento_unitario.toFixed(2)} c/u
												</span>
											{/if}
										</div>
									</div>

									<div class="text-right">
										<p class="text-sm font-medium text-text-light dark:text-text-dark">S/. {item.precio_final.toFixed(2)} c/u</p>
										<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">Subtotal:</p>
										<p class="text-base font-bold text-primary">S/. {item.subtotal.toFixed(2)}</p>
									</div>
								</div>
							{/each}
						</div>
					</div>

					<!-- Shipping Address -->
					{#if venta.direccion_envio}
						<div class="rounded-2xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm p-6 space-y-3">
							<h2 class="text-lg font-semibold text-text-light dark:text-text-dark">Dirección de envío</h2>
							<div class="rounded-xl border border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-800 px-4 py-3 space-y-1">
								<p class="text-sm text-text-light dark:text-text-dark">{venta.direccion_envio}</p>
								<p class="text-sm text-slate-600 dark:text-slate-400">
									{venta.ciudad}{venta.departamento ? `, ${venta.departamento}` : ''}
									{venta.codigo_postal ? ` - ${venta.codigo_postal}` : ''}
								</p>
								{#if venta.telefono_contacto}
									<p class="text-xs text-slate-500 dark:text-slate-400 pt-2 border-t border-border-light dark:border-border-dark">
										Tel: {venta.telefono_contacto}
									</p>
								{/if}
								{#if venta.metodo_envio}
									<p class="text-xs text-primary pt-2 border-t border-border-light dark:border-border-dark">
										Método: {venta.metodo_envio}
									</p>
								{/if}
							</div>
						</div>
					{/if}

					<!-- Notes -->
					{#if venta.notas_cliente}
						<div class="rounded-2xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm p-6 space-y-3">
							<h2 class="text-sm font-semibold uppercase tracking-wide text-slate-600 dark:text-slate-400">Observaciones de entrega</h2>
							<div class="rounded-xl border border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-800 px-4 py-3">
								<p class="text-sm text-text-light dark:text-text-dark">{venta.notas_cliente}</p>
							</div>
						</div>
					{/if}
				</div>

				<!-- Sidebar -->
				<aside class="space-y-6 lg:sticky lg:top-24">
					<!-- Order Summary -->
					<div class="rounded-2xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm p-6 space-y-4">
						<h2 class="text-lg font-semibold text-text-light dark:text-text-dark">Resumen</h2>

						<div class="space-y-3 text-sm">
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

							<div class="border-t border-border-light dark:border-border-dark pt-4 mt-3 flex justify-between items-center">
								<span class="text-base font-semibold text-text-light dark:text-text-dark">Total</span>
								<span class="text-2xl font-bold text-primary">
									S/. {venta.total.toFixed(2)}
								</span>
							</div>
						</div>
					</div>

					<!-- Actions -->
					<div class="space-y-3">
						<button
							type="button"
							class="w-full px-4 py-3 rounded-xl text-sm font-semibold bg-primary text-white hover:bg-primary/90 transition-colors shadow-lg shadow-primary/25"
							on:click={goToCatalog}
						>
							Continuar comprando
						</button>

						<button
							type="button"
							class="w-full px-4 py-3 rounded-xl text-sm font-semibold border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 text-text-light dark:text-text-dark hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
							on:click={goToOrders}
						>
							Ver todos mis pedidos
						</button>
					</div>

					<!-- Help Info -->
					<div class="rounded-xl border border-primary/30 bg-primary/5 dark:bg-primary/10 px-4 py-3 text-center">
						<p class="text-xs text-slate-600 dark:text-slate-400">
							¿Necesitas ayuda con tu pedido? Contáctanos a <span class="text-primary font-medium">soporte@kronostech.com</span>
						</p>
					</div>
				</aside>
			</div>
		{/if}
	</div>
</div>