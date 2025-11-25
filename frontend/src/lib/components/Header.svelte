<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { theme, toggleTheme } from '$lib/stores/theme';
  import { searchHistory } from '$lib/stores/searchHistory';
  import { catalogoService } from '$lib/services/api';
  import { ShoppingCart, User, Heart, Search, Sun, Moon, Menu, Clock, X } from 'lucide-svelte';
  import logo from '$lib/assets/favicon.svg';
  
  let searchQuery = '';
  let mobileMenuOpen = false;
  let showAutocomplete = false;
  let autocompleteResults: any[] = [];
  let searchTimeout: number;
  let searchInputRef: HTMLDivElement;
  
  $: isDark = $theme === 'dark';
  
  async function handleAutocomplete() {
    if (searchQuery.trim().length < 2) {
      autocompleteResults = [];
      return;
    }
    
    clearTimeout(searchTimeout);
    searchTimeout = window.setTimeout(async () => {
      try {
        const response = await catalogoService.getProductos({ 
          search: searchQuery.trim(), 
          limit: 5 
        });
        autocompleteResults = response.data;
      } catch (err) {
        console.error('Error en autocompletado:', err);
      }
    }, 300);
  }
  
  function handleSearch(e: Event) {
    e.preventDefault();
    if (searchQuery.trim()) {
      searchHistory.addSearch(searchQuery.trim());
      goto(`/busqueda?q=${encodeURIComponent(searchQuery.trim())}`);
      showAutocomplete = false;
      autocompleteResults = [];
    }
  }
  
  function selectSuggestion(term: string) {
    searchQuery = term;
    handleSearch(new Event('submit'));
  }
  
  function handleClickOutside(event: MouseEvent) {
    if (searchInputRef && !searchInputRef.contains(event.target as Node)) {
      showAutocomplete = false;
    }
  }
  
  onMount(() => {
    document.addEventListener('click', handleClickOutside);
    return () => {
      document.removeEventListener('click', handleClickOutside);
    };
  });
  
  $: if (searchQuery) {
    handleAutocomplete();
    showAutocomplete = true;
  } else {
    autocompleteResults = [];
  }
</script>

<header class="sticky top-0 z-50 flex items-center justify-between whitespace-nowrap border-b border-border-light dark:border-border-dark px-4 sm:px-6 lg:px-10 py-3 backdrop-blur-sm">
  <div class="flex items-center gap-8">
    <a href="/" class="flex items-center gap-3 text-text-light dark:text-text-dark">
      <img src={logo} alt="KronosTech Logo" class="size-8" />
      <h2 class="hidden sm:block text-lg font-bold leading-tight tracking-[-0.015em]">KronosTech</h2>
    </a>
    <nav class="hidden lg:flex items-center gap-9">
      <a class="text-sm font-medium leading-normal hover:text-primary transition-colors" href="/catalogo">Catálogo</a>
      <a class="text-sm font-medium leading-normal hover:text-primary transition-colors" href="/ofertas">Ofertas</a>
      <a class="text-sm font-medium leading-normal hover:text-primary transition-colors" href="/novedades">Novedades</a>
      <a class="text-sm font-medium leading-normal hover:text-primary transition-colors" href="/destacados">Destacados</a>
    </nav>
  </div>

  <div class="flex flex-1 justify-end items-center gap-4 sm:gap-6 lg:gap-8">
    <!-- Búsqueda -->
    <div class="hidden md:flex relative min-w-40 max-w-64" bind:this={searchInputRef}>
      <form on:submit={handleSearch} class="w-full">
        <div class="flex w-full items-stretch rounded-lg h-10">
          <button 
            type="submit"
            class="text-slate-500 dark:text-slate-400 flex border-none bg-slate-200 dark:bg-surface-dark items-center justify-center pl-3 rounded-l-lg border-r-0 hover:text-primary transition-colors"
          >
            <Search size={20} />
          </button>
          <input 
            type="text"
            bind:value={searchQuery}
            on:focus={() => showAutocomplete = true}
            class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg focus:outline-0 focus:ring-2 focus:ring-primary/50 border-none bg-slate-200 dark:bg-surface-dark h-full placeholder:text-slate-500 dark:placeholder:text-slate-400 px-4 rounded-l-none border-l-0 pl-2 text-base font-normal leading-normal text-text-light dark:text-text-dark" 
            placeholder="Buscar componentes..."
            autocomplete="off"
          />
        </div>
      </form>
      
      <!-- Autocompletado -->
      {#if showAutocomplete && (autocompleteResults.length > 0 || $searchHistory.length > 0)}
        <div class="absolute top-full left-0 right-0 mt-2 bg-surface-light dark:bg-surface-dark border border-border-light dark:border-border-dark rounded-lg shadow-xl z-50 max-h-96 overflow-y-auto">
          <!-- Historial -->
          {#if searchQuery.length === 0 && $searchHistory.length > 0}
            <div class="p-2">
              <div class="flex items-center justify-between px-3 py-2">
                <p class="text-xs font-semibold text-slate-500 dark:text-slate-400 uppercase">Búsquedas recientes</p>
                <button 
                  on:click={() => searchHistory.clearHistory()}
                  class="text-xs text-slate-500 hover:text-red-500 transition-colors"
                >
                  Limpiar
                </button>
              </div>
              {#each $searchHistory as term}
                <button
                  on:click={() => selectSuggestion(term)}
                  class="w-full flex items-center gap-3 px-3 py-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded transition-colors text-left"
                >
                  <Clock size={16} class="text-slate-400" />
                  <span class="text-sm">{term}</span>
                </button>
              {/each}
            </div>
          {/if}
          
          <!-- Sugerencias de productos -->
          {#if autocompleteResults.length > 0}
            <div class="p-2 border-t border-border-light dark:border-border-dark">
              <p class="text-xs font-semibold text-slate-500 dark:text-slate-400 uppercase px-3 py-2">Productos</p>
              {#each autocompleteResults as producto}
                <a
                  href="/producto/{producto.id_producto_detalle}"
                  class="flex items-center gap-3 px-3 py-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded transition-colors"
                  on:click={() => showAutocomplete = false}
                >
                  <img 
                    src={producto.imagen_principal || 'https://placehold.co/40x40'} 
                    alt={producto.nombre}
                    class="w-10 h-10 object-contain bg-white rounded"
                  />
                  <div class="flex-1 min-w-0">
                    <p class="text-sm font-medium truncate">{producto.nombre}</p>
                    <p class="text-xs text-slate-500 dark:text-slate-400">S/. {producto.precio_venta.toFixed(2)}</p>
                  </div>
                </a>
              {/each}
            </div>
          {/if}
        </div>
      {/if}
    </div>

    <div class="flex gap-2">
      <!-- Toggle Dark Mode -->
      <button 
        on:click={toggleTheme}
        class="flex max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 bg-slate-200 dark:bg-surface-dark text-text-light dark:text-text-dark hover:bg-slate-300 dark:hover:bg-slate-700/60 transition-colors gap-2 text-sm font-bold leading-normal tracking-[0.015em] min-w-0 px-2.5"
        aria-label="Cambiar tema"
      >
        {#if isDark}
          <Sun size={20} />
        {:else}
          <Moon size={20} />
        {/if}
      </button>

      <!-- Usuario -->
      <button class="flex max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 bg-slate-200 dark:bg-surface-dark text-text-light dark:text-text-dark hover:bg-slate-300 dark:hover:bg-slate-700/60 transition-colors gap-2 text-sm font-bold leading-normal tracking-[0.015em] min-w-0 px-2.5">
        <User size={20} />
      </button>

      <!-- Favoritos -->
      <button class="flex max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 bg-slate-200 dark:bg-surface-dark text-text-light dark:text-text-dark hover:bg-slate-300 dark:hover:bg-slate-700/60 transition-colors gap-2 text-sm font-bold leading-normal tracking-[0.015em] min-w-0 px-2.5">
        <Heart size={20} />
      </button>

      <!-- Carrito -->
      <button class="relative flex max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 bg-slate-200 dark:bg-surface-dark text-text-light dark:text-text-dark hover:bg-slate-300 dark:hover:bg-slate-700/60 transition-colors gap-2 text-sm font-bold leading-normal tracking-[0.015em] min-w-0 px-2.5">
        <ShoppingCart size={20} />
        <span class="absolute -top-1 -right-1 flex h-4 w-4 items-center justify-center rounded-full bg-primary text-white text-xs font-bold">0</span>
      </button>

      <!-- Menú móvil -->
      <button 
        on:click={() => mobileMenuOpen = !mobileMenuOpen}
        class="lg:hidden flex max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 bg-slate-200 dark:bg-surface-dark text-text-light dark:text-text-dark hover:bg-slate-300 dark:hover:bg-slate-700/60 transition-colors gap-2 text-sm font-bold leading-normal tracking-[0.015em] min-w-0 px-2.5"
      >
        <Menu size={20} />
      </button>
    </div>
  </div>
</header>

<!-- Menú móvil -->
{#if mobileMenuOpen}
  <div class="lg:hidden bg-surface-light dark:bg-surface-dark border-b border-border-light dark:border-border-dark">
    <nav class="flex flex-col gap-4 p-4">
      <a class="text-sm font-medium leading-normal hover:text-primary transition-colors" href="/catalogo">Catálogo</a>
      <a class="text-sm font-medium leading-normal hover:text-primary transition-colors" href="/ofertas">Ofertas</a>
      <a class="text-sm font-medium leading-normal hover:text-primary transition-colors" href="/novedades">Novedades</a>
      <a class="text-sm font-medium leading-normal hover:text-primary transition-colors" href="/destacados">Destacados</a>
      
      <!-- Búsqueda móvil -->
      <form on:submit={handleSearch} class="flex md:hidden">
        <div class="flex w-full flex-1 items-stretch rounded-lg h-10">
          <button 
            type="submit"
            class="text-slate-500 dark:text-slate-400 flex border-none bg-slate-200 dark:bg-surface-dark items-center justify-center pl-3 rounded-l-lg border-r-0 hover:text-primary transition-colors"
          >
            <Search size={20} />
          </button>
          <input 
            type="text"
            bind:value={searchQuery}
            class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg focus:outline-0 focus:ring-2 focus:ring-primary/50 border-none bg-slate-200 dark:bg-surface-dark h-full placeholder:text-slate-500 dark:placeholder:text-slate-400 px-4 rounded-l-none border-l-0 pl-2 text-base font-normal leading-normal text-text-light dark:text-text-dark" 
            placeholder="Buscar componentes..."
          />
        </div>
      </form>
    </nav>
  </div>
{/if}
