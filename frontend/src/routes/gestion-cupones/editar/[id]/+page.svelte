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
  let categorias: Array<{id_categoria: number, nombre: string}> = [];
  let marcas: Array<{id_marca: number, nombre: string}> = [];
  let loading = false;
  let loadingData = true;
  let errorMessage = '';
  let successMessage = '';
  
  // Assigned Users State
  let assignedUsuarios: any[] = [];
  let loadingAssignedUsers = false;
  let unassigningUserId: number | null = null;
  
  // Estado original para revertir
  interface DatosOriginales {
    codigo: string;
    nombre: string;
    descripcion: string;
    tipoCupon: string;
    valor: string;
    aplicaCupon: string;
    idReferencia: number | null;
    compraMinima: string;
    usosMaximosTotales: string;
    usosMaximosPorUsuario: string;
    soloNuevosUsuarios: boolean;
    soloPrimeraCompra: boolean;
    fechaInicio: string;
    fechaFin: string;
    activo: boolean;
    referenciaNombre?: string;
  }
  let datosOriginales: DatosOriginales | null = null;
  let showConfirmModal = false;
  let cambiosPendientes: {campo: string, antes: string, despues: string}[] = [];
  let hayCambios = false;
  
  onMount(async () => {
    await Promise.all([
      fetchProductos(),
      fetchCategorias(),
      fetchMarcas(),
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
  
  async function fetchCategorias() {
    try {
      const response = await fetch('http://localhost:3000/api/categorias');
      if (response.ok) {
        const data = await response.json();
        categorias = data.data || data || [];
      }
    } catch (e) {
      console.error('Error fetching categorias:', e);
    }
  }
  
  async function fetchMarcas() {
    try {
      const response = await fetch('http://localhost:3000/api/marcas');
      if (response.ok) {
        const data = await response.json();
        marcas = data.data || data || [];
      }
    } catch (e) {
      console.error('Error fetching marcas:', e);
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
        const tipoCuponMap: Record<string, string> = {
          'porcentaje': 'Porcentaje',
          'monto_fijo': 'Monto Fijo',
          'envio_gratis': 'Envío Gratis'
        };
        tipoCupon = tipoCuponMap[cupon.tipo_cupon] || 'Porcentaje';
        
        const aplicaCuponMap: Record<string, string> = {
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
        
        // Guardar datos originales para poder revertir
        datosOriginales = {
          codigo,
          nombre,
          descripcion,
          tipoCupon,
          valor,
          aplicaCupon,
          idReferencia,
          compraMinima,
          usosMaximosTotales,
          usosMaximosPorUsuario,
          soloNuevosUsuarios,
          soloPrimeraCompra,
          fechaInicio,
          fechaFin,
          activo,
          referenciaNombre: cupon.referencia_nombre
        };
      } else {
        errorMessage = 'No se pudo cargar el cupón';
      }
    } catch (e: any) {
      errorMessage = 'Error al cargar el cupón: ' + e.message;
    } finally {
      loadingData = false;
    }
  }
  
  // Detectar cambios reactivamente
  $: if (datosOriginales) {
    hayCambios = 
      codigo !== datosOriginales.codigo ||
      nombre !== datosOriginales.nombre ||
      descripcion !== datosOriginales.descripcion ||
      tipoCupon !== datosOriginales.tipoCupon ||
      valor !== datosOriginales.valor ||
      aplicaCupon !== datosOriginales.aplicaCupon ||
      idReferencia !== datosOriginales.idReferencia ||
      compraMinima !== datosOriginales.compraMinima ||
      usosMaximosTotales !== datosOriginales.usosMaximosTotales ||
      usosMaximosPorUsuario !== datosOriginales.usosMaximosPorUsuario ||
      soloNuevosUsuarios !== datosOriginales.soloNuevosUsuarios ||
      soloPrimeraCompra !== datosOriginales.soloPrimeraCompra ||
      fechaInicio !== datosOriginales.fechaInicio ||
      fechaFin !== datosOriginales.fechaFin ||
      activo !== datosOriginales.activo;
  }
  
  function revertirCambios() {
    if (!datosOriginales) return;
    
    codigo = datosOriginales.codigo;
    nombre = datosOriginales.nombre;
    descripcion = datosOriginales.descripcion;
    tipoCupon = datosOriginales.tipoCupon;
    valor = datosOriginales.valor;
    aplicaCupon = datosOriginales.aplicaCupon;
    idReferencia = datosOriginales.idReferencia;
    compraMinima = datosOriginales.compraMinima;
    usosMaximosTotales = datosOriginales.usosMaximosTotales;
    usosMaximosPorUsuario = datosOriginales.usosMaximosPorUsuario;
    soloNuevosUsuarios = datosOriginales.soloNuevosUsuarios;
    soloPrimeraCompra = datosOriginales.soloPrimeraCompra;
    fechaInicio = datosOriginales.fechaInicio;
    fechaFin = datosOriginales.fechaFin;
    activo = datosOriginales.activo;
    
    successMessage = 'Cambios revertidos correctamente';
    setTimeout(() => successMessage = '', 3000);
  }
  
  function calcularCambios(): {campo: string, antes: string, despues: string}[] {
    if (!datosOriginales) return [];
    
    const cambios: {campo: string, antes: string, despues: string}[] = [];
    
    if (codigo !== datosOriginales.codigo) {
      cambios.push({ campo: 'Código', antes: datosOriginales.codigo, despues: codigo });
    }
    if (nombre !== datosOriginales.nombre) {
      cambios.push({ campo: 'Nombre', antes: datosOriginales.nombre, despues: nombre });
    }
    if (descripcion !== datosOriginales.descripcion) {
      cambios.push({ campo: 'Descripción', antes: datosOriginales.descripcion || '(vacío)', despues: descripcion || '(vacío)' });
    }
    if (tipoCupon !== datosOriginales.tipoCupon) {
      cambios.push({ campo: 'Tipo', antes: datosOriginales.tipoCupon, despues: tipoCupon });
    }
    if (valor !== datosOriginales.valor) {
      cambios.push({ campo: 'Valor', antes: datosOriginales.valor, despues: valor });
    }
    if (aplicaCupon !== datosOriginales.aplicaCupon) {
      cambios.push({ campo: 'Aplica a', antes: datosOriginales.aplicaCupon, despues: aplicaCupon });
    }
    if (idReferencia !== datosOriginales.idReferencia) {
      const nombreAntes = obtenerNombreReferencia(datosOriginales.aplicaCupon, datosOriginales.idReferencia) || datosOriginales.referenciaNombre || '(ninguno)';
      const nombreDespues = obtenerNombreReferencia(aplicaCupon, idReferencia) || '(ninguno)';
      cambios.push({ campo: 'Referencia específica', antes: nombreAntes, despues: nombreDespues });
    }
    if (compraMinima !== datosOriginales.compraMinima) {
      cambios.push({ campo: 'Compra mínima', antes: datosOriginales.compraMinima || '(sin límite)', despues: compraMinima || '(sin límite)' });
    }
    if (usosMaximosTotales !== datosOriginales.usosMaximosTotales) {
      cambios.push({ campo: 'Usos máximos totales', antes: datosOriginales.usosMaximosTotales || 'Ilimitado', despues: usosMaximosTotales || 'Ilimitado' });
    }
    if (usosMaximosPorUsuario !== datosOriginales.usosMaximosPorUsuario) {
      cambios.push({ campo: 'Usos máximos por usuario', antes: datosOriginales.usosMaximosPorUsuario || 'Ilimitado', despues: usosMaximosPorUsuario || 'Ilimitado' });
    }
    if (soloNuevosUsuarios !== datosOriginales.soloNuevosUsuarios) {
      cambios.push({ campo: 'Solo nuevos usuarios', antes: datosOriginales.soloNuevosUsuarios ? 'Sí' : 'No', despues: soloNuevosUsuarios ? 'Sí' : 'No' });
    }
    if (soloPrimeraCompra !== datosOriginales.soloPrimeraCompra) {
      cambios.push({ campo: 'Solo primera compra', antes: datosOriginales.soloPrimeraCompra ? 'Sí' : 'No', despues: soloPrimeraCompra ? 'Sí' : 'No' });
    }
    if (fechaInicio !== datosOriginales.fechaInicio) {
      cambios.push({ campo: 'Fecha inicio', antes: formatDateTimeDisplay(datosOriginales.fechaInicio), despues: formatDateTimeDisplay(fechaInicio) });
    }
    if (fechaFin !== datosOriginales.fechaFin) {
      cambios.push({ campo: 'Fecha fin', antes: formatDateTimeDisplay(datosOriginales.fechaFin), despues: formatDateTimeDisplay(fechaFin) });
    }
    if (activo !== datosOriginales.activo) {
      cambios.push({ campo: 'Estado', antes: datosOriginales.activo ? 'Activo' : 'Inactivo', despues: activo ? 'Activo' : 'Inactivo' });
    }
    
    return cambios;
  }
  
  function obtenerNombreReferencia(tipo: string, id: number | null): string | null {
    if (!id) return null;
    
    if (tipo === 'Producto') {
      const producto = productos.find(p => p.id_producto_detalle === id);
      return producto?.nombre || null;
    }
    if (tipo === 'Categoría') {
      const categoria = categorias.find(c => c.id_categoria === id);
      return categoria?.nombre || null;
    }
    if (tipo === 'Marca') {
      const marca = marcas.find(m => m.id_marca === id);
      return marca?.nombre || null;
    }
    return null;
  }
  
  function formatDateTimeDisplay(dateString: string): string {
    if (!dateString) return 'N/A';
    const date = new Date(dateString);
    return date.toLocaleString('es-PE', { 
      year: 'numeric', 
      month: '2-digit', 
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit'
    });
  }
  
  function mostrarConfirmacion() {
    cambiosPendientes = calcularCambios();
    if (cambiosPendientes.length === 0) {
      errorMessage = 'No hay cambios para guardar';
      return;
    }
    showConfirmModal = true;
  }
  
  function cerrarModal() {
    showConfirmModal = false;
    cambiosPendientes = [];
  }
  
  // Obtener opciones de referencia según el tipo seleccionado
  $: showReferenciaSelect = aplicaCupon === 'Producto' || aplicaCupon === 'Categoría' || aplicaCupon === 'Marca';
  
  $: opcionesReferencia = (() => {
    if (aplicaCupon === 'Producto') return productos.map(p => ({ id: p.id_producto_detalle, nombre: p.nombre }));
    if (aplicaCupon === 'Categoría') return categorias.map(c => ({ id: c.id_categoria, nombre: c.nombre }));
    if (aplicaCupon === 'Marca') return marcas.map(m => ({ id: m.id_marca, nombre: m.nombre }));
    return [];
  })();
  
  $: labelReferencia = aplicaCupon === 'Producto' ? 'Producto Específico' : aplicaCupon === 'Categoría' ? 'Categoría Específica' : 'Marca Específica';
  
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
    
    if ((aplicaCupon === 'Producto' || aplicaCupon === 'Categoría' || aplicaCupon === 'Marca') && !idReferencia) {
      return `Debe seleccionar ${aplicaCupon === 'Producto' ? 'un producto' : aplicaCupon === 'Categoría' ? 'una categoría' : 'una marca'} específica`;
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
  
  function iniciarGuardado() {
    errorMessage = '';
    successMessage = '';
    
    const validationError = validateForm();
    if (validationError) {
      errorMessage = validationError;
      return;
    }
    
    // Mostrar modal de confirmación con cambios
    mostrarConfirmacion();
  }
  
  async function handleSubmit() {
    cerrarModal();
    loading = true;
    
    try {
      const formatDateTime = (dateTimeLocal: string) => {
        if (!dateTimeLocal) return null;
        return dateTimeLocal.replace('T', ' ') + ':00';
      };
      
      const mapTipoCupon = (tipo: string) => {
        const map: Record<string, string> = {
          'Porcentaje': 'porcentaje',
          'Monto Fijo': 'monto_fijo',
          'Envío Gratis': 'envio_gratis'
        };
        return map[tipo] || tipo.toLowerCase();
      };
      
      const mapAplicaCupon = (aplica: string) => {
        const map: Record<string, string> = {
          'Todo': 'todo',
          'Producto': 'producto',
          'Categoría': 'categoria',
          'Marca': 'marca',
          'Familia': 'familia'
        };
        return map[aplica] || aplica.toLowerCase();
      };
      
      // Determinar id_referencia según el tipo de aplicación
      let refId: number | null = null;
      if (aplicaCupon === 'Producto' || aplicaCupon === 'Categoría' || aplicaCupon === 'Marca') {
        refId = idReferencia;
      }
      
      const payload = {
        codigo: codigo.trim(),
        nombre: nombre.trim(),
        descripcion: descripcion.trim() || null,
        tipo_cupon: mapTipoCupon(tipoCupon),
        valor: parseFloat(valor),
        aplica_cupon: mapAplicaCupon(aplicaCupon),
        id_referencia: refId,
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
        // Actualizar datos originales con los nuevos valores
        datosOriginales = {
          codigo,
          nombre,
          descripcion,
          tipoCupon,
          valor,
          aplicaCupon,
          idReferencia,
          compraMinima,
          usosMaximosTotales,
          usosMaximosPorUsuario,
          soloNuevosUsuarios,
          soloPrimeraCompra,
          fechaInicio,
          fechaFin,
          activo
        };
        setTimeout(() => {
          goto('/gestion-cupones');
        }, 1500);
      } else {
        const error = await response.text();
        errorMessage = 'Error al actualizar el cupón: ' + error;
      }
    } catch (e: any) {
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

                {#if showReferenciaSelect}
                  <div>
                    <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                      {labelReferencia} *
                    </label>
                    <select
                      bind:value={idReferencia}
                      class="w-full px-4 py-2 border border-slate-300 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white focus:ring-2 focus:ring-primary/50 focus:border-primary"
                      required
                    >
                      <option value={null}>Seleccionar {aplicaCupon.toLowerCase()}...</option>
                      {#each opcionesReferencia as opcion (opcion.id)}
                        <option value={opcion.id}>{opcion.nombre}</option>
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
            <div class="flex flex-wrap items-center justify-between gap-4">
              <div class="flex items-center gap-2">
                {#if hayCambios}
                  <button
                    type="button"
                    on:click={revertirCambios}
                    class="flex items-center justify-center gap-2 px-4 py-2 rounded-lg bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-300 font-medium hover:bg-amber-200 dark:hover:bg-amber-900/50 transition-colors"
                  >
                    <span class="material-symbols-outlined text-xl">undo</span>
                    Revertir Cambios
                  </button>
                  <span class="text-sm text-amber-600 dark:text-amber-400">
                    ({calcularCambios().length} cambio{calcularCambios().length !== 1 ? 's' : ''} pendiente{calcularCambios().length !== 1 ? 's' : ''})
                  </span>
                {/if}
              </div>
              <div class="flex items-center gap-3">
                <button
                  type="button"
                  on:click={handleCancel}
                  class="px-6 py-2 border border-slate-300 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-800 font-medium"
                  disabled={loading}
                >
                  Cancelar
                </button>
                <button
                  type="button"
                  on:click={iniciarGuardado}
                  class="px-6 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 font-medium disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
                  disabled={loading || !hayCambios}
                >
                  {#if loading}
                    <span class="material-symbols-outlined animate-spin">progress_activity</span>
                  {/if}
                  {loading ? 'Actualizando...' : 'Actualizar Cupón'}
                </button>
              </div>
            </div>
          </form>
        {/if}
      </div>
    </main>
  </div>
</div>

<!-- Modal de Confirmación de Cambios -->
{#if showConfirmModal}
  <div class="fixed inset-0 bg-black/50 z-40 flex items-center justify-center p-4" on:click={cerrarModal} on:keydown={(e) => e.key === 'Escape' && cerrarModal()} role="dialog" tabindex="-1">
    <div class="bg-white dark:bg-gray-900 rounded-xl shadow-2xl max-w-lg w-full max-h-[80vh] overflow-hidden" on:click|stopPropagation role="document">
      <!-- Header -->
      <div class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
        <div class="flex items-center gap-3">
          <div class="flex items-center justify-center w-10 h-10 rounded-full bg-amber-100 dark:bg-amber-900/30">
            <span class="material-symbols-outlined text-amber-600 dark:text-amber-400">warning</span>
          </div>
          <div>
            <h3 class="text-lg font-bold text-gray-900 dark:text-white">Confirmar Cambios</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">Revisa los cambios antes de guardar</p>
          </div>
        </div>
        <button on:click={cerrarModal} class="p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-500 dark:text-gray-400">
          <span class="material-symbols-outlined">close</span>
        </button>
      </div>
      
      <!-- Content -->
      <div class="p-4 overflow-y-auto max-h-[50vh]">
        <p class="text-sm text-gray-600 dark:text-gray-300 mb-4">
          Se realizarán los siguientes cambios al cupón "<strong>{datosOriginales?.codigo}</strong>":
        </p>
        
        <div class="space-y-3">
          {#each cambiosPendientes as cambio (cambio.campo)}
            <div class="bg-gray-50 dark:bg-gray-800/50 rounded-lg p-3 border border-gray-200 dark:border-gray-700">
              <p class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider mb-2">{cambio.campo}</p>
              <div class="flex items-center gap-2 text-sm flex-wrap">
                <span class="px-2 py-1 rounded bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300 line-through">
                  {cambio.antes}
                </span>
                <span class="material-symbols-outlined text-gray-400">arrow_forward</span>
                <span class="px-2 py-1 rounded bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300">
                  {cambio.despues}
                </span>
              </div>
            </div>
          {/each}
        </div>
      </div>
      
      <!-- Footer -->
      <div class="flex justify-end gap-3 p-4 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50">
        <button 
          on:click={cerrarModal}
          class="flex items-center justify-center h-10 px-4 rounded-lg bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 text-sm font-medium hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
        >
          Cancelar
        </button>
        <button 
          on:click={handleSubmit}
          disabled={loading}
          class="flex items-center justify-center gap-2 h-10 px-4 rounded-lg bg-primary text-white text-sm font-medium hover:bg-primary/90 transition-colors disabled:opacity-50"
        >
          <span class="material-symbols-outlined text-lg">check</span>
          {loading ? 'Guardando...' : 'Confirmar y Guardar'}
        </button>
      </div>
    </div>
  </div>
{/if}
