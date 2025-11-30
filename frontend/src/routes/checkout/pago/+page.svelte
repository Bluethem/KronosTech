<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { cart, cartItems, cartSubtotal, clearCart } from '$lib/stores/cart';
	import { direccionSeleccionada } from '$lib/stores/direccion';
	import { checkoutService } from '$lib/services/checkout';
	import { cartService } from '$lib/services/cart';
	import type { MetodoPago } from '$lib/services/checkout';

	let acceptTerms = false;
	let globalError = '';
	let processing = false;
	let loading = true;
	let metodosPago: MetodoPago[] = [];
	let selectedPaymentId: number | null = null;

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
			// Cargar métodos de pago
			metodosPago = await checkoutService.getMetodosPago();

			// Seleccionar el primer método por defecto
			if (metodosPago.length > 0 && !selectedPaymentId) {
				selectedPaymentId = metodosPago[0].id_metodo_pago;
			}

			// Calcular total
			calculatedTotal = await checkoutService.calcularTotal($direccionSeleccionada?.id_direccion);
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

		try {
			// Recuperar notas de entrega
			const notasEntrega = sessionStorage.getItem('notas_entrega') || '';

			// Procesar checkout
			const venta = await checkoutService.procesarCheckout({
				id_direccion: $direccionSeleccionada!.id_direccion,
				id_metodo_pago: selectedPaymentId!,
				notas_cliente: notasEntrega || undefined
			});

			// Limpiar carrito y sessionStorage
			clearCart();
			sessionStorage.removeItem('metodo_envio');
			sessionStorage.removeItem('notas_entrega');

			// Redirigir a confirmación
			goto(`/pedido/${venta.id_venta}/confirmacion`);
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

<div class="min-h-[calc(100vh-4rem)] bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950 text-slate-100">
	<div class="max-w-6xl mx-auto px-4 lg:px-0 py-8 space-y-6">
		<!-- STEPPER -->
		<div class="space-y-4">
			<div class="flex items-center gap-3 text-xs text-slate-400">
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full bg-blue-500 text-white flex items-center justify-center text-[10px] font-semibold">1</div>
					<span class="font-medium">Carrito</span>
				</div>
				<div class="h-px flex-1 bg-slate-700/70"></div>
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full bg-blue-500 text-white flex items-center justify-center text-[10px] font-semibold">2</div>
					<span class="font-medium">Dirección</span>
				</div>
				<div class="h-px flex-1 bg-slate-700/70"></div>
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full bg-blue-500 text-white flex items-center justify-center text-[10px] font-semibold">3</div>
					<span class="font-medium">Envío</span>
				</div>
				<div class="h-px flex-1 bg-slate-700/70"></div>
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full bg-slate-100 text-slate-900 flex items-center justify-center text-[10px] font-semibold">4</div>
					<span class="font-semibold text-slate-100">Pago</span>
				</div>
			</div>

			<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-3">
				<div>
					<h1 class="text-3xl font-bold tracking-tight">Pago y Confirmación</h1>
					<p class="text-sm text-slate-400 mt-1">
						Elige tu método de pago, revisa el resumen del pedido y confirma tu compra.
					</p>
				</div>

				<button
					type="button"
					class="inline-flex items-center justify-center px-4 py-2 text-sm font-medium rounded-2xl border border-white/10 bg-white/5 hover:bg-white/10 transition-colors"
					on:click={goBackToShipping}
				>
					← Volver a envío
				</button>
			</div>
		</div>

		{#if globalError}
			<div class="rounded-2xl border border-rose-500/40 bg-rose-500/10 px-4 py-3 text-sm text-rose-100">
				{globalError}
			</div>
		{/if}

		{#if loading}
			<div class="rounded-3xl border border-white/10 bg-white/5 backdrop-blur-xl p-8 text-center">
				<p class="text-slate-400">Cargando métodos de pago...</p>
			</div>
		{:else}
			<div class="grid grid-cols-1 lg:grid-cols-[minmax(0,2.1fr),minmax(0,1fr)] gap-6 items-start">
				<section class="space-y-5">
					<!-- Métodos de pago -->
					<div class="rounded-3xl border border-white/10 bg-white/5 backdrop-blur-xl shadow-[0_18px_50px_rgba(0,0,0,0.45)] p-5 space-y-3">
						<h2 class="text-sm font-semibold uppercase tracking-wide text-slate-300 mb-1">
							Selecciona un método de pago
						</h2>

						<div class="space-y-3">
							{#each metodosPago as method}
								<button
									type="button"
									class={`w-full text-left rounded-2xl border px-4 py-3 flex items-start gap-3 transition-all ${
										selectedPaymentId === method.id_metodo_pago
											? 'border-blue-500 bg-blue-500/10 shadow-[0_0_0_1px_rgba(59,130,246,0.4)]'
											: 'border-white/10 bg-black/40 hover:bg-white/5'
									}`}
									on:click={() => (selectedPaymentId = method.id_metodo_pago)}
								>
									<div class="mt-1">
										<div
											class={`w-4 h-4 rounded-full border flex items-center justify-center ${
												selectedPaymentId === method.id_metodo_pago
													? 'border-blue-400 bg-blue-500'
													: 'border-slate-500 bg-black'
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
											<p class="text-xs text-slate-300">
												{method.descripcion}
											</p>
										{/if}
										{#if method.instrucciones}
											<p class="text-[11px] text-slate-500">
												{method.instrucciones}
											</p>
										{/if}
										{#if method.comision_porcentaje > 0 || method.comision_fija > 0}
											<p class="text-[11px] text-amber-400">
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
						<div class="rounded-3xl border border-white/10 bg-white/5 backdrop-blur-xl shadow-[0_18px_50px_rgba(0,0,0,0.45)] p-5 space-y-4">
							<h2 class="text-sm font-semibold uppercase tracking-wide text-slate-300">
								Detalles de pago
							</h2>

							<div class="rounded-2xl bg-black/40 border border-white/10 px-4 py-3 space-y-2">
								<p class="text-sm font-semibold">{selectedPayment.nombre}</p>
								{#if selectedPayment.descripcion}
									<p class="text-xs text-slate-300">{selectedPayment.descripcion}</p>
								{/if}
								{#if selectedPayment.instrucciones}
									<p class="text-xs text-slate-400 pt-2 border-t border-white/10">
										{selectedPayment.instrucciones}
									</p>
								{/if}
								{#if selectedPayment.tiempo_procesamiento}
									<p class="text-xs text-blue-400">
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
					<div class="rounded-3xl border border-white/10 bg-white/5 backdrop-blur-xl shadow-[0_18px_50px_rgba(0,0,0,0.45)] p-5">
						<div class="flex items-start gap-3">
							<input
								id="terms"
								type="checkbox"
								class="mt-1 rounded border border-white/30 bg-black/40"
								bind:checked={acceptTerms}
							/>
							<label for="terms" class="text-sm text-slate-300">
								He leído y acepto los
								<a href="/terminos" target="_blank" class="text-blue-400 hover:text-blue-300 underline">
									Términos y Condiciones
								</a>
								y la
								<a href="/privacidad" target="_blank" class="text-blue-400 hover:text-blue-300 underline">
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
					class="rounded-3xl border border-white/10 bg-white/5 backdrop-blur-xl p-5 space-y-4 shadow-[0_18px_50px_rgba(0,0,0,0.45)] sticky top-24"
				>
					<h2 class="text-lg font-semibold">Resumen del pedido</h2>

					<div class="space-y-2 text-sm">
						<div class="flex justify-between">
							<span class="text-slate-400">
								Subtotal ({$cartItems.length} artículo{$cartItems.length === 1 ? '' : 's'})
							</span>
							<span class="font-medium">
								S/. {subtotal.toFixed(2)}
							</span>
						</div>

						<div class="flex justify-between">
							<span class="text-slate-400">Descuento</span>
							<span class="font-medium">
								{#if descuento > 0}
									− S/. {descuento.toFixed(2)}
								{:else}
									S/. 0.00
								{/if}
							</span>
						</div>

						<div class="flex justify-between items-center">
							<span class="text-slate-400">Envío</span>
							<span class="font-medium">
								{costoEnvio === 0 ? 'Gratis' : `S/. ${costoEnvio.toFixed(2)}`}
							</span>
						</div>

						<div class="border-t border-white/10 pt-3 mt-2 flex justify-between items-center">
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
