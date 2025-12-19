<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { authUser, isAuthenticated } from '$lib/stores/auth';
	import { logsStore, logsOrdenados, logStats, modulosUnicos, type LogEntry } from '$lib/stores/logs';
	import { logSystem } from '$lib/services/logs';
	import { FileText, Search, Filter, Download, Calendar, User, Activity, AlertTriangle, Info, CheckCircle, XCircle, Trash2, RefreshCw, Shield } from 'lucide-svelte';

	let logsFiltrados: LogEntry[] = [];
	let loading = true;
	let refreshing = false;

	// Filtros
	let filtroNivel = '';
	let filtroModulo = '';
	let busqueda = '';
	let fechaInicio = '';
	let fechaFin = '';

	// Suscribirse a los logs
	$: logs = $logsOrdenados;
	$: stats = $logStats;
	$: modulos = $modulosUnicos;

	onMount(async () => {
		if (!$isAuthenticated) {
			goto('/login?redirect=/admin/logs');
			return;
		}

		if ($authUser?.rol !== 'super_admin') {
			goto('/admin');
			return;
		}

		// Cargar logs desde la API
		await cargarLogs();
	});

	async function cargarLogs() {
		loading = true;
		await logsStore.load({ limit: 500 });
		loading = false;
		aplicarFiltros();
	}

	async function refrescarLogs() {
		refreshing = true;
		await logsStore.refresh();
		refreshing = false;
		aplicarFiltros();
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

	// Reactive: aplicar filtros cuando cambien
	$: {
		filtroNivel;
		filtroModulo;
		busqueda;
		fechaInicio;
		fechaFin;
		logs;
		aplicarFiltros();
	}

	function limpiarFiltros() {
		filtroNivel = '';
		filtroModulo = '';
		busqueda = '';
		fechaInicio = '';
		fechaFin = '';
	}

	async function limpiarLogs() {
		if (confirm('¿Estás seguro de eliminar todos los logs? Esta acción no se puede deshacer.')) {
			const success = await logsStore.clear();
			if (success) {
				await logSystem('Logs eliminados', `Usuario ${$authUser?.email} eliminó todos los logs del sistema`, 'warning');
				await refrescarLogs();
			}
		}
	}

	function exportarLogs() {
		const csv = [
			['Timestamp', 'Nivel', 'Acción', 'Usuario', 'IP', 'Módulo', 'Detalles'].join(','),
			...logsFiltrados.map((log) =>
				[
					log.timestamp,
					log.level,
					`"${log.action}"`,
					log.user,
					log.ip,
					log.module,
					`"${log.details.replace(/"/g, '""')}"`
				].join(',')
			)
		].join('\n');

		const blob = new Blob([csv], { type: 'text/csv;charset=utf-8;' });
		const url = window.URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = `logs_${new Date().toISOString().split('T')[0]}.csv`;
		a.click();
		window.URL.revokeObjectURL(url);

		logSystem('Logs exportados', `Usuario ${$authUser?.email} exportó ${logsFiltrados.length} logs a CSV`, 'info');
	}

	function getLevelIcon(level: string) {
		switch (level) {
			case 'success':
				return CheckCircle;
			case 'error':
				return XCircle;
			case 'warning':
				return AlertTriangle;
			case 'security':
				return Shield;
			default:
				return Info;
		}
	}

	function getLevelClass(level: string): string {
		const classes = {
			success: 'bg-green-100 dark:bg-green-900/40 text-green-700 dark:text-green-300',
			error: 'bg-red-100 dark:bg-red-900/40 text-red-700 dark:text-red-300',
			warning: 'bg-yellow-100 dark:bg-yellow-900/40 text-yellow-700 dark:text-yellow-300',
			info: 'bg-blue-100 dark:bg-blue-900/40 text-blue-700 dark:text-blue-300',
			security: 'bg-purple-100 dark:bg-purple-900/40 text-purple-700 dark:text-purple-300'
		};
		return classes[level as keyof typeof classes] || classes.info;
	}

	function getLevelLabel(level: string): string {
		const labels = {
			success: 'ÉXITO',
			error: 'ERROR',
			warning: 'ALERTA',
			info: 'INFO',
			security: 'SEGURIDAD'
		};
		return labels[level as keyof typeof labels] || level.toUpperCase();
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

	function getRelativeTime(dateString: string): string {
		const date = new Date(dateString);
		const now = new Date();
		const diffMs = now.getTime() - date.getTime();
		const diffMins = Math.floor(diffMs / 60000);
		const diffHours = Math.floor(diffMs / 3600000);
		const diffDays = Math.floor(diffMs / 86400000);

		if (diffMins < 1) return 'Hace un momento';
		if (diffMins < 60) return `Hace ${diffMins} min`;
		if (diffHours < 24) return `Hace ${diffHours} hora${diffHours > 1 ? 's' : ''}`;
		if (diffDays < 7) return `Hace ${diffDays} día${diffDays > 1 ? 's' : ''}`;
		return formatDate(dateString);
	}
</script>

<svelte:head>
	<title>Logs y Auditoría | KronosTech Admin</title>
</svelte:head>

<div class="space-y-6 p-4 md:p-6">
	<!-- Header -->
	<div class="flex items-center justify-between flex-wrap gap-4">
		<div>
			<h1 class="text-3xl font-bold tracking-tight text-text-light dark:text-text-dark">
				Logs y Auditoría
			</h1>
			<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
				Registro de actividad y eventos del sistema (almacenado en base de datos)
			</p>
		</div>
		<div class="flex gap-2">
			<button
				on:click={refrescarLogs}
				disabled={refreshing}
				class="flex items-center gap-2 px-4 py-2 rounded-lg border border-border-light dark:border-border-dark text-slate-600 dark:text-slate-400 hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors disabled:opacity-50"
				title="Refrescar logs"
			>
				<RefreshCw size={18} class={refreshing ? 'animate-spin' : ''} />
				Refrescar
			</button>
			<button
				on:click={limpiarLogs}
				class="flex items-center gap-2 px-4 py-2 rounded-lg border border-red-300 dark:border-red-700 text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors"
				title="Eliminar todos los logs"
			>
				<Trash2 size={18} />
				Limpiar
			</button>
			<button
				on:click={exportarLogs}
				class="flex items-center gap-2 px-4 py-2 rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors"
			>
				<Download size={18} />
				Exportar CSV
			</button>
		</div>
	</div>

	<!-- Estadísticas -->
	<div class="grid grid-cols-2 md:grid-cols-6 gap-4">
		<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-4">
			<div class="flex items-center justify-between">
				<div>
					<p class="text-xs text-slate-600 dark:text-slate-400">Total</p>
					<p class="text-2xl font-bold text-text-light dark:text-text-dark mt-1">
						{stats.total}
					</p>
				</div>
				<Activity class="text-blue-600 dark:text-blue-400" size={20} />
			</div>
		</div>

		<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-4">
			<div class="flex items-center justify-between">
				<div>
					<p class="text-xs text-slate-600 dark:text-slate-400">Info</p>
					<p class="text-2xl font-bold text-blue-600 dark:text-blue-400 mt-1">
						{stats.info}
					</p>
				</div>
				<Info class="text-blue-600 dark:text-blue-400" size={20} />
			</div>
		</div>

		<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-4">
			<div class="flex items-center justify-between">
				<div>
					<p class="text-xs text-slate-600 dark:text-slate-400">Éxitos</p>
					<p class="text-2xl font-bold text-green-600 dark:text-green-400 mt-1">
						{stats.success}
					</p>
				</div>
				<CheckCircle class="text-green-600 dark:text-green-400" size={20} />
			</div>
		</div>

		<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-4">
			<div class="flex items-center justify-between">
				<div>
					<p class="text-xs text-slate-600 dark:text-slate-400">Alertas</p>
					<p class="text-2xl font-bold text-yellow-600 dark:text-yellow-400 mt-1">
						{stats.warning}
					</p>
				</div>
				<AlertTriangle class="text-yellow-600 dark:text-yellow-400" size={20} />
			</div>
		</div>

		<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-4">
			<div class="flex items-center justify-between">
				<div>
					<p class="text-xs text-slate-600 dark:text-slate-400">Errores</p>
					<p class="text-2xl font-bold text-red-600 dark:text-red-400 mt-1">
						{stats.error}
					</p>
				</div>
				<XCircle class="text-red-600 dark:text-red-400" size={20} />
			</div>
		</div>

		<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-4">
			<div class="flex items-center justify-between">
				<div>
					<p class="text-xs text-slate-600 dark:text-slate-400">Seguridad</p>
					<p class="text-2xl font-bold text-purple-600 dark:text-purple-400 mt-1">
						{stats.security}
					</p>
				</div>
				<Shield class="text-purple-600 dark:text-purple-400" size={20} />
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
					<option value="success">Éxito</option>
					<option value="warning">Alerta</option>
					<option value="error">Error</option>
					<option value="security">Seguridad</option>
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
						: 'Los eventos del sistema se registrarán aquí automáticamente'}
				</p>
			</div>
		{:else}
			<div class="space-y-3">
				{#each logsFiltrados as log (log.id)}
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
										{getLevelLabel(log.level)}
									</span>
								</div>

								<!-- Metadata -->
								<div class="flex flex-wrap items-center gap-4 text-xs text-slate-600 dark:text-slate-400">
									<div class="flex items-center gap-1" title={formatDate(log.timestamp)}>
										<Calendar size={14} />
										{getRelativeTime(log.timestamp)}
									</div>
									<div class="flex items-center gap-1">
										<User size={14} />
										{log.user}
									</div>
									{#if log.ip !== '-' && log.ip !== 'N/A' && log.ip !== 'Cliente'}
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

			<!-- Info de almacenamiento -->
			<div class="text-center py-4">
				<p class="text-xs text-slate-500 dark:text-slate-400">
					Los logs se almacenan en la base de datos. Se muestran los últimos 500 eventos.
				</p>
			</div>
		{/if}
	{/if}
</div>
