<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  
  // Get cupon ID from URL
  $: cuponId = $page.params.id;
  
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
  let loadingData = true;
  let errorMessage = '';
  let successMessage = '';
  
  // Assigned Users State
  let assignedUsuarios = [];
  let loadingAssignedUsers = false;
  let unassigningUserId = null;
  
  onMount(async () => {
    await Promise.all([
      fetchProductos(),
      loadCuponData(),
      fetchAssignedUsers()
    ]);
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
  
  async function loadCuponData() {
    try {
      const response = await fetch(`http://localhost:3000/api/cupones/${cuponId}`);
      if (response.ok) {
        const cupon = await response.json();
        
        codigo = cupon.codigo;
        nombre = cupon.nombre;
        descripcion = cupon.descripcion || '';
        valor = cupon.valor.toString();
        compraMinima = cupon.compra_minima?.toString() || '';
        usosMaximosTotales = cupon.usos_maximos_totales?.toString() || '';
        usosMaximosPorUsuario = cupon.usos_maximos_por_usuario?.toString() || '';
        soloNuevosUsuarios = cupon.solo_nuevos_usuarios || false;
        soloPrimeraCompra = cupon.solo_primera_compra || false;
        activo = cupon.activo ?? true;
        idReferencia = cupon.id_referencia;
        
        // Map database values to frontend values
        const tipoCuponMap = {
          'porcentaje': 'Porcentaje',
          'monto_fijo': 'Monto Fijo',
          'envio_gratis': 'Envío Gratis'
        };
        tipoCupon = tipoCuponMap[cupon.tipo_cupon] || 'Porcentaje';
        
        const aplicaCuponMap = {
          'todo': 'Todo',
          'producto': 'Producto',
          'categoria': 'Categoría',
          'marca': 'Marca',
          'familia': 'Familia'
        };
        aplicaCupon = aplicaCuponMap[cupon.aplica_cupon] || 'Todo';
        
        // Format dates for datetime-local input
        if (cupon.fecha_inicio) {
          const inicio = new Date(cupon.fecha_inicio);
          fechaInicio = inicio.toISOString().slice(0, 16);
        }
        if (cupon.fecha_fin) {
          const fin = new Date(cupon.fecha_fin);
          fechaFin = fin.toISOString().slice(0, 16);
        }
      } else {
        errorMessage = 'No se pudo cargar el cupón';
      }
    } catch (e) {
      errorMessage = 'Error al cargar el cupón: ' + e.message;
    } finally {
      loadingData = false;
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
      
      const response = await fetch(`http://localhost:3000/api/cupones/${cuponId}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload)
      });
      
      if (response.ok) {
        successMessage = 'Cupón actualizado exitosamente';
        setTimeout(() => {
          goto('/gestion-cupones');
        }, 1500);
      } else {
        const error = await response.text();
        errorMessage = 'Error al actualizar el cupón: ' + error;
      }
    } catch (e) {
      errorMessage = 'Error de conexión: ' + e.message;
    } finally {
      loading = false;
    }
  }
  
  async function fetchAssignedUsers() {
    loadingAssignedUsers = true;
    try {
      const response = await fetch(`http://localhost:3000/api/cupones/${cuponId}/usuarios`);
      if (response.ok) {
        assignedUsuarios = await response.json();
      } else {
        assignedUsuarios = [];
      }
    } catch (e) {
      console.error('Error fetching assigned users:', e);
      assignedUsuarios = [];
    } finally {
      loadingAssignedUsers = false;
    }
  }
  
  async function unassignUser(id_usuario) {
    if (!confirm('¿Estás seguro de desasignar este cupón del usuario?')) {
      return;
    }
    
    unassigningUserId = id_usuario;
    try {
      const response = await fetch(`http://localhost:3000/api/cupones/${cuponId}/usuarios/${id_usuario}`, {
        method: 'DELETE'
      });
      
      if (response.ok || response.status === 204) {
        // Remove from local array
        assignedUsuarios = assignedUsuarios.filter(u => u.id_usuario !== id_usuario);
        successMessage = 'Usuario desasignado exitosamente';
        setTimeout(() => successMessage = '', 3000);
      } else {
        errorMessage = 'Error al desasignar usuario';
        setTimeout(() => errorMessage = '', 3000);
      }
    } catch (e) {
      errorMessage = 'Error de conexión: ' + e.message;
      setTimeout(() => errorMessage = '', 3000);
    } finally {
      unassigningUserId = null;
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
        {#if loadingData}
          <div class="flex items-center justify-center py-20">
            <span class="material-symbols-outlined animate-spin text-4xl text-primary">progress_activity</span>
          </div>
        {:else}
          <!-- Header -->
          <div class="mb-8">
            <div class="flex items-center gap-3 mb-2">
              <button on:click={handleCancel} class="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg">
                <span class="material-symbols-outlined">arrow_back</span>
              </button>
              <h1 class="text-slate-900 dark:text-white text-4xl font-black leading-tight tracking-[-0.033em]">Editar Cupón</h1>
              <span class="px-3 py-1 bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300 text-sm font-medium rounded-full">
                {codigo}
              </span>
            </div>
            <p class="text-slate-500 dark:text-slate-400 text-base font-normal leading-normal ml-14">Modifica la configuración del cupón</p>
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

          <!-- Form (same as nuevo page) -->
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

            <!-- Assigned Users Section -->
            <div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-6">
              <h2 class="text-lg font-semibold text-slate-900 dark:text-white mb-4">Usuarios Asignados</h2>
              
              {#if loadingAssignedUsers}
                <div class="flex items-center justify-center py-8">
                  <span class="material-symbols-outlined animate-spin text-primary text-3xl">progress_activity</span>
                </div>
              {:else if assignedUsuarios.length > 0}
                <div class="space-y-2 max-h-96 overflow-y-auto">
                  {#each assignedUsuarios as usuario}
                    <div class="flex items-center justify-between p-3 bg-slate-50 dark:bg-slate-800 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors">
                      <div class="flex items-center gap-3">
                        <span class="material-symbols-outlined text-primary text-xl">person</span>
                        <div>
                          <p class="text-sm font-medium text-slate-900 dark:text-white">{usuario.nombre}</p>
                          <p class="text-xs text-slate-500 dark:text-slate-400">{usuario.email}</p>
                        </div>
                      </div>
                      <button
                        on:click={() => unassignUser(usuario.id_usuario)}
                        disabled={unassigningUserId === usuario.id_usuario}
                        class="p-2 text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                        title="Desasignar cupón"
                      >
                        {#if unassigningUserId === usuario.id_usuario}
                          <span class="material-symbols-outlined animate-spin text-base">progress_activity</span>
                        {:else}
                          <span class="material-symbols-outlined text-base">person_remove</span>
                        {/if}
                      </button>
                    </div>
                  {/each}
                </div>
                <p class="text-xs text-slate-500 dark:text-slate-400 mt-3">
                  Total: {assignedUsuarios.length} usuario{assignedUsuarios.length !== 1 ? 's' : ''}
                </p>
              {:else}
                <div class="text-center py-8">
                  <span class="material-symbols-outlined text-slate-300 dark:text-slate-700 text-5xl mb-2">group_off</span>
                  <p class="text-sm text-slate-500 dark:text-slate-400">
                    No hay usuarios asignados a este cupón
                  </p>
                  <p class="text-xs text-slate-400 dark:text-slate-500 mt-1">
                    Usa la asignación masiva desde la lista de cupones para asignar usuarios
                  </p>
                </div>
              {/if}
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
                {loading ? 'Actualizando...' : 'Actualizar Cupón'}
              </button>
            </div>
          </form>
        {/if}
      </div>
    </main>
  </div>
</div>
