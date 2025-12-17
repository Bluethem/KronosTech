<script lang="ts">
	import type { PageData } from './$types';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	
	export let data: PageData;
	
	$: orders = data.orders || [];
	
	// Variables para los filtros
	let searchQuery = data.filtros?.search || '';
	let filtroEstado = data.filtros?.estado || 'Todos';
	let filtroEstadoPago = data.filtros?.estado_pago || 'Todos';
	let fechaDesde = data.filtros?.fecha_inicio || '';
	let fechaHasta = data.filtros?.fecha_fin || '';
	
	let jsPDF: any;
	let autoTable: any;
	
	onMount(async () => {
		// Dynamically import jsPDF and autoTable
		const jsPDFModule = await import('jspdf');
		jsPDF = jsPDFModule.default;
		const autoTableModule = await import('jspdf-autotable');
		autoTable = autoTableModule.default;
	});
	
	// Función para aplicar filtros
	function aplicarFiltros() {
		const params = new URLSearchParams();
		
		if (searchQuery.trim()) params.set('search', searchQuery.trim());
		if (filtroEstado && filtroEstado !== 'Todos') params.set('estado', filtroEstado);
		if (filtroEstadoPago && filtroEstadoPago !== 'Todos') params.set('estado_pago', filtroEstadoPago);
		if (fechaDesde) params.set('fecha_inicio', fechaDesde);
		if (fechaHasta) params.set('fecha_fin', fechaHasta);
		
		const queryString = params.toString();
		goto(`/gestion-pedidos${queryString ? '?' + queryString : ''}`, { invalidateAll: true });
		currentPage = 1; // Reset a la primera página
	}
	
	// Función para limpiar filtros
	function limpiarFiltros() {
		searchQuery = '';
		filtroEstado = 'Todos';
		filtroEstadoPago = 'Todos';
		fechaDesde = '';
		fechaHasta = '';
		goto('/gestion-pedidos', { invalidateAll: true });
		currentPage = 1;
	}
	
	// Manejar búsqueda con Enter
	function handleSearchKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			aplicarFiltros();
		}
	}
	
	function formatDate(dateString: string | null): string {
		if (!dateString) return 'N/A';
		const date = new Date(dateString);
		return date.toLocaleString('es-PE', {
			timeZone: 'America/Lima',
			year: 'numeric',
			month: '2-digit',
			day: '2-digit',
			hour: '2-digit',
			minute: '2-digit'
		});
	}
	
	function formatCurrency(amount: number | null): string {
		if (amount === null) return 'S/ 0.00';
		return `S/ ${amount.toFixed(2)}`;
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
			'fallido': 'bg-red-100 text-red-800 dark:bg-red-900/40 dark:text-red-300 border border-red-200 dark:border-red-800',
			'rechazado': 'bg-red-100 text-red-800 dark:bg-red-900/40 dark:text-red-300 border border-red-200 dark:border-red-800',
			'cancelado': 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300 border border-gray-200 dark:border-gray-600',
			'reembolsado': 'bg-pink-100 text-pink-800 dark:bg-pink-900/40 dark:text-pink-300 border border-pink-200 dark:border-pink-800',
			'parcialmente_reembolsado': 'bg-orange-100 text-orange-800 dark:bg-orange-900/40 dark:text-orange-300 border border-orange-200 dark:border-orange-800'
		};
		
		return statusMap[normalizedStatus] || 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
	}
	
	function isPendingOver24h(fechaPedido: string | null, estado: string | null): boolean {
		if (!fechaPedido || estado !== 'Pendiente') return false;
		const orderDate = new Date(fechaPedido);
		const now = new Date();
		const diffHours = (now.getTime() - orderDate.getTime()) / (1000 * 60 * 60);
		return diffHours > 24;
	}
	
	function generatePDFReport() {
		if (!jsPDF || !autoTable) {
			alert('Cargando librería PDF...');
			return;
		}
		
		const doc = new jsPDF();
		const pageWidth = doc.internal.pageSize.getWidth();
		const pageHeight = doc.internal.pageSize.getHeight();
		
		// Header with gradient background
		doc.setFillColor(59, 130, 246); // Primary blue color
		doc.rect(0, 0, pageWidth, 40, 'F');
		
		// Company name
		doc.setTextColor(255, 255, 255);
		doc.setFontSize(24);
		doc.setFont('helvetica', 'bold');
		doc.text('KronosTech', 15, 20);
		
		// Report title
		doc.setFontSize(14);
		doc.setFont('helvetica', 'normal');
		doc.text('Reporte de Pedidos', 15, 30);
		
		// Date
		doc.setFontSize(10);
		const currentDate = new Date().toLocaleDateString('es-ES', {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
		doc.text(`Generado: ${currentDate}`, pageWidth - 15, 30, { align: 'right' });
		
		// Reset text color
		doc.setTextColor(0, 0, 0);
		
		// Summary statistics
		const totalOrders = orders.length;
		const totalRevenue = orders.reduce((sum, order) => sum + (order.total || 0), 0);
		const pendingOrders = orders.filter(o => o.estado?.toLowerCase() === 'pendiente').length;
		const completedOrders = orders.filter(o => o.estado?.toLowerCase() === 'entregado').length;
		
		let yPosition = 50;
		
		// Summary box
		doc.setFillColor(249, 250, 251);
		doc.roundedRect(15, yPosition, pageWidth - 30, 35, 3, 3, 'F');
		
		doc.setFontSize(11);
		doc.setFont('helvetica', 'bold');
		doc.text('Resumen General', 20, yPosition + 8);
		
		doc.setFont('helvetica', 'normal');
		doc.setFontSize(9);
		doc.text(`Total de Pedidos: ${totalOrders}`, 20, yPosition + 16);
		doc.text(`Ingresos Totales: ${formatCurrency(totalRevenue)}`, 20, yPosition + 23);
		doc.text(`Pendientes: ${pendingOrders}`, 20, yPosition + 30);
		
		doc.text(`Completados: ${completedOrders}`, 80, yPosition + 16);
		doc.text(`En Proceso: ${orders.filter(o => o.estado?.toLowerCase() === 'procesando').length}`, 80, yPosition + 23);
		doc.text(`Cancelados: ${orders.filter(o => o.estado?.toLowerCase() === 'cancelado').length}`, 80, yPosition + 30);
		
		yPosition += 45;
		
		// Table title
		doc.setFontSize(12);
		doc.setFont('helvetica', 'bold');
		doc.text('Detalle de Pedidos', 15, yPosition);
		
		yPosition += 5;
		
		// Prepare table data
		const tableData = orders.map(order => [
			order.numero_pedido || 'N/A',
			order.nombre_usuario || 'N/A',
			formatDate(order.fecha_pedido),
			order.estado || 'N/A',
			order.estado_pago || 'N/A',
			formatCurrency(order.total)
		]);
		
		// Generate table
		autoTable(doc, {
			startY: yPosition,
			head: [['Nº Pedido', 'Cliente', 'Fecha', 'Estado Pedido', 'Estado Pago', 'Total']],
			body: tableData,
			theme: 'striped',
			headStyles: {
				fillColor: [59, 130, 246],
				textColor: [255, 255, 255],
				fontStyle: 'bold',
				fontSize: 9
			},
			bodyStyles: {
				fontSize: 8,
				textColor: [50, 50, 50]
			},
			alternateRowStyles: {
				fillColor: [249, 250, 251]
			},
			columnStyles: {
				0: { cellWidth: 25 },
				1: { cellWidth: 40 },
				2: { cellWidth: 35 },
				3: { cellWidth: 25 },
				4: { cellWidth: 25 },
				5: { cellWidth: 25, halign: 'right' }
			},
			margin: { left: 15, right: 15 },
			didDrawPage: function(data) {
				// Footer
				const footerY = pageHeight - 15;
				doc.setFontSize(8);
				doc.setTextColor(128, 128, 128);
				doc.text(
					`Página ${doc.internal.getCurrentPageInfo().pageNumber}`,
					pageWidth / 2,
					footerY,
					{ align: 'center' }
				);
				doc.text('KronosTech © 2024', 15, footerY);
			}
		});
		
		// Save the PDF
		doc.save(`Reporte_Pedidos_${new Date().toISOString().split('T')[0]}.pdf`);
	}
	
	// Modal state
	let isEditModalOpen = false;
	let isShippingModalOpen = false;
	let selectedOrder: any = null;
	let newStatus = '';
	let shippingCompany = '';
	let trackingNumber = '';
	
	function openEditModal(order: any) {
		selectedOrder = order;
		newStatus = order.estado || '';
		isEditModalOpen = true;
	}
	
	function closeEditModal() {
		isEditModalOpen = false;
		selectedOrder = null;
		newStatus = '';
	}
	
	async function saveOrderStatus() {
		if (!selectedOrder || !newStatus) return;
		
		try {
			const response = await fetch(`http://localhost:3000/api/ventas/${selectedOrder.id_venta}/estado`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ estado: newStatus })
			});
			
			if (response.ok) {
				// Update local data
				selectedOrder.estado = newStatus;
				orders = orders; // Trigger reactivity
				alert('Estado actualizado exitosamente');
				closeEditModal();
			} else {
				alert('Error al actualizar el estado');
			}
		} catch (e) {
			alert('Error de conexión');
		}
	}
	
	function openShippingModal(order: any) {
		selectedOrder = order;
		trackingNumber = order.numero_tracking || '';
		shippingCompany = '';
		isShippingModalOpen = true;
	}
	
	function closeShippingModal() {
		isShippingModalOpen = false;
		selectedOrder = null;
		trackingNumber = '';
		shippingCompany = '';
	}
	
	async function saveShippingInfo() {
		if (!selectedOrder) return;
		
		try {
			const response = await fetch(`http://localhost:3000/api/ventas/${selectedOrder.id_venta}/tracking`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ 
					numero_tracking: trackingNumber,
					empresa_envio: shippingCompany 
				})
			});
			
			if (response.ok) {
				// Update local data
				selectedOrder.numero_tracking = trackingNumber;
				selectedOrder.estado = 'enviado'; // Update status to enviado
				orders = orders; // Trigger reactivity
				alert('Información de envío actualizada exitosamente. Estado cambiado a Enviado.');
				closeShippingModal();
			} else {
				alert('Error al actualizar la información de envío');
			}
		} catch (e) {
			alert('Error de conexión');
		}
	}
	
	// Pagination
	let currentPage = 1;
	let itemsPerPage = 10;
	
	$: totalPages = Math.ceil(orders.length / itemsPerPage);
	$: paginatedOrders = orders.slice((currentPage - 1) * itemsPerPage, currentPage * itemsPerPage);
	$: startIndex = (currentPage - 1) * itemsPerPage + 1;
	$: endIndex = Math.min(currentPage * itemsPerPage, orders.length);
	
	function goToPage(page: number) {
		if (page >= 1 && page <= totalPages) {
			currentPage = page;
		}
	}
	
	function changeItemsPerPage(newLimit: number) {
		itemsPerPage = newLimit;
		currentPage = 1; // Reset to first page
	}
	
	// Generate page numbers for pagination
	$: pageNumbers = (() => {
		const pages = [];
		const maxVisible = 5;
		
		if (totalPages <= maxVisible) {
			for (let i = 1; i <= totalPages; i++) {
				pages.push(i);
			}
		} else {
			if (currentPage <= 3) {
				for (let i = 1; i <= 4; i++) pages.push(i);
				pages.push(-1); // Ellipsis
				pages.push(totalPages);
			} else if (currentPage >= totalPages - 2) {
				pages.push(1);
				pages.push(-1); // Ellipsis
				for (let i = totalPages - 3; i <= totalPages; i++) pages.push(i);
			} else {
				pages.push(1);
				pages.push(-1); // Ellipsis
				for (let i = currentPage - 1; i <= currentPage + 1; i++) pages.push(i);
				pages.push(-1); // Ellipsis
				pages.push(totalPages);
			}
		}
		
		return pages;
	})();
</script>

<div class="relative flex h-auto min-h-screen w-full flex-col group/design-root bg-background-light dark:bg-background-dark font-display text-[#111418] dark:text-gray-200">
<div class="layout-container flex h-full grow flex-col">
<main class="flex-1 p-4 sm:p-6 lg:p-8">
<div class="mx-auto max-w-7xl">
<div class="flex flex-col sm:flex-row flex-wrap justify-between items-start gap-4 mb-6">
<div class="flex items-center gap-4">
<h1 class="text-3xl lg:text-4xl font-black tracking-tight text-[#111418] dark:text-white">Gestión de Pedidos</h1>
<div class="flex h-8 shrink-0 items-center justify-center gap-x-2 rounded-full bg-red-100 dark:bg-red-500/20 px-4">
<span class="material-symbols-outlined text-red-600 dark:text-red-400 !text-base">notifications</span>
<p class="text-red-600 dark:text-red-400 text-sm font-medium">Pedidos Pendientes &gt; 24h</p>
</div>
</div>
<button on:click={generatePDFReport} class="flex items-center justify-center gap-2 min-w-[84px] cursor-pointer rounded-lg h-10 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em] hover:bg-primary/90 transition-colors">
<span class="material-symbols-outlined">download</span>
<span class="truncate">Generar Reporte</span>
</button>
</div>
<div class="mb-6 rounded-xl border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900/50 p-4">
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
<div class="lg:col-span-4">
<label class="flex flex-col min-w-40 h-12 w-full">
<div class="flex w-full flex-1 items-stretch rounded-lg h-full">
<div class="text-[#617589] dark:text-gray-400 flex border-none bg-background-light dark:bg-background-dark items-center justify-center pl-4 rounded-l-lg border-r-0">
<span class="material-symbols-outlined">search</span>
</div>
<input bind:value={searchQuery} on:keydown={handleSearchKeydown} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-gray-200 focus:outline-0 focus:ring-2 focus:ring-primary/50 border-none bg-background-light dark:bg-background-dark h-full placeholder:text-[#617589] dark:placeholder:text-gray-500 px-4 rounded-l-none border-l-0 pl-2 text-sm font-normal" placeholder="Buscar por número de pedido, cliente, email..."/>
</div>
</label>
</div>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2 dark:text-gray-300">Estado del Pedido</p>
<select bind:value={filtroEstado} class="form-select flex w-full min-w-0 flex-1 overflow-hidden rounded-lg text-[#111418] dark:text-gray-200 focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 focus:border-primary/50 h-12 placeholder:text-[#617589] px-3 text-sm font-normal">
<option value="Todos">Todos</option>
<option value="pendiente">Pendiente</option>
<option value="confirmado">Confirmado</option>
<option value="procesando">En Proceso</option>
<option value="enviado">Enviado</option>
<option value="entregado">Entregado</option>
<option value="cancelado">Cancelado</option>
</select>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2 dark:text-gray-300">Estado de Pago</p>
<select bind:value={filtroEstadoPago} class="form-select flex w-full min-w-0 flex-1 overflow-hidden rounded-lg text-[#111418] dark:text-gray-200 focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 focus:border-primary/50 h-12 placeholder:text-[#617589] px-3 text-sm font-normal">
<option value="Todos">Todos</option>
<option value="pendiente">Pendiente</option>
<option value="completado">Completado</option>
<option value="fallido">Fallido</option>
<option value="reembolsado">Reembolsado</option>
</select>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2 dark:text-gray-300">Fecha desde</p>
<div class="relative">
<input bind:value={fechaDesde} type="date" class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-gray-200 focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 focus:border-primary/50 h-12 placeholder:text-[#617589] dark:placeholder:text-gray-500 p-3 text-sm font-normal"/>
</div>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2 dark:text-gray-300">Fecha hasta</p>
<div class="relative">
<input bind:value={fechaHasta} type="date" class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-gray-200 focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 focus:border-primary/50 h-12 placeholder:text-[#617589] dark:placeholder:text-gray-500 p-3 text-sm font-normal"/>
</div>
</label>
</div>
<div class="flex justify-end gap-3 mt-4 pt-4 border-t border-gray-200 dark:border-gray-800">
<button on:click={limpiarFiltros} class="flex min-w-[84px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-transparent text-gray-600 dark:text-gray-300 text-sm font-bold hover:bg-gray-100 dark:hover:bg-gray-800">
<span class="truncate">Limpiar filtros</span>
</button>
<button on:click={aplicarFiltros} class="flex min-w-[84px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em] hover:bg-primary/90 transition-colors">
<span class="truncate">Aplicar filtros</span>
</button>
</div>
</div>
<div class="overflow-x-auto rounded-xl border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900/50">
<table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
<thead class="text-xs text-gray-700 dark:text-gray-300 uppercase bg-gray-50 dark:bg-gray-800">
<tr>
<th class="px-6 py-3" scope="col">Número de Pedido</th>
<th class="px-6 py-3" scope="col">Cliente</th>
<th class="px-6 py-3" scope="col">Fecha/Hora</th>
<th class="px-6 py-3" scope="col">Estado Pedido</th>
<th class="px-6 py-3" scope="col">Estado Pago</th>
<th class="px-6 py-3 text-right" scope="col">Total</th>
<th class="px-6 py-3 text-center" scope="col">Acciones</th>
</tr>
</thead>
<tbody>
{#each paginatedOrders as order (order.id_venta)}
<tr class="{isPendingOver24h(order.fecha_pedido, order.estado) ? 'bg-red-50 dark:bg-red-500/10' : 'bg-white dark:bg-gray-900/50'} border-b dark:border-gray-800 hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors">
<th class="px-6 py-4 font-medium text-gray-900 dark:text-white whitespace-nowrap" scope="row">
<div class="flex items-center gap-2">
<span>{order.numero_pedido}</span>
{#if isPendingOver24h(order.fecha_pedido, order.estado)}
<span class="material-symbols-outlined text-red-500" style="font-size: 18px;" title="Pendiente > 24h">warning</span>
{/if}
</div>
</th>
<td class="px-6 py-4">
<div class="font-semibold text-gray-800 dark:text-gray-100">{order.nombre_usuario || 'N/A'}</div>
<div class="text-xs text-gray-500 dark:text-gray-400">{order.email_usuario || 'N/A'}</div>
</td>
<td class="px-6 py-4">{formatDate(order.fecha_pedido)}</td>
<td class="px-6 py-4">
<span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getStatusBadgeClass(order.estado)}">{order.estado || 'N/A'}</span>
</td>
<td class="px-6 py-4">
<span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getPaymentStatusBadgeClass(order.estado_pago)}">{order.estado_pago || 'N/A'}</span>
</td>
<td class="px-6 py-4 text-right font-medium text-gray-800 dark:text-gray-100">{formatCurrency(order.total)}</td>
<td class="px-6 py-4 text-center">
<div class="flex items-center justify-center gap-2">
<a href="/gestion-pedidos/{order.id_venta}" class="p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors flex items-center justify-center"><span class="material-symbols-outlined">visibility</span></a>
<button on:click={() => openEditModal(order)} class="p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors" title="Editar estado"><span class="material-symbols-outlined">edit</span></button>
<button on:click={() => openShippingModal(order)} class="p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors" title="Actualizar envío"><span class="material-symbols-outlined">local_shipping</span></button>
</div>
</td>
</tr>
{:else}
<tr>
<td colspan="7" class="px-6 py-8 text-center text-gray-500 dark:text-gray-400">
No se encontraron pedidos
</td>
</tr>
{/each}
</tbody>
</table>
</div>
<nav aria-label="Table navigation" class="flex flex-col md:flex-row justify-between items-start md:items-center space-y-3 md:space-y-0 p-4">
<span class="text-sm font-normal text-gray-500 dark:text-gray-400">
	Mostrando <span class="font-semibold text-gray-900 dark:text-white">{startIndex}-{endIndex}</span> de <span class="font-semibold text-gray-900 dark:text-white">{orders.length}</span> resultados
</span>
<div class="flex items-center gap-4">
<div class="flex items-center gap-2 text-sm">
<span class="text-gray-500 dark:text-gray-400">Items por página:</span>
<select bind:value={itemsPerPage} on:change={() => changeItemsPerPage(itemsPerPage)} class="form-select bg-gray-50 dark:bg-gray-800 border-gray-300 dark:border-gray-600 rounded-lg text-sm p-1 pr-7 focus:ring-primary focus:border-primary">
	<option value={10}>10</option>
	<option value={25}>25</option>
	<option value={50}>50</option>
	<option value={100}>100</option>
</select>
</div>
<ul class="inline-flex items-stretch -space-x-px">
<li>
	<button 
		on:click={() => goToPage(currentPage - 1)} 
		disabled={currentPage === 1}
		class="flex items-center justify-center h-full py-1.5 px-3 ml-0 text-gray-500 bg-white rounded-l-lg border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white disabled:opacity-50 disabled:cursor-not-allowed"
	>
		<span class="sr-only">Previous</span>
		<span class="material-symbols-outlined" style="font-size: 18px;">chevron_left</span>
	</button>
</li>
{#each pageNumbers as pageNum}
	{#if pageNum === -1}
		<li>
			<span class="flex items-center justify-center text-sm py-2 px-3 leading-tight text-gray-500 bg-white border border-gray-300 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400">...</span>
		</li>
	{:else}
		<li>
			<button
				on:click={() => goToPage(pageNum)}
				class="flex items-center justify-center text-sm py-2 px-3 leading-tight {currentPage === pageNum ? 'z-10 text-primary bg-blue-50 border border-primary hover:bg-blue-100 hover:text-blue-700 dark:border-gray-700 dark:bg-gray-700 dark:text-white' : 'text-gray-500 bg-white border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white'}"
			>
				{pageNum}
			</button>
		</li>
	{/if}
{/each}
<li>
	<button
		on:click={() => goToPage(currentPage + 1)}
		disabled={currentPage === totalPages}
		class="flex items-center justify-center h-full py-1.5 px-3 leading-tight text-gray-500 bg-white rounded-r-lg border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white disabled:opacity-50 disabled:cursor-not-allowed"
	>
		<span class="sr-only">Next</span>
		<span class="material-symbols-outlined" style="font-size: 18px;">chevron_right</span>
	</button>
</li>
</ul>
</div>
</nav>
</div>
</main>
</div>
</div>

<!-- Edit Status Modal -->
{#if isEditModalOpen}
<div class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4" on:click={closeEditModal} on:keydown={(e) => e.key === 'Escape' && closeEditModal()} role="button" tabindex="0">
	<div class="bg-white dark:bg-gray-900 rounded-xl shadow-xl max-w-md w-full p-6" on:click|stopPropagation>
		<div class="flex justify-between items-center mb-4">
			<h3 class="text-xl font-bold text-gray-900 dark:text-white">Editar Estado del Pedido</h3>
			<button on:click={closeEditModal} class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200">
				<span class="material-symbols-outlined">close</span>
			</button>
		</div>
		{#if selectedOrder}
		<div class="mb-4">
			<p class="text-sm text-gray-600 dark:text-gray-400 mb-2">Pedido: <span class="font-semibold text-gray-900 dark:text-white">{selectedOrder.numero_pedido}</span></p>
			<p class="text-sm text-gray-600 dark:text-gray-400">Cliente: <span class="font-semibold text-gray-900 dark:text-white">{selectedOrder.nombre_usuario}</span></p>
		</div>
		<div class="mb-6">
			<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Nuevo Estado</label>
			<select bind:value={newStatus} class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary/50">
				<option value="Pendiente">Pendiente</option>
				<option value="Confirmado">Confirmado</option>
				<option value="Procesando">Procesando</option>
				<option value="En Proceso">En Proceso</option>
				<option value="Enviado">Enviado</option>
				<option value="Entregado">Entregado</option>
				<option value="Cancelado">Cancelado</option>
				<option value="Devuelto">Devuelto</option>
			</select>
		</div>
		{/if}
		<div class="flex justify-end gap-3">
			<button on:click={closeEditModal} class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors">Cancelar</button>
			<button on:click={saveOrderStatus} class="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors">Guardar</button>
		</div>
	</div>
</div>
{/if}

<!-- Shipping Info Modal -->
{#if isShippingModalOpen}
<div class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4" on:click={closeShippingModal} on:keydown={(e) => e.key === 'Escape' && closeShippingModal()} role="button" tabindex="0">
	<div class="bg-white dark:bg-gray-900 rounded-xl shadow-xl max-w-md w-full p-6" on:click|stopPropagation>
		<div class="flex justify-between items-center mb-4">
			<h3 class="text-xl font-bold text-gray-900 dark:text-white">Actualizar Información de Envío</h3>
			<button on:click={closeShippingModal} class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200">
				<span class="material-symbols-outlined">close</span>
			</button>
		</div>
		{#if selectedOrder}
		<div class="mb-4">
			<p class="text-sm text-gray-600 dark:text-gray-400 mb-2">Pedido: <span class="font-semibold text-gray-900 dark:text-white">{selectedOrder.numero_pedido}</span></p>
			<p class="text-sm text-gray-600 dark:text-gray-400">Cliente: <span class="font-semibold text-gray-900 dark:text-white">{selectedOrder.nombre_usuario}</span></p>
		</div>
		<div class="space-y-4 mb-6">
			<div>
				<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Empresa de Envío</label>
				<select bind:value={shippingCompany} class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary/50">
					<option value="">Seleccionar empresa...</option>
					<option value="Correos">Correos</option>
					<option value="SEUR">SEUR</option>
					<option value="MRW">MRW</option>
					<option value="DHL">DHL</option>
					<option value="FedEx">FedEx</option>
				</select>
			</div>
			<div>
				<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Número de Tracking</label>
				<input bind:value={trackingNumber} type="text" placeholder="Ej: TRK123456789" class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary/50" />
			</div>
		</div>
		{/if}
		<div class="flex justify-end gap-3">
			<button on:click={closeShippingModal} class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors">Cancelar</button>
			<button on:click={saveShippingInfo} class="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors">Guardar</button>
		</div>
	</div>
</div>
{/if}
