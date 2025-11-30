<script lang="ts">
  let activeTab = 'general';
</script>

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
<p class="text-2xl font-bold leading-tight tracking-[-0.015em] dark:text-white">Pedido #108-4567921-33458</p>
<div class="flex items-center gap-3 flex-wrap">
<div class="flex h-7 items-center justify-center gap-x-2 rounded-full bg-blue-100 dark:bg-blue-500/20 px-3">
<p class="text-blue-800 dark:text-blue-300 text-sm font-medium leading-normal">Estado Pedido: Procesando</p>
</div>
<div class="flex h-7 items-center justify-center gap-x-2 rounded-full bg-green-100 dark:bg-green-500/20 px-3">
<p class="text-green-800 dark:text-green-300 text-sm font-medium leading-normal">Estado Pago: Pagado</p>
</div>
</div>
<p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Fecha de pedido: 15 Ago, 2023. Última actualización: 15 Ago, 2023 14:30</p>
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
<p class="font-medium dark:text-white">Ana García Pérez</p>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">DNI</p>
<p class="font-medium dark:text-white">12345678A</p>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Email</p>
<div class="flex items-center gap-2">
<p class="font-medium dark:text-white">ana.garcia@email.com</p>
<span class="material-symbols-outlined text-gray-400 cursor-pointer text-base hover:text-primary">content_copy</span>
</div>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Teléfono</p>
<div class="flex items-center gap-2">
<p class="font-medium dark:text-white">+34 600 123 456</p>
<span class="material-symbols-outlined text-gray-400 cursor-pointer text-base hover:text-primary">content_copy</span>
</div>
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
<p class="font-medium dark:text-white">Calle Falsa 123, Piso 4, Puerta A</p>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Ciudad / Departamento</p>
<p class="font-medium dark:text-white">Madrid, Comunidad de Madrid</p>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Código Postal</p>
<p class="font-medium dark:text-white">28001</p>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Teléfono de Contacto</p>
<p class="font-medium dark:text-white">+34 600 123 456</p>
</div>
<div class="sm:col-span-2">
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Referencia</p>
<p class="font-medium dark:text-white">Edificio rojo con una panadería en la esquina.</p>
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
<p class="font-medium dark:text-white">Envío Estándar</p>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Estado de envío</p>
<div class="flex h-6 w-fit items-center justify-center gap-x-2 rounded-full bg-yellow-100 dark:bg-yellow-500/20 px-3">
<p class="text-yellow-800 dark:text-yellow-300 text-xs font-medium leading-normal">Pendiente de Envío</p>
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
<input class="flex-grow w-full rounded-md border-gray-300 dark:border-gray-600 dark:bg-gray-700 shadow-sm focus:border-primary focus:ring-primary text-sm dark:text-white" placeholder="Añadir tracking" type="text"/>
<button class="flex-shrink-0 cursor-pointer items-center justify-center rounded-lg h-10 px-4 bg-[#f0f2f4] dark:bg-gray-700 text-[#111418] dark:text-white text-sm font-bold leading-normal tracking-[0.015em] hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors">Guardar</button>
</div>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Fecha estimada de entrega</p>
<p class="font-medium dark:text-white">20 Ago, 2023</p>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Costo de envío</p>
<p class="font-medium dark:text-white">4,99 €</p>
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
<tr class="bg-white border-b dark:bg-background-dark dark:border-gray-700">
<td class="px-6 py-4">
<img alt="Producto 1" class="w-10 h-10 rounded object-cover" src="https://lh3.googleusercontent.com/aida-public/AB6AXuDekzd0O2BLWKf_jelvxqJjC48j1Qr3abxoOqMCMJzuuPdRbj2spMp_c3mornsQBVEDjTrtBjDBCE2Urchj0YZM8sV-XPJj4wdzBEdHthYcF2XavO5xqTOGg3CuTVHqQ7MBgEGo-7xzylZe9rqre3hH879gJnPwahdgS7BBrmVhdloNTeHmjqT6g5gTta0knXTvEG2L-CVAgZkISVxVs0NEl_IzcRPuCn43EPNcSgV9D4Yi6mhUV3mvcRwTe8d6E2ewshMWfu8X8Dg"/>
</td>
<th class="px-6 py-4 font-medium text-gray-900 dark:text-white whitespace-nowrap" scope="row">
<div>Zapatillas Deportivas UltraBoost</div>
<div class="text-xs text-gray-500 dark:text-gray-400">SKU: ZD-UB-42-R</div>
</th>
<td class="px-6 py-4 text-center">1</td>
<td class="px-6 py-4 text-right">180,00 €</td>
<td class="px-6 py-4 text-right">-18,00 €</td>
<td class="px-6 py-4 text-right">162,00 €</td>
</tr>
<tr class="bg-white dark:bg-background-dark">
<td class="px-6 py-4">
<img alt="Producto 2" class="w-10 h-10 rounded object-cover" src="https://lh3.googleusercontent.com/aida-public/AB6AXuAGGTWBLUBDc4Y109Xtv1DJatp-bh3coYiEpQg-pPmbFb7-Nbkd-l2BUEq8ktho55fJeQVwDLQRtrvBZ0w8OiIDbq3pQTzKb2CnTWprUSZDpodVAg8gckMHjbopB3Q5AkUqeiR8e9CfVVNayDYvt0Y_A8wF54ebsMXA-J2bvUzigK911eq5pCX6olesYMAo5iBKzJteK340C2wpwIGxkNlaWzpByk6wsft0C3gm_JDyddDIecBDrTEYfsNd-7-J2MQshh3mOlRQZkA"/>
</td>
<th class="px-6 py-4 font-medium text-gray-900 dark:text-white whitespace-nowrap" scope="row">
<div>Auriculares Inalámbricos SonidoHD</div>
<div class="text-xs text-gray-500 dark:text-gray-400">SKU: AU-SHD-BK</div>
</th>
<td class="px-6 py-4 text-center">2</td>
<td class="px-6 py-4 text-right">99,99 €</td>
<td class="px-6 py-4 text-right">0,00 €</td>
<td class="px-6 py-4 text-right">199,98 €</td>
</tr>
</tbody>
</table>
</div>
<div class="p-6 bg-gray-50 dark:bg-gray-900/50 flex justify-end">
<div class="w-full max-w-sm flex flex-col gap-3">
<div class="flex justify-between text-sm">
<span class="text-gray-600 dark:text-gray-400">Subtotal</span>
<span class="font-medium text-gray-800 dark:text-gray-200">361,98 €</span>
</div>
<div class="flex justify-between text-sm">
<span class="text-gray-600 dark:text-gray-400">Descuento Total</span>
<span class="font-medium text-gray-800 dark:text-gray-200">-18,00 €</span>
</div>
<div class="flex justify-between text-sm">
<span class="text-gray-600 dark:text-gray-400">Costo de Envío</span>
<span class="font-medium text-gray-800 dark:text-gray-200">4,99 €</span>
</div>
<div class="border-t border-gray-200 dark:border-gray-700 my-2"></div>
<div class="flex justify-between items-center">
<span class="text-lg font-bold text-gray-900 dark:text-white">Total</span>
<span class="text-2xl font-bold text-gray-900 dark:text-white">348,97 €</span>
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
<p class="text-sm text-gray-500 dark:text-gray-400">Número de transacción</p>
<p class="font-mono text-xs text-gray-800 dark:text-gray-200">ch_3NfG4p2eZvKYlo2C1g9qB8bZ</p>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Método de pago</p>
<div class="flex items-center gap-2">
<svg class="w-6 h-auto" fill="none" viewBox="0 0 42 28" xmlns="http://www.w3.org/2000/svg"><path d="M39.815 1.685a3.91 3.91 0 0 0-3.327-1.46h-28.98a3.91 3.91 0 0 0-3.327 1.46 3.913 3.913 0 0 0-1.46 3.328v17.974c0 1.25.487 2.45 1.46 3.327a3.91 3.91 0 0 0 3.328 1.46h28.98a3.91 3.91 0 0 0 3.327-1.46 3.913 3.913 0 0 0 1.46-3.328V5.013a3.913 3.913 0 0 0-1.46-3.328z" fill="#fff"></path><path d="M4.18 2.91a2.1 2.1 0 0 0-2.1 2.1v2.8h37.84V5.01a2.1 2.1 0 0 0-2.1-2.1z" fill="#404953"></path><path d="M2.08 10.37h37.84v12.614a2.1 2.1 0 0 1-2.1 2.1H4.18a2.1 2.1 0 0 1-2.1-2.1z" fill="#20262C"></path><path d="M28.01 20.31h7.82v2.1h-7.82z" fill="#404953"></path><path d="M5.13 19.34a1.05 1.05 0 1 0 0 2.1 1.05 1.05 0 0 0 0-2.1zm3.32 0a1.05 1.05 0 1 0 0 2.1 1.05 1.05 0 0 0 0-2.1zm3.33 0a1.05 1.05 0 1 0 0 2.1 1.05 1.05 0 0 0 0-2.1z" fill="#fff"></path></svg>
<span class="font-medium text-gray-800 dark:text-white">Tarjeta</span>
</div>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Estado</p>
<div class="flex h-6 w-fit items-center justify-center gap-x-2 rounded-full bg-green-100 dark:bg-green-500/20 px-3">
<p class="text-green-800 dark:text-green-300 text-xs font-medium leading-normal">Completado</p>
</div>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400">Monto</p>
<p class="font-medium text-gray-800 dark:text-white">348,97 €</p>
</div>
<div class="flex flex-col items-start lg:items-end">
<p class="text-sm text-gray-500 dark:text-gray-400">15 Ago, 2023 14:25</p>
<button class="text-primary font-bold text-sm hover:underline">Ver detalles</button>
</div>
</div>
</div>
<div class="bg-white dark:bg-background-dark dark:border dark:border-gray-700/50 rounded-xl shadow-sm p-5">
<div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-4 items-center">
<div class="lg:col-span-2">
<p class="text-sm text-gray-500 dark:text-gray-400">Número de transacción</p>
<p class="font-mono text-xs text-gray-800 dark:text-gray-200">pi_3NfG4p2eZvKYlo2C1f8aB7cD</p>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Método de pago</p>
<div class="flex items-center gap-2">
<svg class="w-6 h-auto" fill="none" viewBox="0 0 42 28" xmlns="http://www.w3.org/2000/svg"><path d="M39.815 1.685a3.91 3.91 0 0 0-3.327-1.46h-28.98a3.91 3.91 0 0 0-3.327 1.46 3.913 3.913 0 0 0-1.46 3.328v17.974c0 1.25.487 2.45 1.46 3.327a3.91 3.91 0 0 0 3.328 1.46h28.98a3.91 3.91 0 0 0 3.327-1.46 3.913 3.913 0 0 0 1.46-3.328V5.013a3.913 3.913 0 0 0-1.46-3.328z" fill="#fff"></path><path d="M4.18 2.91a2.1 2.1 0 0 0-2.1 2.1v2.8h37.84V5.01a2.1 2.1 0 0 0-2.1-2.1z" fill="#404953"></path><path d="M2.08 10.37h37.84v12.614a2.1 2.1 0 0 1-2.1 2.1H4.18a2.1 2.1 0 0 1-2.1-2.1z" fill="#20262C"></path><path d="M28.01 20.31h7.82v2.1h-7.82z" fill="#404953"></path><path d="M5.13 19.34a1.05 1.05 0 1 0 0 2.1 1.05 1.05 0 0 0 0-2.1zm3.32 0a1.05 1.05 0 1 0 0 2.1 1.05 1.05 0 0 0 0-2.1zm3.33 0a1.05 1.05 0 1 0 0 2.1 1.05 1.05 0 0 0 0-2.1z" fill="#fff"></path></svg>
<span class="font-medium text-gray-800 dark:text-white">Tarjeta</span>
</div>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Estado</p>
<div class="flex h-6 w-fit items-center justify-center gap-x-2 rounded-full bg-red-100 dark:bg-red-500/20 px-3">
<p class="text-red-800 dark:text-red-300 text-xs font-medium leading-normal">Fallido</p>
</div>
</div>
<div>
<p class="text-sm text-gray-500 dark:text-gray-400">Monto</p>
<p class="font-medium text-gray-800 dark:text-white">348,97 €</p>
</div>
<div class="flex flex-col items-start lg:items-end">
<p class="text-sm text-gray-500 dark:text-gray-400">15 Ago, 2023 14:24</p>
<button class="text-primary font-bold text-sm hover:underline">Ver detalles</button>
</div>
</div>
</div>
</div>
{:else if activeTab === 'historial'}
<div class="grid grid-cols-1 lg:grid-cols-3 gap-6 pt-6">
<div class="lg:col-span-2 flex flex-col gap-6">
<div class="bg-white dark:bg-background-dark dark:border dark:border-gray-700/50 rounded-xl shadow-sm">
<div class="p-5 border-b border-gray-200 dark:border-gray-700">
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
