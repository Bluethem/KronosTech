<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  
  // Get discount ID from URL
  $: descuentoId = $page.params.id;
  
  // Form state
  let nombre = '';
  let descripcion = '';
  let tipoDescuento = 'Porcentaje';
  let valor = '';
  let aplicaA = 'Global';
  let idReferencia: number | null = null;
  let compraMinima = '';
  let cantidadMinima = '';
  let usosMaximos = '';
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
  
  // Estado original para revertir
  interface DatosOriginales {
    nombre: string;
    descripcion: string;
    tipoDescuento: string;
    valor: string;
    aplicaA: string;
    idReferencia: number | null;
    compraMinima: string;
    cantidadMinima: string;
    usosMaximos: string;
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
      loadDescuentoData()
    ]);
  });
  
  async function loadDescuentoData() {
    try {
      const response = await fetch(`http://localhost:3000/api/descuentos/${descuentoId}`);
      if (response.ok) {
        const data = await response.json();
        
        // Map database values back to UI values
        nombre = data.nombre;
        descripcion = data.descripcion || '';
        tipoDescuento = mapTipoDescuentoFromDB(data.tipo_descuento);
        valor = data.valor.toString();
        aplicaA = mapAplicaAFromDB(data.aplica_a);
        idReferencia = data.id_referencia;
        compraMinima = data.compra_minima ? data.compra_minima.toString() : '';
        cantidadMinima = data.cantidad_minima ? data.cantidad_minima.toString() : '';
        usosMaximos = data.usos_maximos ? data.usos_maximos.toString() : '';
        
        // Format dates for datetime-local input
        fechaInicio = formatDateTimeForInput(data.fecha_inicio);
        fechaFin = formatDateTimeForInput(data.fecha_fin);
        activo = data.activo ?? true;
        
        // Guardar datos originales para poder revertir
        datosOriginales = {
          nombre,
          descripcion,
          tipoDescuento,
          valor,
          aplicaA,
          idReferencia,
          compraMinima,
          cantidadMinima,
          usosMaximos,
          fechaInicio,
          fechaFin,
          activo,
          referenciaNombre: data.referencia_nombre
        };
      } else {
        errorMessage = 'Error al cargar el descuento';
      }
    } catch (e) {
      console.error('Error loading descuento:', e);
      errorMessage = 'Error de conexión al cargar el descuento';
    } finally {
      loadingData = false;
    }
  }
  
  // Detectar cambios reactivamente
  $: if (datosOriginales) {
    hayCambios = 
      nombre !== datosOriginales.nombre ||
      descripcion !== datosOriginales.descripcion ||
      tipoDescuento !== datosOriginales.tipoDescuento ||
      valor !== datosOriginales.valor ||
      aplicaA !== datosOriginales.aplicaA ||
      idReferencia !== datosOriginales.idReferencia ||
      compraMinima !== datosOriginales.compraMinima ||
      cantidadMinima !== datosOriginales.cantidadMinima ||
      usosMaximos !== datosOriginales.usosMaximos ||
      fechaInicio !== datosOriginales.fechaInicio ||
      fechaFin !== datosOriginales.fechaFin ||
      activo !== datosOriginales.activo;
  }
  
  function revertirCambios() {
    if (!datosOriginales) return;
    
    nombre = datosOriginales.nombre;
    descripcion = datosOriginales.descripcion;
    tipoDescuento = datosOriginales.tipoDescuento;
    valor = datosOriginales.valor;
    aplicaA = datosOriginales.aplicaA;
    idReferencia = datosOriginales.idReferencia;
    compraMinima = datosOriginales.compraMinima;
    cantidadMinima = datosOriginales.cantidadMinima;
    usosMaximos = datosOriginales.usosMaximos;
    fechaInicio = datosOriginales.fechaInicio;
    fechaFin = datosOriginales.fechaFin;
    activo = datosOriginales.activo;
    
    successMessage = 'Cambios revertidos correctamente';
    setTimeout(() => successMessage = '', 3000);
  }
  
  function calcularCambios(): {campo: string, antes: string, despues: string}[] {
    if (!datosOriginales) return [];
    
    const cambios: {campo: string, antes: string, despues: string}[] = [];
    
    if (nombre !== datosOriginales.nombre) {
      cambios.push({ campo: 'Nombre', antes: datosOriginales.nombre, despues: nombre });
    }
    if (descripcion !== datosOriginales.descripcion) {
      cambios.push({ campo: 'Descripción', antes: datosOriginales.descripcion || '(vacío)', despues: descripcion || '(vacío)' });
    }
    if (tipoDescuento !== datosOriginales.tipoDescuento) {
      cambios.push({ campo: 'Tipo', antes: datosOriginales.tipoDescuento, despues: tipoDescuento });
    }
    if (valor !== datosOriginales.valor) {
      cambios.push({ campo: 'Valor', antes: datosOriginales.valor, despues: valor });
    }
    if (aplicaA !== datosOriginales.aplicaA) {
      cambios.push({ campo: 'Aplica a', antes: datosOriginales.aplicaA, despues: aplicaA });
    }
    if (idReferencia !== datosOriginales.idReferencia) {
      const nombreAntes = obtenerNombreReferencia(datosOriginales.aplicaA, datosOriginales.idReferencia) || datosOriginales.referenciaNombre || '(ninguno)';
      const nombreDespues = obtenerNombreReferencia(aplicaA, idReferencia) || '(ninguno)';
      cambios.push({ campo: 'Referencia específica', antes: nombreAntes, despues: nombreDespues });
    }
    if (compraMinima !== datosOriginales.compraMinima) {
      cambios.push({ campo: 'Compra mínima', antes: datosOriginales.compraMinima || '(sin límite)', despues: compraMinima || '(sin límite)' });
    }
    if (cantidadMinima !== datosOriginales.cantidadMinima) {
      cambios.push({ campo: 'Cantidad mínima', antes: datosOriginales.cantidadMinima || '(sin límite)', despues: cantidadMinima || '(sin límite)' });
    }
    if (usosMaximos !== datosOriginales.usosMaximos) {
      cambios.push({ campo: 'Usos máximos', antes: datosOriginales.usosMaximos || 'Ilimitado', despues: usosMaximos || 'Ilimitado' });
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
  
  function mapTipoDescuentoFromDB(tipo: string): string {
    const map = {
      'porcentaje': 'Porcentaje',
      'monto_fijo': 'Monto Fijo',
      'envio_gratis': 'Envío Gratis'
    };
    return map[tipo] || tipo;
  }
  
  function mapAplicaAFromDB(aplica: string): string {
    const map = {
      'global': 'Global',
      'producto': 'Producto',
      'categoria': 'Categoría',
      'marca': 'Marca',
      'familia': 'Familia'
    };
    return map[aplica] || aplica;
  }
  
  function formatDateTimeForInput(dateString: string): string {
    if (!dateString) return '';
    // Convert "2025-12-03 14:30:00" to "2025-12-03T14:30"
    return dateString.slice(0, 16).replace(' ', 'T');
  }
  
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
  
  function validateForm(): string | null {
    if (!nombre.trim()) {
      return 'El nombre es requerido';
    }
    
    if (!valor || parseFloat(valor) <= 0) {
      return 'El valor debe ser mayor a 0';
    }
    
    if (tipoDescuento === 'Porcentaje' && parseFloat(valor) > 100) {
      return 'El porcentaje no puede ser mayor a 100';
    }
    
    if ((aplicaA === 'Producto' || aplicaA === 'Categoría' || aplicaA === 'Marca') && !idReferencia) {
      return `Debe seleccionar ${aplicaA === 'Producto' ? 'un producto' : aplicaA === 'Categoría' ? 'una categoría' : 'una marca'} específica`;
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
      // Format dates properly for NaiveDateTime (YYYY-MM-DD HH:MM:SS)
      const formatDateTime = (dateTimeLocal: string) => {
        if (!dateTimeLocal) return null;
        // datetime-local format: "2025-12-03T14:30"
        // We need: "2025-12-03 14:30:00"
        return dateTimeLocal.replace('T', ' ') + ':00';
      };
      
      // Map frontend values to database ENUM values
      const mapTipoDescuento = (tipo: string) => {
        const map: Record<string, string> = {
          'Porcentaje': 'porcentaje',
          'Monto Fijo': 'monto_fijo',
          'Envío Gratis': 'envio_gratis'
        };
        return map[tipo] || tipo.toLowerCase();
      };
      
      const mapAplicaA = (aplica: string) => {
        const map: Record<string, string> = {
          'Global': 'global',
          'Producto': 'producto',
          'Categoría': 'categoria',
          'Marca': 'marca',
          'Familia': 'familia'
        };
        return map[aplica] || aplica.toLowerCase();
      };
      
      // Determinar id_referencia según el tipo de aplicación
      let refId: number | null = null;
      if (aplicaA === 'Producto' || aplicaA === 'Categoría' || aplicaA === 'Marca') {
        refId = idReferencia;
      }
      
      const payload = {
        nombre: nombre.trim(),
        descripcion: descripcion.trim() || null,
        tipo_descuento: mapTipoDescuento(tipoDescuento),
        valor: parseFloat(valor),
        aplica_a: mapAplicaA(aplicaA),
        id_referencia: refId,
        compra_minima: compraMinima ? parseFloat(compraMinima) : null,
        cantidad_minima: cantidadMinima ? parseInt(cantidadMinima) : null,
        usos_maximos: usosMaximos ? parseInt(usosMaximos) : null,
        fecha_inicio: formatDateTime(fechaInicio),
        fecha_fin: formatDateTime(fechaFin),
        activo: activo
      };
      
      console.log('Sending payload:', payload); // Debug
      
      const response = await fetch(`http://localhost:3000/api/descuentos/${descuentoId}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload)
      });
      
      if (response.ok) {
        successMessage = 'Descuento actualizado exitosamente';
        // Actualizar datos originales con los nuevos valores
        datosOriginales = {
          nombre,
          descripcion,
          tipoDescuento,
          valor,
          aplicaA,
          idReferencia,
          compraMinima,
          cantidadMinima,
          usosMaximos,
          fechaInicio,
          fechaFin,
          activo
        };
        setTimeout(() => {
          goto('/gestion-descuentos');
        }, 1500);
      } else {
        const errorText = await response.text();
        errorMessage = `Error al actualizar descuento: ${errorText || response.statusText}`;
      }
    } catch (e: any) {
      errorMessage = `Error de conexión: ${e.message}`;
      console.error('Error updating descuento:', e);
    } finally {
      loading = false;
    }
  }
  
  $: showReferenciaSelect = aplicaA === 'Producto' || aplicaA === 'Categoría' || aplicaA === 'Marca';
  $: valueIcon = tipoDescuento === 'Porcentaje' ? 'percent' : 'attach_money';
  
  // Obtener opciones de referencia según el tipo seleccionado
  $: opcionesReferencia = (() => {
    if (aplicaA === 'Producto') return productos.map(p => ({ id: p.id_producto_detalle, nombre: p.nombre }));
    if (aplicaA === 'Categoría') return categorias.map(c => ({ id: c.id_categoria, nombre: c.nombre }));
    if (aplicaA === 'Marca') return marcas.map(m => ({ id: m.id_marca, nombre: m.nombre }));
    return [];
  })();
  
  $: labelReferencia = aplicaA === 'Producto' ? 'Producto Específico' : aplicaA === 'Categoría' ? 'Categoría Específica' : 'Marca Específica';
</script>

<div class="relative flex h-auto min-h-screen w-full flex-col group/design-root bg-background-light dark:bg-background-dark font-display text-[#111418] dark:text-gray-200">
  <div class="layout-container flex h-full grow flex-col">
    <main class="flex-1 p-4 sm:p-6 lg:p-8">
      <div class="mx-auto max-w-7xl">
        <!-- Header matching gestion-descuentos -->
        <div class="flex flex-col sm:flex-row flex-wrap justify-between items-start gap-4 mb-6">
          <div class="flex items-center gap-4">
            <h1 class="text-3xl lg:text-4xl font-black tracking-tight text-[#111418] dark:text-white">Gestión de Descuentos</h1>
            <div class="flex h-8 shrink-0 items-center justify-center gap-x-2 rounded-full bg-blue-100 dark:bg-blue-900/20 px-4">
              <span class="material-symbols-outlined text-blue-600 dark:text-blue-400 !text-base">edit</span>
              <p class="text-blue-600 dark:text-blue-400 text-sm font-medium">Editar Descuento</p>
            </div>
          </div>
        </div>

        <!-- Loading State -->
        {#if loadingData}
          <div class="mb-4 p-8 text-center bg-white dark:bg-background-dark rounded-xl shadow-sm border border-gray-200 dark:border-slate-800">
            <p class="text-gray-600 dark:text-gray-400">Cargando datos del descuento...</p>
          </div>
        {:else}
        
        <!-- Success/Error Messages -->
        {#if successMessage}
          <div class="mb-4 p-4 bg-green-100 dark:bg-green-900/50 border border-green-400 dark:border-green-700 text-green-800 dark:text-green-200 rounded-lg">
            {successMessage}
          </div>
        {/if}
        
        {#if errorMessage}
          <div class="mb-4 p-4 bg-red-100 dark:bg-red-900/50 border border-red-400 dark:border-red-700 text-red-800 dark:text-red-200 rounded-lg">
            {errorMessage}
          </div>
        {/if}

        <!-- Form Content -->
        <div class="px-4 sm:px-10 md:px-20 lg:px-40 flex flex-1 justify-center py-5">
          <div class="layout-content-container flex flex-col w-full max-w-[960px] flex-1 bg-white dark:bg-background-dark rounded-xl shadow-sm border border-gray-200 dark:border-slate-800">
            <!-- Page Heading inside form card -->
            <div class="flex flex-wrap justify-between gap-3 p-4 border-b border-gray-200 dark:border-slate-800">
              <div class="flex min-w-72 flex-col gap-3">
                <p class="text-[#111418] dark:text-white tracking-light text-[32px] font-bold leading-tight">Editar Descuento</p>
                <p class="text-[#617589] dark:text-slate-400 text-sm font-normal leading-normal">Modifica la información del descuento seleccionado.</p>
              </div>
            </div>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-2 p-4">
              <!-- Nombre del descuento -->
              <label class="flex flex-col min-w-40 flex-1 py-3 md:col-span-2">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Nombre *</p>
                <input bind:value={nombre} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 h-14 placeholder:text-[#617589] dark:placeholder:text-slate-500 p-[15px] text-base font-normal leading-normal" placeholder="Ej: Descuento de Verano"/>
              </label>
              <!-- Descripción -->
              <label class="flex flex-col min-w-40 flex-1 py-3 md:col-span-2">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Descripción</p>
                <textarea bind:value={descripcion} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 min-h-36 placeholder:text-[#617589] dark:placeholder:text-slate-500 p-[15px] text-base font-normal leading-normal" placeholder="Describe brevemente en qué consiste el descuento"></textarea>
              </label>
            </div>

            <!-- Section Header -->
            <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4">Detalles del Descuento</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 p-4">
              <!-- Tipo -->
              <div class="flex flex-col py-3">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Tipo</p>
                <div class="flex h-14 items-center justify-center rounded-lg bg-[#f0f2f4] dark:bg-slate-800 p-1">
                  <label class="flex cursor-pointer h-full grow items-center justify-center overflow-hidden rounded-lg px-2 has-[:checked]:bg-white has-[:checked]:dark:bg-slate-700 has-[:checked]:shadow-[0_0_4px_rgba(0,0,0,0.1)] has-[:checked]:text-[#111418] dark:has-[:checked]:text-white text-[#617589] dark:text-slate-400 text-sm font-medium leading-normal transition-all">
                    <span class="truncate">Porcentaje</span>
                    <input bind:group={tipoDescuento} checked class="invisible w-0" type="radio" value="Porcentaje"/>
                  </label>
                  <label class="flex cursor-pointer h-full grow items-center justify-center overflow-hidden rounded-lg px-2 has-[:checked]:bg-white has-[:checked]:dark:bg-slate-700 has-[:checked]:shadow-[0_0_4px_rgba(0,0,0,0.1)] has-[:checked]:text-[#111418] dark:has-[:checked]:text-white text-[#617589] dark:text-slate-400 text-sm font-medium leading-normal transition-all">
                    <span class="truncate">Monto Fijo</span>
                    <input bind:group={tipoDescuento} class="invisible w-0" type="radio" value="Monto Fijo"/>
                  </label>
                </div>
              </div>
              <!-- Valor -->
              <label class="flex flex-col min-w-40 flex-1 py-3">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Valor *</p>
                <div class="relative flex items-center">
                  <input bind:value={valor} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 h-14 placeholder:text-[#617589] dark:placeholder:text-slate-500 px-4 pl-10 text-base font-normal leading-normal" placeholder="0" type="number" step="0.01"/>
                  <span class="material-symbols-outlined absolute left-3 text-slate-400">{valueIcon}</span>
                </div>
              </label>
            </div>

            <!-- Section Header -->
            <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4">Condiciones de Aplicación</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 p-4">
              <!-- Aplica a -->
              <label class="flex flex-col min-w-40 flex-1 py-3">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Aplica a</p>
                <div class="relative">
                  <select bind:value={aplicaA} class="form-input appearance-none flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 h-14 placeholder:text-[#617589] dark:placeholder:text-slate-500 p-[15px] text-base font-normal leading-normal pr-10">
                    <option>Global</option>
                    <option>Producto</option>
                    <option>Categoría</option>
                    <option>Marca</option>
                    <option>Familia</option>
                  </select>
                  <span class="material-symbols-outlined absolute right-3 top-1/2 -translate-y-1/2 text-slate-400 pointer-events-none">expand_more</span>
                </div>
              </label>
              <!-- Referencia Específica (Producto/Categoría/Marca) -->
              {#if showReferenciaSelect}
                <label class="flex flex-col min-w-40 flex-1 py-3">
                  <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">{labelReferencia} *</p>
                  <div class="relative">
                    <select bind:value={idReferencia} class="form-input appearance-none flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 h-14 placeholder:text-[#617589] dark:placeholder:text-slate-500 p-[15px] text-base font-normal leading-normal pr-10">
                      <option value={null}>Seleccionar {aplicaA.toLowerCase()}...</option>
                      {#each opcionesReferencia as opcion (opcion.id)}
                        <option value={opcion.id}>{opcion.nombre}</option>
                      {/each}
                    </select>
                    <span class="material-symbols-outlined absolute right-3 top-1/2 -translate-y-1/2 text-slate-400 pointer-events-none">expand_more</span>
                  </div>
                </label>
              {/if}
              <!-- Compra mínima -->
              <label class="flex flex-col min-w-40 flex-1 py-3">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Compra mínima (opcional)</p>
                <div class="relative flex items-center">
                  <input bind:value={compraMinima} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 h-14 placeholder:text-[#617589] dark:placeholder:text-slate-500 px-4 pl-10 text-base font-normal leading-normal" placeholder="0.00" type="number" step="0.01"/>
                  <span class="material-symbols-outlined absolute left-3 text-slate-400">attach_money</span>
                </div>
              </label>
              <!-- Cantidad mínima -->
              <label class="flex flex-col min-w-40 flex-1 py-3">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Cantidad mínima (opcional)</p>
                <input bind:value={cantidadMinima} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 h-14 placeholder:text-[#617589] dark:placeholder:text-slate-500 p-[15px] text-base font-normal leading-normal" placeholder="1" type="number"/>
              </label>
              <!-- Usos máximos -->
              <label class="flex flex-col min-w-40 flex-1 py-3 md:col-span-2">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Usos máximos (opcional)</p>
                <input bind:value={usosMaximos} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 h-14 placeholder:text-[#617589] dark:placeholder:text-slate-500 p-[15px] text-base font-normal leading-normal" placeholder="Dejar vacío para ilimitado" type="number"/>
              </label>
            </div>

            <!-- Section Header -->
            <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4">Vigencia y Estado</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 p-4">
              <!-- Fecha inicio -->
              <label class="flex flex-col min-w-40 flex-1 py-3">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Fecha de inicio *</p>
                <div class="relative">
                  <input bind:value={fechaInicio} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 h-14 placeholder:text-[#617589] dark:placeholder:text-slate-500 pl-12 pr-4 text-base font-normal leading-normal" type="datetime-local"/>
                  <span class="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-slate-400 pointer-events-none">calendar_today</span>
                </div>
              </label>
              <!-- Fecha fin -->
              <label class="flex flex-col min-w-40 flex-1 py-3">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Fecha de fin *</p>
                <div class="relative">
                  <input bind:value={fechaFin} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 h-14 placeholder:text-[#617589] dark:placeholder:text-slate-500 pl-12 pr-4 text-base font-normal leading-normal" type="datetime-local"/>
                  <span class="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-slate-400 pointer-events-none">event_busy</span>
                </div>
              </label>
              <!-- Estado activo -->
              <div class="flex items-center justify-between min-w-40 flex-1 py-3 md:col-span-2">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal">Estado activo</p>
                <label class="relative inline-flex items-center cursor-pointer">
                  <input bind:checked={activo} class="sr-only peer" type="checkbox"/>
                  <div class="w-11 h-6 bg-slate-200 dark:bg-slate-700 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-slate-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary"></div>
                </label>
              </div>
            </div>

            <!-- Action Buttons -->
            <div class="flex flex-wrap justify-between gap-4 p-4 mt-6 border-t border-gray-200 dark:border-slate-800">
              <div class="flex items-center gap-2">
                {#if hayCambios}
                  <button 
                    on:click={revertirCambios} 
                    class="flex items-center justify-center gap-2 h-12 px-6 rounded-lg bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-300 text-base font-medium hover:bg-amber-200 dark:hover:bg-amber-900/50 transition-colors"
                  >
                    <span class="material-symbols-outlined text-xl">undo</span>
                    Revertir Cambios
                  </button>
                  <span class="text-sm text-amber-600 dark:text-amber-400">
                    ({calcularCambios().length} cambio{calcularCambios().length !== 1 ? 's' : ''} pendiente{calcularCambios().length !== 1 ? 's' : ''})
                  </span>
                {/if}
              </div>
              <div class="flex gap-4">
                <a href="/gestion-descuentos" class="flex items-center justify-center h-12 px-6 rounded-lg bg-slate-100 dark:bg-slate-700 text-[#111418] dark:text-white text-base font-medium hover:bg-slate-200 dark:hover:bg-slate-600 transition-colors">Cancelar</a>
                <button 
                  on:click={iniciarGuardado} 
                  disabled={loading || !hayCambios} 
                  class="flex items-center justify-center h-12 px-6 rounded-lg bg-primary text-white text-base font-medium hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {loading ? 'Actualizando...' : 'Actualizar Descuento'}
                </button>
              </div>
            </div>
          </div>
        </div>
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
          Se realizarán los siguientes cambios al descuento "<strong>{datosOriginales?.nombre}</strong>":
        </p>
        
        <div class="space-y-3">
          {#each cambiosPendientes as cambio (cambio.campo)}
            <div class="bg-gray-50 dark:bg-gray-800/50 rounded-lg p-3 border border-gray-200 dark:border-gray-700">
              <p class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider mb-2">{cambio.campo}</p>
              <div class="flex items-center gap-2 text-sm">
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
