<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { cart, cartItems, cartSubtotal, clearCart } from '$lib/stores/cart';
	import { direccionSeleccionada } from '$lib/stores/direccion';
	import { checkoutService, type Venta } from '$lib/services/checkout';
	import { tarjetaService, type MetodoPagoCliente } from '$lib/services/tarjeta';

	import { cartService } from '$lib/services/cart';
	import type { MetodoPago } from '$lib/services/checkout';

	let acceptTerms = false;
	let globalError = '';
	let processing = false;
	let loading = true;
	let metodosPago: MetodoPago[] = [];
	let selectedPaymentId: number | null = null;
	let metodosPagoCliente: MetodoPagoCliente[] = [];
	let selectedMetodoPagoClienteId: number | null = null;
	let ventaConQR: Venta | null = null;
	let mostrarQR = false;
	$: qrUrl =
		ventaConQR?.info_pago?.qr_data
			? `https://api.qrserver.com/v1/create-qr-code/?size=220x220&data=${encodeURIComponent(ventaConQR.info_pago.qr_data)}`
			: null;

	// Calcular totales
	let calculatedTotal: any = null;
	$: currentCart = $cart;
	$: subtotal = calculatedTotal?.subtotal ?? currentCart?.subtotal ?? $cartSubtotal ?? 0;
	$: descuento = calculatedTotal?.descuento_total ?? currentCart?.descuento_total ?? 0;

	// Recuperar costo de envío de sessionStorage
	let metodoEnvio: any = null;
	$: costoEnvio = metodoEnvio?.costo ?? 0;
	$: total = subtotal - descuento + costoEnvio;

	$: selectedPayment = metodosPago.find((m) => m.id_metodo_pago === selectedPaymentId) ?? null;

	onMount(async () => {
		// Verificar que hay dirección seleccionada
		if (!$direccionSeleccionada) {
			goto('/checkout/direccion');
			return;
		}

		// Cargar carrito primero
		try {
			await cartService.getCart();
		} catch (error) {
			console.error('Error al cargar carrito:', error);
		}

		// Verificar que hay items en el carrito después de cargarlo
		if ($cartItems.length === 0) {
			goto('/carrito');
			return;
		}

		// Recuperar método de envío
		const envioStr = sessionStorage.getItem('metodo_envio');
		if (envioStr) {
			metodoEnvio = JSON.parse(envioStr);
		}

		await loadData();
	});

	async function loadData() {
		loading = true;
		globalError = '';

		try {
			// Cargar métodos de pago disponibles
			metodosPago = await checkoutService.getMetodosPago();

			// Seleccionar el primer método por defecto
			if (metodosPago.length > 0 && !selectedPaymentId) {
				selectedPaymentId = metodosPago[0].id_metodo_pago;
			}

			// Cargar métodos de pago del cliente (tarjetas guardadas)
			metodosPagoCliente = await tarjetaService.getMetodosPago();
			if (metodosPagoCliente.length > 0 && !selectedMetodoPagoClienteId) {
				const predeterminada = metodosPagoCliente.find((m) => m.es_predeterminado);
				selectedMetodoPagoClienteId =
					predeterminada?.id_metodo_pago_cliente ?? metodosPagoCliente[0].id_metodo_pago_cliente;
			}

			// Recuperar cupón aplicado (si existe)
			const codigoCupon = localStorage.getItem('applied_coupon') || undefined;

			// Calcular total (con cupón si existe)
			calculatedTotal = await checkoutService.calcularTotal(
				$direccionSeleccionada?.id_direccion,
				codigoCupon
			);
		} catch (err: any) {
			globalError = err.message ?? 'Error al cargar información';
		} finally {
			loading = false;
		}
	}

	function goBackToShipping() {
		goto('/checkout/envio');
	}

	function validateForm(): boolean {
		globalError = '';

		if (!acceptTerms) {
			globalError = 'Debes aceptar los Términos y Condiciones para continuar.';
			return false;
		}

		if (!selectedPaymentId) {
			globalError = 'Selecciona un método de pago.';
			return false;
		}

		// Si el método seleccionado es tarjeta, exigir selección de tarjeta del usuario
		if (
			selectedPayment &&
			(selectedPayment.tipo === 'tarjeta_credito' || selectedPayment.tipo === 'tarjeta_debito') &&
			!selectedMetodoPagoClienteId
		) {
			globalError = 'Selecciona una tarjeta para realizar el pago.';
			return false;
		}

		if (!$direccionSeleccionada) {
			globalError = 'No hay dirección de envío seleccionada.';
			return false;
		}

		return true;
	}

	async function handlePay() {
		if (!validateForm()) return;

		processing = true;
		globalError = '';
		ventaConQR = null;
		mostrarQR = false;

		try {
			// Recuperar notas de entrega
			const notasEntrega = sessionStorage.getItem('notas_entrega') || '';

			// Recuperar cupón aplicado (si existe)
			const codigoCupon = localStorage.getItem('applied_coupon') || undefined;

			// Procesar checkout
			const venta = await checkoutService.procesarCheckout({
				id_direccion: $direccionSeleccionada!.id_direccion,
				id_metodo_pago: selectedPaymentId!,
				notas_cliente: notasEntrega || undefined,
				codigo_cupon: codigoCupon,
				id_metodo_pago_cliente: selectedMetodoPagoClienteId ?? null
			});

			// Limpiar carrito, sessionStorage y cupón
			clearCart();
			sessionStorage.removeItem('metodo_envio');
			sessionStorage.removeItem('notas_entrega');
			localStorage.removeItem('applied_coupon');

			// Si el backend devuelve info de pago para Yape/Plin, mostrar QR antes de redirigir
			if (venta.info_pago && (venta.info_pago.tipo === 'yape' || venta.info_pago.tipo === 'plin')) {
				ventaConQR = venta;
				mostrarQR = true;
			} else {
				// Redirigir directamente a confirmación
				goto(`/pedido/${venta.id_venta}/confirmacion`);
			}
		} catch (err: any) {
			globalError = err.message ?? 'Ocurrió un error al procesar el pago.';
		} finally {
			processing = false;
		}
	}
</script>

<svelte:head>
	<title>Pago y confirmación | KronosTech</title>
</svelte:head>

<div class="min-h-[calc(100vh-4rem)] bg-gradient-to-b from-white via-slate-50 to-white text-slate-900">
	<div class="max-w-6xl mx-auto px-4 lg:px-0 py-8 space-y-6">
		<!-- STEPPER -->
		<div class="space-y-4">
			<div class="flex items-center gap-3 text-xs text-slate-600">
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full bg-blue-500 text-white flex items-center justify-center text-[10px] font-semibold">1</div>
					<span class="font-medium">Carrito</span>
				</div>
				<div class="h-px flex-1 bg-slate-300"></div>
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full bg-blue-500 text-white flex items-center justify-center text-[10px] font-semibold">2</div>
					<span class="font-medium">Dirección</span>
				</div>
				<div class="h-px flex-1 bg-slate-300"></div>
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full bg-blue-500 text-white flex items-center justify-center text-[10px] font-semibold">3</div>
					<span class="font-medium">Envío</span>
				</div>
				<div class="h-px flex-1 bg-slate-300"></div>
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full bg-slate-900 text-white flex items-center justify-center text-[10px] font-semibold">4</div>
					<span class="font-semibold text-slate-900">Pago</span>
				</div>
			</div>

			<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-3">
				<div>
					<h1 class="text-3xl font-bold tracking-tight">Pago y Confirmación</h1>
					<p class="text-sm text-slate-600 mt-1">
						Elige tu método de pago, revisa el resumen del pedido y confirma tu compra.
					</p>
				</div>

				<button
					type="button"
					class="inline-flex items-center justify-center px-4 py-2 text-sm font-medium rounded-2xl border border-slate-200 bg-white hover:bg-slate-50 transition-colors"
					on:click={goBackToShipping}
				>
					← Volver a envío
				</button>
			</div>
		</div>

		{#if globalError}
			<div class="rounded-2xl border border-rose-300 bg-rose-50 px-4 py-3 text-sm text-rose-900">
				{globalError}
			</div>
		{/if}

		{#if loading}
			<div class="rounded-3xl border border-slate-200 bg-white backdrop-blur-xl p-8 text-center shadow-sm">
				<p class="text-slate-600">Cargando métodos de pago...</p>
			</div>
		{:else}
			<div class="grid grid-cols-1 lg:grid-cols-[minmax(0,2.1fr),minmax(0,1fr)] gap-6 items-start">
				<section class="space-y-5">
					<!-- Métodos de pago -->
					<div class="rounded-3xl border border-slate-200 bg-white backdrop-blur-xl shadow-lg p-5 space-y-3">
						<h2 class="text-sm font-semibold uppercase tracking-wide text-slate-700 mb-1">
							Selecciona un método de pago
						</h2>

						<div class="space-y-3">
							{#each metodosPago as method}
								<button
									type="button"
									class={`w-full text-left rounded-2xl border px-4 py-3 flex items-start gap-3 transition-all ${
										selectedPaymentId === method.id_metodo_pago
											? 'border-blue-500 bg-blue-50 shadow-[0_0_0_1px_rgba(59,130,246,0.4)]'
											: 'border-slate-200 bg-slate-50 hover:bg-slate-100'
									}`}
									on:click={() => (selectedPaymentId = method.id_metodo_pago)}
								>
									<div class="mt-1">
										<div
											class={`w-4 h-4 rounded-full border flex items-center justify-center ${
												selectedPaymentId === method.id_metodo_pago
													? 'border-blue-400 bg-blue-500'
													: 'border-slate-400 bg-white'
											}`}
										>
											{#if selectedPaymentId === method.id_metodo_pago}
												<div class="w-1.5 h-1.5 rounded-full bg-white"></div>
											{/if}
										</div>
									</div>

									<div class="flex-1 min-w-0 space-y-1">
										<p class="text-sm font-semibold truncate">
											{method.nombre}
										</p>
										{#if method.descripcion}
											<p class="text-xs text-slate-700">
												{method.descripcion}
											</p>
										{/if}
										{#if method.instrucciones}
											<p class="text-[11px] text-slate-500">
												{method.instrucciones}
											</p>
										{/if}
										{#if method.comision_porcentaje > 0 || method.comision_fija > 0}
											<p class="text-[11px] text-amber-600">
												Comisión: {method.comision_porcentaje > 0 ? `${method.comision_porcentaje}%` : ''}
												{method.comision_fija > 0 ? `+ S/. ${method.comision_fija}` : ''}
											</p>
										{/if}
									</div>
								</button>
							{/each}
						</div>
					</div>

					<!-- Información del método seleccionado -->
					{#if selectedPayment}
						<div class="rounded-3xl border border-slate-200 bg-white backdrop-blur-xl shadow-lg p-5 space-y-4">
							<h2 class="text-sm font-semibold uppercase tracking-wide text-slate-700">
								Detalles de pago
							</h2>

							<div class="rounded-2xl bg-slate-50 border border-slate-200 px-4 py-3 space-y-2">
								<p class="text-sm font-semibold">{selectedPayment.nombre}</p>
								{#if selectedPayment.descripcion}
									<p class="text-xs text-slate-700">{selectedPayment.descripcion}</p>
								{/if}
								{#if selectedPayment.instrucciones}
									<p class="text-xs text-slate-600 pt-2 border-t border-slate-200">
										{selectedPayment.instrucciones}
									</p>
								{/if}
								{#if selectedPayment.tiempo_procesamiento}
									<p class="text-xs text-blue-600">
										Tiempo de procesamiento: {selectedPayment.tiempo_procesamiento}
									</p>
								{/if}
							</div>

							<p class="text-[11px] text-slate-500">
								💳 No guardamos los datos de tu tarjeta. El procesamiento de pago es seguro y encriptado.
							</p>
						</div>
					{/if}

					<!-- Aceptar términos -->
					<div class="rounded-3xl border border-slate-200 bg-white backdrop-blur-xl shadow-lg p-5">
						<div class="flex items-start gap-3">
							<input
								id="terms"
								type="checkbox"
								class="mt-1 rounded border border-slate-300 bg-slate-50"
								bind:checked={acceptTerms}
							/>
							<label for="terms" class="text-sm text-slate-700">
								He leído y acepto los
								<a href="/terminos" target="_blank" class="text-blue-600 hover:text-blue-700 underline">
									Términos y Condiciones
								</a>
								y la
								<a href="/privacidad" target="_blank" class="text-blue-600 hover:text-blue-700 underline">
									Política de Privacidad
								</a>
								de KronosTech.
							</label>
						</div>
					</div>
				</section>

				<!-- Resumen -->
				<aside
					aria-label="Resumen del pedido"
					class="rounded-3xl border border-slate-200 bg-white backdrop-blur-xl p-5 space-y-4 shadow-lg sticky top-24"
				>
					<h2 class="text-lg font-semibold">Resumen del pedido</h2>

					<div class="space-y-2 text-sm">
						<div class="flex justify-between">
							<span class="text-slate-600">
								Subtotal ({$cartItems.length} artículo{$cartItems.length === 1 ? '' : 's'})
							</span>
							<span class="font-medium">
								S/. {subtotal.toFixed(2)}
							</span>
						</div>

						{#if calculatedTotal?.cupon_aplicado}
							<div class="flex justify-between items-center text-emerald-600">
								<span class="text-xs flex items-center gap-1">
									<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"></path></svg>
									Cupón: {calculatedTotal.cupon_aplicado}
								</span>
								<span class="font-medium text-emerald-600">
									− S/. {calculatedTotal.descuento_cupon?.toFixed(2) || '0.00'}
								</span>
							</div>
						{/if}

						<div class="flex justify-between">
							<span class="text-slate-600">Descuento total</span>
							<span class="font-medium">
								{#if descuento > 0}
									− S/. {descuento.toFixed(2)}
								{:else}
									S/. 0.00
								{/if}
							</span>
						</div>

						<div class="flex justify-between items-center">
							<span class="text-slate-600">Envío</span>
							<span class="font-medium">
								{costoEnvio === 0 ? 'Gratis' : `S/. ${costoEnvio.toFixed(2)}`}
							</span>
						</div>

						<div class="border-t border-slate-200 pt-3 mt-2 flex justify-between items-center">
							<span class="text-sm font-semibold">Total a pagar</span>
							<span class="text-xl font-bold">
								S/. {total.toFixed(2)}
							</span>
						</div>
					</div>

					<button
						type="button"
						class="w-full mt-2 px-4 py-3 rounded-2xl text-sm font-semibold bg-blue-500 text-white hover:bg-blue-600 transition-colors shadow-lg shadow-blue-500/30 disabled:opacity-60 disabled:cursor-not-allowed"
						on:click={handlePay}
						disabled={!$cartItems.length || processing || !acceptTerms}
					>
						{#if processing}
							Procesando pago...
						{:else}
							Pagar S/. {total.toFixed(2)}
						{/if}
					</button>

					<p class="text-[11px] text-slate-500 mt-1">
						Al confirmar el pago crearemos tu pedido y recibirás un correo con el resumen de la compra.
					</p>
				</aside>
			</div>
		{/if}
	</div>
</div>

{#if mostrarQR && ventaConQR && ventaConQR.info_pago && qrUrl}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
		<div class="w-full max-w-sm rounded-3xl bg-white shadow-2xl border border-slate-200 p-6 space-y-4">
			<h2 class="text-lg font-semibold text-slate-900 flex items-center gap-2">
				Pago con {ventaConQR.info_pago.tipo === 'yape' ? 'Yape' : 'Plin'}
			</h2>
			<p class="text-sm text-slate-700">
				Escanea este código QR con tu app de {ventaConQR.info_pago.tipo === 'yape' ? 'Yape' : 'Plin'} para completar el
				pago por <span class="font-semibold">S/. {ventaConQR.info_pago.monto.toFixed(2)}</span>.
			</p>
			<div class="flex justify-center py-3">
				<img src={qrUrl} alt="QR de pago" class="w-56 h-56 rounded-xl border border-slate-200 bg-white" />
			</div>
			<p class="text-[11px] text-slate-500 text-center">
				Una vez realizado el pago podrás continuar a la pantalla de confirmación de tu pedido.
			</p>
			<div class="flex gap-3 justify-end pt-2">
				<button
					type="button"
					class="px-4 py-2 rounded-xl border border-slate-200 text-sm text-slate-700 hover:bg-slate-50"
					on:click={() => {
						mostrarQR = false;
					}}
				>
					Cerrar
				</button>
				<button
					type="button"
					class="px-4 py-2 rounded-xl bg-blue-500 text-white text-sm font-semibold hover:bg-blue-600"
					on:click={() => {
						if (ventaConQR) {
							goto(`/pedido/${ventaConQR.id_venta}/confirmacion`);
						}
					}}
				>
					Ya pagué, continuar
				</button>
			</div>
		</div>
	</div>
{/if}