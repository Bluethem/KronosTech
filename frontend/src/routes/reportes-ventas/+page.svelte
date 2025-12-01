<script>
    import { onMount } from 'svelte';
    import { fade } from 'svelte/transition';
    import { Chart, registerables } from 'chart.js';
    
    Chart.register(...registerables);

    let reportData = {
        stats: {
            total_pedidos: 0,
            ingresos_totales: 0,
            ticket_promedio: 0,
            tasa_conversion: 0
        },
        ventas_por_dia: [],
        ventas_por_estado: [],
        top_productos: [],
        tabla_productos: []
    };

    let loading = true;
    let error = null;
    let fechaInicio = '';
    let fechaFin = '';
    
    let barChartCanvas;
    let pieChartCanvas;
    let barChart;
    let pieChart;

    onMount(async () => {
        // Set default dates (last 30 days)
        const end = new Date();
        const start = new Date();
        start.setDate(end.getDate() - 30);
        
        fechaFin = end.toISOString().split('T')[0];
        fechaInicio = start.toISOString().split('T')[0];

        await fetchReportData();
    });

    async function fetchReportData() {
        loading = true;
        error = null;
        try {
            const queryParams = new URLSearchParams({
                fecha_inicio: new Date(fechaInicio).toISOString(),
                fecha_fin: new Date(fechaFin).toISOString()
            });

            const response = await fetch(`http://localhost:3000/api/reportes/ventas?${queryParams}`);
            if (!response.ok) {
                throw new Error('Error al cargar los datos del reporte');
            }
            reportData = await response.json();
            
            // Update charts after data is loaded
            updateCharts();
        } catch (e) {
            console.error(e);
            error = e.message;
        } finally {
            loading = false;
        }
    }

    function updateCharts() {
        // Destroy existing charts
        if (barChart) barChart.destroy();
        if (pieChart) pieChart.destroy();

        // Bar Chart - Ventas por Día
        if (barChartCanvas && reportData.ventas_por_dia.length > 0) {
            const ctx = barChartCanvas.getContext('2d');
            barChart = new Chart(ctx, {
                type: 'bar',
                data: {
                    labels: reportData.ventas_por_dia.map(d => formatDate(d.fecha)),
                    datasets: [{
                        label: 'Ventas (S/)',
                        data: reportData.ventas_por_dia.map(d => parseFloat(d.total_ventas)),
                        backgroundColor: 'rgba(59, 130, 246, 0.8)',
                        borderColor: 'rgba(59, 130, 246, 1)',
                        borderWidth: 1
                    }]
                },
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    plugins: {
                        legend: {
                            display: false
                        }
                    },
                    scales: {
                        y: {
                            beginAtZero: true,
                            ticks: {
                                callback: function(value) {
                                    return 'S/ ' + value.toLocaleString();
                                }
                            }
                        }
                    }
                }
            });
        }

        // Pie Chart - Ventas por Estado
        if (pieChartCanvas && reportData.ventas_por_estado.length > 0) {
            const ctx = pieChartCanvas.getContext('2d');
            const colors = [
                'rgba(59, 130, 246, 0.8)',
                'rgba(16, 185, 129, 0.8)',
                'rgba(245, 158, 11, 0.8)',
                'rgba(239, 68, 68, 0.8)',
                'rgba(139, 92, 246, 0.8)',
                'rgba(236, 72, 153, 0.8)'
            ];
            
            pieChart = new Chart(ctx, {
                type: 'pie',
                data: {
                    labels: reportData.ventas_por_estado.map(e => e.estado),
                    datasets: [{
                        data: reportData.ventas_por_estado.map(e => e.cantidad),
                        backgroundColor: colors.slice(0, reportData.ventas_por_estado.length),
                        borderWidth: 2,
                        borderColor: '#fff'
                    }]
                },
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    plugins: {
                        legend: {
                            position: 'bottom'
                        }
                    }
                }
            });
        }
    }

    function formatCurrency(amount) {
        return new Intl.NumberFormat('es-PE', { style: 'currency', currency: 'PEN' }).format(amount);
    }

    function formatDate(dateString) {
        return new Date(dateString).toLocaleDateString('es-PE', { day: 'numeric', month: 'short', year: 'numeric' });
    }
</script>

<div class="relative flex h-auto min-h-screen w-full flex-col group/design-root overflow-x-hidden bg-background-light dark:bg-background-dark font-display">
    <div class="layout-container flex h-full grow flex-col">
        <div class="px-4 sm:px-8 md:px-12 lg:px-16 xl:px-20 flex flex-1 justify-center py-5">
            <div class="layout-content-container flex flex-col w-full max-w-7xl flex-1 gap-8">
                <!-- Page Heading -->
                <div class="flex flex-wrap justify-between items-center gap-4">
                    <div class="flex flex-col gap-2">
                        <p class="text-slate-900 dark:text-white text-3xl md:text-4xl font-black leading-tight tracking-[-0.033em]">Reportes de Ventas</p>
                        <p class="text-slate-500 dark:text-slate-400 text-base font-normal leading-normal">Seleccione los filtros para visualizar y exportar los datos de ventas.</p>
                    </div>
                </div>

                <!-- Filters Section -->
                <div class="flex flex-col gap-6 rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 p-6">
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 items-end">
                        <div class="flex flex-col col-span-1 md:col-span-2">
                            <p class="text-slate-900 dark:text-white text-base font-medium leading-normal pb-2">Rango de Fechas</p>
                            <div class="flex items-center border border-slate-200 dark:border-slate-700 rounded-lg">
                                <input 
                                    class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden text-slate-900 dark:text-white focus:outline-0 focus:ring-0 bg-transparent h-14 placeholder:text-slate-500 p-3.5 text-base font-normal leading-normal border-0" 
                                    type="date" 
                                    bind:value={fechaInicio}
                                    on:change={fetchReportData}
                                />
                                <span class="text-slate-400 dark:text-slate-500 px-2">-</span>
                                <input 
                                    class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden text-slate-900 dark:text-white focus:outline-0 focus:ring-0 bg-transparent h-14 placeholder:text-slate-500 p-3.5 text-base font-normal leading-normal border-0 text-right" 
                                    type="date" 
                                    bind:value={fechaFin}
                                    on:change={fetchReportData}
                                />
                            </div>
                        </div>
                        
                        <div class="flex justify-start items-end h-14">
                            <button 
                                on:click={fetchReportData}
                                class="flex min-w-[84px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-12 px-5 bg-primary text-white gap-2 text-base font-bold leading-normal tracking-[0.015em] hover:bg-primary/90 transition-colors"
                            >
                                <span class="material-symbols-outlined text-white" style="font-size: 20px;">refresh</span>
                                <span class="truncate">Actualizar</span>
                            </button>
                        </div>
                    </div>
                </div>

                {#if loading}
                    <div class="flex justify-center items-center h-64">
                        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
                    </div>
                {:else if error}
                    <div class="p-4 mb-4 text-sm text-red-800 rounded-lg bg-red-50 dark:bg-gray-800 dark:text-red-400" role="alert">
                        <span class="font-medium">Error:</span> {error}
                    </div>
                {:else}
                    <!-- Results Section -->
                    <div class="flex flex-col gap-8" transition:fade>
                        <!-- Stats Cards -->
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
                            <div class="flex flex-col gap-2 rounded-xl p-6 border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900">
                                <p class="text-slate-900 dark:text-white text-base font-medium leading-normal">Total de Pedidos</p>
                                <p class="text-slate-900 dark:text-white tracking-light text-3xl font-bold leading-tight">{reportData.stats.total_pedidos}</p>
                            </div>
                            <div class="flex flex-col gap-2 rounded-xl p-6 border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900">
                                <p class="text-slate-900 dark:text-white text-base font-medium leading-normal">Ingresos Totales</p>
                                <p class="text-slate-900 dark:text-white tracking-light text-3xl font-bold leading-tight">{formatCurrency(reportData.stats.ingresos_totales)}</p>
                            </div>
                            <div class="flex flex-col gap-2 rounded-xl p-6 border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900">
                                <p class="text-slate-900 dark:text-white text-base font-medium leading-normal">Ticket Promedio</p>
                                <p class="text-slate-900 dark:text-white tracking-light text-3xl font-bold leading-tight">{formatCurrency(reportData.stats.ticket_promedio)}</p>
                            </div>
                            <div class="flex flex-col gap-2 rounded-xl p-6 border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900">
                                <p class="text-slate-900 dark:text-white text-base font-medium leading-normal">Tasa de Conversión</p>
                                <p class="text-slate-900 dark:text-white tracking-light text-3xl font-bold leading-tight">{reportData.stats.tasa_conversion}%</p>
                            </div>
                        </div>

                        <!-- Charts Section -->
                        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                            <div class="lg:col-span-2 flex flex-col gap-4 rounded-xl p-6 border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900">
                                <p class="text-slate-900 dark:text-white text-lg font-bold">Evolución de Ventas por Día</p>
                                <div class="w-full h-80 rounded-lg flex items-center justify-center p-4">
                                    {#if reportData.ventas_por_dia.length === 0}
                                        <div class="flex items-center justify-center h-full text-slate-500">No hay datos para mostrar</div>
                                    {:else}
                                        <canvas bind:this={barChartCanvas}></canvas>
                                    {/if}
                                </div>
                            </div>
                            <div class="flex flex-col gap-4 rounded-xl p-6 border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900">
                                <p class="text-slate-900 dark:text-white text-lg font-bold">Por Estado</p>
                                <div class="w-full h-80 rounded-lg flex items-center justify-center p-4">
                                    {#if reportData.ventas_por_estado.length === 0}
                                        <div class="flex items-center justify-center h-full text-slate-500">No hay datos</div>
                                    {:else}
                                        <canvas bind:this={pieChartCanvas}></canvas>
                                    {/if}
                                </div>
                            </div>
                        </div>

                        <!-- Data Table -->
                        <div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 overflow-hidden">
                            <div class="p-6">
                                <h3 class="text-lg font-bold text-slate-900 dark:text-white">Detalle de Ventas por Producto</h3>
                            </div>
                            <div class="overflow-x-auto">
                                <table class="w-full text-left">
                                    <thead class="border-b border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-slate-800/50">
                                        <tr>
                                            <th class="p-4 text-sm font-semibold text-slate-600 dark:text-slate-300">Producto</th>
                                            <th class="p-4 text-sm font-semibold text-slate-600 dark:text-slate-300">SKU</th>
                                            <th class="p-4 text-sm font-semibold text-slate-600 dark:text-slate-300">Categoría</th>
                                            <th class="p-4 text-sm font-semibold text-slate-600 dark:text-slate-300">Unidades</th>
                                            <th class="p-4 text-sm font-semibold text-slate-600 dark:text-slate-300">Monto Total</th>
                                        </tr>
                                    </thead>
                                    <tbody class="divide-y divide-slate-200 dark:divide-slate-800">
                                        {#each reportData.tabla_productos as producto}
                                            <tr class="hover:bg-primary/5 dark:hover:bg-primary/10">
                                                <td class="p-4 text-sm text-slate-800 dark:text-slate-200">{producto.nombre_producto}</td>
                                                <td class="p-4 text-sm text-slate-500 dark:text-slate-400">{producto.sku}</td>
                                                <td class="p-4 text-sm text-slate-500 dark:text-slate-400">{producto.categoria}</td>
                                                <td class="p-4 text-sm text-slate-800 dark:text-slate-200">{producto.unidades_vendidas}</td>
                                                <td class="p-4 text-sm text-slate-800 dark:text-slate-200 font-medium">{formatCurrency(producto.monto_total)}</td>
                                            </tr>
                                        {/each}
                                        {#if reportData.tabla_productos.length === 0}
                                            <tr>
                                                <td colspan="5" class="p-4 text-center text-slate-500">No se encontraron ventas en este periodo.</td>
                                            </tr>
                                        {/if}
                                    </tbody>
                                </table>
                            </div>
                        </div>
                    </div>
                {/if}
            </div>
        </div>
    </div>
</div>
