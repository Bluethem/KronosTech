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
			year: 'numeric'
		});
	}

	function getEstadoBadgeClass(estado: string): string {
		switch (estado) {
			case 'pendiente':
				return 'bg-yellow-500 text-yellow-200 border-yellow-500';
			case 'confirmado':
				return 'bg-blue-500 text-blue-200 border-blue-500';
			case 'procesando':
				return 'bg-purple-500 text-purple-200 border-purple-500';
			case 'enviado':
				return 'bg-indigo-500 text-indigo-200 border-indigo-500';
			case 'entregado':
				return 'bg-green-500 text-green-200 border-green-500';
			case 'cancelado':
			case 'devuelto':
				return 'bg-red-500 text-red-200 border-red-500';
			default:
				return 'bg-slate-500 text-slate-200 border-slate-500';
		}
	}

	function getEstadoPagoClass(estadoPago: string): string {
		switch (estadoPago) {
			case 'completado':
				return 'bg-green-500 text-green-200 border-green-500';
			case 'procesando':
			case 'pendiente':
				return 'bg-yellow-500 text-yellow-200 border-yellow-500';
			case 'fallido':
			case 'rechazado':
			case 'cancelado':
				return 'bg-red-500 text-red-200 border-red-500';
			case 'reembolsado':
			case 'parcialmente_reembolsado':
				return 'bg-orange-500 text-orange-200 border-orange-500';
			default:
				return 'bg-slate-500 text-slate-200 border-slate-500';
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

	function continueShopping() {
		goto('/catalogo');
	}

	function viewOrderDetails() {
		goto(`/pedido/${idVenta}`);
	}
</script>

<svelte:head>
	<title>Pedido Confirmado | KronosTech</title>
</svelte:head>

<div class="min-h-[calc(100vh-4rem)] bg-gradient-to-b from-white via-slate-50 to-white text-slate-900">
	<div class="max-w-4xl mx-auto px-4 lg:px-0 py-8 space-y-6">
		{#if loading}
			<div class="rounded-3xl border border-slate-200 bg-white backdrop-blur-xl p-8 text-center shadow-sm">
				<p class="text-slate-600">Cargando pedido...</p>
			</div>
		{:else if error}
			<div class="rounded-3xl border border-rose-300 bg-rose-50 backdrop-blur-xl p-8 text-center space-y-4 shadow-sm">
				<p class="text-rose-900 text-lg">{error}</p>
				<button
					type="button"
					class="px-6 py-3 rounded-2xl text-sm font-semibold bg-blue-500 text-white hover:bg-blue-600 transition-colors"
					on:click={continueShopping}
				>
					Volver al catálogo
				</button>
			</div>
		{:else if venta}
			<!-- Success Header -->
			<div class="rounded-3xl border border-green-300 bg-green-50 backdrop-blur-xl p-8 text-center space-y-3 shadow-sm">
				<div class="flex justify-center mb-4">
					<svg class="w-16 h-16 text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
					</svg>
				</div>
				<h1 class="text-3xl font-bold tracking-tight text-green-900">¡Pedido Confirmado!</h1>
				<p class="text-slate-700">
					Tu pedido ha sido registrado exitosamente. Recibirás un correo con los detalles de tu compra.
				</p>
			</div>

			<!-- Order Number and Status -->
			<div class="rounded-3xl border border-slate-200 bg-white backdrop-blur-xl shadow-lg p-6 space-y-4">
				<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
					<div>
						<p class="text-xs text-slate-600 uppercase tracking-wide mb-1">Número de pedido</p>
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

				<div class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-4 border-t border-slate-200">
					<div>
						<p class="text-xs text-slate-600 mb-1">Fecha del pedido</p>
						<p class="text-sm font-medium">{formatDate(venta.fecha_pedido)}</p>
					</div>
					{#if venta.fecha_entrega_estimada}
						<div>
							<p class="text-xs text-slate-600 mb-1">Entrega estimada</p>
							<p class="text-sm font-medium">{formatDate(venta.fecha_entrega_estimada)}</p>
						</div>
					{/if}
					{#if venta.numero_tracking}
						<div>
							<p class="text-xs text-slate-600 mb-1">Número de seguimiento</p>
							<p class="text-sm font-mono font-medium">{venta.numero_tracking}</p>
						</div>
					{/if}
				</div>
			</div>

			<!-- Items Purchased -->
			<div class="rounded-3xl border border-slate-200 bg-white backdrop-blur-xl shadow-lg p-6 space-y-4">
				<h2 class="text-lg font-semibold">Artículos del pedido</h2>

				<div class="space-y-3">
					{#each venta.items as item}
						<div class="rounded-2xl border border-slate-200 bg-slate-50 p-4 flex gap-4">
							{#if item.imagen}
								<img
									src={item.imagen}
									alt={item.nombre_producto}
									class="w-16 h-16 object-cover rounded-lg"
								/>
							{:else}
								<div class="w-16 h-16 bg-slate-100 rounded-lg flex items-center justify-center border border-slate-200">
									<svg class="w-8 h-8 text-slate-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
									</svg>
								</div>
							{/if}

							<div class="flex-1 min-w-0">
								<p class="text-sm font-semibold truncate">{item.nombre_producto}</p>
								<p class="text-xs text-slate-600">SKU: {item.sku}</p>
								<div class="flex items-center gap-3 mt-2">
									<p class="text-xs text-slate-600">Cantidad: {item.cantidad}</p>
									{#if item.descuento_unitario > 0}
										<p class="text-xs text-green-600">Descuento: S/. {item.descuento_unitario.toFixed(2)}</p>
									{/if}
								</div>
							</div>

							<div class="text-right">
								<p class="text-sm font-medium">S/. {item.precio_final.toFixed(2)} c/u</p>
								<p class="text-xs text-slate-600 mt-1">Subtotal:</p>
								<p class="text-sm font-semibold">S/. {item.subtotal.toFixed(2)}</p>
							</div>
						</div>
					{/each}
				</div>
			</div>

			<!-- Shipping Address -->
			{#if venta.direccion_envio}
				<div class="rounded-3xl border border-slate-200 bg-white backdrop-blur-xl shadow-lg p-6 space-y-3">
					<h2 class="text-lg font-semibold">Dirección de envío</h2>
					<div class="rounded-2xl border border-slate-200 bg-slate-50 px-4 py-3 space-y-1">
						<p class="text-sm">{venta.direccion_envio}</p>
						<p class="text-sm text-slate-700">
							{venta.ciudad}{venta.departamento ? `, ${venta.departamento}` : ''}
							{venta.codigo_postal ? ` - ${venta.codigo_postal}` : ''}
						</p>
						{#if venta.telefono_contacto}
							<p class="text-xs text-slate-600 pt-2 border-t border-slate-200">
								Tel: {venta.telefono_contacto}
							</p>
						{/if}
						{#if venta.metodo_envio}
							<p class="text-xs text-blue-600 pt-2 border-t border-slate-200">
								Método: {venta.metodo_envio}
							</p>
						{/if}
					</div>
				</div>
			{/if}

			<!-- Order Summary -->
			<div class="rounded-3xl border border-slate-200 bg-white backdrop-blur-xl shadow-lg p-6 space-y-4">
				<h2 class="text-lg font-semibold">Resumen del pedido</h2>

				<div class="space-y-2 text-sm">
					<div class="flex justify-between">
						<span class="text-slate-600">Subtotal</span>
						<span class="font-medium">S/. {venta.subtotal.toFixed(2)}</span>
					</div>

					{#if venta.descuento_total > 0}
						<div class="flex justify-between">
							<span class="text-slate-600">Descuento</span>
							<span class="font-medium text-green-600">− S/. {venta.descuento_total.toFixed(2)}</span>
						</div>
					{/if}

					<div class="flex justify-between">
						<span class="text-slate-600">Envío</span>
						<span class="font-medium">
							{venta.costo_envio === 0 ? 'Gratis' : `S/. ${venta.costo_envio.toFixed(2)}`}
						</span>
					</div>

					<div class="border-t border-slate-200 pt-3 mt-2 flex justify-between items-center">
						<span class="text-base font-semibold">Total pagado</span>
						<span class="text-2xl font-bold">
							S/. {venta.total.toFixed(2)}
						</span>
					</div>
				</div>
			</div>

			<!-- Notes -->
			{#if venta.notas_cliente}
				<div class="rounded-3xl border border-slate-200 bg-white backdrop-blur-xl shadow-lg p-6 space-y-3">
					<h2 class="text-sm font-semibold uppercase tracking-wide text-slate-700">Observaciones</h2>
					<div class="rounded-2xl border border-slate-200 bg-slate-50 px-4 py-3">
						<p class="text-sm text-slate-700">{venta.notas_cliente}</p>
					</div>
				</div>
			{/if}

			<!-- Action Buttons -->
			<div class="flex flex-col sm:flex-row gap-3">
				<button
					type="button"
					class="flex-1 px-6 py-3 rounded-2xl text-sm font-semibold bg-blue-500 text-white hover:bg-blue-600 transition-colors shadow-lg shadow-blue-500/30"
					on:click={continueShopping}
				>
					Continuar comprando
				</button>
				<button
					type="button"
					class="flex-1 px-6 py-3 rounded-2xl text-sm font-semibold border border-slate-200 bg-white hover:bg-slate-50 transition-colors"
					on:click={viewOrderDetails}
				>
					Ver detalles del pedido
				</button>
			</div>

			<!-- Info Message -->
			<div class="rounded-2xl border border-blue-200 bg-blue-50 px-4 py-3 text-center">
				<p class="text-sm text-blue-900">
					Hemos enviado un correo de confirmación a tu dirección de email con todos los detalles de tu pedido.
				</p>
			</div>
		{/if}
	</div>
</div>