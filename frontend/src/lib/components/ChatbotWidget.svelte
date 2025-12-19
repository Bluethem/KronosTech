<script lang="ts">
	import { MessageCircle, X, Send } from 'lucide-svelte';
	import { tick } from 'svelte';

	type Msg = { role: 'user' | 'assistant'; content: string };

	let open = false;
	let loading = false;
	let input = '';
	let error = '';
	let messages: Msg[] = [
		{
			role: 'assistant',
			content:
				'Hola, soy el asistente de KronosTech. ¿En qué te ayudo? (productos, envíos, garantías, compatibilidad)'
		}
	];

	let scrollEl: HTMLDivElement | null = null;

	function toggle() {
		open = !open;
		if (open) {
			setTimeout(() => {
				inputEl?.focus();
			}, 0);
		}
	}

	let inputEl: HTMLInputElement | null = null;

	async function sendMessage() {
		if (loading) return;
		error = '';

		const text = input.trim();
		if (!text) return;

		messages = [...messages, { role: 'user', content: text }];
		input = '';
		loading = true;

		await tick();
		scrollEl?.scrollTo({ top: scrollEl.scrollHeight, behavior: 'smooth' });

		try {
			const resp = await fetch('/api/chat', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ message: text })
			});

			const data = (await resp.json().catch(() => ({}))) as { reply?: string; error?: string };

			if (!resp.ok) {
				error = data?.error || 'No se pudo enviar el mensaje.';
				return;
			}

			messages = [...messages, { role: 'assistant', content: data.reply ?? '...' }];
			await tick();
			scrollEl?.scrollTo({ top: scrollEl.scrollHeight, behavior: 'smooth' });
		} catch (e) {
			error = 'Error de red. Intenta nuevamente.';
		} finally {
			loading = false;
			inputEl?.focus();
		}
	}

	function onKeyDown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			void sendMessage();
		}
	}
</script>

<div class="relative">
	{#if open}
		<div
			class="absolute bottom-full right-0 mb-4 w-[340px] sm:w-[380px] max-w-[calc(100vw-3rem)] rounded-2xl border border-border-light dark:border-border-dark bg-white dark:bg-surface-dark shadow-xl overflow-hidden"
			role="dialog"
			aria-label="Chat de ayuda"
		>
			<div class="flex items-center justify-between gap-3 px-4 py-3 bg-primary text-white">
				<div class="min-w-0">
					<p class="font-bold leading-tight">Asistente KronosTech</p>
					<p class="text-xs opacity-90">Respuestas rápidas para tu compra</p>
				</div>
				<button
					class="p-2 rounded-lg hover:bg-white/10 transition-colors"
					on:click={toggle}
					aria-label="Cerrar chat"
				>
					<X size={18} />
				</button>
			</div>

			<div bind:this={scrollEl} class="h-[320px] px-4 py-3 overflow-auto bg-white dark:bg-surface-dark">
				<div class="space-y-3">
					{#each messages as m (m)}
						<div class={m.role === 'user' ? 'flex justify-end' : 'flex justify-start'}>
							<div
								class={
									(m.role === 'user'
										? 'bg-primary text-white'
										: 'bg-slate-100 dark:bg-slate-800 text-slate-900 dark:text-slate-100') +
									' max-w-[85%] rounded-2xl px-4 py-2 text-sm leading-relaxed whitespace-pre-wrap'
								}
							>
								{m.content}
							</div>
						</div>
					{/each}

					{#if loading}
						<div class="flex justify-start">
							<div class="bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-200 max-w-[85%] rounded-2xl px-4 py-2 text-sm">
								Escribiendo...
							</div>
						</div>
					{/if}
				</div>
			</div>

			{#if error}
				<div class="px-4 pb-2 text-xs text-red-600 dark:text-red-400">{error}</div>
			{/if}

			<div class="px-3 py-3 border-t border-border-light dark:border-border-dark bg-white dark:bg-surface-dark">
				<div class="flex items-center gap-2">
					<input
						bind:this={inputEl}
						bind:value={input}
						on:keydown={onKeyDown}
						disabled={loading}
						placeholder="Escribe tu pregunta..."
						class="flex-1 px-3 py-2 rounded-xl border border-border-light dark:border-border-dark bg-white dark:bg-surface-dark focus:ring-2 focus:ring-primary/50 focus:border-primary transition-colors disabled:opacity-60"
					/>
					<button
						on:click={() => void sendMessage()}
						disabled={loading}
						class="inline-flex items-center justify-center gap-2 px-3 py-2 rounded-xl bg-primary text-white font-bold hover:bg-primary/90 transition-colors disabled:opacity-60"
						aria-label="Enviar mensaje"
					>
						<Send size={16} />
					</button>
				</div>
			</div>
		</div>
	{/if}

	<button
		class="p-4 bg-primary hover:bg-primary/90 text-white rounded-full shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-110"
		on:click={toggle}
		aria-label={open ? 'Cerrar chat' : 'Abrir chat'}
	>
		<MessageCircle size={28} />
	</button>
</div>
