<script lang="ts">
  import type { Producto } from '$lib/services/api';
  import { ShoppingCart } from 'lucide-svelte';
  import { cartService } from '$lib/services/cart';
  import { isAuthenticated } from '$lib/stores/auth';
  import { goto } from '$app/navigation';

  export let producto: Producto;

  const imagenPlaceholder = 'https://placehold.co/400x300/1e293b/94a3b8?text=Producto';

  let isAdding = false;

  $: descuento = producto.descuento_porcentaje
    ? Math.round(((producto.precio_base - producto.precio_venta) / producto.precio_base) * 100)
    : null;

  async function handleAddToCart(e: Event) {
    e.preventDefault();
    e.stopPropagation();

    // Verificar si está autenticado
    if (!$isAuthenticated) {
      await goto('/login?redirect=' + encodeURIComponent(window.location.pathname));
      return;
    }

    if (isAdding || producto.stock_disponible === 0) return;

    isAdding = true;
    try {
      await cartService.addItem({
        id_producto_detalle: producto.id_producto_detalle,
        cantidad: 1
      });
      // Redirigir al carrito sin mostrar alert
      await goto('/carrito');
    } catch (error: any) {
      alert(error.message || 'Error al agregar al carrito');
      isAdding = false;
    }
  }
</script>

<a 
  href="/producto/{producto.id_producto_detalle}"
  class="flex h-full flex-1 flex-col gap-4 rounded-xl shadow-md hover:shadow-xl transition-all duration-300 overflow-hidden group"
>
  <div class="relative w-full bg-center bg-no-repeat aspect-video bg-cover flex flex-col overflow-hidden">
    <img 
      src={producto.imagen_principal || imagenPlaceholder} 
      alt={producto.nombre}
      class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
    />
    
    <!-- Badges -->
    <div class="absolute top-2 left-2 flex gap-2">
      {#if producto.es_nuevo}
        <span class="bg-green-500 text-white text-xs font-bold px-2 py-1 rounded">NUEVO</span>
      {/if}
      {#if producto.es_oferta && descuento}
        <span class="bg-red-500 text-white text-xs font-bold px-2 py-1 rounded">-{descuento}%</span>
      {/if}
      {#if producto.es_destacado}
        <span class="bg-primary text-white text-xs font-bold px-2 py-1 rounded">★ DESTACADO</span>
      {/if}
    </div>

    <!-- Stock badge -->
    {#if producto.stock_disponible === 0}
      <div class="absolute inset-0 bg-black/60 flex items-center justify-center">
        <span class="bg-red-500 text-white text-sm font-bold px-4 py-2 rounded">AGOTADO</span>
      </div>
    {:else if producto.stock_disponible < 5}
      <span class="absolute top-2 right-2 bg-orange-500 text-white text-xs font-bold px-2 py-1 rounded">
        ¡Solo {producto.stock_disponible}!
      </span>
    {/if}
  </div>

  <div class="flex flex-col flex-1 justify-between p-4 pt-0 gap-4">
    <div>
      <p class="text-xs text-slate-500 dark:text-slate-400 mb-1">{producto.marca}</p>
      <p class="text-base font-medium leading-normal mb-2 line-clamp-2">{producto.nombre}</p>
      
      <!-- Rating -->
      {#if producto.valoracion_promedio && producto.total_valoraciones > 0}
        <div class="flex items-center gap-2 mb-2">
          <div class="flex items-center">
            <span class="text-yellow-500">★</span>
            <span class="text-sm font-medium ml-1">{producto.valoracion_promedio.toFixed(1)}</span>
          </div>
          <span class="text-xs text-slate-500 dark:text-slate-400">({producto.total_valoraciones})</span>
        </div>
      {/if}

      <div class="flex items-center gap-2">
        <p class="text-lg font-bold text-primary">S/. {producto.precio_venta.toFixed(2)}</p>
        {#if producto.precio_base > producto.precio_venta}
          <p class="text-sm line-through text-slate-500 dark:text-slate-400">S/. {producto.precio_base.toFixed(2)}</p>
        {/if}
      </div>
    </div>

    <button
      class="flex w-full min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-primary/20 dark:bg-primary/30 text-primary text-sm font-bold leading-normal tracking-[0.015em] hover:bg-primary hover:text-white transition-colors gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
      disabled={producto.stock_disponible === 0 || isAdding}
      on:click={handleAddToCart}
    >
      <ShoppingCart size={18} />
      <span class="truncate">
        {#if isAdding}
          Agregando...
        {:else if producto.stock_disponible === 0}
          Agotado
        {:else}
          Añadir
        {/if}
      </span>
    </button>
  </div>
</a>
