<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { cart, cartItems, cartSubtotal, cartItemCount } from '$lib/stores/cart';
  import { cartService } from '$lib/services/cart';
  import { checkoutService } from '$lib/services/checkout';
  import { isAuthenticated } from '$lib/stores/auth';
  import { ShoppingCart, Trash2, Plus, Minus, ArrowLeft, Tag } from 'lucide-svelte';

  // Estado de UI
  let isUpdatingId: number | null = null;
  let isClearing = false;
  let isLoading = true;
  let isValidatingCoupon = false;

  // Cupón
  let couponCode = '';
  let couponMessage = '';
  let couponStatus: 'success' | 'error' | '' = '';
  let appliedCoupon = '';
  let couponDiscount = 0;

  // Derivados del carrito
  $: subtotal = $cart?.subtotal ?? 0;
  $: items = $cartItems ?? [];
  $: itemCount = $cartItemCount ?? 0;

  // Valores fijos por ahora (se pueden calcular en el backend después)
  $: descuento = couponDiscount;
  $: envio = null; // null => se calcula en checkout
  $: total = Number(subtotal) - descuento + (envio ?? 0);

  onMount(async () => {
    // Si no está autenticado, redirigir al login
    if (!$isAuthenticated) {
      await goto('/login?redirect=/carrito');
      return;
    }

    // Cargar carrito
    try {
      await cartService.getCart();
    } catch (error) {
      console.error('Error al cargar carrito:', error);
    } finally {
      isLoading = false;
    }
  });

  async function handleUpdateQuantity(item: any, delta: number) {
    const cantidadActual = item.cantidad;
    const nuevaCantidad = cantidadActual + delta;

    if (nuevaCantidad < 1) return;
    if (nuevaCantidad > item.stock_disponible) {
      alert(`Stock insuficiente. Disponible: ${item.stock_disponible}`);
      return;
    }

    isUpdatingId = item.id_carrito_detalle;

    try {
      await cartService.updateQuantity(item.id_carrito_detalle, { cantidad: nuevaCantidad });
    } catch (error: any) {
      alert(error.message || 'Error al actualizar cantidad');
    } finally {
      isUpdatingId = null;
    }
  }

  async function handleRemoveItem(item: any) {
    if (!confirm('¿Estás seguro de eliminar este producto del carrito?')) return;

    isUpdatingId = item.id_carrito_detalle;

    try {
      await cartService.removeItem(item.id_carrito_detalle);
    } catch (error: any) {
      alert(error.message || 'Error al eliminar producto');
    } finally {
      isUpdatingId = null;
    }
  }

  async function handleClearCart() {
    if (!items.length) return;
    if (!confirm('¿Estás seguro de vaciar el carrito?')) return;

    isClearing = true;
    try {
      await cartService.clearCart();
    } catch (error: any) {
      alert(error.message || 'Error al vaciar carrito');
    } finally {
      isClearing = false;
    }
  }

  async function handleApplyCoupon() {
    couponMessage = '';
    couponStatus = '';

    if (!couponCode.trim()) {
      couponMessage = 'Ingresa un código de cupón.';
      couponStatus = 'error';
      return;
    }

    isValidatingCoupon = true;

    try {
      // Validar cupón llamando al backend
      const result = await checkoutService.calcularTotal(undefined, couponCode);

      if (result.cupon_aplicado) {
        appliedCoupon = result.cupon_aplicado;
        couponDiscount = result.descuento_cupon;
        couponMessage = `¡Cupón "${result.cupon_aplicado}" aplicado! Descuento: S/. ${result.descuento_cupon.toFixed(2)}`;
        couponStatus = 'success';

        // Guardar en localStorage para usarlo en checkout
        localStorage.setItem('applied_coupon', result.cupon_aplicado);
      } else {
        couponMessage = 'Cupón no válido.';
        couponStatus = 'error';
      }
    } catch (error: any) {
      couponMessage = error.message || 'Error al validar cupón';
      couponStatus = 'error';
      appliedCoupon = '';
      couponDiscount = 0;
      localStorage.removeItem('applied_coupon');
    } finally {
      isValidatingCoupon = false;
    }
  }

  function removeCoupon() {
    couponCode = '';
    appliedCoupon = '';
    couponDiscount = 0;
    couponMessage = '';
    couponStatus = '';
    localStorage.removeItem('applied_coupon');
  }

  function goToCatalog() {
    goto('/catalogo');
  }

  function goToCheckout() {
    // Verificar que hay items en el carrito
    if (!items.length) {
      return;
    }

    // Redirigir al primer paso del checkout: selección de dirección
    goto('/checkout/direccion');
  }
</script>

<svelte:head>
  <title>Carrito de compras | KronosTech</title>
</svelte:head>

<div class="min-h-[calc(100vh-4rem)] bg-surface-light dark:bg-surface-dark">
  <div class="max-w-6xl mx-auto px-4 lg:px-6 py-8 space-y-6">

    <!-- Header -->
    <div class="flex flex-col md:flex-row md:items-center md:justify-between gap-3">
      <div>
        <h1 class="text-3xl font-bold tracking-tight text-text-light dark:text-text-dark">
          Carrito de Compras
        </h1>
        <p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
          Revisa tus componentes antes de continuar con el pago.
        </p>
      </div>

      <button
        type="button"
        class="inline-flex items-center justify-center gap-2 px-4 py-2 text-sm font-medium rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors"
        on:click={goToCatalog}
      >
        <ArrowLeft size={16} />
        Seguir comprando
      </button>
    </div>

    {#if isLoading}
      <!-- Loading state -->
      <div class="flex items-center justify-center py-20">
        <div class="text-center">
          <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto mb-4"></div>
          <p class="text-slate-600 dark:text-slate-400">Cargando carrito...</p>
        </div>
      </div>
    {:else if !items.length}
      <!-- Carrito vacío -->
      <div class="mt-10 flex flex-col items-center justify-center text-center py-16 rounded-2xl border-2 border-dashed border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900/30">
        <div class="w-16 h-16 rounded-xl border border-border-light dark:border-border-dark bg-slate-100 dark:bg-slate-800 flex items-center justify-center mb-4">
          <ShoppingCart size={32} class="text-slate-400 dark:text-slate-500" />
        </div>
        <h2 class="text-xl font-semibold mb-2 text-text-light dark:text-text-dark">
          Tu carrito está vacío
        </h2>
        <p class="text-sm text-slate-600 dark:text-slate-400 mb-6 max-w-md">
          Explora el catálogo y añade procesadores, gráficas, RAM y más a tu carrito.
        </p>
        <button
          type="button"
          class="px-5 py-2.5 rounded-lg text-sm font-semibold bg-primary text-white hover:bg-primary/90 transition-colors shadow-lg"
          on:click={goToCatalog}
        >
          Ir al catálogo
        </button>
      </div>
    {:else}
      <!-- Grid principal -->
      <div class="grid grid-cols-1 lg:grid-cols-[minmax(0,2.1fr),minmax(0,1fr)] gap-6 items-start">

        <!-- Lista de items -->
        <section aria-label="Productos en el carrito" class="space-y-4">
          {#each items as item (item.id_carrito_detalle)}
            <article
              class="relative rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm overflow-hidden"
            >
              <!-- Botón eliminar (desktop) -->
              <button
                type="button"
                class="hidden md:flex absolute top-3 right-3 w-9 h-9 rounded-lg bg-slate-100 dark:bg-slate-700 border border-border-light dark:border-border-dark items-center justify-center text-slate-600 dark:text-slate-400 hover:text-red-600 dark:hover:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors"
                on:click={() => handleRemoveItem(item)}
                disabled={isUpdatingId === item.id_carrito_detalle}
                aria-label="Eliminar producto del carrito"
              >
                <Trash2 size={16} />
              </button>

              <div class="p-4 sm:p-5 flex flex-col gap-4 md:grid md:grid-cols-[auto,minmax(0,1.5fr)] md:gap-5">
                <!-- Imagen -->
                <div class="flex md:block">
                  <div class="w-24 h-24 rounded-lg bg-white dark:bg-slate-900 border border-border-light dark:border-border-dark flex items-center justify-center overflow-hidden">
                    {#if item.imagen_principal}
                      <img
                        src={item.imagen_principal}
                        alt={item.nombre}
                        class="w-full h-full object-contain p-2"
                      />
                    {:else}
                      <span class="text-xs text-slate-400">
                        Sin imagen
                      </span>
                    {/if}
                  </div>
                </div>

                <!-- Info principal -->
                <div class="flex-1 space-y-3">
                  <div class="flex flex-col gap-1">
                    <h3 class="text-sm sm:text-base font-semibold text-text-light dark:text-text-dark">
                      {item.nombre}
                    </h3>
                    <p class="text-xs text-slate-600 dark:text-slate-400">
                      Stock disponible:
                      {#if item.stock_disponible > 10}
                        <span class="text-emerald-600 dark:text-emerald-400 font-medium">
                          {item.stock_disponible} unidades
                        </span>
                      {:else if item.stock_disponible > 0}
                        <span class="text-amber-600 dark:text-amber-400 font-medium">
                          Solo {item.stock_disponible} unidades
                        </span>
                      {:else}
                        <span class="text-red-600 dark:text-red-400 font-medium">
                          Agotado
                        </span>
                      {/if}
                    </p>
                  </div>

                  <div class="border-t border-border-light dark:border-border-dark pt-3 grid grid-cols-1 sm:grid-cols-3 gap-3 text-sm">
                    <!-- Precio -->
                    <div>
                      <p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400 mb-1">
                        Precio
                      </p>
                      <p class="text-base font-semibold text-text-light dark:text-text-dark">
                        S/. {item.precio_unitario.toFixed(2)}
                      </p>
                    </div>

                    <!-- Cantidad -->
                    <div class="flex flex-col items-start">
                      <p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400 mb-1">
                        Cantidad
                      </p>
                      <div class="inline-flex items-center gap-2 rounded-lg bg-slate-100 dark:bg-slate-700 border border-border-light dark:border-border-dark px-2 py-1">
                        <button
                          type="button"
                          class="w-7 h-7 rounded-md flex items-center justify-center text-sm text-text-light dark:text-text-dark hover:bg-slate-200 dark:hover:bg-slate-600 disabled:opacity-40 transition-colors"
                          on:click={() => handleUpdateQuantity(item, -1)}
                          disabled={isUpdatingId === item.id_carrito_detalle}
                        >
                          <Minus size={14} />
                        </button>
                        <div class="min-w-[2.5rem] text-center text-sm font-semibold text-text-light dark:text-text-dark">
                          {item.cantidad}
                        </div>
                        <button
                          type="button"
                          class="w-7 h-7 rounded-md flex items-center justify-center text-sm text-text-light dark:text-text-dark hover:bg-slate-200 dark:hover:bg-slate-600 disabled:opacity-40 transition-colors"
                          on:click={() => handleUpdateQuantity(item, +1)}
                          disabled={isUpdatingId === item.id_carrito_detalle}
                        >
                          <Plus size={14} />
                        </button>
                      </div>
                    </div>

                    <!-- Subtotal + eliminar móvil -->
                    <div class="flex sm:flex-col sm:items-end justify-between items-center gap-2">
                      <div>
                        <p class="text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400 mb-1">
                          Subtotal
                        </p>
                        <p class="text-base font-semibold text-text-light dark:text-text-dark">
                          S/. {item.subtotal.toFixed(2)}
                        </p>
                      </div>

                      <!-- Eliminar en mobile -->
                      <button
                        type="button"
                        class="md:hidden inline-flex items-center justify-center gap-1 px-3 py-1.5 rounded-lg text-xs font-medium bg-slate-100 dark:bg-slate-700 border border-border-light dark:border-border-dark text-slate-600 dark:text-slate-300 hover:text-red-600 dark:hover:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 disabled:opacity-40 transition-colors"
                        on:click={() => handleRemoveItem(item)}
                        disabled={isUpdatingId === item.id_carrito_detalle}
                      >
                        <Trash2 size={14} />
                        Eliminar
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </article>
          {/each}

          <!-- Clear cart -->
          <div class="flex items-center justify-between text-sm mt-1">
            <button
              type="button"
              class="text-red-600 dark:text-red-400 hover:text-red-700 dark:hover:text-red-300 disabled:opacity-50 text-sm font-medium"
              on:click={handleClearCart}
              disabled={isClearing || !items.length}
            >
              {#if isClearing}
                Vaciando carrito...
              {:else}
                Vaciar carrito
              {/if}
            </button>

            <p class="text-xs text-slate-500 dark:text-slate-400">
              Los productos no se reservan hasta que completes el pago.
            </p>
          </div>
        </section>

        <!-- Resumen del pedido -->
        <aside
          aria-label="Resumen del pedido"
          class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-5 space-y-4 shadow-sm sticky top-24"
        >
          <h2 class="text-lg font-semibold text-text-light dark:text-text-dark">
            Resumen del Pedido
          </h2>

          <div class="space-y-2 text-sm">
            <div class="flex justify-between text-slate-600 dark:text-slate-400">
              <span>Subtotal ({itemCount} {itemCount === 1 ? 'producto' : 'productos'})</span>
              <span class="font-medium text-text-light dark:text-text-dark">
                S/. {subtotal.toFixed(2)}
              </span>
            </div>

            <div class="flex justify-between text-slate-600 dark:text-slate-400">
              <span>Descuento</span>
              <span class="font-medium text-text-light dark:text-text-dark">
                {#if descuento > 0}
                  − S/. {descuento.toFixed(2)}
                {:else}
                  S/. 0.00
                {/if}
              </span>
            </div>

            <div class="flex justify-between text-slate-600 dark:text-slate-400">
              <span>Envío</span>
              {#if envio == null}
                <span class="text-xs">
                  Se calcula en checkout
                </span>
              {:else}
                <span class="font-medium text-text-light dark:text-text-dark">
                  S/. {envio.toFixed(2)}
                </span>
              {/if}
            </div>

            <div class="border-t border-border-light dark:border-border-dark pt-3 mt-2 flex justify-between items-center">
              <span class="text-sm font-semibold text-text-light dark:text-text-dark">Total (incl. IGV)</span>
              <span class="text-xl font-bold text-text-light dark:text-text-dark">
                S/. {total.toFixed(2)}
              </span>
            </div>
          </div>

          <!-- Cupón -->
          <div class="pt-2 space-y-2 border-t border-border-light dark:border-border-dark">
            <label class="flex items-center gap-2 text-xs font-semibold text-slate-600 dark:text-slate-400 uppercase tracking-wide">
              <Tag size={14} />
              Código de cupón
            </label>
            <div class="flex gap-2">
              <input
                type="text"
                class="flex-1 rounded-lg border border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900 px-3 py-2 text-sm text-text-light dark:text-text-dark placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-primary/50 disabled:opacity-50 disabled:cursor-not-allowed"
                placeholder="Ingresa tu cupón"
                bind:value={couponCode}
                disabled={appliedCoupon !== '' || isValidatingCoupon}
              />
              {#if appliedCoupon}
                <button
                  type="button"
                  class="px-4 py-2 rounded-lg text-sm font-semibold border border-red-300 dark:border-red-700 bg-red-50 dark:bg-red-900/20 hover:bg-red-100 dark:hover:bg-red-900/40 transition-colors text-red-600 dark:text-red-400"
                  on:click={removeCoupon}
                >
                  Remover
                </button>
              {:else}
                <button
                  type="button"
                  class="px-4 py-2 rounded-lg text-sm font-semibold border border-border-light dark:border-border-dark bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600 transition-colors text-text-light dark:text-text-dark disabled:opacity-50 disabled:cursor-not-allowed"
                  on:click={handleApplyCoupon}
                  disabled={isValidatingCoupon}
                >
                  {isValidatingCoupon ? 'Validando...' : 'Aplicar'}
                </button>
              {/if}
            </div>
            {#if couponMessage}
              <p
                class="text-xs {couponStatus === 'success'
                  ? 'text-emerald-600 dark:text-emerald-400'
                  : 'text-red-600 dark:text-red-400'}"
              >
                {couponMessage}
              </p>
            {/if}
          </div>

          <!-- CTA -->
          <button
            type="button"
            class="w-full mt-2 px-4 py-3 rounded-lg text-sm font-semibold bg-primary text-white hover:bg-primary/90 transition-colors shadow-lg disabled:opacity-60 disabled:cursor-not-allowed"
            on:click={goToCheckout}
            disabled={!items.length}
          >
            Finalizar compra
          </button>

          <p class="text-[11px] text-slate-500 dark:text-slate-400 mt-2 text-center">
            Al continuar aceptas los Términos y Condiciones de KronosTech.
          </p>
        </aside>
      </div>
    {/if}
  </div>
</div>
