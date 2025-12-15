<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { authUser, isAuthenticated } from '$lib/stores/auth';
	import { FileText, Search, Filter, Download, Calendar, User, Activity, AlertTriangle, Info, CheckCircle, XCircle } from 'lucide-svelte';

	interface LogEntry {
		id: number;
		timestamp: string;
		level: 'info' | 'warning' | 'error' | 'success';
		action: string;
		user: string;
		ip: string;
		details: string;
		module: string;
	}

	let logs: LogEntry[] = [];
	let logsFiltrados: LogEntry[] = [];
	let loading = true;

	// Filtros
	let filtroNivel = '';
	let filtroModulo = '';
	let busqueda = '';
	let fechaInicio = '';
	let fechaFin = '';

	onMount(() => {
		if (!$isAuthenticated) {
			goto('/login?redirect=/admin/logs');
			return;
		}

		if ($authUser?.rol !== 'super_admin') {
			goto('/admin');
			return;
		}

		cargarLogs();
	});

	function cargarLogs() {
		loading = true;

		// Datos simulados para demostración
		logs = [
			{
				id: 1,
				timestamp: new Date(Date.now() - 1000 * 60 * 5).toISOString(),
				level: 'success',
				action: 'Login exitoso',
				user: 'admin@kronostech.com',
				ip: '192.168.1.100',
				details: 'Usuario autenticado correctamente',
				module: 'Autenticación'
			},
			{
				id: 2,
				timestamp: new Date(Date.now() - 1000 * 60 * 15).toISOString(),
				level: 'info',
				action: 'Producto creado',
				user: 'admin@kronostech.com',
				ip: '192.168.1.100',
				details: 'Producto "Laptop Dell XPS 15" creado en el catálogo',
				module: 'Productos'
			},
			{
				id: 3,
				timestamp: new Date(Date.now() - 1000 * 60 * 30).toISOString(),
				level: 'warning',
				action: 'Stock bajo',
				user: 'Sistema',
				ip: '-',
				details: 'Producto "Mouse Logitech" tiene stock bajo (5 unidades)',
				module: 'Inventario'
			},
			{
				id: 4,
				timestamp: new Date(Date.now() - 1000 * 60 * 45).toISOString(),
				level: 'success',
				action: 'Venta procesada',
				user: 'Sistema',
				ip: '-',
				details: 'Pedido #PED-20250105-0042 procesado correctamente',
				module: 'Ventas'
			},
			{
				id: 5,
				timestamp: new Date(Date.now() - 1000 * 60 * 60).toISOString(),
				level: 'error',
				action: 'Error de pago',
				user: 'cliente@example.com',
				ip: '186.148.200.50',
				details: 'Error al procesar pago con tarjeta ****1234',
				module: 'Pagos'
			},
			{
				id: 6,
				timestamp: new Date(Date.now() - 1000 * 60 * 90).toISOString(),
				level: 'info',
				action: 'Usuario actualizado',
				user: 'admin@kronostech.com',
				ip: '192.168.1.100',
				details: 'Datos de usuario ID 45 actualizados',
				module: 'Usuarios'
			},
			{
				id: 7,
				timestamp: new Date(Date.now() - 1000 * 60 * 120).toISOString(),
				level: 'success',
				action: 'Cupón creado',
				user: 'admin@kronostech.com',
				ip: '192.168.1.100',
				details: 'Cupón "VERANO2025" creado con 20% de descuento',
				module: 'Cupones'
			},
			{
				id: 8,
				timestamp: new Date(Date.now() - 1000 * 60 * 180).toISOString(),
				level: 'warning',
				action: 'Intento de login fallido',
				user: 'unknown@test.com',
				ip: '203.45.67.89',
				details: '3 intentos fallidos de autenticación',
				module: 'Seguridad'
			},
			{
				id: 9,
				timestamp: new Date(Date.now() - 1000 * 60 * 240).toISOString(),
				level: 'info',
				action: 'Backup creado',
				user: 'Sistema',
				ip: '-',
				details: 'Backup automático de base de datos completado',
				module: 'Sistema'
			},
			{
				id: 10,
				timestamp: new Date(Date.now() - 1000 * 60 * 300).toISOString(),
				level: 'success',
				action: 'Reembolso aprobado',
				user: 'admin@kronostech.com',
				ip: '192.168.1.100',
				details: 'Reembolso #REF-001 aprobado por $250.00',
				module: 'Reembolsos'
			}
		];

		logsFiltrados = [...logs];
		loading = false;
	}

	function aplicarFiltros() {
		logsFiltrados = logs.filter((log) => {
			// Filtro por nivel
			if (filtroNivel && log.level !== filtroNivel) return false;

			// Filtro por módulo
			if (filtroModulo && log.module !== filtroModulo) return false;

			// Búsqueda por acción, usuario o detalles
			if (busqueda) {
				const searchLower = busqueda.toLowerCase();
				if (
					!log.action.toLowerCase().includes(searchLower) &&
					!log.user.toLowerCase().includes(searchLower) &&
					!log.details.toLowerCase().includes(searchLower)
				) {
					return false;
				}
			}

			// Filtro por fecha
			if (fechaInicio) {
				const logDate = new Date(log.timestamp);
				const startDate = new Date(fechaInicio);
				if (logDate < startDate) return false;
			}

			if (fechaFin) {
				const logDate = new Date(log.timestamp);
				const endDate = new Date(fechaFin);
				endDate.setHours(23, 59, 59); // Hasta el final del día
				if (logDate > endDate) return false;
			}

			return true;
		});
	}

	$: {
		// Reactive: aplicar filtros cuando cambien
		filtroNivel;
		filtroModulo;
		busqueda;
		fechaInicio;
		fechaFin;
		if (logs.length > 0) {
			aplicarFiltros();
		}
	}

	function limpiarFiltros() {
		filtroNivel = '';
		filtroModulo = '';
		busqueda = '';
		fechaInicio = '';
		fechaFin = '';
	}

	function exportarLogs() {
		// Simular exportación de logs
		const csv = [
			['Timestamp', 'Nivel', 'Acción', 'Usuario', 'IP', 'Módulo', 'Detalles'].join(','),
			...logsFiltrados.map((log) =>
				[
					log.timestamp,
					log.level,
					log.action,
					log.user,
					log.ip,
					log.module,
					`"${log.details}"`
				].join(',')
			)
		].join('\n');

		const blob = new Blob([csv], { type: 'text/csv' });
		const url = window.URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = `logs_${new Date().toISOString().split('T')[0]}.csv`;
		a.click();
		window.URL.revokeObjectURL(url);
	}

	function getLevelIcon(level: string) {
		switch (level) {
			case 'success':
				return CheckCircle;
			case 'error':
				return XCircle;
			case 'warning':
				return AlertTriangle;
			default:
				return Info;
		}
	}

	function getLevelClass(level: string): string {
		const classes = {
			success: 'bg-green-100 dark:bg-green-900/40 text-green-700 dark:text-green-300',
			error: 'bg-red-100 dark:bg-red-900/40 text-red-700 dark:text-red-300',
			warning: 'bg-yellow-100 dark:bg-yellow-900/40 text-yellow-700 dark:text-yellow-300',
			info: 'bg-blue-100 dark:bg-blue-900/40 text-blue-700 dark:text-blue-300'
		};
		return classes[level as keyof typeof classes] || classes.info;
	}

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleString('es-ES', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit',
			second: '2-digit'
		});
	}

	// Obtener módulos únicos para el filtro
	$: modulos = Array.from(new Set(logs.map((log) => log.module)));
</script>

<svelte:head>
	<title>Logs y Auditoría | KronosTech Admin</title>
</svelte:head>

<div class="space-y-6 p-4 md:p-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold tracking-tight text-text-light dark:text-text-dark">
				Logs y Auditoría
			</h1>
			<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
				Registro de actividad y eventos del sistema
			</p>
		</div>
		<button
			on:click={exportarLogs}
			class="flex items-center gap-2 px-4 py-2 rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors"
		>
			<Download size={18} />
			Exportar Logs
		</button>
	</div>

	<!-- Estadísticas -->
	<div class="grid grid-cols-1 md:grid-cols-4 gap-4">
		<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-6">
			<div class="flex items-center justify-between">
				<div>
					<p class="text-sm text-slate-600 dark:text-slate-400">Total Eventos</p>
					<p class="text-3xl font-bold text-text-light dark:text-text-dark mt-1">
						{logs.length}
					</p>
				</div>
				<Activity class="text-blue-600 dark:text-blue-400" size={24} />
			</div>
		</div>

		<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-6">
			<div class="flex items-center justify-between">
				<div>
					<p class="text-sm text-slate-600 dark:text-slate-400">Errores</p>
					<p class="text-3xl font-bold text-red-600 dark:text-red-400 mt-1">
						{logs.filter((l) => l.level === 'error').length}
					</p>
				</div>
				<XCircle class="text-red-600 dark:text-red-400" size={24} />
			</div>
		</div>

		<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-6">
			<div class="flex items-center justify-between">
				<div>
					<p class="text-sm text-slate-600 dark:text-slate-400">Advertencias</p>
					<p class="text-3xl font-bold text-yellow-600 dark:text-yellow-400 mt-1">
						{logs.filter((l) => l.level === 'warning').length}
					</p>
				</div>
				<AlertTriangle class="text-yellow-600 dark:text-yellow-400" size={24} />
			</div>
		</div>

		<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-6">
			<div class="flex items-center justify-between">
				<div>
					<p class="text-sm text-slate-600 dark:text-slate-400">Exitosos</p>
					<p class="text-3xl font-bold text-green-600 dark:text-green-400 mt-1">
						{logs.filter((l) => l.level === 'success').length}
					</p>
				</div>
				<CheckCircle class="text-green-600 dark:text-green-400" size={24} />
			</div>
		</div>
	</div>

	<!-- Filtros -->
	<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-4">
		<div class="grid grid-cols-1 md:grid-cols-5 gap-4">
			<!-- Búsqueda -->
			<div class="md:col-span-2">
				<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
					<Search size={16} class="inline mr-2" />
					Buscar
				</label>
				<input
					type="text"
					bind:value={busqueda}
					class="form-input w-full"
					placeholder="Buscar en logs..."
				/>
			</div>

			<!-- Filtro por Nivel -->
			<div>
				<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
					<Filter size={16} class="inline mr-2" />
					Nivel
				</label>
				<select bind:value={filtroNivel} class="form-input w-full">
					<option value="">Todos</option>
					<option value="info">Info</option>
					<option value="success">Success</option>
					<option value="warning">Warning</option>
					<option value="error">Error</option>
				</select>
			</div>

			<!-- Filtro por Módulo -->
			<div>
				<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
					Módulo
				</label>
				<select bind:value={filtroModulo} class="form-input w-full">
					<option value="">Todos</option>
					{#each modulos as modulo}
						<option value={modulo}>{modulo}</option>
					{/each}
				</select>
			</div>

			<!-- Fecha Inicio -->
			<div>
				<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
					<Calendar size={16} class="inline mr-2" />
					Desde
				</label>
				<input
					type="date"
					bind:value={fechaInicio}
					class="form-input w-full"
				/>
			</div>
		</div>

		{#if filtroNivel || filtroModulo || busqueda || fechaInicio || fechaFin}
			<div class="mt-4 flex items-center justify-between">
				<p class="text-sm text-slate-600 dark:text-slate-400">
					Mostrando {logsFiltrados.length} de {logs.length} eventos
				</p>
				<button
					on:click={limpiarFiltros}
					class="text-sm text-primary hover:underline"
				>
					Limpiar filtros
				</button>
			</div>
		{/if}
	</div>

	<!-- Loading -->
	{#if loading}
		<div class="flex items-center justify-center py-12">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
		</div>
	{/if}

	<!-- Lista de Logs -->
	{#if !loading}
		{#if logsFiltrados.length === 0}
			<div class="text-center py-12 rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50">
				<FileText class="mx-auto text-slate-400 mb-4" size={48} />
				<h3 class="text-lg font-semibold text-text-light dark:text-text-dark mb-2">
					No se encontraron logs
				</h3>
				<p class="text-sm text-slate-600 dark:text-slate-400">
					{busqueda || filtroNivel || filtroModulo || fechaInicio || fechaFin
						? 'Intenta ajustar los filtros de búsqueda'
						: 'No hay eventos registrados'}
				</p>
			</div>
		{:else}
			<div class="space-y-3">
				{#each logsFiltrados as log}
					<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-4 hover:shadow-md transition-shadow">
						<div class="flex items-start gap-4">
							<!-- Icono -->
							<div class="flex-shrink-0 w-10 h-10 rounded-lg {getLevelClass(log.level)} flex items-center justify-center">
								<svelte:component this={getLevelIcon(log.level)} size={20} />
							</div>

							<!-- Contenido -->
							<div class="flex-1 min-w-0">
								<div class="flex items-start justify-between gap-4 mb-2">
									<div>
										<h3 class="font-semibold text-text-light dark:text-text-dark">
											{log.action}
										</h3>
										<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
											{log.details}
										</p>
									</div>
									<span class="px-2 py-1 rounded text-xs font-medium {getLevelClass(log.level)} whitespace-nowrap">
										{log.level.toUpperCase()}
									</span>
								</div>

								<!-- Metadata -->
								<div class="flex flex-wrap items-center gap-4 text-xs text-slate-600 dark:text-slate-400">
									<div class="flex items-center gap-1">
										<Calendar size={14} />
										{formatDate(log.timestamp)}
									</div>
									<div class="flex items-center gap-1">
										<User size={14} />
										{log.user}
									</div>
									{#if log.ip !== '-'}
										<div class="flex items-center gap-1">
											IP: {log.ip}
										</div>
									{/if}
									<div class="px-2 py-0.5 rounded bg-slate-100 dark:bg-slate-700">
										{log.module}
									</div>
								</div>
							</div>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	{/if}
</div>
