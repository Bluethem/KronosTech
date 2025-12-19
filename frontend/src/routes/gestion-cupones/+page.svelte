<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  // Data from API
  let cupones = [];
  let allCupones = []; // Store all cupones unfiltered
  let originalCupones = []; // Store original data from API
  let stats = {
    total_activos: 0,
    usos_hoy: 0,
    descuento_mes: 0
  };
  
  // Pagination
  let currentPage = 1;
  let itemsPerPage = 10;
  let totalPages = 1;
  
  // UI State
  let isCreateSidebarOpen = false;
  let isDetailsSidebarOpen = false;
  let activeTab = 'info';
  let isMassAssignModalOpen = false;
  let massAssignStep = 1;
  let selectedCupon = null;
  
  // Mass Assignment State
  let usuarios = [];
  let selectedUsuarios = [];
  let userSearchQuery = '';
  let assignmentResult = null;
  let isAssigning = false;
  
  // Assigned Users State
  let assignedUsuarios = [];
  let loadingAssignedUsers = false;
  
  // Search and filters
  let searchQuery = '';
  let tipoFilter = '';
  let estadoFilter = '';
  let tipoEnvioFilter = ''; // New filter
  let restriccionesFilter = ''; // New filter
  
  // PDF libraries
  let jsPDF: any;
  let autoTable: any;
  
  onMount(async () => {
    // Dynamically import jsPDF and autoTable
    const jsPDFModule = await import('jspdf');
    jsPDF = jsPDFModule.default;
    const autoTableModule = await import('jspdf-autotable');
    autoTable = autoTableModule.default;
    
    await Promise.all([
      fetchCupones(),
      fetchStats()
    ]);
  });
  
  async function fetchCupones() {
    try {
      const response = await fetch('http://localhost:3000/api/cupones');
      if (response.ok) {
        originalCupones = await response.json();
        applyFilters();
      }
    } catch (e) {
      console.error('Error fetching cupones:', e);
    }
  }
  
  // Frontend filtering
  function applyFilters() {
    let filtered = [...originalCupones]; // Start with original data
    
    // Search filter
    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter(c => 
        c.codigo?.toLowerCase().includes(query) ||
        c.nombre?.toLowerCase().includes(query)
      );
    }
    
    // Tipo filter (case-insensitive)
    if (tipoFilter) {
      filtered = filtered.filter(c => 
        c.tipo_cupon?.toLowerCase() === tipoFilter.toLowerCase()
      );
    }
    
    // Estado filter
    if (estadoFilter) {
      filtered = filtered.filter(c => {
        const now = new Date();
        const start = new Date(c.fecha_inicio);
        const end = new Date(c.fecha_fin);
        
        if (estadoFilter === 'activo' && c.activo && now >= start && now <= end) return true;
        if (estadoFilter === 'inactivo' && !c.activo) return true;
        if (estadoFilter === 'vencido' && now > end) return true;
        if (estadoFilter === 'proximo' && now < start) return true;
        return false;
      });
    }
    
    // Tipo de envío filter
    if (tipoEnvioFilter) {
      if (tipoEnvioFilter === 'envio_gratis') {
        filtered = filtered.filter(c => c.tipo_cupon?.toLowerCase() === 'envio_gratis');
      } else if (tipoEnvioFilter === 'con_costo') {
        filtered = filtered.filter(c => c.tipo_cupon?.toLowerCase() !== 'envio_gratis');
      }
    }
    
    // Restricciones filter
    if (restriccionesFilter) {
      if (restriccionesFilter === 'con_restricciones') {
        filtered = filtered.filter(c => 
          c.compra_minima > 0 || 
          c.usos_maximos_totales || 
          c.usos_maximos_por_usuario === 1
        );
      } else if (restriccionesFilter === 'sin_restricciones') {
        filtered = filtered.filter(c => 
          !c.compra_minima && 
          !c.usos_maximos_totales && 
          c.usos_maximos_por_usuario !== 1
        );
      }
    }
    
    allCupones = filtered; // Update filtered results
    updatePagination();
  }
  
  function clearFilters() {
    searchQuery = '';
    tipoFilter = '';
    estadoFilter = '';
    tipoEnvioFilter = '';
    restriccionesFilter = '';
    fetchCupones();
  }
  
  function updatePagination() {
    totalPages = Math.ceil(allCupones.length / itemsPerPage);
    if (currentPage > totalPages && totalPages > 0) {
      currentPage = totalPages;
    }
    if (currentPage < 1) currentPage = 1;
    
    const startIndex = (currentPage - 1) * itemsPerPage;
    const endIndex = startIndex + itemsPerPage;
    cupones = allCupones.slice(startIndex, endIndex);
  }
  
  function goToPage(page: number) {
    if (page >= 1 && page <= totalPages) {
      currentPage = page;
      updatePagination();
    }
  }
  
  function nextPage() {
    if (currentPage < totalPages) {
      currentPage++;
      updatePagination();
    }
  }
  
  function prevPage() {
    if (currentPage > 1) {
      currentPage--;
      updatePagination();
    }
  }
  
  function getPageNumbers() {
    const pages = [];
    const maxVisible = 5;
    
    if (totalPages <= maxVisible) {
      for (let i = 1; i <= totalPages; i++) {
        pages.push(i);
      }
    } else {
      if (currentPage <= 3) {
        for (let i = 1; i <= 4; i++) pages.push(i);
        pages.push('...');
        pages.push(totalPages);
      } else if (currentPage >= totalPages - 2) {
        pages.push(1);
        pages.push('...');
        for (let i = totalPages - 3; i <= totalPages; i++) pages.push(i);
      } else {
        pages.push(1);
        pages.push('...');
        pages.push(currentPage - 1);
        pages.push(currentPage);
        pages.push(currentPage + 1);
        pages.push('...');
        pages.push(totalPages);
      }
    }
    
    return pages;
  }
  
  async function fetchStats() {
    try {
      const response = await fetch('http://localhost:3000/api/cupones/stats');
      if (response.ok) {
        stats = await response.json();
      }
    } catch (e) {
      console.error('Error fetching stats:', e);
    }
  }
  
  async function deleteCupon(cupon) {
    if (!confirm(`¿Estás seguro de eliminar el cupón "${cupon.codigo}"?`)) {
      return;
    }
    
    try {
      const response = await fetch(`http://localhost:3000/api/cupones/${cupon.id_cupon}`, {
        method: 'DELETE'
      });
      
      if (response.ok) {
        await Promise.all([fetchCupones(), fetchStats()]);
      }
    } catch (e) {
      console.error('Error deleting cupon:', e);
    }
  }
  
  function openCreateSidebar() {
    goto('/gestion-cupones/nuevo');
  }

  function closeCreateSidebar() {
    isCreateSidebarOpen = false;
  }

  async function openDetailsSidebar(cupon) {
    selectedCupon = cupon;
    isDetailsSidebarOpen = true;
    await fetchAssignedUsers(cupon.id_cupon);
  }

  function closeDetailsSidebar() {
    isDetailsSidebarOpen = false;
    assignedUsuarios = [];
    selectedCupon = null;
  }

  function openMassAssignModal() {
    isMassAssignModalOpen = true;
    massAssignStep = 1;
  }

  function closeMassAssignModal() {
    isMassAssignModalOpen = false;
    massAssignStep = 1;
    selectedUsuarios = [];
    assignmentResult = null;
  }
  
  async function fetchUsuarios() {
    try {
      const params = new URLSearchParams();
      if (userSearchQuery) params.append('search', userSearchQuery);
      
      const response = await fetch(`http://localhost:3000/api/usuarios?${params}`);
      if (response.ok) {
        usuarios = await response.json();
      }
    } catch (e) {
      console.error('Error fetching usuarios:', e);
    }
  }
  
  function toggleUsuarioSelection(id_usuario) {
    const index = selectedUsuarios.indexOf(id_usuario);
    if (index > -1) {
      selectedUsuarios = selectedUsuarios.filter(id => id !== id_usuario);
    } else {
      selectedUsuarios = [...selectedUsuarios, id_usuario];
    }
  }
  
  function selectAllUsuarios() {
    selectedUsuarios = usuarios.map(u => u.id_usuario);
  }
  
  function deselectAllUsuarios() {
    selectedUsuarios = [];
  }
  
  async function proceedToConfirmation() {
    if (selectedUsuarios.length === 0) {
      alert('Debes seleccionar al menos un usuario');
      return;
    }
    massAssignStep = 2;
  }
  
  async function assignCuponToUsers() {
    if (!selectedCupon || selectedUsuarios.length === 0) {
      return;
    }
    
    isAssigning = true;
    
    try {
      const response = await fetch('http://localhost:3000/api/cupones/assign', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          id_cupon: selectedCupon.id_cupon,
          usuarios: selectedUsuarios
        })
      });
      
      if (response.ok) {
        assignmentResult = await response.json();
        massAssignStep = 3;
      } else {
        alert('Error al asignar cupones');
      }
    } catch (e) {
      console.error('Error assigning cupones:', e);
      alert('Error de conexión');
    } finally {
      isAssigning = false;
    }
  }
  
  async function fetchAssignedUsers(id_cupon) {
    loadingAssignedUsers = true;
    try {
      const response = await fetch(`http://localhost:3000/api/cupones/${id_cupon}/usuarios`);
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
  
  function formatDate(dateString) {
    if (!dateString) return '-';
    const date = new Date(dateString);
    return date.toLocaleDateString('es-ES', { day: '2-digit', month: 'short' });
  }
  
  function getTipoBadgeClass(tipo) {
    const map = {
      'porcentaje': 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300',
      'monto_fijo': 'bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-300',
      'envio_gratis': 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300'
    };
    return map[tipo] || 'bg-gray-100 text-gray-800';
  }
  
  function getTipoLabel(tipo) {
    const map = {
      'porcentaje': 'Porcentaje',
      'monto_fijo': 'Monto Fijo',
      'envio_gratis': 'Envío Gratis'
    };
    return map[tipo] || tipo;
  }
  
  function formatValor(tipo, valor) {
    if (tipo === 'envio_gratis') return '-';
    if (tipo === 'porcentaje') return `${valor}%`;
    return `$${valor}`;
  }
  
  // Calcular estado real del cupón basado en fechas y campo activo
  function getEstadoCupon(cupon) {
    if (!cupon.activo) return 'inactivo';
    
    const now = new Date();
    const inicio = new Date(cupon.fecha_inicio);
    const fin = new Date(cupon.fecha_fin);
    
    if (now > fin) return 'vencido';
    if (now < inicio) return 'proximo';
    return 'vigente';
  }
  
  function getEstadoBadgeClass(estado: string) {
    const map: Record<string, string> = {
      'vigente': 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300',
      'proximo': 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300',
      'vencido': 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300',
      'inactivo': 'bg-slate-100 text-slate-800 dark:bg-slate-700 dark:text-slate-300'
    };
    return map[estado] || map['inactivo'];
  }
  
  function getEstadoLabel(estado: string) {
    const map: Record<string, string> = {
      'vigente': 'Vigente',
      'proximo': 'Próximo',
      'vencido': 'Vencido',
      'inactivo': 'Inactivo'
    };
    return map[estado] || estado;
  }
  
  function getRestricciones(cupon) {
    const restricciones: string[] = [];
    
    if (cupon.compra_minima && cupon.compra_minima > 0) {
      restricciones.push(`Min: $${cupon.compra_minima}`);
    }
    if (cupon.solo_nuevos_usuarios) {
      restricciones.push('Nuevos usuarios');
    }
    if (cupon.solo_primera_compra) {
      restricciones.push('1ra compra');
    }
    if (cupon.usos_maximos_por_usuario) {
      restricciones.push(`${cupon.usos_maximos_por_usuario} uso/usuario`);
    }
    
    return restricciones.length > 0 ? restricciones : ['Sin restricciones'];
  }

  function getRestriccionesText(cupon) {
    const restricciones = getRestricciones(cupon);
    return restricciones.join(' · ');
  }
  
  // Debounce search
  let searchTimeout;
  function handleSearch() {
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(() => {
      currentPage = 1; // Reset to first page on search
      applyFilters();
    }, 300);
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
    doc.text('Reporte de Cupones', 15, 30);
    
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
    const totalCupones = allCupones.length;
    const activeCupones = allCupones.filter(c => c.activo).length;
    const porcentajeCupones = allCupones.filter(c => c.tipo_cupon === 'porcentaje').length;
    const envioGratisCupones = allCupones.filter(c => c.tipo_cupon === 'envio_gratis').length;
    
    let yPosition = 50;
    
    // Summary box
    doc.setFillColor(249, 250, 251);
    doc.roundedRect(15, yPosition, pageWidth - 30, 35, 3, 3, 'F');
    
    doc.setFontSize(11);
    doc.setFont('helvetica', 'bold');
    doc.text('Resumen General', 20, yPosition + 8);
    
    doc.setFont('helvetica', 'normal');
    doc.setFontSize(9);
    doc.text(`Total de Cupones: ${totalCupones}`, 20, yPosition + 16);
    doc.text(`Cupones Activos: ${activeCupones}`, 20, yPosition + 23);
    doc.text(`Usos Hoy: ${stats.usos_hoy}`, 20, yPosition + 30);
    
    doc.text(`Tipo Porcentaje: ${porcentajeCupones}`, 80, yPosition + 16);
    doc.text(`Envío Gratis: ${envioGratisCupones}`, 80, yPosition + 23);
    doc.text(`Descuento Mes: $${stats.descuento_mes || 0}`, 80, yPosition + 30);
    
    yPosition += 45;
    
    // Table title
    doc.setFontSize(12);
    doc.setFont('helvetica', 'bold');
    doc.text('Detalle de Cupones', 15, yPosition);
    
    yPosition += 5;
    
    // Prepare table data
    const tableData = allCupones.map(cupon => [
      cupon.codigo || 'N/A',
      cupon.nombre || 'N/A',
      getTipoLabel(cupon.tipo_cupon),
      formatValor(cupon.tipo_cupon, cupon.valor),
      `${formatDate(cupon.fecha_inicio)} - ${formatDate(cupon.fecha_fin)}`,
      cupon.activo ? 'Activo' : 'Inactivo'
    ]);
    
    // Generate table
    autoTable(doc, {
      startY: yPosition,
      head: [['Código', 'Nombre', 'Tipo', 'Valor', 'Vigencia', 'Estado']],
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
        2: { cellWidth: 25 },
        3: { cellWidth: 20 },
        4: { cellWidth: 40 },
        5: { cellWidth: 20 }
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
    doc.save(`Reporte_Cupones_${new Date().toISOString().split('T')[0]}.pdf`);
  }
</script>

<div class="relative flex h-auto min-h-screen w-full flex-col group/design-root bg-background-light dark:bg-background-dark font-display text-[#111418] dark:text-gray-200">
  <div class="layout-container flex h-full grow flex-col">
    <main class="flex-1 p-6 lg:p-10">
      <div class="max-w-7xl mx-auto">
        <!-- PageHeading -->
        <div class="flex flex-wrap justify-between items-center gap-4 mb-8">
          <div class="flex flex-col">
            <h1 class="text-slate-900 dark:text-white text-4xl font-black leading-tight tracking-[-0.033em]">Gestión de Cupones</h1>
            <p class="text-slate-500 dark:text-slate-400 text-base font-normal leading-normal mt-2">Crea, administra y monitorea el rendimiento de tus cupones de descuento.</p>
          </div>
          <button class="flex min-w-[84px] cursor-pointer items-center justify-center gap-2 overflow-hidden rounded-lg h-10 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em] hover:bg-primary/90" on:click={openCreateSidebar}>
            <span class="material-symbols-outlined" style="font-variation-settings: 'wght' 600;">add</span>
            <span class="truncate">Crear Cupón</span>
          </button>
        </div>
        <!-- Stats -->
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
          <div class="flex flex-col gap-2 rounded-xl p-6 bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800">
            <p class="text-slate-600 dark:text-slate-300 text-sm font-medium">Cupones activos ahora</p>
            <p class="text-slate-900 dark:text-white tracking-light text-3xl font-bold">{stats.total_activos}</p>
          </div>
          <div class="flex flex-col gap-2 rounded-xl p-6 bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800">
            <p class="text-slate-600 dark:text-slate-300 text-sm font-medium">Usos totales hoy</p>
            <p class="text-slate-900 dark:text-white tracking-light text-3xl font-bold">{stats.usos_hoy}</p>
          </div>
          <div class="flex flex-col gap-2 rounded-xl p-6 bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800">
            <p class="text-slate-600 dark:text-slate-300 text-sm font-medium">Descuento total (mes)</p>
            <p class="text-slate-900 dark:text-white tracking-light text-3xl font-bold">${stats.descuento_mes || 0}</p>
          </div>
        </div>
        <!-- Toolbar and Search -->
        <div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-4 mb-6">
          <div class="flex flex-wrap justify-between items-center gap-4">
            <div class="flex-1 min-w-[250px]">
              <label class="flex flex-col w-full">
                <div class="flex w-full flex-1 items-stretch rounded-lg h-10">
                  <div class="text-slate-500 dark:text-slate-400 flex bg-background-light dark:bg-background-dark items-center justify-center pl-3 rounded-l-lg border border-r-0 border-slate-300 dark:border-slate-700">
                    <span class="material-symbols-outlined text-base">search</span>
                  </div>
                  <input bind:value={searchQuery} on:input={handleSearch} class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-slate-800 dark:text-slate-200 focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-l-0 border-slate-300 dark:border-slate-700 bg-background-light dark:bg-background-dark h-full placeholder:text-slate-500 dark:placeholder:text-slate-400 px-4 rounded-l-none pl-2 text-sm font-normal" placeholder="Buscar por código o nombre de cupón..."/>
                </div>
              </label>
            </div>
            <div class="flex items-center gap-2">
              <button class="p-2 text-slate-500 dark:text-slate-400 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-primary">
                <span class="material-symbols-outlined">filter_list</span>
              </button>
              <button on:click={generatePDFReport} class="p-2 text-slate-500 dark:text-slate-400 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-primary" title="Descargar reporte PDF">
                <span class="material-symbols-outlined">download</span>
              </button>
              <button class="flex cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 bg-primary/10 dark:bg-primary/20 text-primary gap-2 text-sm font-bold leading-normal tracking-[0.015em] min-w-0 px-4 hover:bg-primary/20 dark:hover:bg-primary/30" on:click={openMassAssignModal}>
                <span class="material-symbols-outlined text-base" style="font-variation-settings: 'FILL' 1;">groups</span>
                <span class="truncate">Asignación Masiva</span>
              </button>
            </div>
          </div>
          <!-- Filters Row -->
          <div class="flex flex-wrap items-center gap-3 mt-4 pt-4 border-t border-slate-200 dark:border-slate-800">
            <select bind:value={tipoFilter} on:change={applyFilters} class="flex h-10 shrink-0 items-center rounded-lg bg-background-light dark:bg-slate-800 px-3 text-slate-800 dark:text-slate-200 border border-slate-300 dark:border-slate-700 text-sm">
              <option value="">Todos los tipos</option>
              <option value="porcentaje">Porcentaje</option>
              <option value="monto_fijo">Monto Fijo</option>
              <option value="envio_gratis">Envío Gratis</option>
            </select>
            <select bind:value={estadoFilter} on:change={applyFilters} class="flex h-10 shrink-0 items-center rounded-lg bg-background-light dark:bg-slate-800 px-3 text-slate-800 dark:text-slate-200 border border-slate-300 dark:border-slate-700 text-sm">
              <option value="">Todos los estados</option>
              <option value="activo">Activo</option>
              <option value="inactivo">Inactivo</option>
              <option value="vencido">Vencido</option>
              <option value="proximo">Próximo</option>
            </select>
            <select bind:value={tipoEnvioFilter} on:change={applyFilters} class="flex h-10 shrink-0 items-center rounded-lg bg-background-light dark:bg-slate-800 px-3 text-slate-800 dark:text-slate-200 border border-slate-300 dark:border-slate-700 text-sm">
              <option value="">Tipo de envío</option>
              <option value="envio_gratis">Envío Gratis</option>
              <option value="con_costo">Con Costo</option>
            </select>
            <select bind:value={restriccionesFilter} on:change={applyFilters} class="flex h-10 shrink-0 items-center rounded-lg bg-background-light dark:bg-slate-800 px-3 text-slate-800 dark:text-slate-200 border border-slate-300 dark:border-slate-700 text-sm">
              <option value="">Restricciones</option>
              <option value="con_restricciones">Con Restricciones</option>
              <option value="sin_restricciones">Sin Restricciones</option>
            </select>
            <button 
              on:click={clearFilters}
              class="flex h-10 shrink-0 items-center justify-center gap-x-2 rounded-lg bg-slate-200 dark:bg-slate-700 px-4 text-slate-800 dark:text-slate-200 hover:bg-slate-300 dark:hover:bg-slate-600 transition-colors text-sm"
            >
              <span class="material-symbols-outlined text-sm">filter_alt_off</span>
              <span class="font-medium">Limpiar</span>
            </button>
          </div>
        </div>
        <!-- Table -->
        <div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl">
          <table class="w-full text-sm text-left">
            <thead class="bg-slate-50 dark:bg-slate-800 text-xs text-slate-500 dark:text-slate-400 uppercase">
              <tr>
                <th class="px-4 py-3" scope="col">Código</th>
                <th class="px-4 py-3" scope="col">Nombre</th>
                <th class="px-4 py-3 text-center" scope="col">Tipo</th>
                <th class="px-4 py-3 text-center" scope="col">Valor</th>
                <th class="px-4 py-3 text-center" scope="col">Vigencia</th>
                <th class="px-4 py-3 text-center" scope="col">Usos</th>
                <th class="px-4 py-3 text-center" scope="col">Estado</th>
                <th class="px-4 py-3 text-right" scope="col">Acciones</th>
              </tr>
            </thead>
            <tbody>
              {#each cupones as cupon (cupon.id_cupon)}
              <tr class="border-b dark:border-slate-800 hover:bg-slate-50 dark:hover:bg-slate-800/50">
                <td class="px-4 py-3">
                  <span class="font-mono bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 rounded px-2 py-1 text-xs font-semibold">{cupon.codigo}</span>
                </td>
                <td class="px-4 py-3">
                  <div class="font-medium text-slate-900 dark:text-white">{cupon.nombre}</div>
                  <div class="text-xs text-slate-500 dark:text-slate-400">{getRestriccionesText(cupon)}</div>
                </td>
                <td class="px-4 py-3 text-center">
                  <span class="{getTipoBadgeClass(cupon.tipo_cupon)} text-xs font-medium px-2 py-0.5 rounded-full">{getTipoLabel(cupon.tipo_cupon)}</span>
                </td>
                <td class="px-4 py-3 text-center font-semibold">{formatValor(cupon.tipo_cupon, cupon.valor)}</td>
                <td class="px-4 py-3 text-center text-xs text-slate-600 dark:text-slate-400 whitespace-nowrap">
                  <div>{formatDate(cupon.fecha_inicio)}</div>
                  <div>{formatDate(cupon.fecha_fin)}</div>
                </td>
                <td class="px-4 py-3 text-center text-sm">{cupon.usos_actuales || 0}{cupon.usos_maximos_totales ? `/${cupon.usos_maximos_totales}` : ''}</td>
                <td class="px-4 py-3 text-center">
                  <span class="{getEstadoBadgeClass(getEstadoCupon(cupon))} text-xs font-medium px-2.5 py-1 rounded-full">
                    {getEstadoLabel(getEstadoCupon(cupon))}
                  </span>
                </td>
                <td class="px-4 py-3">
                  <div class="flex items-center justify-end gap-1">
                    <button on:click={() => openDetailsSidebar(cupon)} class="p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-500 hover:text-primary transition-colors" title="Ver detalles"><span class="material-symbols-outlined text-xl">visibility</span></button>
                    <a href="/gestion-cupones/editar/{cupon.id_cupon}" class="p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-500 hover:text-primary transition-colors" title="Editar"><span class="material-symbols-outlined text-xl">edit</span></a>
                    <button on:click={() => deleteCupon(cupon)} class="p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-500 hover:text-red-500 transition-colors" title="Eliminar"><span class="material-symbols-outlined text-xl">delete</span></button>
                  </div>
                </td>
              </tr>
              {/each}
              {#if cupones.length === 0}
              <tr>
                <td colspan="8" class="px-4 py-12 text-center text-slate-500 dark:text-slate-400">
                  No se encontraron cupones
                </td>
              </tr>
              {/if}
            </tbody>
          </table>
          <!-- Pagination -->
          <nav aria-label="Table navigation" class="flex items-center justify-between p-4">
            <span class="text-sm font-normal text-slate-500 dark:text-slate-400">
              Mostrando 
              <span class="font-semibold text-slate-900 dark:text-white">
                {((currentPage - 1) * itemsPerPage) + 1}-{Math.min(currentPage * itemsPerPage, allCupones.length)}
              </span> 
              de 
              <span class="font-semibold text-slate-900 dark:text-white">{allCupones.length}</span>
            </span>
            <ul class="inline-flex items-center -space-x-px">
              <li>
                <button 
                  on:click={prevPage}
                  disabled={currentPage === 1}
                  class="block px-3 py-2 ml-0 leading-tight text-slate-500 bg-white border border-slate-300 rounded-l-lg hover:bg-slate-100 hover:text-slate-700 dark:bg-slate-800 dark:border-slate-700 dark:text-slate-400 dark:hover:bg-slate-700 dark:hover:text-white disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  <span class="sr-only">Previous</span>
                  <span class="material-symbols-outlined text-lg">chevron_left</span>
                </button>
              </li>
              {#each getPageNumbers() as page}
                <li>
                  {#if page === '...'}
                    <span class="px-3 py-2 leading-tight text-slate-500 bg-white border border-slate-300 dark:bg-slate-800 dark:border-slate-700 dark:text-slate-400">...</span>
                  {:else}
                    <button
                      on:click={() => goToPage(page)}
                      class="{page === currentPage 
                        ? 'z-10 px-3 py-2 leading-tight text-primary bg-primary/10 border border-primary hover:bg-primary/20 dark:border-primary dark:bg-primary/20 dark:text-white' 
                        : 'px-3 py-2 leading-tight text-slate-500 bg-white border border-slate-300 hover:bg-slate-100 hover:text-slate-700 dark:bg-slate-800 dark:border-slate-700 dark:text-slate-400 dark:hover:bg-slate-700 dark:hover:text-white'}"
                    >
                      {page}
                    </button>
                  {/if}
                </li>
              {/each}
              <li>
                <button
                  on:click={nextPage}
                  disabled={currentPage === totalPages}
                  class="block px-3 py-2 leading-tight text-slate-500 bg-white border border-slate-300 rounded-r-lg hover:bg-slate-100 hover:text-slate-700 dark:bg-slate-800 dark:border-slate-700 dark:text-slate-400 dark:hover:bg-slate-700 dark:hover:text-white disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  <span class="sr-only">Next</span>
                  <span class="material-symbols-outlined text-lg">chevron_right</span>
                </button>
              </li>
            </ul>
          </nav>
        </div>
      </div>
    </main>
  </div>
</div>

<!-- Create Coupon Sidebar Overlay -->
{#if isCreateSidebarOpen}
  <div class="fixed inset-0 bg-black/50 z-40 transition-opacity" on:click={closeCreateSidebar} on:keydown={(e) => e.key === 'Escape' && closeCreateSidebar()} role="button" tabindex="0"></div>
{/if}

<!-- Create Coupon Sidebar -->
<div class="fixed inset-y-0 right-0 flex max-w-full z-50 transform transition-transform duration-300 ease-in-out {isCreateSidebarOpen ? 'translate-x-0' : 'translate-x-full'}">
  <div class="relative w-screen max-w-2xl">
    <div class="flex h-full flex-col overflow-y-scroll bg-white dark:bg-[#18212a] shadow-xl">
      <div class="flex-1">
        <!-- Header -->
        <div class="bg-white dark:bg-[#18212a] px-4 py-6 sm:px-6 sticky top-0 z-10 border-b border-gray-200 dark:border-gray-700">
          <div class="flex items-start justify-between space-x-3">
            <div class="space-y-1">
              <h2 class="text-lg font-medium text-gray-900 dark:text-white">Crear Nuevo Cupón</h2>
              <p class="text-sm text-gray-500 dark:text-gray-400">Completa los detalles para generar un cupón de descuento.</p>
            </div>
            <button class="p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-500 dark:text-gray-400" on:click={closeCreateSidebar}>
              <span class="material-symbols-outlined">close</span>
            </button>
          </div>
        </div>
        <!-- Content -->
        <div class="space-y-6 sm:space-y-0 sm:divide-y sm:divide-gray-200 dark:sm:divide-gray-700 p-4 sm:p-6">
          <!-- Preview Section -->
          <div>
            <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] pb-2 pt-4">Vista Previa del Cupón</h3>
            <div class="p-4 @container">
              <div class="flex flex-col items-stretch justify-start rounded-lg @xl:flex-row @xl:items-start shadow-[0_0_4px_rgba(0,0,0,0.1)] bg-white dark:bg-background-dark border border-gray-200 dark:border-gray-700">
                <div class="w-full @xl:w-1/3 bg-center bg-no-repeat aspect-video @xl:aspect-square bg-cover rounded-t-lg @xl:rounded-l-lg @xl:rounded-tr-none" data-alt="Abstract blue and purple geometric pattern for a coupon background" style="background-image: url('https://lh3.googleusercontent.com/aida-public/AB6AXuBpGpAgrEW_nM_6EzEg1yZVMIS1BSjv8raBBj8zFGXUJc1Z1AQY_EhOVCv185OboKqK8d2-oqaPdprh6QdAovpbacsSfzI5ePhCzFIyadK9zoKx4wea2vNRaTspmBX34AC2kTTu8qJVkYeC3OVpDa-VdWxeo3acR0zKlRpwVyU5jmkZTicDthaX7hDWUNF_zW8_GkEvahENL23fBBBzuMdyGiBcqEv42wlgEgRUcZF5TRAGHoBCtpVcN3YY9Qd42igBs66wkiF4E74');"></div>
                <div class="flex w-full min-w-72 grow flex-col items-stretch justify-center gap-1 py-4 @xl:px-4 px-4">
                  <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Promo Lanzamiento RTX 4070</p>
                  <p class="text-[#111418] dark:text-white text-2xl font-bold leading-tight tracking-[-0.015em]">PROMO4070</p>
                  <div class="flex items-end gap-3 justify-between mt-2">
                    <div class="flex flex-col gap-1">
                      <p class="text-[#111418] dark:text-gray-300 text-base font-medium leading-normal">20% de descuento en tu próxima compra.</p>
                      <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Válido hasta 31/12/2024</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
          <!-- Basic Info Section -->
          <div class="pt-6">
            <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] pb-4">Información Básica</h3>
            <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
              <div class="sm:col-span-4">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="coupon-code">Código del Cupón</label>
                <div class="mt-1 flex rounded-md shadow-sm">
                  <input class="form-input block w-full min-w-0 flex-1 rounded-none rounded-l-md border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white focus:border-primary focus:ring-primary sm:text-sm" id="coupon-code" name="coupon-code" placeholder="EJ: PROMO4070" type="text"/>
                  <button class="relative -ml-px inline-flex items-center space-x-2 rounded-r-md border border-gray-300 dark:border-gray-600 bg-gray-50 dark:bg-gray-700 px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-600 focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary" type="button">
                    <span class="material-symbols-outlined text-base">auto_awesome</span>
                    <span>Generar</span>
                  </button>
                </div>
              </div>
              <div class="sm:col-span-6">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="coupon-name">Nombre del Cupón</label>
                <div class="mt-1">
                  <input class="form-input shadow-sm block w-full sm:text-sm border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:border-primary focus:ring-primary" id="coupon-name" name="coupon-name" placeholder="Promo Lanzamiento RTX 4070" type="text"/>
                </div>
              </div>
              <div class="sm:col-span-6">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="description">Descripción (opcional)</label>
                <div class="mt-1">
                  <textarea class="form-textarea shadow-sm block w-full sm:text-sm border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:border-primary focus:ring-primary" id="description" name="description" placeholder="20% de descuento en tu próxima compra." rows="3"></textarea>
                </div>
              </div>
            </div>
          </div>
          <!-- Type and Value Section -->
          <div class="pt-6">
            <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] pb-4">Tipo y Valor</h3>
            <div class="space-y-4">
              <div>
                <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Tipo de Descuento</label>
                <fieldset class="mt-2">
                  <legend class="sr-only">Tipo de Descuento</legend>
                  <div class="grid grid-cols-3 gap-3">
                    <label class="border rounded-md py-3 px-3 flex items-center justify-center text-sm font-medium sm:flex-1 cursor-pointer focus:outline-none ring-2 ring-primary border-transparent bg-primary text-white">
                      <input checked="" class="sr-only" name="discount-type" type="radio" value="percentage"/>
                      <span>Porcentaje</span>
                    </label>
                    <label class="border rounded-md py-3 px-3 flex items-center justify-center text-sm font-medium sm:flex-1 cursor-pointer focus:outline-none border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white hover:bg-gray-50 dark:hover:bg-gray-600">
                      <input class="sr-only" name="discount-type" type="radio" value="fixed"/>
                      <span>Monto Fijo</span>
                    </label>
                    <label class="border rounded-md py-3 px-3 flex items-center justify-center text-sm font-medium sm:flex-1 cursor-pointer focus:outline-none border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white hover:bg-gray-50 dark:hover:bg-gray-600">
                      <input class="sr-only" name="discount-type" type="radio" value="free-shipping"/>
                      <span>Envío Gratis</span>
                    </label>
                  </div>
                </fieldset>
              </div>
              <div class="sm:col-span-3">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="value">Valor</label>
                <div class="mt-1 relative rounded-md shadow-sm">
                  <input class="form-input block w-full sm:text-sm border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md pr-12 focus:border-primary focus:ring-primary" id="value" name="value" placeholder="20" type="number"/>
                  <div class="absolute inset-y-0 right-0 flex items-center">
                    <span class="text-gray-500 dark:text-gray-400 sm:text-sm pr-4">%</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
          <!-- Applicability Section -->
          <div class="pt-6">
            <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] pb-4">Aplicabilidad</h3>
            <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
              <div class="sm:col-span-3">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="apply-to">Aplica a</label>
                <select class="form-select mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white focus:outline-none focus:ring-primary focus:border-primary sm:text-sm rounded-md" id="apply-to" name="apply-to">
                  <option>Todo</option>
                  <option selected="">Producto específico</option>
                  <option>Categoría</option>
                  <option>Marca</option>
                </select>
              </div>
              <div class="sm:col-span-3">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="specific-item">Producto</label>
                <select class="form-select mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white focus:outline-none focus:ring-primary focus:border-primary sm:text-sm rounded-md" id="specific-item" name="specific-item">
                  <option>Nvidia GeForce RTX 4070</option>
                  <option>AMD Ryzen 7 7800X3D</option>
                  <option>Samsung 980 Pro SSD 1TB</option>
                </select>
              </div>
              <div class="sm:col-span-3">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="min-purchase">Compra Mínima</label>
                <div class="mt-1 relative rounded-md shadow-sm">
                  <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                    <span class="text-gray-500 dark:text-gray-400 sm:text-sm">$</span>
                  </div>
                  <input class="form-input block w-full pl-7 sm:text-sm border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:border-primary focus:ring-primary" id="min-purchase" name="min-purchase" placeholder="0.00" type="number"/>
                </div>
              </div>
            </div>
          </div>
          <!-- Usage Restrictions -->
          <div class="pt-6">
            <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] pb-4">Restricciones de Uso</h3>
            <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
              <div class="sm:col-span-3">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="max-uses-total">Usos máximos totales</label>
                <input class="form-input mt-1 block w-full sm:text-sm border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:border-primary focus:ring-primary" id="max-uses-total" name="max-uses-total" placeholder="Ilimitado" type="number"/>
              </div>
              <div class="sm:col-span-3">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="max-uses-user">Usos máximos por usuario</label>
                <input class="form-input mt-1 block w-full sm:text-sm border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:border-primary focus:ring-primary" id="max-uses-user" name="max-uses-user" type="number" value="1"/>
              </div>
              <div class="sm:col-span-6 space-y-4 pt-2">
                <div class="relative flex items-start">
                  <div class="flex items-center h-5">
                    <input class="form-checkbox h-4 w-4 text-primary border-gray-300 rounded focus:ring-primary" id="new-users" name="new-users" type="checkbox"/>
                  </div>
                  <div class="ml-3 text-sm">
                    <label class="font-medium text-gray-700 dark:text-gray-300" for="new-users">Solo para nuevos usuarios</label>
                  </div>
                </div>
                <div class="relative flex items-start">
                  <div class="flex items-center h-5">
                    <input class="form-checkbox h-4 w-4 text-primary border-gray-300 rounded focus:ring-primary" id="first-purchase" name="first-purchase" type="checkbox"/>
                  </div>
                  <div class="ml-3 text-sm">
                    <label class="font-medium text-gray-700 dark:text-gray-300" for="first-purchase">Solo para la primera compra
                    </label></div>
                </div>
              </div>
            </div>
          </div>
          <!-- Validity Section -->
          <div class="pt-6">
            <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] pb-4">Vigencia</h3>
            <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
              <div class="sm:col-span-3">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="start-date">Fecha y hora de inicio</label>
                <input class="form-input mt-1 block w-full sm:text-sm border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:border-primary focus:ring-primary" id="start-date" name="start-date" type="datetime-local"/>
              </div>
              <div class="sm:col-span-3">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="end-date">Fecha y hora de fin</label>
                <input class="form-input mt-1 block w-full sm:text-sm border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:border-primary focus:ring-primary" id="end-date" name="end-date" type="datetime-local"/>
              </div>
              <div class="sm:col-span-6">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Presets rápidos</label>
                <div class="mt-2 flex items-center space-x-2">
                  <button class="px-2.5 py-1.5 border border-gray-300 dark:border-gray-600 shadow-sm text-xs font-medium rounded text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-1 focus:ring-primary" type="button">1 semana</button>
                  <button class="px-2.5 py-1.5 border border-gray-300 dark:border-gray-600 shadow-sm text-xs font-medium rounded text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-1 focus:ring-primary" type="button">1 mes</button>
                  <button class="px-2.5 py-1.5 border border-gray-300 dark:border-gray-600 shadow-sm text-xs font-medium rounded text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-1 focus:ring-primary" type="button">3 meses</button>
                </div>
              </div>
            </div>
          </div>
          <!-- Status Toggle -->
          <div class="pt-6">
            <div class="flex items-center justify-between">
              <div>
                <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em]">Estado</h3>
                <p class="text-sm text-gray-500 dark:text-gray-400">Si está inactivo, el cupón no podrá ser utilizado.</p>
              </div>
              <button aria-checked="true" class="bg-primary relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2" role="switch" type="button">
                <span class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out translate-x-5"></span>
              </button>
            </div>
          </div>
        </div>
      </div>


      <!-- Actions -->
      <div class="flex-shrink-0 border-t border-gray-200 dark:border-gray-700 px-4 py-5 sm:px-6 sticky bottom-0 bg-white/80 dark:bg-[#18212a]/80 backdrop-blur-sm">
        <div class="flex justify-end space-x-3">
          <button class="rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 py-2 px-4 text-sm font-medium text-gray-700 dark:text-gray-300 shadow-sm hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2" type="button" on:click={closeCreateSidebar}>Cancelar</button>
          <button class="inline-flex justify-center rounded-md border border-transparent bg-primary py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2" type="submit">Guardar Cupón</button>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Coupon Details Sidebar Overlay -->
{#if isDetailsSidebarOpen}
  <div class="fixed inset-0 bg-black/50 z-40 transition-opacity" on:click={closeDetailsSidebar} on:keydown={(e) => e.key === 'Escape' && closeDetailsSidebar()} role="button" tabindex="0"></div>
{/if}

<!-- Coupon Details Sidebar -->
<div class="fixed inset-y-0 right-0 z-50 flex h-full max-h-screen w-full max-w-2xl transform transition-transform duration-300 ease-in-out {isDetailsSidebarOpen ? 'translate-x-0' : 'translate-x-full'}">
  <div class="relative flex h-full w-full flex-col overflow-y-auto border-l border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 shadow-xl">
    <header class="sticky top-0 z-10 flex items-center justify-between border-b border-slate-200 dark:border-slate-800 bg-white/75 dark:bg-slate-900/75 px-6 py-4 backdrop-blur-sm">
      <div class="flex items-center gap-3">
        <div class="bg-center bg-no-repeat aspect-square bg-cover rounded-full size-10" data-alt="Abstract gradient with blue and purple tones" style='background-image: url("https://lh3.googleusercontent.com/aida-public/AB6AXuAKy7o-j0Gc29GqrXlNRmdb7kPHtlamRGnhORJJDJueeYkNnwd6q16Dz5N0tc1qMvAyLGVs2epr2w8eQWF-xfSGoMiewI86nvtvplDu4z4J-nX3So6omVi0sjLUCaXD9aZdryPo128kiBTVUNr5kx4N8nGewNwtUFgq-MhUEmzWZR23CRTWgbIR6TAAtFKHenzlv3gcygeO1h1HgP_udchn9rHng7O8RmYdGZ7zJ4G3dpHMXZ53s1aFrR_ipoSszAePmSdAzRrjSaE");'></div>
        <div class="flex flex-col">
          <h1 class="text-slate-900 dark:text-white text-base font-medium leading-normal">CYBERPC25</h1>
          <p class="text-green-600 dark:text-green-400 text-sm font-normal leading-normal">Activo</p>
        </div>
      </div>
      <div class="flex items-center gap-4">
        <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em]">
          <span class="truncate">Editar Cupón</span>
        </button>
        <button class="text-slate-500 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-full p-2" on:click={closeDetailsSidebar}>
          <span class="material-symbols-outlined">close</span>
        </button>
      </div>
    </header>
    <div class="flex-1 p-6">
      <div class="flex flex-col gap-8">
        <!-- Navigation -->
        <div class="border-b border-slate-200 dark:border-slate-800">
          <nav aria-label="Tabs" class="flex -mb-px gap-6">
            <button class="flex items-center gap-2 px-1 py-3 text-sm font-medium border-b-2 {activeTab === 'info' ? 'border-primary text-primary' : 'border-transparent text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:border-slate-300 dark:hover:border-slate-700'}" on:click={() => activeTab = 'info'}>
              <span class="material-symbols-outlined !text-xl" style="font-variation-settings: 'FILL' {activeTab === 'info' ? 1 : 0};">info</span>
              Información General
            </button>
            <button class="flex items-center gap-2 px-1 py-3 text-sm font-medium border-b-2 {activeTab === 'stats' ? 'border-primary text-primary' : 'border-transparent text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:border-slate-300 dark:hover:border-slate-700'}" on:click={() => activeTab = 'stats'}>
              <span class="material-symbols-outlined !text-xl" style="font-variation-settings: 'FILL' {activeTab === 'stats' ? 1 : 0};">bar_chart</span>
              Estadísticas
            </button>
            <button class="flex items-center gap-2 px-1 py-3 text-sm font-medium border-b-2 {activeTab === 'history' ? 'border-primary text-primary' : 'border-transparent text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:border-slate-300 dark:hover:border-slate-700'}" on:click={() => activeTab = 'history'}>
              <span class="material-symbols-outlined !text-xl" style="font-variation-settings: 'FILL' {activeTab === 'history' ? 1 : 0};">history</span>
              Actividad
            </button>
            <button class="flex items-center gap-2 px-1 py-3 text-sm font-medium border-b-2 {activeTab === 'settings' ? 'border-primary text-primary' : 'border-transparent text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:border-slate-300 dark:hover:border-slate-700'}" on:click={() => activeTab = 'settings'}>
              <span class="material-symbols-outlined !text-xl" style="font-variation-settings: 'FILL' {activeTab === 'settings' ? 1 : 0};">settings</span>
              Configuración
            </button>
          </nav>
        </div>
        <!-- Information Section -->
        {#if activeTab === 'info'}
        <section class="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <div class="lg:col-span-2">
            <div class="grid grid-cols-[30%_1fr] gap-x-6 text-sm">
              <div class="col-span-2 grid grid-cols-subgrid border-b border-b-slate-200 dark:border-b-slate-800 py-4">
                <p class="text-slate-500 dark:text-slate-400 font-normal leading-normal">Código</p>
                <p class="text-slate-900 dark:text-slate-50 font-normal leading-normal">CYBERPC25</p>
              </div>
              <div class="col-span-2 grid grid-cols-subgrid border-b border-b-slate-200 dark:border-b-slate-800 py-4">
                <p class="text-slate-500 dark:text-slate-400 font-normal leading-normal">Nombre</p>
                <p class="text-slate-900 dark:text-slate-50 font-normal leading-normal">Cupón Cyber Week</p>
              </div>
              <div class="col-span-2 grid grid-cols-subgrid border-b border-b-slate-200 dark:border-b-slate-800 py-4">
                <p class="text-slate-500 dark:text-slate-400 font-normal leading-normal">Descripción</p>
                <p class="text-slate-900 dark:text-slate-50 font-normal leading-normal">25% de descuento en componentes seleccionados.</p>
              </div>
              <div class="col-span-2 grid grid-cols-subgrid border-b border-b-slate-200 dark:border-b-slate-800 py-4">
                <p class="text-slate-500 dark:text-slate-400 font-normal leading-normal">Tipo</p>
                <p class="text-slate-900 dark:text-slate-50 font-normal leading-normal">Porcentaje</p>
              </div>
              <div class="col-span-2 grid grid-cols-subgrid border-b border-b-slate-200 dark:border-b-slate-800 py-4">
                <p class="text-slate-500 dark:text-slate-400 font-normal leading-normal">Valor</p>
                <p class="text-slate-900 dark:text-slate-50 font-normal leading-normal">25%</p>
              </div>
              <div class="col-span-2 grid grid-cols-subgrid border-b border-b-slate-200 dark:border-b-slate-800 py-4">
                <p class="text-slate-500 dark:text-slate-400 font-normal leading-normal">Aplicabilidad</p>
                <p class="text-slate-900 dark:text-slate-50 font-normal leading-normal">Productos seleccionados</p>
              </div>
              <div class="col-span-2 grid grid-cols-subgrid border-b border-b-slate-200 dark:border-b-slate-800 py-4">
                <p class="text-slate-500 dark:text-slate-400 font-normal leading-normal">Restricciones</p>
                <p class="text-slate-900 dark:text-slate-50 font-normal leading-normal">Uso único por cliente</p>
              </div>
              <div class="col-span-2 grid grid-cols-subgrid py-4">
                <p class="text-slate-500 dark:text-slate-400 font-normal leading-normal">Vigencia</p>
                <p class="text-slate-900 dark:text-slate-50 font-normal leading-normal">01/11/2024 - 07/11/2024</p>
              </div>
            </div>
          </div>
          <div class="lg:col-span-1">
            <div class="flex flex-col gap-4">
              <h3 class="text-slate-900 dark:text-white text-base font-bold leading-tight">Vista Previa del Cupón</h3>
              <div class="flex flex-col items-stretch justify-start rounded-lg shadow-[0_0_4px_rgba(0,0,0,0.1)] bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700">
                <div class="w-full bg-center bg-no-repeat aspect-video bg-cover rounded-t-lg" data-alt="Promotional image for PC components with a blue color scheme" style='background-image: url("https://lh3.googleusercontent.com/aida-public/AB6AXuApVkzc24ObnT-DZYQ1O6n9mRX_uc0bDaymtjmfE7q00HhFDsoBAbeSkWt2zYaGVoZnWFazUivJ5XEhJAY5LMbbWSDCnEblwVGuJ0F8mfY6dMLpE9_1i_hHb6m_6P5EoiCSMEoN3YpnsRtwwYZdIjRYcHdvvuSe00qUtit5cNzlISeDLFU6jh--fDZ_E-Se-Mw5f0nyb-eZ1B80ug7UvQX2gG2vCT2eWTOENA4_G60nEPYQy6PQr1iJAonrQXBNRjq88Lt3c8E6DM8");'></div>
                <div class="flex w-full min-w-0 grow flex-col items-stretch justify-center gap-2 p-4">
                  <p class="text-slate-900 dark:text-white text-lg font-bold leading-tight tracking-[-0.015em]">25% OFF en Componentes</p>
                  <div class="flex flex-col gap-3">
                    <div class="flex flex-col gap-1">
                      <p class="text-slate-500 dark:text-slate-400 text-sm font-normal leading-normal">Usa el código CYBERPC25 al finalizar tu compra.</p>
                      <p class="text-slate-500 dark:text-slate-400 text-sm font-normal leading-normal">Válido hasta el 07/11/2024</p>
                    </div>
                    <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-8 px-4 bg-primary/20 text-primary text-sm font-medium leading-normal">
                      <span class="truncate">Copiar Código</span>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>
        <!-- Stats Section (Original) -->
        <section>
          <h3 class="text-slate-900 dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] pb-4">Estadísticas</h3>
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
            <div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 p-4">
              <p class="text-sm text-slate-500 dark:text-slate-400">Usos</p>
              <p class="text-2xl font-bold text-slate-900 dark:text-white">850 / 1000</p>
              <p class="text-xs text-slate-400 dark:text-slate-500">Totales vs. Disponibles</p>
            </div>
            <div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 p-4">
              <p class="text-sm text-slate-500 dark:text-slate-400">Descuento Total</p>
              <p class="text-2xl font-bold text-slate-900 dark:text-white">$12,750.00</p>
              <p class="text-xs text-slate-400 dark:text-slate-500">Monto otorgado</p>
            </div>
            <div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 p-4">
              <p class="text-sm text-slate-500 dark:text-slate-400">Conversión</p>
              <p class="text-2xl font-bold text-slate-900 dark:text-white">92.5%</p>
              <p class="text-xs text-slate-400 dark:text-slate-500">Usos que generaron venta</p>
            </div>
            <div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 p-4">
              <p class="text-sm text-slate-500 dark:text-slate-400">Valor Promedio</p>
              <p class="text-2xl font-bold text-slate-900 dark:text-white">$15.00</p>
              <p class="text-xs text-slate-400 dark:text-slate-500">Descuento por uso</p>
            </div>
          </div>
        </section>
        {:else if activeTab === 'stats'}
        <section class="flex flex-col gap-6">
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
            <div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-slate-800/50 p-4">
              <p class="text-sm text-slate-500 dark:text-slate-400">Descuento total otorgado</p>
              <p class="text-2xl font-bold text-slate-900 dark:text-white">$12,750.00</p>
            </div>
            <div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-slate-800/50 p-4">
              <p class="text-sm text-slate-500 dark:text-slate-400">Conversión</p>
              <p class="text-2xl font-bold text-slate-900 dark:text-white">92.5%</p>
              <p class="text-xs text-slate-400 dark:text-slate-500">% de usos que generaron venta</p>
            </div>
          </div>
          <div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-slate-800/50 p-4 flex flex-col gap-3">
            <div class="flex justify-between items-baseline">
              <h3 class="text-base font-medium text-slate-900 dark:text-white">Usos totales vs disponibles</h3>
              <p class="text-sm font-bold text-slate-900 dark:text-white">850 <span class="font-normal text-slate-500 dark:text-slate-400">/ 1000</span></p>
            </div>
            <div class="w-full bg-slate-200 dark:bg-slate-700 rounded-full h-2.5">
              <div class="bg-primary h-2.5 rounded-full" style="width: 85%"></div>
            </div>
            <p class="text-sm text-slate-500 dark:text-slate-400">Quedan 150 usos disponibles.</p>
          </div>
          <div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-slate-800/50 p-4 flex flex-col gap-4">
            <h3 class="text-base font-medium text-slate-900 dark:text-white">Top 10 Usuarios</h3>
            <div class="flex flex-col gap-3">
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">1.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">Ana García</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 100%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">12</p>
                </div>
              </div>
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">2.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">Carlos Rodriguez</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 91.6%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">11</p>
                </div>
              </div>
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">3.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">Luisa Martínez</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 75%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">9</p>
                </div>
              </div>
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">4.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">Javier López</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 66.6%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">8</p>
                </div>
              </div>
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">5.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">Sofía Hernández</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 58.3%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">7</p>
                </div>
              </div>
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">6.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">user_84321</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 50%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">6</p>
                </div>
              </div>
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">7.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">Pedro Pérez</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 41.6%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">5</p>
                </div>
              </div>
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">8.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">user_55678</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 41.6%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">5</p>
                </div>
              </div>
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">9.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">Laura Gómez</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 33.3%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">4</p>
                </div>
              </div>
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">10.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">user_99012</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 33.3%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">4</p>
                </div>
              </div>
            </div>
          </div>
        </section>
        {:else if activeTab === 'history'}
        <section class="flex flex-col gap-8">
          <div>
            <h3 class="text-slate-900 dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] mb-4">Historial de usos recientes</h3>
            <div class="overflow-x-auto rounded-lg border border-slate-200 dark:border-slate-800">
              <table class="w-full text-sm text-left text-slate-500 dark:text-slate-400">
                <thead class="text-xs text-slate-700 dark:text-slate-300 uppercase bg-slate-50 dark:bg-slate-800">
                  <tr>
                    <th class="px-6 py-3" scope="col">Fecha / Hora</th>
                    <th class="px-6 py-3" scope="col">ID Usuario</th>
                    <th class="px-6 py-3" scope="col">ID Pedido</th>
                    <th class="px-6 py-3" scope="col">Descuento</th>
                    <th class="px-6 py-3" scope="col">Estado Pedido</th>
                  </tr>
                </thead>
                <tbody>
                  <tr class="bg-white dark:bg-slate-900 border-b dark:border-slate-800">
                    <td class="px-6 py-4">07/11/2024, 14:30</td>
                    <td class="px-6 py-4 font-medium text-slate-900 dark:text-white">USR-G5H9</td>
                    <td class="px-6 py-4">ORD-K2L4</td>
                    <td class="px-6 py-4">$25.50</td>
                    <td class="px-6 py-4">
                      <span class="inline-flex items-center px-2 py-1 text-xs font-medium text-green-700 bg-green-100 rounded-full dark:bg-green-900 dark:text-green-300">Completado</span>
                    </td>
                  </tr>
                  <tr class="bg-slate-50 dark:bg-slate-900/50 border-b dark:border-slate-800">
                    <td class="px-6 py-4">07/11/2024, 11:15</td>
                    <td class="px-6 py-4 font-medium text-slate-900 dark:text-white">USR-J8P1</td>
                    <td class="px-6 py-4">ORD-B7N3</td>
                    <td class="px-6 py-4">$18.75</td>
                    <td class="px-6 py-4">
                      <span class="inline-flex items-center px-2 py-1 text-xs font-medium text-green-700 bg-green-100 rounded-full dark:bg-green-900 dark:text-green-300">Completado</span>
                    </td>
                  </tr>
                  <tr class="bg-white dark:bg-slate-900 border-b dark:border-slate-800">
                    <td class="px-6 py-4">06/11/2024, 20:45</td>
                    <td class="px-6 py-4 font-medium text-slate-900 dark:text-white">USR-M4C6</td>
                    <td class="px-6 py-4">ORD-X9V5</td>
                    <td class="px-6 py-4">$32.00</td>
                    <td class="px-6 py-4">
                      <span class="inline-flex items-center px-2 py-1 text-xs font-medium text-yellow-700 bg-yellow-100 rounded-full dark:bg-yellow-900 dark:text-yellow-300">Pendiente</span>
                    </td>
                  </tr>
                  <tr class="bg-slate-50 dark:bg-slate-900/50 border-b dark:border-slate-800">
                    <td class="px-6 py-4">06/11/2024, 18:02</td>
                    <td class="px-6 py-4 font-medium text-slate-900 dark:text-white">USR-F2R9</td>
                    <td class="px-6 py-4">ORD-Z1T8</td>
                    <td class="px-6 py-4">$15.20</td>
                    <td class="px-6 py-4">
                      <span class="inline-flex items-center px-2 py-1 text-xs font-medium text-red-700 bg-red-100 rounded-full dark:bg-red-900 dark:text-red-300">Cancelado</span>
                    </td>
                  </tr>
                  <tr class="bg-white dark:bg-slate-900">
                    <td class="px-6 py-4">05/11/2024, 09:55</td>
                    <td class="px-6 py-4 font-medium text-slate-900 dark:text-white">USR-A3S7</td>
                    <td class="px-6 py-4">ORD-Q6W2</td>
                    <td class="px-6 py-4">$45.10</td>
                    <td class="px-6 py-4">
                      <span class="inline-flex items-center px-2 py-1 text-xs font-medium text-green-700 bg-green-100 rounded-full dark:bg-green-900 dark:text-green-300">Completado</span>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
          <div>
            <h3 class="text-slate-900 dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] mb-4">Timeline de eventos</h3>
            <div class="relative pl-6 border-l-2 border-slate-200 dark:border-slate-700">
              <div class="mb-8">
                <div class="absolute w-4 h-4 bg-primary rounded-full -left-[9px] mt-1.5 border-2 border-white dark:border-slate-900"></div>
                <time class="mb-1 text-xs font-normal text-slate-400 dark:text-slate-500">07/11/2024, 15:00</time>
                <p class="text-sm font-medium text-slate-900 dark:text-white">Cupón desactivado</p>
                <p class="text-sm text-slate-500 dark:text-slate-400">Desactivado por <span class="font-semibold text-slate-700 dark:text-slate-300">Admin</span>. Razón: Fin de la promoción.</p>
              </div>
              <div class="mb-8">
                <div class="absolute w-4 h-4 bg-primary/50 rounded-full -left-[9px] mt-1.5 border-2 border-white dark:border-slate-900"></div>
                <time class="mb-1 text-xs font-normal text-slate-400 dark:text-slate-500">05/11/2024, 10:00</time>
                <p class="text-sm font-medium text-slate-900 dark:text-white">Uso N° 500 alcanzado</p>
                <p class="text-sm text-slate-500 dark:text-slate-400">El cupón ha sido utilizado 500 veces.</p>
              </div>
              <div class="mb-8">
                <div class="absolute w-4 h-4 bg-primary/50 rounded-full -left-[9px] mt-1.5 border-2 border-white dark:border-slate-900"></div>
                <time class="mb-1 text-xs font-normal text-slate-400 dark:text-slate-500">01/11/2024, 09:00</time>
                <p class="text-sm font-medium text-slate-900 dark:text-white">Cupón activado</p>
                <p class="text-sm text-slate-500 dark:text-slate-400">Activado por <span class="font-semibold text-slate-700 dark:text-slate-300">Admin</span>.</p>
              </div>
              <div class="mb-8">
                <div class="absolute w-4 h-4 bg-primary/50 rounded-full -left-[9px] mt-1.5 border-2 border-white dark:border-slate-900"></div>
                <time class="mb-1 text-xs font-normal text-slate-400 dark:text-slate-500">31/10/2024, 18:30</time>
                <p class="text-sm font-medium text-slate-900 dark:text-white">Cupón editado</p>
                <p class="text-sm text-slate-500 dark:text-slate-400">Límite de uso cambiado a 1000 por <span class="font-semibold text-slate-700 dark:text-slate-300">Admin</span>.</p>
              </div>
              <div>
                <div class="absolute w-4 h-4 bg-primary/50 rounded-full -left-[9px] mt-1.5 border-2 border-white dark:border-slate-900"></div>
                <time class="mb-1 text-xs font-normal text-slate-400 dark:text-slate-500">30/10/2024, 12:00</time>
                <p class="text-sm font-medium text-slate-900 dark:text-white">Cupón creado</p>
                <p class="text-sm text-slate-500 dark:text-slate-400">Creado por <span class="font-semibold text-slate-700 dark:text-slate-300">Admin</span>.</p>
              </div>
            </div>
          </div>
        </section>
        {:else if activeTab === 'settings'}
        <form class="space-y-6">
          <div class="space-y-2">
            <label class="text-sm font-medium text-slate-700 dark:text-slate-300" for="coupon-name">Nombre del Cupón</label>
            <input class="w-full rounded-md border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-50 placeholder:text-slate-400 dark:placeholder:text-slate-500 focus:border-primary focus:ring-primary" id="coupon-name" placeholder="Ej: Descuento de Verano" type="text" value="Cupón Cyber Week"/>
          </div>
          <div class="space-y-2">
            <label class="text-sm font-medium text-slate-700 dark:text-slate-300" for="coupon-description">Descripción</label>
            <textarea class="w-full rounded-md border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-50 placeholder:text-slate-400 dark:placeholder:text-slate-500 focus:border-primary focus:ring-primary" id="coupon-description" placeholder="Breve descripción del propósito del cupón" rows="3">25% de descuento en componentes seleccionados.</textarea>
          </div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-2">
              <label class="text-sm font-medium text-slate-700 dark:text-slate-300" for="coupon-type">Tipo de Cupón</label>
              <select class="w-full rounded-md border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-50 focus:border-primary focus:ring-primary" id="coupon-type">
                <option>Monto Fijo</option>
                <option selected="">Porcentaje</option>
                <option>Envío Gratis</option>
              </select>
            </div>
            <div class="space-y-2">
              <label class="text-sm font-medium text-slate-700 dark:text-slate-300" for="coupon-value">Valor</label>
              <div class="relative">
                <input class="w-full rounded-md border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-50 placeholder:text-slate-400 dark:placeholder:text-slate-500 focus:border-primary focus:ring-primary pl-7" id="coupon-value" placeholder="25" type="number" value="25"/>
                <span class="absolute left-2.5 top-1/2 -translate-y-1/2 text-slate-400 dark:text-slate-500">%</span>
              </div>
            </div>
          </div>
          <div class="space-y-2">
            <label class="text-sm font-medium text-slate-700 dark:text-slate-300" for="coupon-applicability">Aplicabilidad</label>
            <select class="w-full rounded-md border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-50 focus:border-primary focus:ring-primary" id="coupon-applicability">
              <option>Todo el pedido</option>
              <option selected="">Productos seleccionados</option>
              <option>Categorías seleccionadas</option>
            </select>
          </div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-2">
              <label class="text-sm font-medium text-slate-700 dark:text-slate-300" for="coupon-restrictions">Restricciones de Uso</label>
              <select class="w-full rounded-md border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-50 focus:border-primary focus:ring-primary" id="coupon-restrictions">
                <option>Sin límite</option>
                <option selected="">Uso único por cliente</option>
                <option>Límite de usos totales</option>
              </select>
            </div>
            <div class="space-y-2">
              <label class="text-sm font-medium text-slate-700 dark:text-slate-300" for="usage-limit">Límite de usos totales</label>
              <input class="w-full rounded-md border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-50 placeholder:text-slate-400 dark:placeholder:text-slate-500 focus:border-primary focus:ring-primary" id="usage-limit" placeholder="1000" type="number" value="1000"/>
            </div>
          </div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-2">
              <label class="text-sm font-medium text-slate-700 dark:text-slate-300" for="start-date">Fecha de Inicio</label>
              <input class="w-full rounded-md border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-50 focus:border-primary focus:ring-primary" id="start-date" type="date" value="2024-11-01"/>
            </div>
            <div class="space-y-2">
              <label class="text-sm font-medium text-slate-700 dark:text-slate-300" for="end-date">Fecha de Fin</label>
              <input class="w-full rounded-md border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-50 focus:border-primary focus:ring-primary" id="end-date" type="date" value="2024-11-07"/>
            </div>
          </div>
          <div class="flex items-center space-x-3 pt-2">
            <label class="relative inline-flex items-center cursor-pointer"><input checked="" class="sr-only peer" type="checkbox" value=""/>
              <div class="w-11 h-6 bg-slate-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-primary/30 dark:peer-focus:ring-primary/80 rounded-full peer dark:bg-slate-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-slate-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-slate-600 peer-checked:bg-primary"></div>
            </label>
            <span class="text-sm font-medium text-slate-700 dark:text-slate-300">Cupón Activo</span>
          </div>
          <div class="flex justify-end items-center gap-4 pt-4 border-t border-slate-200 dark:border-slate-800">
            <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-transparent text-slate-700 dark:text-slate-200 text-sm font-bold leading-normal tracking-[0.015em] border border-slate-300 dark:border-slate-700 hover:bg-slate-100 dark:hover:bg-slate-800" type="button">
              <span class="truncate">Cancelar</span>
            </button>
            <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em] hover:bg-primary/90" type="submit">
              <span class="truncate">Guardar Cambios</span>
            </button>
          </div>
        </form>
        {/if}
      </div>
    </div>
  </div>
</div>


{#if isDetailsSidebarOpen}
  <div class="fixed inset-0 bg-black/50 z-40 transition-opacity" on:click={closeDetailsSidebar} on:keydown={(e) => e.key === 'Escape' && closeDetailsSidebar()} role="button" tabindex="0"></div>
{/if}

<!-- Details Sidebar -->
<div class="fixed inset-y-0 right-0 flex max-w-full z-50 transform transition-transform duration-300 ease-in-out {isDetailsSidebarOpen ? 'translate-x-0' : 'translate-x-full'}">
  <div class="relative w-screen max-w-2xl">
    <div class="flex h-full flex-col overflow-y-scroll bg-white dark:bg-slate-900 shadow-xl">
      <div class="flex-1">
        <!-- Header -->
        <div class="bg-white dark:bg-slate-900 px-6 py-6 sticky top-0 z-10 border-b border-slate-200 dark:border-slate-800">
          <div class="flex items-start justify-between">
            <div>
              <h2 class="text-2xl font-bold text-slate-900 dark:text-white">Detalles del Cupón</h2>
              {#if selectedCupon}
                <p class="mt-1 text-sm text-slate-500 dark:text-slate-400">Código: <span class="font-mono font-semibold">{selectedCupon.codigo}</span></p>
              {/if}
            </div>
            <button on:click={closeDetailsSidebar} class="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg">
              <span class="material-symbols-outlined">close</span>
            </button>
          </div>
        </div>

        {#if selectedCupon}
          <!-- Content -->
          <div class="px-6 py-6 space-y-6">
            <!-- Basic Info -->
            <div class="bg-slate-50 dark:bg-slate-800 rounded-lg p-4">
              <h3 class="text-lg font-semibold text-slate-900 dark:text-white mb-4">Información General</h3>
              <dl class="space-y-3">
                <div>
                  <dt class="text-sm font-medium text-slate-500 dark:text-slate-400">Nombre</dt>
                  <dd class="mt-1 text-sm text-slate-900 dark:text-white">{selectedCupon.nombre}</dd>
                </div>
                <div>
                  <dt class="text-sm font-medium text-slate-500 dark:text-slate-400">Tipo</dt>
                  <dd class="mt-1">
                    <span class="{getTipoBadgeClass(selectedCupon.tipo_cupon)} text-xs font-medium px-2.5 py-0.5 rounded-full">
                      {getTipoLabel(selectedCupon.tipo_cupon)}
                    </span>
                  </dd>
                </div>
                <div>
                  <dt class="text-sm font-medium text-slate-500 dark:text-slate-400">Valor</dt>
                  <dd class="mt-1 text-sm text-slate-900 dark:text-white font-semibold">{formatValor(selectedCupon.tipo_cupon, selectedCupon.valor)}</dd>
                </div>
                <div>
                  <dt class="text-sm font-medium text-slate-500 dark:text-slate-400">Vigencia</dt>
                  <dd class="mt-1 text-sm text-slate-900 dark:text-white">{formatDate(selectedCupon.fecha_inicio)} - {formatDate(selectedCupon.fecha_fin)}</dd>
                </div>
                <div>
                  <dt class="text-sm font-medium text-slate-500 dark:text-slate-400">Estado</dt>
                  <dd class="mt-1">
                    <span class="{getEstadoBadgeClass(getEstadoCupon(selectedCupon))} text-xs font-medium px-2.5 py-0.5 rounded-full">
                      {getEstadoLabel(getEstadoCupon(selectedCupon))}
                    </span>
                  </dd>
                </div>
              </dl>
            </div>

            <!-- Usage Stats -->
            <div class="bg-slate-50 dark:bg-slate-800 rounded-lg p-4">
              <h3 class="text-lg font-semibold text-slate-900 dark:text-white mb-4">Estadísticas de Uso</h3>
              <dl class="space-y-3">
                <div>
                  <dt class="text-sm font-medium text-slate-500 dark:text-slate-400">Usos Actuales</dt>
                  <dd class="mt-1 text-2xl font-bold text-slate-900 dark:text-white">{selectedCupon.usos_actuales || 0}</dd>
                </div>
                {#if selectedCupon.usos_maximos_totales}
                  <div>
                    <dt class="text-sm font-medium text-slate-500 dark:text-slate-400">Usos Máximos</dt>
                    <dd class="mt-1 text-sm text-slate-900 dark:text-white">{selectedCupon.usos_maximos_totales}</dd>
                  </div>
                  <div>
                    <dt class="text-sm font-medium text-slate-500 dark:text-slate-400">Progreso</dt>
                    <dd class="mt-2">
                      <div class="w-full h-3 bg-slate-200 dark:bg-slate-700 rounded-full">
                        <div class="h-3 bg-primary rounded-full" style="width: {(selectedCupon.usos_actuales / selectedCupon.usos_maximos_totales * 100)}%"></div>
                      </div>
                      <p class="mt-1 text-xs text-slate-500 dark:text-slate-400">
                        {Math.round((selectedCupon.usos_actuales / selectedCupon.usos_maximos_totales * 100))}% utilizado
                      </p>
                    </dd>
                  </div>
                {/if}
              </dl>
            </div>

            <!-- Assigned Users -->
            <div class="bg-slate-50 dark:bg-slate-800 rounded-lg p-4">
              <h3 class="text-lg font-semibold text-slate-900 dark:text-white mb-4">Usuarios Asignados</h3>
              
              {#if loadingAssignedUsers}
                <div class="flex items-center justify-center py-8">
                  <span class="material-symbols-outlined animate-spin text-primary text-3xl">progress_activity</span>
                </div>
              {:else if assignedUsuarios.length > 0}
                <div class="space-y-2 max-h-64 overflow-y-auto">
                  {#each assignedUsuarios as usuario}
                    <div class="flex items-center gap-3 p-2 bg-white dark:bg-slate-900 rounded-lg">
                      <span class="material-symbols-outlined text-primary text-xl">person</span>
                      <div class="flex-1">
                        <p class="text-sm font-medium text-slate-900 dark:text-white">{usuario.nombre}</p>
                        <p class="text-xs text-slate-500 dark:text-slate-400">{usuario.email}</p>
                      </div>
                    </div>
                  {/each}
                </div>
                <p class="text-xs text-slate-500 dark:text-slate-400 mt-3">
                  Total: {assignedUsuarios.length} usuario{assignedUsuarios.length !== 1 ? 's' : ''}
                </p>
              {:else}
                <p class="text-sm text-slate-500 dark:text-slate-400">
                  Este cupón está disponible para todos los usuarios que cumplan con las restricciones configuradas.
                </p>
              {/if}
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>

<!-- Mass Assignment Modal -->
{#if isMassAssignModalOpen}
  <div class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4" on:click={closeMassAssignModal} on:keydown={(e) => e.key === 'Escape' && closeMassAssignModal()} role="button" tabindex="0">
    <div class="bg-white dark:bg-slate-900 rounded-xl shadow-xl max-w-4xl w-full max-h-[90vh] overflow-hidden" on:click|stopPropagation>
      <!-- Header -->
      <div class="px-6 py-4 border-b border-slate-200 dark:border-slate-800">
        <div class="flex items-center justify-between">
          <h2 class="text-2xl font-bold text-slate-900 dark:text-white">Asignación Masiva de Cupones</h2>
          <button on:click={closeMassAssignModal} class="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg">
            <span class="material-symbols-outlined">close</span>
          </button>
        </div>
        
        <!-- Steps -->
        <div class="flex gap-4 mt-4">
          <button 
            class="flex-1 pb-3 border-b-2 {massAssignStep === 1 ? 'border-primary text-primary' : 'border-transparent text-slate-400'}"
            on:click={() => massAssignStep = 1}
          >
            <span class="font-semibold">1. Selección</span>
          </button>
          <button 
            class="flex-1 pb-3 border-b-2 {massAssignStep === 2 ? 'border-primary text-primary' : 'border-transparent text-slate-400'}"
            disabled={massAssignStep < 2}
          >
            <span class="font-semibold">2. Confirmación</span>
          </button>
          <button 
            class="flex-1 pb-3 border-b-2 {massAssignStep === 3 ? 'border-primary text-primary' : 'border-transparent text-slate-400'}"
            disabled={massAssignStep < 3}
          >
            <span class="font-semibold">3. Resultado</span>
          </button>
        </div>
      </div>

      <!-- Content -->
      <div class="p-6 overflow-y-auto max-h-[calc(90vh-200px)]">
        {#if massAssignStep === 1}
          <!-- Step 1: User Selection -->
          <div class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                Buscar usuarios
              </label>
              <div class="flex gap-2">
                <div class="flex-1 relative">
                  <span class="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-slate-400">search</span>
                  <input
                    bind:value={userSearchQuery}
                    on:input={fetchUsuarios}
                    type="text"
                    placeholder="Buscar por nombre o email..."
                    class="w-full pl-10 pr-4 py-2 border border-slate-300 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white focus:ring-2 focus:ring-primary/50"
                  />
                </div>
                <button on:click={fetchUsuarios} class="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90">
                  Buscar
                </button>
              </div>
            </div>

            <div class="flex gap-2 items-center">
              <button on:click={selectAllUsuarios} class="text-sm text-primary hover:underline">Seleccionar todos</button>
              <span class="text-slate-300 dark:text-slate-700">|</span>
              <button on:click={deselectAllUsuarios} class="text-sm text-primary hover:underline">Deseleccionar todos</button>
              <span class="ml-auto text-sm text-slate-600 dark:text-slate-400">
                {selectedUsuarios.length} usuario{selectedUsuarios.length !== 1 ? 's' : ''} seleccionado{selectedUsuarios.length !== 1 ? 's' : ''}
              </span>
            </div>

            <div class="border border-slate-200 dark:border-slate-800 rounded-lg divide-y divide-slate-200 dark:divide-slate-800 max-h-96 overflow-y-auto">
              {#each usuarios as usuario}
                <label class="flex items-center gap-3 p-3 hover:bg-slate-50 dark:hover:bg-slate-800/50 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={selectedUsuarios.includes(usuario.id_usuario)}
                    on:change={() => toggleUsuarioSelection(usuario.id_usuario)}
                    class="w-5 h-5 text-primary bg-white dark:bg-slate-800 border-slate-300 dark:border-slate-700 rounded focus:ring-2 focus:ring-primary/50"
                  />
                  <div class="flex-1">
                    <p class="font-medium text-slate-900 dark:text-white">{usuario.nombre}</p>
                    <p class="text-sm text-slate-500 dark:text-slate-400">{usuario.email}</p>
                  </div>
                </label>
              {/each}

              {#if usuarios.length === 0}
                <div class="p-8 text-center text-slate-500 dark:text-slate-400">
                  {userSearchQuery ? 'No se encontraron usuarios' : 'Busca usuarios para comenzar'}
                </div>
              {/if}
            </div>
          </div>
        {:else if massAssignStep === 2}
          <!-- Step 2: Confirmation -->
          <div class="space-y-4">
            <!-- Coupon Selection -->
            <div class="border border-slate-200 dark:border-slate-800 rounded-lg p-4">
              <h4 class="font-semibold text-slate-900 dark:text-white mb-3">Selecciona el cupón a asignar:</h4>
              <select 
                bind:value={selectedCupon}
                class="w-full px-4 py-2 border border-slate-300 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white focus:ring-2 focus:ring-primary/50"
              >
                <option value={null}>Selecciona un cupón...</option>
                {#each cupones as cupon}
                  <option value={cupon}>{cupon.codigo} - {cupon.nombre} ({getTipoLabel(cupon.tipo_cupon)}: {formatValor(cupon.tipo_cupon, cupon.valor)})</option>
                {/each}
              </select>
            </div>

            {#if selectedCupon}
              <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
                <h3 class="font-semibold text-blue-900 dark:text-blue-200 mb-2">Confirmar asignación</h3>
                <p class="text-blue-800 dark:text-blue-300 text-sm">
                  Estás a punto de asignar el cupón <span class="font-mono font-semibold">{selectedCupon.codigo}</span> a {selectedUsuarios.length} usuario{selectedUsuarios.length !== 1 ? 's' : ''}.
                </p>
              </div>
            {/if}

            <div class="border border-slate-200 dark:border-slate-800 rounded-lg p-4">
              <h4 class="font-semibold text-slate-900 dark:text-white mb-3">Usuarios seleccionados:</h4>
              <div class="space-y-2 max-h-64 overflow-y-auto">
                {#each usuarios.filter(u => selectedUsuarios.includes(u.id_usuario)) as usuario}
                  <div class="flex items-center gap-2 text-sm">
                    <span class="material-symbols-outlined text-primary text-base">check_circle</span>
                    <span class="text-slate-700 dark:text-slate-300">{usuario.nombre} ({usuario.email})</span>
                  </div>
                {/each}
              </div>
            </div>
          </div>
        {:else if massAssignStep === 3}
          <!-- Step 3: Results -->
          <div class="space-y-4">
            {#if assignmentResult}
              <div class="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg p-6 text-center">
                <span class="material-symbols-outlined text-green-600 dark:text-green-400 text-5xl mb-3">check_circle</span>
                <h3 class="text-xl font-bold text-green-900 dark:text-green-200 mb-2">¡Asignación completada!</h3>
                <div class="grid grid-cols-3 gap-4 mt-4">
                  <div>
                    <p class="text-2xl font-bold text-green-600 dark:text-green-400">{assignmentResult.assigned}</p>
                    <p class="text-sm text-green-800 dark:text-green-300">Asignados</p>
                  </div>
                  <div>
                    <p class="text-2xl font-bold text-slate-600 dark:text-slate-400">{assignmentResult.failed}</p>
                    <p class="text-sm text-slate-700 dark:text-slate-400">Fallidos</p>
                  </div>
                  <div>
                    <p class="text-2xl font-bold text-slate-900 dark:text-white">{assignmentResult.total}</p>
                    <p class="text-sm text-slate-700 dark:text-slate-400">Total</p>
                  </div>
                </div>
              </div>
            {/if}
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div class="px-6 py-4 border-t border-slate-200 dark:border-slate-800 flex justify-between">
        <button
          on:click={closeMassAssignModal}
          class="px-4 py-2 border border-slate-300 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-800"
        >
          {massAssignStep === 3 ? 'Cerrar' : 'Cancelar'}
        </button>

        <div class="flex gap-2">
          {#if massAssignStep === 2}
            <button
              on:click={() => massAssignStep = 1}
              class="px-4 py-2 border border-slate-300 dark:border-slate-700 rounded-lg text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-800"
            >
              Atrás
            </button>
            <button
              on:click={assignCuponToUsers}
              disabled={isAssigning || !selectedCupon}
              class="px-6 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
            >
              {#if isAssigning}
                <span class="material-symbols-outlined animate-spin">progress_activity</span>
              {/if}
              {isAssigning ? 'Asignando...' : 'Confirmar Asignación'}
            </button>
          {:else if massAssignStep === 1}
            <button
              on:click={proceedToConfirmation}
              disabled={selectedUsuarios.length === 0}
              class="px-6 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Continuar
            </button>
          {/if}
        </div>
      </div>
    </div>
  </div>
{/if}
