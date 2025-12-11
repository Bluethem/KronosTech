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
		switch (estado) {
			case 'pendiente':
				return 'bg-yellow-500/20 text-yellow-200 border-yellow-500/50';
			case 'confirmado':
				return 'bg-blue-500/20 text-blue-200 border-blue-500/50';
			case 'procesando':
				return 'bg-purple-500/20 text-purple-200 border-purple-500/50';
			case 'enviado':
				return 'bg-indigo-500/20 text-indigo-200 border-indigo-500/50';
			case 'entregado':
				return 'bg-green-500/20 text-green-200 border-green-500/50';
			case 'cancelado':
			case 'devuelto':
				return 'bg-red-500/20 text-red-200 border-red-500/50';
			default:
				return 'bg-slate-500/20 text-slate-200 border-slate-500/50';
		}
	}

	function getEstadoPagoClass(estadoPago: string): string {
		switch (estadoPago) {
			case 'completado':
				return 'bg-green-500/20 text-green-200 border-green-500/50';
			case 'procesando':
			case 'pendiente':
				return 'bg-yellow-500/20 text-yellow-200 border-yellow-500/50';
			case 'fallido':
			case 'rechazado':
			case 'cancelado':
				return 'bg-red-500/20 text-red-200 border-red-500/50';
			case 'reembolsado':
			case 'parcialmente_reembolsado':
				return 'bg-orange-500/20 text-orange-200 border-orange-500/50';
			default:
				return 'bg-slate-500/20 text-slate-200 border-slate-500/50';
		}
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
		goto('/pedidos');
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

<div class="min-h-[calc(100vh-4rem)] bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950 text-slate-100">
	<div class="max-w-5xl mx-auto px-4 lg:px-0 py-8 space-y-6">
		<!-- Header -->
		<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-3">
			<div>
				<h1 class="text-3xl font-bold tracking-tight">Detalle del Pedido</h1>
				<p class="text-sm text-slate-400 mt-1">
					Información completa de tu pedido
				</p>
			</div>

			<button
				type="button"
				class="inline-flex items-center justify-center px-4 py-2 text-sm font-medium rounded-2xl border border-white/10 bg-white/5 hover:bg-white/10 transition-colors"
				on:click={goToOrders}
			>
				← Ver todos mis pedidos
			</button>
		</div>

		{#if loading}
			<div class="rounded-3xl border border-white/10 bg-white/5 backdrop-blur-xl p-8 text-center">
				<p class="text-slate-400">Cargando pedido...</p>
			</div>
		{:else if error}
			<div class="rounded-3xl border border-rose-500/40 bg-rose-500/10 backdrop-blur-xl p-8 text-center space-y-4">
				<p class="text-rose-100 text-lg">{error}</p>
				<button
					type="button"
					class="px-6 py-3 rounded-2xl text-sm font-semibold bg-blue-500 text-white hover:bg-blue-600 transition-colors"
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
					<div class="rounded-3xl border border-white/10 bg-white/5 backdrop-blur-xl shadow-[0_18px_50px_rgba(0,0,0,0.45)] p-6 space-y-4">
						<div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
							<div>
								<p class="text-xs text-slate-400 uppercase tracking-wide mb-1">Número de pedido</p>
								<p class="text-2xl font-bold">{venta.numero_pedido}</p>
							</div>
							<div class="flex flex-wrap gap-2">
								<span class={`text-xs px-3 py-1.5 rounded-full border font-medium ${getEstadoBadgeClass(venta.estado)}`}>
									{getEstadoLabel(venta.estado)}
								</span>
								<span class={`text-xs px-3 py-1.5 rounded-full border font-medium ${getEstadoPagoClass(venta.estado_pago)}`}>
									{getEstadoPagoLabel(venta.estado_pago)}
								</span>
							</div>
						</div>

						<div class="grid grid-cols-1 sm:grid-cols-2 gap-4 pt-4 border-t border-white/10">
							<div>
								<p class="text-xs text-slate-400 mb-1">Fecha del pedido</p>
								<p class="text-sm font-medium">{formatDate(venta.fecha_pedido)}</p>
							</div>
							{#if venta.fecha_pago}
								<div>
									<p class="text-xs text-slate-400 mb-1">Fecha de pago</p>
									<p class="text-sm font-medium">{formatDate(venta.fecha_pago)}</p>
								</div>
							{/if}
							{#if venta.fecha_entrega_estimada}
								<div>
									<p class="text-xs text-slate-400 mb-1">Entrega estimada</p>
									<p class="text-sm font-medium">{formatDate(venta.fecha_entrega_estimada)}</p>
								</div>
							{/if}
							{#if venta.numero_tracking}
								<div>
									<p class="text-xs text-slate-400 mb-1">Número de seguimiento</p>
									<p class="text-sm font-mono font-medium">{venta.numero_tracking}</p>
								</div>
							{/if}
						</div>
					</div>

					<!-- Items -->
					<div class="rounded-3xl border border-white/10 bg-white/5 backdrop-blur-xl shadow-[0_18px_50px_rgba(0,0,0,0.45)] p-6 space-y-4">
						<h2 class="text-lg font-semibold">Artículos ({venta.items.length})</h2>

						<div class="space-y-3">
							{#each venta.items as item}
								<div class="rounded-2xl border border-white/10 bg-black/40 p-4 flex gap-4">
									{#if item.imagen}
										<img
											src={item.imagen}
											alt={item.nombre_producto}
											class="w-20 h-20 object-cover rounded-lg"
										/>
									{:else}
										<div class="w-20 h-20 bg-slate-800 rounded-lg flex items-center justify-center">
											<svg class="w-10 h-10 text-slate-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
												<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
											</svg>
										</div>
									{/if}

									<div class="flex-1 min-w-0">
										<p class="text-sm font-semibold">{item.nombre_producto}</p>
										<p class="text-xs text-slate-400 mt-1">SKU: {item.sku}</p>
										<div class="flex flex-wrap items-center gap-3 mt-2">
											<p class="text-xs text-slate-400">
												S/. {item.precio_unitario.toFixed(2)} × {item.cantidad}
											</p>
											{#if item.descuento_unitario > 0}
												<span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-300 border border-green-500/50">
													−S/. {item.descuento_unitario.toFixed(2)} c/u
												</span>
											{/if}
										</div>
									</div>

									<div class="text-right">
										<p class="text-sm font-medium">S/. {item.precio_final.toFixed(2)} c/u</p>
										<p class="text-xs text-slate-400 mt-1">Subtotal:</p>
										<p class="text-base font-semibold">S/. {item.subtotal.toFixed(2)}</p>
									</div>
								</div>
							{/each}
						</div>
					</div>

					<!-- Shipping Address -->
					{#if venta.direccion_envio}
						<div class="rounded-3xl border border-white/10 bg-white/5 backdrop-blur-xl shadow-[0_18px_50px_rgba(0,0,0,0.45)] p-6 space-y-3">
							<h2 class="text-lg font-semibold">Dirección de envío</h2>
							<div class="rounded-2xl border border-white/10 bg-black/40 px-4 py-3 space-y-1">
								<p class="text-sm">{venta.direccion_envio}</p>
								<p class="text-sm text-slate-300">
									{venta.ciudad}{venta.departamento ? `, ${venta.departamento}` : ''}
									{venta.codigo_postal ? ` - ${venta.codigo_postal}` : ''}
								</p>
								{#if venta.telefono_contacto}
									<p class="text-xs text-slate-400 pt-2 border-t border-white/10">
										Tel: {venta.telefono_contacto}
									</p>
								{/if}
								{#if venta.metodo_envio}
									<p class="text-xs text-blue-400 pt-2 border-t border-white/10">
										Método: {venta.metodo_envio}
									</p>
								{/if}
							</div>
						</div>
					{/if}

					<!-- Notes -->
					{#if venta.notas_cliente}
						<div class="rounded-3xl border border-white/10 bg-white/5 backdrop-blur-xl shadow-[0_18px_50px_rgba(0,0,0,0.45)] p-6 space-y-3">
							<h2 class="text-sm font-semibold uppercase tracking-wide text-slate-300">Observaciones de entrega</h2>
							<div class="rounded-2xl border border-white/10 bg-black/40 px-4 py-3">
								<p class="text-sm text-slate-300">{venta.notas_cliente}</p>
							</div>
						</div>
					{/if}
				</div>

				<!-- Sidebar -->
				<aside class="space-y-6 sticky top-24">
					<!-- Order Summary -->
					<div class="rounded-3xl border border-white/10 bg-white/5 backdrop-blur-xl shadow-[0_18px_50px_rgba(0,0,0,0.45)] p-6 space-y-4">
						<h2 class="text-lg font-semibold">Resumen</h2>

						<div class="space-y-2 text-sm">
							<div class="flex justify-between">
								<span class="text-slate-400">Subtotal</span>
								<span class="font-medium">S/. {venta.subtotal.toFixed(2)}</span>
							</div>

							{#if venta.descuento_total > 0}
								<div class="flex justify-between">
									<span class="text-slate-400">Descuento</span>
									<span class="font-medium text-green-400">− S/. {venta.descuento_total.toFixed(2)}</span>
								</div>
							{/if}

							<div class="flex justify-between">
								<span class="text-slate-400">Envío</span>
								<span class="font-medium">
									{venta.costo_envio === 0 ? 'Gratis' : `S/. ${venta.costo_envio.toFixed(2)}`}
								</span>
							</div>

							<div class="border-t border-white/10 pt-3 mt-2 flex justify-between items-center">
								<span class="text-base font-semibold">Total</span>
								<span class="text-2xl font-bold">
									S/. {venta.total.toFixed(2)}
								</span>
							</div>
						</div>
					</div>

					<!-- Actions -->
					<div class="space-y-3">
						<button
							type="button"
							class="w-full px-4 py-3 rounded-2xl text-sm font-semibold bg-blue-500 text-white hover:bg-blue-600 transition-colors shadow-lg shadow-blue-500/30"
							on:click={goToCatalog}
						>
							Continuar comprando
						</button>

						<!-- TODO: Implementar cancelación de pedido -->
						<!-- {#if canCancelOrder()}
							<button
								type="button"
								class="w-full px-4 py-3 rounded-2xl text-sm font-semibold border border-rose-500/50 bg-rose-500/10 text-rose-200 hover:bg-rose-500/20 transition-colors"
								on:click={cancelOrder}
							>
								Cancelar pedido
							</button>
						{/if} -->
					</div>

					<!-- Help Info -->
					<div class="rounded-2xl border border-blue-500/40 bg-blue-500/10 px-4 py-3 text-center">
						<p class="text-xs text-blue-100">
							¿Necesitas ayuda con tu pedido? Contáctanos a soporte@kronostech.com
						</p>
					</div>
				</aside>
			</div>
		{/if}
	</div>
</div>
