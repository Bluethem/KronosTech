<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  
  // Form state
  let codigo = '';
  let nombre = '';
  let descripcion = '';
  let tipoCupon = 'Porcentaje';
  let valor = '';
  let aplicaCupon = 'Todo';
  let idReferencia: number | null = null;
  let compraMinima = '';
  let usosMaximosTotales = '';
  let usosMaximosPorUsuario = '';
  let soloNuevosUsuarios = false;
  let soloPrimeraCompra = false;
  let fechaInicio = '';
  let fechaFin = '';
  let activo = true;
  
  // UI state
  let productos: Array<{id_producto_detalle: number, nombre: string}> = [];
  let loading = false;
  let errorMessage = '';
  let successMessage = '';
  
  onMount(async () => {
    await fetchProductos();
  });
  
  async function fetchProductos() {
    try {
      const response = await fetch('http://localhost:3000/api/productos/dropdown');
      if (response.ok) {
        productos = await response.json();
      }
    } catch (e) {
      console.error('Error fetching productos:', e);
    }
  }
  
  function validateForm(): string | null {
    if (!codigo.trim()) {
      return 'El código es requerido';
    }
    
    if (!nombre.trim()) {
      return 'El nombre es requerido';
    }
    
    if (!valor || parseFloat(valor) <= 0) {
      return 'El valor debe ser mayor a 0';
    }
    
    if (tipoCupon === 'Porcentaje' && parseFloat(valor) > 100) {
      return 'El porcentaje no puede ser mayor a 100';
    }
    
    if (aplicaCupon === 'Producto' && !idReferencia) {
      return 'Debe seleccionar un producto específico';
    }
    
    if (!fechaInicio) {
      return 'La fecha de inicio es requerida';
    }
    
    if (!fechaFin) {
      return 'La fecha de fin es requerida';
    }
    
    if (new Date(fechaFin) <= new Date(fechaInicio)) {
      return 'La fecha de fin debe ser posterior a la fecha de inicio';
    }
    
    return null;
  }
  
  async function handleSubmit() {
    errorMessage = '';
    successMessage = '';
    
    const validationError = validateForm();
    if (validationError) {
      errorMessage = validationError;
      return;
    }
    
    loading = true;
    
    try {
      const formatDateTime = (dateTimeLocal: string) => {
        if (!dateTimeLocal) return null;
        return dateTimeLocal.replace('T', ' ') + ':00';
      };
      
      const mapTipoCupon = (tipo: string) => {
        const map = {
          'Porcentaje': 'porcentaje',
          'Monto Fijo': 'monto_fijo',
          'Envío Gratis': 'envio_gratis'
        };
        return map[tipo] || tipo.toLowerCase();
      };
      
      const mapAplicaCupon = (aplica: string) => {
        const map = {
          'Todo': 'todo',
          'Producto': 'producto',
          'Categoría': 'categoria',
          'Marca': 'marca',
          'Familia': 'familia'
        };
        return map[aplica] || aplica.toLowerCase();
      };
      
      const payload = {
        codigo: codigo.trim(),
        nombre: nombre.trim(),
        descripcion: descripcion.trim() || null,
        tipo_cupon: mapTipoCupon(tipoCupon),
        valor: parseFloat(valor),
        aplica_cupon: mapAplicaCupon(aplicaCupon),
        id_referencia: aplicaCupon === 'Producto' ? idReferencia : null,
        compra_minima: compraMinima ? parseFloat(compraMinima) : null,
        usos_maximos_totales: usosMaximosTotales ? parseInt(usosMaximosTotales) : null,
        usos_maximos_por_usuario: usosMaximosPorUsuario ? parseInt(usosMaximosPorUsuario) : null,
        solo_nuevos_usuarios: soloNuevosUsuarios,
        solo_primera_compra: soloPrimeraCompra,
        fecha_inicio: formatDateTime(fechaInicio),
        fecha_fin: formatDateTime(fechaFin),
        activo: activo
      };
      
      const response = await fetch('http://localhost:3000/api/cupones', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload)
      });
      
      if (response.ok) {
        successMessage = 'Cupón creado exitosamente';
        setTimeout(() => {
          goto('/gestion-cupones');
        }, 1500);
      } else {
        const error = await response.text();
        errorMessage = 'Error al crear el cupón: ' + error;
      }
    } catch (e) {
      errorMessage = 'Error de conexión: ' + e.message;
    } finally {
      loading = false;
    }
  }
  
  function handleCancel() {
    goto('/gestion-cupones');
  }
</script>

<div class="relative flex h-auto min-h-screen w-full flex-col bg-background-light dark:bg-background-dark font-display text-[#111418] dark:text-gray-200">
  <div class="layout-container flex h-full grow flex-col">
    <main class="flex-1 p-6 lg:p-10">
      <div class="max-w-4xl mx-auto">
        <!-- Header -->
        <div class="mb-8">
          <div class="flex items-center gap-3 mb-2">
            <button on:click={handleCancel} class="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg">
              <span class="material-symbols-outlined">arrow_back</span>
            </button>
            <h1 class="text-slate-900 dark:text-white text-4xl font-black leading-tight tracking-[-0.033em]">Crear Nuevo Cupón</h1>
          </div>
          <p class="text-slate-500 dark:text-slate-400 text-base font-normal leading-normal ml-14">Configura un nuevo cupón de descuento para tus clientes</p>
        </div>

        <!-- Messages -->
        {#if errorMessage}
          <div class="mb-6 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
            <p class="text-red-800 dark:text-red-200 text-sm">{errorMessage}</p>
          </div>
        {/if}

        {#if successMessage}
          <div class="mb-6 p-4 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg">
            <p class="text-green-800 dark:text-green-200 text-sm">{successMessage}</p>
          </div>
        {/if}

        <!-- Form -->
        <form on:submit|preventDefault={handleSubmit} class="space-y-6">
          <!-- Basic Info -->
          <div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-6">
            <h2 class="text-lg font-semibold text-slate-900 dark:text-white mb-4">Información Básica</h2>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                  Código del Cupón *
                </label>
                <input
                  bind:value={codigo}
                  type="text"
                  placeholder="VERANO2024"
                  class="w-full px-4 py-2 border border-slate-300 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white focus:ring-2 focus:ring-primary/50 focus:border-primary"
                  required
                />
              </div>

              <div>
                <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                  Nombre del Cupón *
                </label>
                <input
                  bind:value={nombre}
                  type="text"
                  placeholder="Descuento de Verano"
                  class="w-full px-4 py-2 border border-slate-300 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white focus:ring-2 focus:ring-primary/50 focus:border-primary"
                  required
                />
              </div>
            </div>

            <div class="mt-4">
              <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                Descripción
              </label>
              <textarea
                bind:value={descripcion}
                rows="3"
                placeholder="Descripción opcional del cupón"
                class="w-full px-4 py-2 border border-slate-300 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white focus:ring-2 focus:ring-primary/50 focus:border-primary"
              ></textarea>
            </div>
          </div>

          <!-- Discount Configuration -->
          <div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-6">
            <h2 class="text-lg font-semibold text-slate-900 dark:text-white mb-4">Configuración del Descuento</h2>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                  Tipo de Cupón *
                </label>
                <select
                  bind:value={tipoCupon}
                  class="w-full px-4 py-2 border border-slate-300 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white focus:ring-2 focus:ring-primary/50 focus:border-primary"
                >
                  <option>Porcentaje</option>
                  <option>Monto Fijo</option>
                  <option>Envío Gratis</option>
                </select>
              </div>

              <div>
                <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                  Valor *
                </label>
                <input
                  bind:value={valor}
                  type="number"
                  step="0.01"
                  min="0"
                  placeholder={tipoCupon === 'Porcentaje' ? '15' : '100.00'}
                  class="w-full px-4 py-2 border border-slate-300 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white focus:ring-2 focus:ring-primary/50 focus:border-primary"
                  required
                  disabled={tipoCupon === 'Envío Gratis'}
                />
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-4">
              <div>
                <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                  Aplica a *
                </label>
                <select
                  bind:value={aplicaCupon}
                  class="w-full px-4 py-2 border border-slate-300 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white focus:ring-2 focus:ring-primary/50 focus:border-primary"
                >
                  <option>Todo</option>
                  <option>Producto</option>
                  <option>Categoría</option>
                  <option>Marca</option>
                  <option>Familia</option>
                </select>
              </div>

              {#if aplicaCupon === 'Producto'}
                <div>
                  <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                    Producto Específico *
                  </label>
                  <select
                    bind:value={idReferencia}
                    class="w-full px-4 py-2 border border-slate-300 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white focus:ring-2 focus:ring-primary/50 focus:border-primary"
                    required
                  >
                    <option value={null}>Seleccionar producto...</option>
                    {#each productos as producto}
                      <option value={producto.id_producto_detalle}>{producto.nombre}</option>
                    {/each}
                  </select>
                </div>
              {/if}
            </div>
          </div>

          <!-- Restrictions -->
          <div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-6">
            <h2 class="text-lg font-semibold text-slate-900 dark:text-white mb-4">Restricciones</h2>
            
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div>
                <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                  Compra Mínima
                </label>
                <input
                  bind:value={compraMinima}
                  type="number"
                  step="0.01"
                  min="0"
                  placeholder="0.00"
                  class="w-full px-4 py-2 border border-slate-300 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white focus:ring-2 focus:ring-primary/50 focus:border-primary"
                />
              </div>

              <div>
                <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                  Usos Máximos Totales
                </label>
                <input
                  bind:value={usosMaximosTotales}
                  type="number"
                  min="0"
                  placeholder="Ilimitado"
                  class="w-full px-4 py-2 border border-slate-300 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white focus:ring-2 focus:ring-primary/50 focus:border-primary"
                />
              </div>

              <div>
                <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                  Usos Máximos por Usuario
                </label>
                <input
                  bind:value={usosMaximosPorUsuario}
                  type="number"
                  min="0"
                  placeholder="Ilimitado"
                  class="w-full px-4 py-2 border border-slate-300 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white focus:ring-2 focus:ring-primary/50 focus:border-primary"
                />
              </div>
            </div>

            <div class="mt-4 space-y-3">
              <label class="flex items-center gap-3 cursor-pointer">
                <input
                  bind:checked={soloNuevosUsuarios}
                  type="checkbox"
                  class="w-4 h-4 text-primary bg-white dark:bg-slate-800 border-slate-300 dark:border-slate-700 rounded focus:ring-2 focus:ring-primary/50"
                />
                <span class="text-sm text-slate-700 dark:text-slate-300">Solo para nuevos usuarios</span>
              </label>

              <label class="flex items-center gap-3 cursor-pointer">
                <input
                  bind:checked={soloPrimeraCompra}
                  type="checkbox"
                  class="w-4 h-4 text-primary bg-white dark:bg-slate-800 border-slate-300 dark:border-slate-700 rounded focus:ring-2 focus:ring-primary/50"
                />
                <span class="text-sm text-slate-700 dark:text-slate-300">Solo para primera compra</span>
              </label>
            </div>
          </div>

          <!-- Validity Period -->
          <div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-6">
            <h2 class="text-lg font-semibold text-slate-900 dark:text-white mb-4">Período de Vigencia</h2>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                  Fecha de Inicio *
                </label>
                <input
                  bind:value={fechaInicio}
                  type="datetime-local"
                  class="w-full px-4 py-2 border border-slate-300 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white focus:ring-2 focus:ring-primary/50 focus:border-primary"
                  required
                />
              </div>

              <div>
                <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                  Fecha de Fin *
                </label>
                <input
                  bind:value={fechaFin}
                  type="datetime-local"
                  class="w-full px-4 py-2 border border-slate-300 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white focus:ring-2 focus:ring-primary/50 focus:border-primary"
                  required
                />
              </div>
            </div>

            <div class="mt-4">
              <label class="flex items-center gap-3 cursor-pointer">
                <input
                  bind:checked={activo}
                  type="checkbox"
                  class="w-4 h-4 text-primary bg-white dark:bg-slate-800 border-slate-300 dark:border-slate-700 rounded focus:ring-2 focus:ring-primary/50"
                />
                <span class="text-sm text-slate-700 dark:text-slate-300">Cupón activo</span>
              </label>
            </div>
          </div>

          <!-- Actions -->
          <div class="flex items-center justify-end gap-3">
            <button
              type="button"
              on:click={handleCancel}
              class="px-6 py-2 border border-slate-300 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-800 font-medium"
              disabled={loading}
            >
              Cancelar
            </button>
            <button
              type="submit"
              class="px-6 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 font-medium disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
              disabled={loading}
            >
              {#if loading}
                <span class="material-symbols-outlined animate-spin">progress_activity</span>
              {/if}
              {loading ? 'Guardando...' : 'Guardar Cupón'}
            </button>
          </div>
        </form>
      </div>
    </main>
  </div>
</div>
