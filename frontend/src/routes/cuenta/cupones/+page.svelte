<script lang="ts">
	import { onMount } from 'svelte';
	import { cuponService, type CuponUsuario } from '$lib/services/cupon';

	let cupones: CuponUsuario[] = [];
	let loading = true;
	let error = '';

	function getEstado(cupon: CuponUsuario): 'vigente' | 'proximo' | 'vencido' | 'usado' {
		const ahora = new Date();
		const inicio = new Date(cupon.fecha_inicio);
		const fin = new Date(cupon.fecha_fin);

		const sinUsosDisponibles =
			(cupon.usos_maximos_por_usuario !== undefined &&
				cupon.usos_usuario >= cupon.usos_maximos_por_usuario) ||
			cupon.usado;

		if (sinUsosDisponibles) return 'usado';
		if (ahora < inicio) return 'proximo';
		if (ahora > fin) return 'vencido';
		return 'vigente';
	}

	function formatValor(cupon: CuponUsuario): string {
		if (cupon.tipo_cupon === 'porcentaje') {
			return `${cupon.valor}%`; // porcentaje
		}
		if (cupon.tipo_cupon === 'monto_fijo') {
			return `S/. ${cupon.valor.toFixed(2)}`;
		}
		if (cupon.tipo_cupon === 'envio_gratis') {
			return 'Envío gratis';
		}
		return String(cupon.valor);
	}

	function formatRangoFechas(cupon: CuponUsuario): string {
		const inicio = new Date(cupon.fecha_inicio).toLocaleDateString('es-PE');
		const fin = new Date(cupon.fecha_fin).toLocaleDateString('es-PE');
		return `${inicio} - ${fin}`;
	}

	function getEstadoLabel(estado: ReturnType<typeof getEstado>): string {
		if (estado === 'vigente') return 'Vigente';
		if (estado === 'proximo') return 'Próximo';
		if (estado === 'vencido') return 'Vencido';
		return 'Usado';
	}

	function getEstadoClasses(estado: ReturnType<typeof getEstado>): string {
		if (estado === 'vigente') return 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-300';
		if (estado === 'proximo') return 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300';
		if (estado === 'vencido') return 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-300';
		return 'bg-slate-100 text-slate-800 dark:bg-slate-800/60 dark:text-slate-300';
	}

	async function copiarCodigo(codigo: string) {
		try {
			await navigator.clipboard.writeText(codigo);
			// No mostramos toast global para no tocar otros componentes
		} catch (e) {
			console.error('No se pudo copiar el código de cupón', e);
		}
	}

	onMount(async () => {
		loading = true;
		error = '';
		try {
			cupones = await cuponService.getMisCupones();
		} catch (e: any) {
			error = e.message || 'Error al cargar tus cupones';
		} finally {
			loading = false;
		}
	});
</script>

<svelte:head>
	<title>Mis Cupones | KronosTech</title>
</svelte:head>

<div class="space-y-6">
	<div>
		<h1 class="text-3xl font-bold tracking-tight text-text-light dark:text-text-dark">
			Mis Cupones
		</h1>
		<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
			Aquí encontrarás los cupones asignados a tu cuenta. Copia el código y utilízalo en el checkout.
		</p>
	</div>

	{#if loading}
		<div class="flex items-center justify-center py-12">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
		</div>
	{:else}
		{#if error}
			<div class="rounded-lg bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 p-4 text-sm text-red-800 dark:text-red-200">
				{error}
			</div>
		{:else if cupones.length === 0}
			<div class="text-center py-12 rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50">
				<h3 class="text-lg font-semibold text-text-light dark:text-text-dark mb-2">
					No tienes cupones asignados
				</h3>
				<p class="text-sm text-slate-600 dark:text-slate-400">
					Cuando recibas cupones promocionales, aparecerán aquí.
				</p>
			</div>
		{:else}
			<div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
				{#each cupones as cupon}
					{@const estado = getEstado(cupon)}
					<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/70 shadow-sm p-4 flex flex-col gap-3">
						<div class="flex items-start justify-between gap-3">
							<div>
								<p class="text-xs font-mono uppercase tracking-wide text-slate-500 dark:text-slate-400">
									Código
								</p>
								<p class="text-lg font-semibold text-text-light dark:text-text-dark">
									{cupon.codigo}
								</p>
							</div>
							<span class={`inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium ${getEstadoClasses(estado)}`}>
								{getEstadoLabel(estado)}
							</span>
						</div>

						<div>
							<p class="text-sm font-medium text-text-light dark:text-text-dark">
								{cupon.nombre}
							</p>
							{#if cupon.descripcion}
								<p class="text-xs text-slate-600 dark:text-slate-400 mt-1 line-clamp-2">
									{cupon.descripcion}
								</p>
							{/if}
						</div>

						<div class="flex items-center justify-between text-sm">
							<div>
								<p class="text-xs text-slate-500 dark:text-slate-400">Beneficio</p>
								<p class="font-semibold text-text-light dark:text-text-dark">
									{formatValor(cupon)}
								</p>
							</div>
							<div class="text-right">
								<p class="text-xs text-slate-500 dark:text-slate-400">Vigencia</p>
								<p class="text-xs text-text-light dark:text-text-dark">
									{formatRangoFechas(cupon)}
								</p>
							</div>
						</div>

						<div class="flex items-center justify-between text-xs text-slate-600 dark:text-slate-400">
							{#if cupon.usos_maximos_por_usuario}
								<p>
									Usos: {Math.min(cupon.usos_usuario, cupon.usos_maximos_por_usuario)}/{cupon.usos_maximos_por_usuario}
								</p>
							{:else}
								<p>Usos: {cupon.usos_usuario}</p>
							{/if}
							<p>
								Asignado el {new Date(cupon.fecha_asignacion).toLocaleDateString('es-PE')}
							</p>
						</div>

						<div class="flex justify-end">
							<button
								type="button"
								on:click={() => copiarCodigo(cupon.codigo)}
								class="px-3 py-1.5 rounded-lg text-xs font-medium bg-primary text-white hover:bg-primary/90 transition-colors"
								disabled={estado === 'vencido' || estado === 'usado'}
							>
								Copiar código
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	{/if}
</div>
