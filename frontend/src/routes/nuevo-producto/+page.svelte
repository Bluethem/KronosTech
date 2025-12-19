<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  
  // Form state
  let formData = $state({
    nombre: '',
    descripcion: '',
    modelo: '',
    sku: '',
    codigo_barras: '',
    id_categoria: 0,
    id_marca: 0,
    precio_base: '',
    precio_venta: '',
    costo: '',
    cantidad_inicial: '',
    cantidad_minima: '5',
    ubicacion_fisica: '',
    peso: '',
    dimensiones: '',
    garantia_meses: '12',
    activo: true,
    es_destacado: false,
    es_nuevo: false,
    es_oferta: false
  });
  
  // Catalog data
  let categorias = $state([]);
  let marcas = $state([]);
  let loading = $state(true);
  let submitting = $state(false);
  
  // SKU validation
  let skuAvailable = $state(null);
  let checkingSku = $state(false);
  
  // Calculated margin
  let margin = $state(0);
  
  // Image URL
  let imagenUrl = $state('');
  
  onMount(async () => {
    await fetchCatalogData();
  });
  
  async function fetchCatalogData() {
    loading = true;
    try {
      const [categoriasRes, marcasRes] = await Promise.all([
        fetch('http://localhost:3000/api/categorias'),
        fetch('http://localhost:3000/api/marcas')
      ]);
      
      const categoriasData = await categoriasRes.json();
      const marcasData = await marcasRes.json();
      
      categorias = categoriasData.data || [];
      marcas = marcasData.data || [];
      
      if (categorias.length > 0) formData.id_categoria = categorias[0].id_categoria;
      if (marcas.length > 0) formData.id_marca = marcas[0].id_marca;
    } catch (e) {
      console.error('Error fetching catalog data:', e);
    } finally {
      loading = false;
    }
  }
  
  async function checkSkuAvailability() {
    if (!formData.sku || formData.sku.length < 3) {
      skuAvailable = null;
      return;
    }
    
    checkingSku = true;
    try {
      const response = await fetch('http://localhost:3000/api/productos/check-sku', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formData.sku)
      });
      skuAvailable = await response.json();
    } catch (e) {
      console.error('Error checking SKU:', e);
      skuAvailable = null;
    } finally {
      checkingSku = false;
    }
  }
  
  function calculateMargin() {
    const costo = parseFloat(formData.costo) || 0;
    const precioVenta = parseFloat(formData.precio_venta) || 0;
    
    if (costo > 0 && precioVenta > 0) {
      margin = ((precioVenta - costo) / costo) * 100;
    } else {
      margin = 0;
    }
  }
  
  async function handleSubmit() {
    if (!formData.nombre || !formData.sku || !formData.id_categoria || !formData.id_marca) {
      alert('Por favor complete los campos requeridos');
      return;
    }
    
    if (skuAvailable === false) {
      alert('El SKU ya está en uso. Por favor use uno diferente.');
      return;
    }
    
    submitting = true;
    try {
      const payload = {
        nombre: formData.nombre,
        descripcion: formData.descripcion || null,
        modelo: formData.modelo || null,
        sku: formData.sku,
        codigo_barras: formData.codigo_barras || null,
        id_categoria: parseInt(formData.id_categoria),
        id_marca: parseInt(formData.id_marca),
        precio_base: parseFloat(formData.precio_base) || 0,
        precio_venta: parseFloat(formData.precio_venta) || 0,
        costo: formData.costo ? parseFloat(formData.costo) : null,
        cantidad_inicial: formData.cantidad_inicial ? parseInt(formData.cantidad_inicial) : null,
        cantidad_minima: formData.cantidad_minima ? parseInt(formData.cantidad_minima) : null,
        ubicacion_fisica: formData.ubicacion_fisica || null,
        peso: formData.peso ? parseFloat(formData.peso) : null,
        dimensiones: formData.dimensiones || null,
        garantia_meses: formData.garantia_meses ? parseInt(formData.garantia_meses) : null,
        activo: formData.activo,
        es_destacado: formData.es_destacado,
        es_nuevo: formData.es_nuevo,
        es_oferta: formData.es_oferta,
        imagen_principal: imagenUrl || null
      };
      
      const response = await fetch('http://localhost:3000/api/productos', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });
      
      if (!response.ok) throw new Error('Error al crear producto');
      
      const result = await response.json();
      alert('Producto creado exitosamente!');
      goto('/inventario');
    } catch (e) {
      console.error('Error creating product:', e);
      alert('Error al crear el producto');
    } finally {
      submitting = false;
    }
  }
  
  // Reactive effects
  $effect(() => {
    if (formData.sku && formData.sku.length >= 3) {
      checkSkuAvailability();
    }
  });
  
  $effect(() => {
    calculateMargin();
  });
</script>

<div class="relative flex h-auto min-h-screen w-full flex-col group/design-root overflow-x-hidden bg-background-light dark:bg-background-dark font-display">
<div class="layout-container flex h-full grow flex-col">
<main class="flex-1 flex flex-col p-6 md:p-8">
<header class="flex flex-wrap items-center justify-between gap-4 mb-8">
<h1 class="text-gray-900 dark:text-white text-3xl font-bold leading-tight tracking-tight">Crear Nuevo Producto</h1>
</header>
<div class="grid grid-cols-1 lg:grid-cols-3 gap-8 mb-28">
<!-- Columna Izquierda -->
<div class="lg:col-span-2 flex flex-col gap-6">
<!-- Información Básica -->
<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[#DEE2E6] dark:border-gray-700">
<h2 class="text-[#111418] dark:text-white text-xl font-bold p-5 border-b border-[#DEE2E6] dark:border-gray-700">Información Básica</h2>
<div class="p-5 grid grid-cols-1 md:grid-cols-2 gap-6">
<label class="flex flex-col md:col-span-2">
<p class="text-sm font-medium leading-normal pb-2">Nombre del Producto *</p>
<input bind:value={formData.nombre} class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 focus:outline-0 focus:ring-0 h-12 p-3 text-base" placeholder="Ej: Laptop Pro 15 pulgadas"/>
</label>
<label class="flex flex-col md:col-span-2">
<p class="text-sm font-medium leading-normal pb-2">Descripción</p>
<textarea bind:value={formData.descripcion} class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 focus:outline-0 focus:ring-0 min-h-32 p-3 text-base" placeholder="Describe las características principales de tu producto..."></textarea>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Modelo</p>
<input bind:value={formData.modelo} class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 focus:outline-0 focus:ring-0 h-12 p-3 text-base" placeholder="LP-15-PRO"/>
</label>
<div class="relative flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">SKU *</p>
<input 
  bind:value={formData.sku}
  class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 h-12 p-3 text-base pr-10"
  class:border-green-600={skuAvailable === true}
  class:border-red-600={skuAvailable === false}
  class:border-gray-300={skuAvailable === null}
  placeholder="LP-PRO-15-BLK-512"
/>
{#if checkingSku}
  <span class="material-symbols-outlined absolute right-3 top-[43px] text-gray-400 animate-spin">progress_activity</span>
{:else if skuAvailable === true}
  <span class="material-symbols-outlined absolute right-3 top-[43px] text-green-600">check_circle</span>
  <p class="text-xs text-green-600 mt-1">SKU disponible</p>
{:else if skuAvailable === false}
  <span class="material-symbols-outlined absolute right-3 top-[43px] text-red-600">cancel</span>
  <p class="text-xs text-red-600 mt-1">SKU ya está en uso</p>
{/if}
</div>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Código de Barras (EAN, UPC)</p>
<input bind:value={formData.codigo_barras} class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 focus:outline-0 focus:ring-0 h-12 p-3 text-base" placeholder="Ej: 7891234567890"/>
</label>
</div>
</div>
<!-- Categorización -->
<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[#DEE2E6] dark:border-gray-700">
<h2 class="text-[#111418] dark:text-white text-xl font-bold p-5 border-b border-[#DEE2E6] dark:border-gray-700">Categorización</h2>
<div class="p-5 grid grid-cols-1 md:grid-cols-2 gap-6">
<label class="flex flex-col md:col-span-2">
<p class="text-sm font-medium leading-normal pb-2">Categoría *</p>
<select bind:value={formData.id_categoria} class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base">
  {#if loading}
    <option>Cargando...</option>
  {:else}
    {#each categorias as categoria}
      <option value={categoria.id_categoria}>{categoria.nombre}</option>
    {/each}
  {/if}
</select>
</label>
<label class="flex flex-col md:col-span-2">
<p class="text-sm font-medium leading-normal pb-2">Marca *</p>
<select bind:value={formData.id_marca} class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base">
  {#if loading}
    <option>Cargando...</option>
  {:else}
    {#each marcas as marca}
      <option value={marca.id_marca}>{marca.nombre}</option>
    {/each}
  {/if}
</select>
</label>
<!-- URL de Imagen -->
<label class="flex flex-col md:col-span-2">
<p class="text-sm font-medium leading-normal pb-2">URL de Imagen del Producto</p>
<input bind:value={imagenUrl} class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" placeholder="https://images.unsplash.com/photo-example?w=400" type="url"/>
<p class="text-xs text-[#6C757D] mt-1">Ingresa la URL de la imagen del producto (ej: desde Unsplash, Imgur, etc.)</p>
</label>
</div>
</div>
<!-- Precios -->
<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[#DEE2E6] dark:border-gray-700">
<h2 class="text-[#111418] dark:text-white text-xl font-bold p-5 border-b border-[#DEE2E6] dark:border-gray-700">Precios</h2>
<div class="p-5 grid grid-cols-1 md:grid-cols-2 gap-6">
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Precio Base (PEN) *</p>
<input bind:value={formData.precio_base} class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" placeholder="4500.00" type="number"/>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Precio de Venta (PEN) *</p>
<input bind:value={formData.precio_venta} class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" placeholder="5200.00" type="number"/>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Costo (Opcional)</p>
<input bind:value={formData.costo} class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" placeholder="4000.00" type="number"/>
</label>
<div class="flex flex-col justify-end">
<p class="text-sm font-medium leading-normal pb-2">Margen</p>
<div class="bg-accent-light dark:bg-primary/20 h-12 rounded-lg flex items-center px-4">
<p class="text-[#212529] dark:text-white font-semibold">{margin.toFixed(2)}%</p>
</div>
</div>
</div>
</div>
<!-- Inventario -->
<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[#DEE2E6] dark:border-gray-700">
<h2 class="text-[#111418] dark:text-white text-xl font-bold p-5 border-b border-[#DEE2E6] dark:border-gray-700">Inventario Inicial</h2>
<div class="p-5 grid grid-cols-1 md:grid-cols-3 gap-6">
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Cantidad Inicial</p>
<input bind:value={formData.cantidad_inicial} class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" placeholder="100" type="number"/>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Stock Mínimo</p>
<input bind:value={formData.cantidad_minima} class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" type="number"/>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Ubicación Física</p>
<input bind:value={formData.ubicacion_fisica} class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" placeholder="Almacén A, Pasillo 3"/>
</label>
</div>
</div>
<!-- Físico y Garantía -->
<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[#DEE2E6] dark:border-gray-700">
<h2 class="text-[#111418] dark:text-white text-xl font-bold p-5 border-b border-[#DEE2E6] dark:border-gray-700">Físico y Garantía</h2>
<div class="p-5 grid grid-cols-1 md:grid-cols-3 gap-6">
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Peso (kg)</p>
<input bind:value={formData.peso} class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" placeholder="1.8" type="number"/>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Dimensiones (cm)</p>
<input bind:value={formData.dimensiones} class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" placeholder="30x20x10"/>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Garantía (meses)</p>
<input bind:value={formData.garantia_meses} class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" type="number"/>
</label>
</div>
</div>
</div>
<!-- Columna Derecha -->
<div class="lg:col-span-1 flex flex-col gap-6">
<!-- Imágenes del Producto -->
<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[#DEE2E6] dark:border-gray-700">
<h2 class="text-[#111418] dark:text-white text-xl font-bold p-5 border-b border-[#DEE2E6] dark:border-gray-700">Imágenes del Producto</h2>
<div class="p-5 flex flex-col gap-4">
  <!-- Input para agregar URL -->
  <div class="flex gap-2">
    <input 
      bind:value={imagenUrl}
      class="form-input flex-1 rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" 
      placeholder="https://images.unsplash.com/photo-example?w=400" 
      type="url"
    />
    <button 
      type="button"
      class="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 font-semibold text-sm whitespace-nowrap"
    >
      Agregar
    </button>
  </div>
  <p class="text-xs text-[#6C757D] dark:text-gray-400">Ingresa la URL de la imagen del producto (ej: desde Unsplash, Imgur, etc.)</p>
  
  <!-- Preview de imagen actual -->
  {#if imagenUrl}
    <div class="border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg p-4">
      <p class="text-sm font-medium text-[#111418] dark:text-white mb-2">Vista previa:</p>
      <img 
        src={imagenUrl} 
        alt="Vista previa" 
        class="w-full h-48 object-cover rounded-lg"
        on:error={() => {}}
      />
    </div>
  {:else}
    <div class="border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg p-8 text-center">
      <span class="material-symbols-outlined text-4xl text-gray-400 dark:text-gray-500 mx-auto">image</span>
      <p class="mt-2 text-sm text-[#6C757D] dark:text-gray-400">Ingresa una URL para ver la vista previa</p>
    </div>
  {/if}
</div>
</div>
<!-- Estado -->
<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[#DEE2E6] dark:border-gray-700">
<h2 class="text-[#111418] dark:text-white text-xl font-bold p-5 border-b border-[#DEE2E6] dark:border-gray-700">Estado</h2>
<div class="p-5 flex flex-col gap-2">
<div class="flex items-center justify-between">
<label class="font-medium text-[#212529] dark:text-white" for="product-status">Estado del Producto</label>
<label class="relative inline-flex items-center cursor-pointer">
<input bind:checked={formData.activo} class="sr-only peer" id="product-status" type="checkbox"/>
<div class="w-14 h-7 bg-gray-200 peer-focus:outline-none rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:left-[4px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-6 after:w-6 after:transition-all dark:border-gray-600 peer-checked:bg-primary"></div>
<span class="ml-3 text-sm font-medium text-gray-900 dark:text-gray-300">Activo</span>
</label>
</div>
<p class="text-sm text-[#6C757D] dark:text-gray-400">Los productos inactivos no aparecen en la tienda.</p>
</div>
</div>
<!-- Destacados -->
<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[#DEE2E6] dark:border-gray-700">
<h2 class="text-[#111418] dark:text-white text-xl font-bold p-5 border-b border-[#DEE2E6] dark:border-gray-700">Destacados</h2>
<div class="p-5 flex flex-col gap-4">
<label class="flex items-center gap-3">
<input bind:checked={formData.es_destacado} class="h-5 w-5 rounded border-gray-300 text-primary focus:ring-primary" type="checkbox"/>
<span class="text-base text-[#212529] dark:text-white">Es Destacado</span>
</label>
<label class="flex items-center gap-3">
<input bind:checked={formData.es_nuevo} class="h-5 w-5 rounded border-gray-300 text-primary focus:ring-primary" type="checkbox"/>
<span class="text-base text-[#212529] dark:text-white">Es Nuevo</span>
</label>
<label class="flex items-center gap-3">
<input bind:checked={formData.es_oferta} class="h-5 w-5 rounded border-gray-300 text-primary focus:ring-primary" type="checkbox"/>
<span class="text-base text-[#212529] dark:text-white">Es Oferta</span>
</label>
</div>
</div>
<!-- SEO -->
<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[#DEE2E6] dark:border-gray-700">
<details>
<summary class="text-[#111418] dark:text-white text-xl font-bold p-5 flex justify-between items-center cursor-pointer list-none">
Configuración SEO
<span class="material-symbols-outlined transition-transform duration-300 transform">expand_more</span>
</summary>
<div class="p-5 border-t border-[#DEE2E6] dark:border-gray-700 flex flex-col gap-4">
<label class="flex flex-col">
<div class="flex justify-between">
<p class="text-sm font-medium leading-normal pb-2">Meta Título</p>
<p class="text-sm text-[#6C757D]">23/60</p>
</div>
<input class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" value="Laptop Pro 15 pulgadas"/>
</label>
<label class="flex flex-col">
<div class="flex justify-between">
<p class="text-sm font-medium leading-normal pb-2">Meta Descripción</p>
<p class="text-sm text-[#6C757D]">45/160</p>
</div>
<textarea class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 min-h-28 p-3 text-base">La mejor laptop para profesionales creativos.</textarea>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Slug URL</p>
<input class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" value="laptop-pro-15-pulgadas"/>
<p class="text-xs text-[#6C757D] mt-1">tutienda.com/productos/<strong>laptop-pro-15-pulgadas</strong></p>
</label>
</div>
</details>
</div>
</div>
</div>
</main>
</div>
<footer class="fixed bottom-0 left-0 right-0 bg-white dark:bg-gray-800 border-t border-[#DEE2E6] dark:border-gray-700 shadow-lg p-4 z-20">
<div class="max-w-screen-2xl mx-auto px-4 sm:px-6 lg:px-20 flex justify-between items-center">
<button on:click={() => goto('/inventario')} class="px-5 py-2.5 rounded-lg text-base font-semibold text-[#212529] dark:text-white bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600">Cancelar</button>
<div class="flex items-center gap-4">
<button 
  on:click={handleSubmit}
  disabled={submitting}
  class="px-5 py-2.5 rounded-lg text-base font-semibold text-white bg-primary hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed"
>
  {submitting ? 'Guardando...' : 'Guardar y Publicar'}
</button>
</div>
</div>
</footer>
</div>
