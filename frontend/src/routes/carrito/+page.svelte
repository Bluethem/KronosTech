<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import type { CartResponse, CartItem } from '$lib/services/cart';
  import { getCurrentCart, updateItemQuantity, removeItem } from '$lib/services/cart';

  let loading = true;
  let error: string | null = null;
  let cart: CartResponse | null = null;

  async function loadCart() {
    loading = true;
    error = null;

    try {
      cart = await getCurrentCart();
    } catch (err) {
      console.error(err);
      error = err instanceof Error ? err.message : 'No se pudo cargar el carrito.';
    } finally {
      loading = false;
    }
  }

  onMount(loadCart);

  function calcSubtotal(items: CartItem[] = []) {
    return items.reduce((acc, item) => acc + item.precio_unitario * item.cantidad, 0);
  }

  $: subtotal = cart ? calcSubtotal(cart.items) : 0;
  $: descuentos = 0;
  $: envio = subtotal >= 500 ? 0 : subtotal > 0 ? 25 : 0;
  $: total = subtotal - descuentos + envio;
  $: totalItems = cart ? cart.items.reduce((acc, item) => acc + item.cantidad, 0) : 0;

  async function changeQuantity(item: CartItem, delta: number) {
    const nuevaCantidad = item.cantidad + delta;
    if (nuevaCantidad <= 0) return;

    try {
      loading = true;
      error = null;
      cart = await updateItemQuantity(item.id_carrito_detalle, nuevaCantidad);
    } catch (err) {
      console.error(err);
      error = err instanceof Error ? err.message : 'No se pudo actualizar la cantidad.';
    } finally {
      loading = false;
    }
  }

  async function deleteItem(item: CartItem) {
    try {
      loading = true;
      error = null;
      cart = await removeItem(item.id_carrito_detalle);
    } catch (err) {
      console.error(err);
      error = err instanceof Error ? err.message : 'No se pudo eliminar el producto.';
    } finally {
      loading = false;
    }
  }

  function formatCurrency(value: number) {
    return new Intl.NumberFormat('es-PE', {
      style: 'currency',
      currency: 'PEN',
      maximumFractionDigits: 2
    }).format(value);
  }

  function proceedToCheckout() {
    goto('/checkout');
  }
</script>

<svelte:head>
  <title>Carrito de compras | KronosTech</title>
</svelte:head>

<div class="max-w-screen-xl mx-auto px-4 sm:px-6 lg:px-10 py-10 space-y-8">
  <div class="flex flex-col sm:flex-row sm:items-baseline sm:justify-between gap-3">
    <div>
      <h1 class="text-2xl md:text-3xl font-bold tracking-[-0.02em] text-slate-900 dark:text-white">Tu carrito</h1>
      <p class="text-sm text-slate-600 dark:text-slate-400">
        {#if totalItems > 0}
          Tienes <span class="font-semibold">{totalItems}</span> {totalItems === 1 ? 'producto' : 'productos'} en tu carrito.
        {:else}
          No tienes productos en el carrito.
        {/if}
      </p>
    </div>

    <a href="/catalogo" class="text-sm font-semibold text-primary hover:underline">← Seguir comprando</a>
  </div>

  {#if error}
    <div class="rounded-xl bg-red-500/10 border border-red-500/40 px-4 py-3 text-sm text-red-200">
      {error}
    </div>
  {/if}

  {#if loading}
    <div class="flex items-center justify-center py-16">
      <div class="h-8 w-8 border-4 border-primary border-t-transparent rounded-full animate-spin" />
    </div>
  {:else if !cart || cart.items.length === 0}
    <div class="bg-surface-light dark:bg-slate-900 rounded-2xl shadow-lg p-10 text-center space-y-4 border border-slate-200/60 dark:border-slate-800/60">
      <p class="text-lg font-semibold text-slate-800 dark:text-slate-100">Tu carrito está vacío</p>
      <p class="text-sm text-slate-600 dark:text-slate-400">Explora el catálogo y añade componentes para armar tu PC ideal.</p>
      <a
        href="/catalogo"
        class="inline-flex items-center justify-center h-12 px-6 rounded-lg bg-primary text-white font-bold shadow-lg hover:bg-primary/90 transition"
      >
        Ir al catálogo
      </a>
    </div>
  {:else}
    <div class="grid grid-cols-1 lg:grid-cols-[minmax(0,2.2fr)_minmax(0,1fr)] gap-8">
      <div class="space-y-4">
        {#each cart.items as item (item.id_carrito_detalle)}
          <article class="bg-surface-light dark:bg-slate-900 rounded-xl shadow-sm border border-slate-200/60 dark:border-slate-800/80 p-4 sm:p-5 flex gap-4">
            <div class="h-24 w-24 sm:h-28 sm:w-28 rounded-lg bg-slate-200/40 dark:bg-slate-800/80 flex items-center justify-center overflow-hidden">
              {#if item.imagen_principal}
                <img src={item.imagen_principal} alt={item.nombre} class="h-full w-full object-cover" />
              {:else}
                <span class="text-xs text-slate-500">Imagen</span>
              {/if}
            </div>

            <div class="flex-1 flex flex-col gap-2">
              <div class="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-1">
                <div class="space-y-1">
                  <h2 class="text-sm sm:text-base font-semibold text-slate-900 dark:text-white">{item.nombre}</h2>
                  <p class="text-xs text-slate-500 dark:text-slate-400">SKU: {item.sku}</p>
                  <div class="inline-flex items-center gap-2 mt-1">
                    <span class="inline-flex items-center px-3 py-1 rounded-full text-[11px] font-semibold bg-emerald-500/15 text-emerald-400">
                      ● En stock
                    </span>
                    {#if item.stock_disponible != null}
                      <span class="text-[11px] text-slate-500 dark:text-slate-400">{item.stock_disponible} disponibles</span>
                    {/if}
                  </div>
                </div>

                <div class="text-right space-y-1 mt-2 sm:mt-0">
                  <p class="text-sm text-slate-500 dark:text-slate-400">Precio</p>
                  <p class="text-base font-semibold text-slate-900 dark:text-white">{formatCurrency(item.precio_unitario)}</p>
                </div>
              </div>

              <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3 pt-2 border-t border-slate-200/60 dark:border-slate-800/60 mt-auto">
                <div class="inline-flex items-center gap-3">
                  <button
                    class="h-9 w-9 rounded-full bg-slate-200 dark:bg-slate-800 text-slate-900 dark:text-white flex items-center justify-center text-lg hover:bg-slate-300 dark:hover:bg-slate-700 disabled:opacity-40"
                    on:click={() => changeQuantity(item, -1)}
                    disabled={loading || item.cantidad <= 1}
                    aria-label="Disminuir cantidad"
                  >
                    −
                  </button>
                  <span class="w-10 text-center text-sm font-semibold text-slate-900 dark:text-white">{item.cantidad}</span>
                  <button
                    class="h-9 w-9 rounded-full bg-slate-900 dark:bg-white text-white dark:text-slate-900 flex items-center justify-center text-lg hover:bg-slate-800 dark:hover:bg-slate-100 disabled:opacity-40"
                    on:click={() => changeQuantity(item, 1)}
                    disabled={loading}
                    aria-label="Aumentar cantidad"
                  >
                    +
                  </button>
                </div>

                <button
                  class="text-xs font-semibold text-red-500 hover:text-red-400 hover:underline self-start sm:self-center"
                  on:click={() => deleteItem(item)}
                  disabled={loading}
                >
                  Eliminar
                </button>

                <div class="text-right space-y-0.5">
                  <p class="text-xs text-slate-500 dark:text-slate-400">Subtotal</p>
                  <p class="text-base font-bold text-slate-900 dark:text-white">{formatCurrency(item.precio_unitario * item.cantidad)}</p>
                </div>
              </div>
            </div>
          </article>
        {/each}
      </div>

      <aside class="bg-surface-light dark:bg-slate-900 rounded-2xl shadow-lg border border-slate-200/60 dark:border-slate-800/80 p-6 space-y-5 h-fit sticky top-24">
        <h2 class="text-lg font-semibold text-slate-900 dark:text-white">Resumen de compra</h2>

        <div class="space-y-2 text-sm">
          <div class="flex justify-between">
            <span class="text-slate-500 dark:text-slate-400">Subtotal</span>
            <span class="font-medium text-slate-900 dark:text-white">{formatCurrency(subtotal)}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-slate-500 dark:text-slate-400">Descuentos</span>
            <span class="font-medium text-emerald-500">
              {descuentos > 0 ? `- ${formatCurrency(descuentos)}` : formatCurrency(0)}
            </span>
          </div>
          <div class="flex justify-between">
            <span class="text-slate-500 dark:text-slate-400">Envío estimado</span>
            <span class="font-medium text-slate-900 dark:text-white">{envio === 0 ? 'Gratis' : formatCurrency(envio)}</span>
          </div>
        </div>

        <div class="border-t border-slate-200/60 dark:border-slate-800/80 pt-4 flex justify-between items-baseline">
          <span class="text-sm font-semibold text-slate-600 dark:text-slate-300">Total</span>
          <span class="text-2xl font-bold text-slate-900 dark:text-white">{formatCurrency(total)}</span>
        </div>

        <button
          class="w-full h-12 rounded-lg bg-primary text-white font-bold shadow-lg hover:bg-primary/90 transition disabled:opacity-50 disabled:cursor-not-allowed"
          disabled={totalItems === 0 || loading}
          on:click={proceedToCheckout}
        >
          Ir a pagar
        </button>

        <p class="text-[11px] text-slate-500 dark:text-slate-400">
          Aceptamos Yape, Plin, Visa y Mastercard. Tus datos están protegidos con cifrado SSL.
        </p>
      </aside>
    </div>
  {/if}
</div>
