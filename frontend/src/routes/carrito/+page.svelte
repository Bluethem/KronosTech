<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { cart, cartItems, cartSubtotal, cartItemCount } from '$lib/stores/cart';
  import { cartService } from '$lib/services/cart';
  import { checkoutService } from '$lib/services/checkout';
  import { isAuthenticated } from '$lib/stores/auth';
  import { siteConfig } from '$lib/stores/config';
  import { ShoppingCart, Trash2, Plus, Minus, ArrowLeft, Tag, Truck, AlertTriangle, X, Loader2 } from 'lucide-svelte';

  // Estado de UI
  let isUpdatingId: number | null = null;
  let isClearing = false;
  let isLoading = true;
  let isValidatingCoupon = false;
  let isDeletingId: number | null = null;

  // Modal de confirmación
  let showDeleteModal = false;
  let itemToDelete: any = null;
  let showClearModal = false;

  // Notificación toast
  let toastMessage = '';
  let toastType: 'success' | 'error' | '' = '';
  let toastTimeout: any = null;

  function showToast(message: string, type: 'success' | 'error') {
    toastMessage = message;
    toastType = type;
    if (toastTimeout) clearTimeout(toastTimeout);
    toastTimeout = setTimeout(() => {
      toastMessage = '';
      toastType = '';
    }, 3000);
  }

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

  // Configuración de envío gratis
  $: freeShippingThreshold = $siteConfig.freeShippingThreshold;
  $: qualifiesForFreeShipping = subtotal >= freeShippingThreshold;
  $: amountToFreeShipping = Math.max(0, freeShippingThreshold - subtotal);

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
      showToast(`Stock insuficiente. Disponible: ${item.stock_disponible}`, 'error');
      return;
    }

    isUpdatingId = item.id_carrito_detalle;

    try {
      await cartService.updateQuantity(item.id_carrito_detalle, { cantidad: nuevaCantidad });
    } catch (error: any) {
      showToast(error.message || 'Error al actualizar cantidad', 'error');
    } finally {
      isUpdatingId = null;
    }
  }

  // Abrir modal para eliminar item
  function openDeleteModal(item: any) {
    itemToDelete = item;
    showDeleteModal = true;
  }

  // Confirmar eliminación de item
  async function confirmDeleteItem() {
    if (!itemToDelete) return;

    isDeletingId = itemToDelete.id_carrito_detalle;
    showDeleteModal = false;

    try {
      await cartService.removeItem(itemToDelete.id_carrito_detalle);
      showToast(`"${itemToDelete.nombre}" eliminado del carrito`, 'success');
    } catch (error: any) {
      showToast(error.message || 'Error al eliminar producto', 'error');
    } finally {
      isDeletingId = null;
      itemToDelete = null;
    }
  }

  // Cancelar eliminación
  function cancelDelete() {
    showDeleteModal = false;
    itemToDelete = null;
  }

  // Abrir modal para vaciar carrito
  function openClearModal() {
    if (!items.length) return;
    showClearModal = true;
  }

  // Confirmar vaciar carrito
  async function confirmClearCart() {
    showClearModal = false;
    isClearing = true;

    try {
      await cartService.clearCart();
      showToast('Carrito vaciado correctamente', 'success');
    } catch (error: any) {
      showToast(error.message || 'Error al vaciar carrito', 'error');
    } finally {
      isClearing = false;
    }
  }

  // Cancelar vaciar carrito
  function cancelClear() {
    showClearModal = false;
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
  <div class="max-w-6xl mx-auto px-4 lg:px-6 py-6 space-y-4">

    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold tracking-tight text-text-light dark:text-text-dark">
          Carrito de Compras
        </h1>
        <p class="text-sm text-slate-600 dark:text-slate-400">
          {itemCount} {itemCount === 1 ? 'producto' : 'productos'}
        </p>
      </div>

      <button
        type="button"
        class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-lg border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors text-text-light dark:text-text-dark"
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
      <!-- Grid principal: productos a la izquierda, resumen a la derecha -->
      <div class="flex flex-col md:flex-row gap-6 items-start">

        <!-- Lista de items -->
        <section aria-label="Productos en el carrito" class="flex-1 min-w-0 space-y-2">
          {#each items as item (item.id_carrito_detalle)}
            <article
              class="relative rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm overflow-hidden"
            >
              <div class="p-3 flex items-center gap-4">
                <!-- Imagen -->
                <div class="flex-shrink-0">
                  <div class="w-16 h-16 rounded-lg bg-white dark:bg-slate-900 border border-border-light dark:border-border-dark flex items-center justify-center overflow-hidden">
                    {#if item.imagen_principal}
                      <img
                        src={item.imagen_principal}
                        alt={item.nombre}
                        class="w-full h-full object-contain p-1"
                      />
                    {:else}
                      <span class="text-xs text-slate-400">Sin img</span>
                    {/if}
                  </div>
                </div>

                <!-- Info del producto -->
                <div class="flex-1 min-w-0">
                  <h3 class="text-sm font-semibold text-text-light dark:text-text-dark truncate">
                    {item.nombre}
                  </h3>
                  <p class="text-xs text-slate-500 dark:text-slate-400 mt-0.5">
                    Stock: 
                    <span class="{item.stock_disponible > 10 ? 'text-emerald-600 dark:text-emerald-400' : item.stock_disponible > 0 ? 'text-amber-600 dark:text-amber-400' : 'text-red-600 dark:text-red-400'} font-medium">
                      {item.stock_disponible} uds
                    </span>
                  </p>
                  <p class="text-sm font-bold text-primary mt-1">
                    S/. {item.precio_unitario.toFixed(2)}
                  </p>
                </div>

                <!-- Cantidad -->
                <div class="flex-shrink-0">
                  <div class="inline-flex items-center rounded-lg bg-slate-100 dark:bg-slate-700 border border-border-light dark:border-border-dark">
                    <button
                      type="button"
                      class="w-8 h-8 flex items-center justify-center text-text-light dark:text-text-dark hover:bg-slate-200 dark:hover:bg-slate-600 disabled:opacity-40 transition-colors rounded-l-lg"
                      on:click={() => handleUpdateQuantity(item, -1)}
                      disabled={isUpdatingId === item.id_carrito_detalle}
                    >
                      <Minus size={14} />
                    </button>
                    <div class="w-10 text-center text-sm font-semibold text-text-light dark:text-text-dark">
                      {item.cantidad}
                    </div>
                    <button
                      type="button"
                      class="w-8 h-8 flex items-center justify-center text-text-light dark:text-text-dark hover:bg-slate-200 dark:hover:bg-slate-600 disabled:opacity-40 transition-colors rounded-r-lg"
                      on:click={() => handleUpdateQuantity(item, +1)}
                      disabled={isUpdatingId === item.id_carrito_detalle}
                    >
                      <Plus size={14} />
                    </button>
                  </div>
                </div>

                <!-- Subtotal -->
                <div class="flex-shrink-0 text-right hidden sm:block">
                  <p class="text-xs text-slate-500 dark:text-slate-400">Subtotal</p>
                  <p class="text-sm font-bold text-text-light dark:text-text-dark">
                    S/. {item.subtotal.toFixed(2)}
                  </p>
                </div>

                <!-- Eliminar -->
                <button
                  type="button"
                  class="flex-shrink-0 w-8 h-8 rounded-lg flex items-center justify-center text-slate-400 hover:text-red-500 hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors disabled:opacity-50"
                  on:click={() => openDeleteModal(item)}
                  disabled={isDeletingId === item.id_carrito_detalle}
                  aria-label="Eliminar producto"
                >
                  {#if isDeletingId === item.id_carrito_detalle}
                    <Loader2 size={16} class="animate-spin" />
                  {:else}
                    <Trash2 size={16} />
                  {/if}
                </button>
              </div>

              <!-- Subtotal en mobile -->
              <div class="sm:hidden px-3 pb-3 pt-0 flex justify-end">
                <p class="text-sm font-bold text-text-light dark:text-text-dark">
                  Subtotal: S/. {item.subtotal.toFixed(2)}
                </p>
              </div>
            </article>
          {/each}

          <!-- Clear cart -->
          <div class="flex items-center justify-between text-sm pt-2">
            <button
              type="button"
              class="text-red-600 dark:text-red-400 hover:text-red-700 dark:hover:text-red-300 disabled:opacity-50 text-xs font-medium flex items-center gap-1"
              on:click={openClearModal}
              disabled={isClearing || !items.length}
            >
              {#if isClearing}
                <Loader2 size={12} class="animate-spin" />
                Vaciando...
              {:else}
                Vaciar carrito
              {/if}
            </button>
            <p class="text-[11px] text-slate-500 dark:text-slate-400">
              Los productos no se reservan hasta completar el pago.
            </p>
          </div>
        </section>

        <!-- Resumen del pedido -->
        <aside
          aria-label="Resumen del pedido"
          class="w-full md:w-80 flex-shrink-0 rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-4 space-y-3 shadow-sm md:sticky md:top-20"
        >
          <h2 class="text-base font-bold text-text-light dark:text-text-dark">
            Resumen del Pedido
          </h2>

          <div class="space-y-2 text-sm">
            <div class="flex justify-between text-slate-600 dark:text-slate-400">
              <span>Subtotal</span>
              <span class="font-medium text-text-light dark:text-text-dark">
                S/. {subtotal.toFixed(2)}
              </span>
            </div>

            {#if descuento > 0}
              <div class="flex justify-between text-slate-600 dark:text-slate-400">
                <span>Descuento</span>
                <span class="font-medium text-green-600 dark:text-green-400">
                  − S/. {descuento.toFixed(2)}
                </span>
              </div>
            {/if}

            <div class="flex justify-between text-slate-600 dark:text-slate-400">
              <span>Envío</span>
              {#if qualifiesForFreeShipping}
                <span class="text-emerald-600 dark:text-emerald-400 font-medium text-xs flex items-center gap-1">
                  <Truck size={12} />
                  ¡Gratis!
                </span>
              {:else}
                <span class="text-xs text-slate-500">En checkout</span>
              {/if}
            </div>

            <!-- Mensaje de envío gratis -->
            {#if !qualifiesForFreeShipping && amountToFreeShipping > 0}
              <div class="rounded-lg bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 p-2">
                <p class="text-[11px] text-blue-700 dark:text-blue-300 flex items-center gap-1.5">
                  <Truck size={12} />
                  <span>Agrega <strong>S/. {amountToFreeShipping.toFixed(2)}</strong> más para envío gratis</span>
                </p>
              </div>
            {/if}

            <div class="border-t border-border-light dark:border-border-dark pt-2 flex justify-between items-center">
              <span class="text-sm font-semibold text-text-light dark:text-text-dark">Total</span>
              <span class="text-lg font-bold text-primary">
                S/. {total.toFixed(2)}
              </span>
            </div>
          </div>

          <!-- Cupón -->
          <div class="pt-2 space-y-2 border-t border-border-light dark:border-border-dark">
            <label class="flex items-center gap-1.5 text-xs font-medium text-slate-600 dark:text-slate-400">
              <Tag size={12} />
              Cupón de descuento
            </label>
            <div class="flex gap-2">
              <input
                type="text"
                class="flex-1 rounded-lg border border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900 px-3 py-2 text-sm text-text-light dark:text-text-dark placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-primary/50 disabled:opacity-50"
                placeholder="Código"
                bind:value={couponCode}
                disabled={appliedCoupon !== '' || isValidatingCoupon}
              />
              {#if appliedCoupon}
                <button
                  type="button"
                  class="px-3 py-2 rounded-lg text-xs font-semibold bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 hover:bg-red-100 dark:hover:bg-red-900/40 transition-colors"
                  on:click={removeCoupon}
                >
                  Quitar
                </button>
              {:else}
                <button
                  type="button"
                  class="px-3 py-2 rounded-lg text-xs font-semibold bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600 transition-colors text-text-light dark:text-text-dark disabled:opacity-50"
                  on:click={handleApplyCoupon}
                  disabled={isValidatingCoupon}
                >
                  {isValidatingCoupon ? '...' : 'Aplicar'}
                </button>
              {/if}
            </div>
            {#if couponMessage}
              <p class="text-xs {couponStatus === 'success' ? 'text-emerald-600 dark:text-emerald-400' : 'text-red-600 dark:text-red-400'}">
                {couponMessage}
              </p>
            {/if}
          </div>

          <!-- CTA -->
          <button
            type="button"
            class="w-full px-4 py-3 rounded-lg text-sm font-bold bg-primary text-white hover:bg-primary/90 transition-colors shadow-lg shadow-primary/25 disabled:opacity-60"
            on:click={goToCheckout}
            disabled={!items.length}
          >
            Finalizar compra
          </button>

          <p class="text-[10px] text-slate-500 dark:text-slate-400 text-center">
            Al continuar aceptas los Términos y Condiciones.
          </p>
        </aside>
      </div>
    {/if}
  </div>
</div>

<!-- Modal: Confirmar eliminar producto -->
{#if showDeleteModal && itemToDelete}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <!-- Overlay -->
    <div 
      class="absolute inset-0 bg-black/60 backdrop-blur-sm"
      on:click={cancelDelete}
      on:keydown={(e) => e.key === 'Escape' && cancelDelete()}
      role="button"
      tabindex="0"
    ></div>
    
    <!-- Modal -->
    <div class="relative bg-surface-light dark:bg-slate-800 rounded-2xl shadow-2xl max-w-md w-full p-6 space-y-4 animate-in fade-in zoom-in-95 duration-200">
      <!-- Header -->
      <div class="flex items-start gap-4">
        <div class="w-12 h-12 rounded-xl bg-red-100 dark:bg-red-900/30 flex items-center justify-center flex-shrink-0">
          <Trash2 class="text-red-600 dark:text-red-400" size={24} />
        </div>
        <div class="flex-1">
          <h3 class="text-lg font-bold text-text-light dark:text-text-dark">
            Eliminar producto
          </h3>
          <p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
            ¿Estás seguro de eliminar este producto del carrito?
          </p>
        </div>
        <button
          type="button"
          class="p-1 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-700 text-slate-400 transition-colors"
          on:click={cancelDelete}
        >
          <X size={20} />
        </button>
      </div>

      <!-- Producto a eliminar -->
      <div class="flex items-center gap-3 p-3 rounded-xl bg-slate-50 dark:bg-slate-900/50 border border-border-light dark:border-border-dark">
        <div class="w-12 h-12 rounded-lg bg-white dark:bg-slate-800 border border-border-light dark:border-border-dark flex items-center justify-center overflow-hidden">
          {#if itemToDelete.imagen_principal}
            <img src={itemToDelete.imagen_principal} alt={itemToDelete.nombre} class="w-full h-full object-contain p-1" />
          {:else}
            <ShoppingCart size={20} class="text-slate-400" />
          {/if}
        </div>
        <div class="flex-1 min-w-0">
          <p class="font-medium text-sm text-text-light dark:text-text-dark truncate">{itemToDelete.nombre}</p>
          <p class="text-xs text-slate-500">Cantidad: {itemToDelete.cantidad} · S/. {itemToDelete.subtotal.toFixed(2)}</p>
        </div>
      </div>

      <!-- Acciones -->
      <div class="flex gap-3 pt-2">
        <button
          type="button"
          class="flex-1 px-4 py-2.5 rounded-xl text-sm font-medium border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-700 hover:bg-slate-100 dark:hover:bg-slate-600 transition-colors text-text-light dark:text-text-dark"
          on:click={cancelDelete}
        >
          Cancelar
        </button>
        <button
          type="button"
          class="flex-1 px-4 py-2.5 rounded-xl text-sm font-medium bg-red-600 hover:bg-red-700 text-white transition-colors flex items-center justify-center gap-2"
          on:click={confirmDeleteItem}
        >
          <Trash2 size={16} />
          Eliminar
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Modal: Confirmar vaciar carrito -->
{#if showClearModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <!-- Overlay -->
    <div 
      class="absolute inset-0 bg-black/60 backdrop-blur-sm"
      on:click={cancelClear}
      on:keydown={(e) => e.key === 'Escape' && cancelClear()}
      role="button"
      tabindex="0"
    ></div>
    
    <!-- Modal -->
    <div class="relative bg-surface-light dark:bg-slate-800 rounded-2xl shadow-2xl max-w-md w-full p-6 space-y-4 animate-in fade-in zoom-in-95 duration-200">
      <!-- Header -->
      <div class="flex items-start gap-4">
        <div class="w-12 h-12 rounded-xl bg-amber-100 dark:bg-amber-900/30 flex items-center justify-center flex-shrink-0">
          <AlertTriangle class="text-amber-600 dark:text-amber-400" size={24} />
        </div>
        <div class="flex-1">
          <h3 class="text-lg font-bold text-text-light dark:text-text-dark">
            Vaciar carrito
          </h3>
          <p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
            Esta acción eliminará todos los productos de tu carrito. ¿Deseas continuar?
          </p>
        </div>
        <button
          type="button"
          class="p-1 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-700 text-slate-400 transition-colors"
          on:click={cancelClear}
        >
          <X size={20} />
        </button>
      </div>

      <!-- Resumen -->
      <div class="p-3 rounded-xl bg-slate-50 dark:bg-slate-900/50 border border-border-light dark:border-border-dark">
        <p class="text-sm text-slate-600 dark:text-slate-400">
          Se eliminarán <span class="font-bold text-text-light dark:text-text-dark">{itemCount} productos</span> con un total de <span class="font-bold text-primary">S/. {subtotal.toFixed(2)}</span>
        </p>
      </div>

      <!-- Acciones -->
      <div class="flex gap-3 pt-2">
        <button
          type="button"
          class="flex-1 px-4 py-2.5 rounded-xl text-sm font-medium border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-700 hover:bg-slate-100 dark:hover:bg-slate-600 transition-colors text-text-light dark:text-text-dark"
          on:click={cancelClear}
        >
          Cancelar
        </button>
        <button
          type="button"
          class="flex-1 px-4 py-2.5 rounded-xl text-sm font-medium bg-red-600 hover:bg-red-700 text-white transition-colors flex items-center justify-center gap-2"
          on:click={confirmClearCart}
        >
          <Trash2 size={16} />
          Vaciar todo
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Toast de notificación -->
{#if toastMessage}
  <div class="fixed bottom-6 right-6 z-50 animate-in slide-in-from-bottom-4 fade-in duration-300">
    <div class="flex items-center gap-3 px-4 py-3 rounded-xl shadow-lg {toastType === 'success' ? 'bg-green-600' : 'bg-red-600'} text-white">
      {#if toastType === 'success'}
        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
        </svg>
      {:else}
        <AlertTriangle size={20} />
      {/if}
      <p class="text-sm font-medium">{toastMessage}</p>
      <button
        type="button"
        class="p-1 hover:bg-white/20 rounded-lg transition-colors"
        on:click={() => { toastMessage = ''; toastType = ''; }}
      >
        <X size={16} />
      </button>
    </div>
  </div>
{/if}
