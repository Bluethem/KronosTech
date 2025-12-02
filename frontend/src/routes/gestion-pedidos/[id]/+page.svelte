<script lang="ts">
  import type { PageData } from './$types';
  
  export let data: PageData;
  let activeTab = 'general';

  $: order = data.order?.venta;
  $: products = data.order?.productos || [];

  function formatDate(dateString: string | null): string {
    if (!dateString) return 'N/A';
    const date = new Date(dateString);
    return date.toLocaleString('es-ES', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  }

  function formatCurrency(amount: number | null): string {
    if (amount === null) return 'S/ 0.00';
    return `S/ ${Number(amount).toFixed(2)}`;
  }

  function getStatusBadgeClass(status: string | null): string {
    if (!status) return 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
    
    const normalizedStatus = status.toLowerCase();
    const statusMap: Record<string, string> = {
      'pendiente': 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/40 dark:text-yellow-300 border border-yellow-200 dark:border-yellow-800',
      'confirmado': 'bg-blue-100 text-blue-800 dark:bg-blue-900/40 dark:text-blue-300 border border-blue-200 dark:border-blue-800',
      'procesando': 'bg-purple-100 text-purple-800 dark:bg-purple-900/40 dark:text-purple-300 border border-purple-200 dark:border-purple-800',
      'en proceso': 'bg-purple-100 text-purple-800 dark:bg-purple-900/40 dark:text-purple-300 border border-purple-200 dark:border-purple-800',
      'enviado': 'bg-indigo-100 text-indigo-800 dark:bg-indigo-900/40 dark:text-indigo-300 border border-indigo-200 dark:border-indigo-800',
      'entregado': 'bg-emerald-100 text-emerald-800 dark:bg-emerald-900/40 dark:text-emerald-300 border border-emerald-200 dark:border-emerald-800',
      'cancelado': 'bg-red-100 text-red-800 dark:bg-red-900/40 dark:text-red-300 border border-red-200 dark:border-red-800',
      'devuelto': 'bg-orange-100 text-orange-800 dark:bg-orange-900/40 dark:text-orange-300 border border-orange-200 dark:border-orange-800'
    };
    
    return statusMap[normalizedStatus] || 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
  }

  function getPaymentStatusBadgeClass(status: string | null): string {
    if (!status) return 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
    
    const normalizedStatus = status.toLowerCase();
    const statusMap: Record<string, string> = {
      'pendiente': 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/40 dark:text-yellow-300 border border-yellow-200 dark:border-yellow-800',
      'procesando': 'bg-blue-100 text-blue-800 dark:bg-blue-900/40 dark:text-blue-300 border border-blue-200 dark:border-blue-800',
      'completado': 'bg-green-100 text-green-800 dark:bg-green-900/40 dark:text-green-300 border border-green-200 dark:border-green-800',
      'pagado': 'bg-green-100 text-green-800 dark:bg-green-900/40 dark:text-green-300 border border-green-200 dark:border-green-800',
      'fallido': 'bg-red-100 text-red-800 dark:bg-red-900/40 dark:text-red-300 border border-red-200 dark:border-red-800',
      'rechazado': 'bg-red-100 text-red-800 dark:bg-red-900/40 dark:text-red-300 border border-red-200 dark:border-red-800',
      'cancelado': 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300 border border-gray-200 dark:border-gray-600',
      'reembolsado': 'bg-pink-100 text-pink-800 dark:bg-pink-900/40 dark:text-pink-300 border border-pink-200 dark:border-pink-800',
      'parcialmente_reembolsado': 'bg-orange-100 text-orange-800 dark:bg-orange-900/40 dark:text-orange-300 border border-orange-200 dark:border-orange-800'
    };
    
    return statusMap[normalizedStatus] || 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
  }
</script>

{#if !order}
  <div class="flex items-center justify-center min-h-screen">
    <div class="text-center">
      <h2 class="text-2xl font-bold text-gray-800 dark:text-white mb-2">Pedido no encontrado</h2>
      <p class="text-gray-600 dark:text-gray-400">No se pudo cargar la información del pedido.</p>
      <a href="/gestion-pedidos" class="mt-4 inline-block px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors">Volver a la lista</a>
    </div>
  </div>
{:else}

<div class="relative flex h-auto min-h-screen w-full flex-col group/design-root overflow-x-hidden bg-background-light dark:bg-background-dark font-display text-[#111418] dark:text-gray-200">
<div class="layout-container flex h-full grow flex-col">
<div class="flex flex-1 justify-center py-5 sm:py-8 lg:py-12">
<div class="layout-content-container flex flex-col w-full max-w-6xl px-4 sm:px-6 lg:px-8">
<!-- Page Heading -->
<div class="flex flex-wrap justify-between items-center gap-4 pb-4">
<div class="flex flex-col gap-1">
<p class="text-4xl font-black leading-tight tracking-[-0.033em] dark:text-white">Detalles del Pedido</p>
<p class="text-[#617589] dark:text-gray-400 text-base font-normal leading-normal">
                Gestiona todos los aspectos del pedido, desde la información del cliente hasta el historial de cambios.
              </p>
</div>
</div>
<!-- Top Card and Actions -->
<div class="w-full bg-white dark:bg-background-dark dark:border dark:border-gray-700/50 rounded-xl shadow-sm p-4 sm:p-6 mb-6">
<div class="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-4">
<div class="flex flex-col gap-3">
<p class="text-2xl font-bold leading-tight tracking-[-0.015em] dark:text-white">Pedido #{order.numero_pedido}</p>
<div class="flex items-center gap-3 flex-wrap">
<div class="flex h-7 items-center justify-center gap-x-2 rounded-full px-3 {getStatusBadgeClass(order.estado)}">
<p class="text-sm font-medium leading-normal">Estado Pedido: {order.estado || 'N/A'}</p>
</div>
<div class="flex h-7 items-center justify-center gap-x-2 rounded-full px-3 {getPaymentStatusBadgeClass(order.estado_pago)}">
<p class="text-sm font-medium leading-normal">Estado Pago: {order.estado_pago || 'N/A'}</p>
</div>
</div>
<p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Fecha de pedido: {formatDate(order.fecha_pedido)}. Última actualización: {formatDate(order.fecha_actualizacion)}</p>
</div>
<div class="flex gap-3 flex-wrap justify-start sm:justify-end shrink-0">
<button class="flex min-w-[84px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em] hover:bg-primary/90 transition-colors">
<span class="truncate">Cambiar Estado</span>
</button>
<button class="flex min-w-[84px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-red-600/10 text-red-600 dark:bg-red-500/20 dark:text-red-400 text-sm font-bold leading-normal tracking-[0.015em] hover:bg-red-600/20 transition-colors">
<span class="truncate">Cancelar Pedido</span>
</button>
</div>
</div>
</div>
<!-- Tabs Navigation -->
<div class="w-full">
<div class="flex border-b border-[#dbe0e6] dark:border-gray-700 gap-2 sm:gap-8">
<button 
  class="flex flex-col shrink-0 items-center justify-center border-b-[3px] {activeTab === 'general' ? 'border-b-primary text-primary' : 'border-b-transparent text-[#617589] dark:text-gray-400 hover:text-primary dark:hover:text-primary'} pb-3 pt-2 px-2 sm:px-4 transition-colors" 
  on:click={() => activeTab = 'general'}
>
<p class="text-sm font-bold leading-normal tracking-[0.015em]">Información General</p>
</button>
<button 
  class="flex flex-col shrink-0 items-center justify-center border-b-[3px] {activeTab === 'productos' ? 'border-b-primary text-primary' : 'border-b-transparent text-[#617589] dark:text-gray-400 hover:text-primary dark:hover:text-primary'} pb-3 pt-2 px-2 sm:px-4 transition-colors" 
  on:click={() => activeTab = 'productos'}
>
<p class="text-sm font-bold leading-normal tracking-[0.015em]">Productos</p>
</button>
<button 
  class="flex flex-col shrink-0 items-center justify-center border-b-[3px] {activeTab === 'pagos' ? 'border-b-primary text-primary' : 'border-b-transparent text-[#617589] dark:text-gray-400 hover:text-primary dark:hover:text-primary'} pb-3 pt-2 px-2 sm:px-4 transition-colors" 
  on:click={() => activeTab = 'pagos'}
>
<p class="text-sm font-bold leading-normal tracking-[0.015em]">Pagos</p>
</button>
<button 
  class="flex flex-col shrink-0 items-center justify-center border-b-[3px] {activeTab === 'historial' ? 'border-b-primary text-primary' : 'border-b-transparent text-[#617589] dark:text-gray-400 hover:text-primary dark:hover:text-primary'} pb-3 pt-2 px-2 sm:px-4 transition-colors" 
  on:click={() => activeTab = 'historial'}
>
<p class="text-sm font-bold leading-normal tracking-[0.015em]">Historial y Notas</p>
</button>
</div>
</div>

{#if activeTab === 'general'}
<!-- Tab Content: Información General -->
<div class="grid grid-cols-1 lg:grid-cols-3 gap-6 pt-6">
<!-- Left Column -->
<div class="lg:col-span-2 flex flex-col gap-6">
<!-- Datos del Cliente -->
<div class="bg-white dark:bg-background-dark dark:border dark:border-gray-700/50 rounded-xl shadow-sm">
<div class="p-5 border-b border-gray-200 dark:border-gray-700">
<h3 class="text-lg font-bold tracking-tight dark:text-white">Datos del Cliente</h3>
</div>
<div class="p-5 grid grid-cols-1 sm:grid-cols-2 gap-y-4 gap-x-6">
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Nombre completo</p>
<p class="font-medium dark:text-white">{order.nombre_usuario || 'N/A'}</p>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">DNI</p>
<p class="font-medium dark:text-white">{order.dni_usuario || 'N/A'}</p>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Email</p>
<div class="flex items-center gap-2">
<p class="font-medium dark:text-white">{order.email_usuario || 'N/A'}</p>
<span class="material-symbols-outlined text-gray-400 cursor-pointer text-base hover:text-primary">content_copy</span>
</div>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Teléfono</p>
<p class="font-medium dark:text-white">{order.telefono_usuario || 'N/A'}</p>
</div>
</div>
</div>
<!-- Dirección de Envío -->
<div class="bg-white dark:bg-background-dark dark:border dark:border-gray-700/50 rounded-xl shadow-sm">
<div class="p-5 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
<h3 class="text-lg font-bold tracking-tight dark:text-white">Dirección de Envío</h3>
<button class="flex items-center gap-1 text-sm text-primary font-bold">
<span class="material-symbols-outlined text-lg">map</span>
<span>Ver en mapa</span>
</button>
</div>
<div class="p-5 grid grid-cols-1 sm:grid-cols-2 gap-y-4 gap-x-6">
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Dirección</p>
<p class="font-medium dark:text-white">{order.direccion_envio || 'N/A'}</p>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Ciudad / Departamento</p>
<p class="font-medium dark:text-white">{order.ciudad || 'N/A'}, {order.departamento || 'N/A'}</p>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Código Postal</p>
<p class="font-medium dark:text-white">{order.codigo_postal || 'N/A'}</p>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Teléfono de Contacto</p>
<p class="font-medium dark:text-white">{order.telefono_contacto || 'N/A'}</p>
</div>
<div class="sm:col-span-2">
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Referencia</p>
<p class="font-medium dark:text-white">N/A</p>
</div>
</div>
</div>
</div>
<!-- Right Column -->
<div class="lg:col-span-1 flex flex-col gap-6">
<!-- Información de Envío -->
<div class="bg-white dark:bg-background-dark dark:border dark:border-gray-700/50 rounded-xl shadow-sm">
<div class="p-5 border-b border-gray-200 dark:border-gray-700">
<h3 class="text-lg font-bold tracking-tight dark:text-white">Información de Envío</h3>
</div>
<div class="p-5 flex flex-col gap-4">
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Método de envío</p>
<p class="font-medium dark:text-white">{order.metodo_envio || 'N/A'}</p>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Estado de envío</p>
<div class="flex h-6 w-fit items-center justify-center gap-x-2 rounded-full px-3 {getStatusBadgeClass(order.estado)}">
<p class="text-xs font-medium leading-normal">{order.estado || 'N/A'}</p>
</div>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Empresa de envío</p>
<select class="w-full rounded-md border-gray-300 dark:border-gray-600 dark:bg-gray-700 shadow-sm focus:border-primary focus:ring-primary text-sm dark:text-white">
<option>Seleccionar empresa...</option>
<option>Correos</option>
<option>SEUR</option>
<option>MRW</option>
</select>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Número de tracking</p>
<div class="flex gap-2">
<input class="flex-grow w-full rounded-md border-gray-300 dark:border-gray-600 dark:bg-gray-700 shadow-sm focus:border-primary focus:ring-primary text-sm dark:text-white" placeholder="Añadir tracking" type="text" value="{order.numero_tracking || ''}"/>
<button class="flex-shrink-0 cursor-pointer items-center justify-center rounded-lg h-10 px-4 bg-[#f0f2f4] dark:bg-gray-700 text-[#111418] dark:text-white text-sm font-bold leading-normal tracking-[0.015em] hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors">Guardar</button>
</div>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Fecha estimada de entrega</p>
<p class="font-medium dark:text-white">{order.fecha_entrega_estimada || 'N/A'}</p>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Costo de envío</p>
<p class="font-medium dark:text-white">{formatCurrency(order.costo_envio)}</p>
</div>
</div>
</div>
</div>
</div>
{:else if activeTab === 'productos'}
<div class="pt-6">
<div class="bg-white dark:bg-background-dark dark:border dark:border-gray-700/50 rounded-xl shadow-sm overflow-hidden">
<div class="overflow-x-auto">
<table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
<thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
<tr>
<th class="px-6 py-3" scope="col">Imagen miniatura</th>
<th class="px-6 py-3" scope="col">Producto</th>
<th class="px-6 py-3 text-center" scope="col">Cantidad</th>
<th class="px-6 py-3 text-right" scope="col">Precio Unitario</th>
<th class="px-6 py-3 text-right" scope="col">Descuento</th>
<th class="px-6 py-3 text-right" scope="col">Subtotal</th>
</tr>
</thead>
<tbody>
{#each products as product}
<tr class="bg-white border-b dark:bg-background-dark dark:border-gray-700">
<td class="px-6 py-4">
<img alt={product.nombre_producto} class="w-10 h-10 rounded object-cover" src={product.imagen_principal || 'https://via.placeholder.com/40'}/>
</td>
<th class="px-6 py-4 font-medium text-gray-900 dark:text-white whitespace-nowrap" scope="row">
<div>{product.nombre_producto}</div>
<div class="text-xs text-gray-500 dark:text-gray-400">SKU: {product.sku}</div>
</th>
<td class="px-6 py-4 text-center">{product.cantidad}</td>
<td class="px-6 py-4 text-right">{formatCurrency(product.precio_unitario)}</td>
<td class="px-6 py-4 text-right">{formatCurrency(0)}</td> <!-- Descuento unitario no disponible en struct actual -->
<td class="px-6 py-4 text-right">{formatCurrency(product.subtotal)}</td>
</tr>
{/each}
{#if products.length === 0}
<tr>
<td colspan="6" class="px-6 py-4 text-center text-gray-500 dark:text-gray-400">No hay productos en este pedido.</td>
</tr>
{/if}
</tbody>
</table>
</div>
<div class="p-6 bg-gray-50 dark:bg-gray-900/50 flex justify-end">
<div class="w-full max-w-sm flex flex-col gap-3">
<div class="flex justify-between text-sm">
<span class="text-gray-600 dark:text-gray-400">Subtotal</span>
<span class="font-medium text-gray-800 dark:text-gray-200">{formatCurrency(order.subtotal)}</span>
</div>
<div class="flex justify-between text-sm">
<span class="text-gray-600 dark:text-gray-400">Descuento Total</span>
<span class="font-medium text-gray-800 dark:text-gray-200">-{formatCurrency(order.descuento_total)}</span>
</div>
<div class="flex justify-between text-sm">
<span class="text-gray-600 dark:text-gray-400">Costo de Envío</span>
<span class="font-medium text-gray-800 dark:text-gray-200">{formatCurrency(order.costo_envio)}</span>
</div>
<div class="border-t border-gray-200 dark:border-gray-700 my-2"></div>
<div class="flex justify-between items-center">
<span class="text-lg font-bold text-gray-900 dark:text-white">Total</span>
<span class="text-2xl font-bold text-gray-900 dark:text-white">{formatCurrency(order.total)}</span>
</div>
</div>
</div>
</div>
</div>
{:else if activeTab === 'pagos'}
<div class="flex flex-col gap-6 pt-6">
<div class="flex justify-end">
<button class="flex min-w-[84px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em] hover:bg-primary/90 transition-colors">
<span class="material-symbols-outlined mr-2 text-base">credit_card</span>
<span class="truncate">Procesar Reembolso</span>
</button>
</div>
<div class="bg-white dark:bg-background-dark dark:border dark:border-gray-700/50 rounded-xl shadow-sm p-5">
<div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-4 items-center">
<div class="lg:col-span-2">
<p class="text-sm text-gray-500 dark:text-gray-400">Número de pedido</p>
<p class="font-mono text-xs text-gray-800 dark:text-gray-200">{order.numero_pedido}</p>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Método de pago</p>
<div class="flex items-center gap-2">
<span class="material-symbols-outlined text-gray-600 dark:text-gray-300">payments</span>
<span class="font-medium text-gray-800 dark:text-white">N/A</span>
</div>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Estado</p>
<div class="flex h-6 w-fit items-center justify-center gap-x-2 rounded-full px-3 {getPaymentStatusBadgeClass(order.estado_pago)}">
<p class="text-xs font-medium leading-normal">{order.estado_pago || 'N/A'}</p>
</div>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400">Monto</p>
<p class="font-medium text-gray-800 dark:text-white">{formatCurrency(order.total)}</p>
</div>
<div class="flex flex-col items-start lg:items-end">
<h3 class="text-lg font-bold tracking-tight dark:text-white">Notas Internas</h3>
</div>
<div class="p-5 flex flex-col gap-4">
<textarea class="block w-full rounded-md border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white shadow-sm focus:border-primary focus:ring-primary sm:text-sm" placeholder="Agregar nota interna..." rows="4"></textarea>
<div class="flex justify-end">
<button class="flex min-w-[84px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em] hover:bg-primary/90 transition-colors">Guardar Nota</button>
</div>
</div>
<div class="p-5 border-t border-gray-200 dark:border-gray-700 flex flex-col gap-4">
<div class="flex justify-between items-start gap-4">
<div>
<p class="text-gray-800 dark:text-gray-200">Cliente llamó para confirmar la dirección de entrega. Todo correcto.</p>
<p class="text-xs text-gray-500 dark:text-gray-400 mt-1">Por: Juan Pérez - 15 Ago, 2023 11:45</p>
</div>
<button class="text-gray-400 hover:text-red-500 dark:hover:text-red-400 transition-colors">
<span class="material-symbols-outlined text-lg">close</span>
</button>
</div>
<div class="flex justify-between items-start gap-4">
<div>
<p class="text-gray-800 dark:text-gray-200">Se aplicó un descuento manual del 10% por ser cliente recurrente.</p>
<p class="text-xs text-gray-500 dark:text-gray-400 mt-1">Por: María López - 15 Ago, 2023 10:30</p>
</div>
<button class="text-gray-400 hover:text-red-500 dark:hover:text-red-400 transition-colors">
<span class="material-symbols-outlined text-lg">close</span>
</button>
</div>
</div>
</div>
<div class="bg-white dark:bg-background-dark dark:border dark:border-gray-700/50 rounded-xl shadow-sm">
<div class="p-5 border-b border-gray-200 dark:border-gray-700">
<h3 class="text-lg font-bold tracking-tight dark:text-white">Notas del Cliente</h3>
</div>
<div class="p-5">
<p class="text-gray-600 dark:text-gray-300 italic">"Por favor, entregar en horario de tarde si es posible. El timbre del piso 4A a veces no funciona, llamar al móvil si no contesto. Gracias."</p>
</div>
</div>
</div>
<div class="lg:col-span-1">
<div class="bg-white dark:bg-background-dark dark:border dark:border-gray-700/50 rounded-xl shadow-sm">
<div class="p-5 border-b border-gray-200 dark:border-gray-700">
<h3 class="text-lg font-bold tracking-tight dark:text-white">Historial de Estados</h3>
</div>
<div class="p-5">
<ol class="relative border-l border-gray-200 dark:border-gray-700">
<li class="mb-8 ml-6">
<span class="absolute flex items-center justify-center w-6 h-6 bg-blue-100 rounded-full -left-3 ring-8 ring-white dark:ring-gray-900 dark:bg-blue-900">
<span class="material-symbols-outlined text-sm text-blue-800 dark:text-blue-300">shopping_cart</span>
</span>
<h3 class="flex items-center mb-1 text-base font-semibold text-gray-900 dark:text-white">Pedido Creado</h3>
<time class="block mb-2 text-xs font-normal leading-none text-gray-400 dark:text-gray-500">Por: Sistema - 15 Ago, 2023 14:24</time>
<p class="text-sm font-normal text-gray-500 dark:text-gray-400">El pedido fue recibido y está pendiente de procesamiento.</p>
</li>
<li class="mb-8 ml-6">
<span class="absolute flex items-center justify-center w-6 h-6 bg-green-100 rounded-full -left-3 ring-8 ring-white dark:ring-gray-900 dark:bg-green-900">
<span class="material-symbols-outlined text-sm text-green-800 dark:text-green-300">credit_card</span>
</span>
<h3 class="mb-1 text-base font-semibold text-gray-900 dark:text-white">Pago Completado</h3>
<time class="block mb-2 text-xs font-normal leading-none text-gray-400 dark:text-gray-500">Por: Sistema - 15 Ago, 2023 14:25</time>
</li>
<li class="ml-6">
<span class="absolute flex items-center justify-center w-6 h-6 bg-blue-100 rounded-full -left-3 ring-8 ring-white dark:ring-gray-900 dark:bg-blue-900">
<span class="material-symbols-outlined text-sm text-blue-800 dark:text-blue-300">inventory_2</span>
</span>
<h3 class="mb-1 text-base font-semibold text-gray-900 dark:text-white">Estado cambiado a "Procesando"</h3>
<time class="block mb-2 text-xs font-normal leading-none text-gray-400 dark:text-gray-500">Por: María López - 15 Ago, 2023 14:30</time>
</li>
</ol>
</div>
</div>
</div>
</div>
{/if}
</div>
</div>
</div>
</div>
{/if}
