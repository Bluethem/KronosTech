<script lang="ts">
	import { Facebook, Instagram, Twitter, Mail, Phone, MapPin, Clock, Send, MessageCircle, Shield, CreditCard } from 'lucide-svelte';
	import ChatbotWidget from '$lib/components/ChatbotWidget.svelte';
	
	let emailNewsletter = '';
	let subscribing = false;
	let subscribeMessage = '';
	let subscribeSuccess = false;
	
	async function handleNewsletterSubmit(e: Event) {
		e.preventDefault();
		
		if (!emailNewsletter || !emailNewsletter.includes('@')) {
			subscribeMessage = 'Por favor ingresa un email válido';
			subscribeSuccess = false;
			return;
		}
		
		subscribing = true;
		subscribeMessage = '';
		
		// Simular llamada a API (reemplazar con API real)
		try {
			await new Promise(resolve => setTimeout(resolve, 1000));
			
			subscribeMessage = '¡Gracias por suscribirte! Recibirás nuestras ofertas exclusivas.';
			subscribeSuccess = true;
			emailNewsletter = '';
			
			// Limpiar mensaje después de 5 segundos
			setTimeout(() => {
				subscribeMessage = '';
			}, 5000);
		} catch (err) {
			subscribeMessage = 'Hubo un error. Intenta nuevamente.';
			subscribeSuccess = false;
		} finally {
			subscribing = false;
		}
	}
</script>


<footer class="mt-16 border-t border-border-light dark:border-border-dark">
	<!-- Newsletter Section -->
	<div class="bg-gradient-to-r from-primary/10 to-primary/5 dark:from-background-dark dark:to-background-dark border-b border-border-light dark:border-border-dark">
		<div class="max-w-screen-xl mx-auto px-4 sm:px-6 lg:px-10 py-8">
			<div class="flex flex-col md:flex-row items-center justify-between gap-6">
				<div class="text-center md:text-left">
					<h3 class="text-xl font-bold mb-2 flex items-center justify-center md:justify-start gap-2">
						<Mail class="text-primary" size={24} />
						Suscríbete a nuestro Newsletter
					</h3>
					<p class="text-sm text-slate-600 dark:text-slate-400">
						Recibe ofertas exclusivas, novedades y descuentos especiales en tu email
					</p>
				</div>
				
				<form on:submit={handleNewsletterSubmit} class="w-full md:w-auto">
					<div class="flex flex-col sm:flex-row gap-3 min-w-[300px]">
						<input
							type="email"
							bind:value={emailNewsletter}
							placeholder="tu@email.com"
							disabled={subscribing}
							class="flex-1 px-4 py-3 rounded-lg border border-border-light dark:border-border-dark bg-white dark:bg-surface-dark focus:ring-2 focus:ring-primary/50 focus:border-primary transition-colors disabled:opacity-50"
							required
						/>
						<button
							type="submit"
							disabled={subscribing}
							class="inline-flex items-center justify-center gap-2 px-6 py-3 bg-primary text-white font-bold rounded-lg hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
						>
							{#if subscribing}
								Enviando...
							{:else}
								<Send size={18} />
								Suscribirse
							{/if}
						</button>
					</div>
					{#if subscribeMessage}
						<p class="mt-2 text-sm {subscribeSuccess ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'}">
							{subscribeMessage}
						</p>
					{/if}
				</form>
			</div>
		</div>
	</div>
	
	<div class="max-w-screen-xl mx-auto px-4 sm:px-6 lg:px-10 py-12">
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8 mb-8">
			<!-- Columna 1: Sobre KronosTech -->
			<div>
				<div class="flex items-center gap-2 mb-4">
					<div class="size-8 text-primary">
						<img
							src="/src/lib/assets/apple-touch-icon.png"
							alt="Icono"
							class="h-9 w-9 object-contain"
						/>
					</div>
					<h3 class="font-bold text-xl text-primary">KronosTech</h3>
				</div>
				<p class="text-sm text-slate-600 dark:text-slate-400 mb-4">
					Tu tienda de confianza para componentes de PC y sistemas gaming de alta gama.
				</p>
				
				<!-- Información de contacto -->
				<div class="space-y-2 text-sm text-slate-600 dark:text-slate-400">
					<div class="flex items-center gap-2">
						<Phone size={16} class="text-primary" />
						<a href="tel:+51987654321" class="hover:text-primary transition-colors">+51 987 654 321</a>
					</div>
					<div class="flex items-center gap-2">
						<Mail size={16} class="text-primary" />
						<a href="mailto:ventas@kronostech.pe" class="hover:text-primary transition-colors">ventas@kronostech.pe</a>
					</div>
					<div class="flex items-start gap-2">
						<MapPin size={16} class="text-primary mt-0.5" />
						<span>Lima, Perú</span>
					</div>
					<div class="flex items-start gap-2">
						<Clock size={16} class="text-primary mt-0.5" />
						<span>Lun-Vie: 9am - 6pm<br/>Sáb: 9am - 1pm</span>
					</div>
				</div>
			</div>
			
			<!-- Columna 2: Tienda -->
			<div>
				<h4 class="font-semibold mb-4 text-sm uppercase tracking-wide">Tienda</h4>
				<ul class="space-y-2 text-sm text-slate-600 dark:text-slate-400">
					<li><a class="hover:text-primary transition-colors" href="/catalogo">Catálogo Completo</a></li>
					<li><a class="hover:text-primary transition-colors" href="/ofertas">Ofertas</a></li>
					<li><a class="hover:text-primary transition-colors" href="/novedades">Novedades</a></li>
					<li><a class="hover:text-primary transition-colors" href="/destacados">Destacados</a></li>
					<li><a class="hover:text-primary transition-colors" href="/marcas">Marcas</a></li>
				</ul>
			</div>
			
			<!-- Columna 3: Atención al Cliente -->
			<div>
				<h4 class="font-semibold mb-4 text-sm uppercase tracking-wide">Atención al Cliente</h4>
				<ul class="space-y-2 text-sm text-slate-600 dark:text-slate-400">
					<li><a class="hover:text-primary transition-colors" href="mailto:ventas@kronostech.pe">Contáctanos</a></li>
					<li><a class="hover:text-primary transition-colors" href="/busqueda">Buscar Productos</a></li>
					<li><a class="hover:text-primary transition-colors" href="/informacion#preguntas">Preguntas Frecuentes</a></li>
					<li><a class="hover:text-primary transition-colors" href="/informacion#envios">Política de Envíos</a></li>
					<li><a class="hover:text-primary transition-colors" href="/informacion#garantia">Devoluciones</a></li>
				</ul>
			</div>
			
			<!-- Columna 4: Empresa -->
			<div>
				<h4 class="font-semibold mb-4 text-sm uppercase tracking-wide">Empresa</h4>
				<ul class="space-y-2 text-slate-600 dark:text-slate-400">
					<li><a class="hover:text-primary transition-colors" href="/informacion#nosotros">Sobre Nosotros</a></li>
					<li><a class="hover:text-primary transition-colors" href="/informacion#garantia">Garantía Oficial</a></li>
					<li><a class="hover:text-primary transition-colors" href="/informacion#privacidad">Privacidad</a></li>
					<li><a class="hover:text-primary transition-colors" href="/informacion#terminos">Términos y Condiciones</a></li>
				</ul>
			</div>
		</div>
		
		<!-- Métodos de Pago y Seguridad -->
		<div class="border-t border-border-light dark:border-border-dark pt-8 mt-8">
			<div class="grid grid-cols-1 md:grid-cols-2 gap-8">
				<!-- Métodos de Pago -->
				<div>
					<h4 class="text-sm font-semibold mb-4 flex items-center gap-2">
						<CreditCard size={18} class="text-primary" />
						Métodos de Pago Aceptados
					</h4>
					<div class="flex flex-wrap items-center gap-4">
						<!-- Tarjetas -->
						<div class="flex items-center gap-3 px-4 py-2 rounded-lg border border-border-light dark:border-border-dark">
							<div class="text-2xl">💳</div>
							<div class="text-xs">
								<p class="font-semibold">Visa</p>
								<p class="text-slate-500">Crédito/Débito</p>
							</div>
						</div>
						<div class="flex items-center gap-3 px-4 py-2 rounded-lg border border-border-light dark:border-border-dark">
							<div class="text-2xl">💳</div>
							<div class="text-xs">
								<p class="font-semibold">Mastercard</p>
								<p class="text-slate-500">Crédito/Débito</p>
							</div>
						</div>
						<!-- Billeteras digitales -->
						<div class="flex items-center gap-3 px-4 py-2 rounded-lg border border-border-light dark:border-border-dark">
							<div class="text-2xl">📱</div>
							<div class="text-xs">
								<p class="font-semibold">Yape / Plin</p>
								<p class="text-slate-500">Instant</p>
							</div>
						</div>
					</div>
				</div>
				
				<!-- Seguridad y Certificaciones -->
				<div>
					<h4 class="text-sm font-semibold mb-4 flex items-center gap-2">
						<Shield size={18} class="text-primary" />
						Compra Segura
					</h4>
					<div class="flex flex-wrap items-center gap-4">
						<div class="flex items-center gap-3 px-4 py-2 rounded-lg border border-border-light dark:border-border-dark">
							<Shield size={16} class="text-green-600 dark:text-green-400" />
							<span class="text-xs font-semibold text-green-700 dark:text-green-300">Certificado SSL</span>
						</div>
						<div class="flex items-center gap-3 px-4 py-2 rounded-lg border border-border-light dark:border-border-dark">
							<Shield size={16} class="text-blue-600 dark:text-blue-400" />
							<span class="text-xs font-semibold text-blue-700 dark:text-blue-300">Garantía Oficial</span>
						</div>
						<a 
							href="https://www.indecopi.gob.pe/en/-/libro-de-reclamaciones" 
							target="_blank"
							rel="noopener noreferrer"
							class="flex items-center gap-2 bg-primary/10 dark:bg-primary/20 px-4 py-2 rounded-lg border border-primary/30 hover:bg-primary/20 transition-colors"
						>
							<span class="text-xs font-semibold text-primary">📖 Libro de Reclamaciones</span>
						</a>
					</div>
				</div>
			</div>
		</div>
	</div>
	
	<!-- Copyright y Redes Sociales -->
	<div class="border-t border-border-light dark:border-border-dark py-6">
		<div class="max-w-screen-xl mx-auto px-4 sm:px-6 lg:px-10">
			<div class="flex flex-col sm:flex-row justify-between items-center gap-4 mb-4">
				<p class="text-sm text-slate-500 dark:text-slate-400 text-center sm:text-left">
					© {new Date().getFullYear()} KronosTech Perú. Todos los derechos reservados.
				</p>
				
				<!-- Redes Sociales con Iconos -->
				<div class="flex items-center gap-4">
					<span class="text-sm text-slate-500 dark:text-slate-400">Síguenos:</span>
					<div class="flex gap-3">
						<a 
							href="https://facebook.com/kronostech" 
							target="_blank" 
							rel="noopener noreferrer"
							aria-label="Facebook"
							class="p-2 rounded-full bg-slate-100 dark:bg-slate-700 text-slate-600 dark:text-slate-400 hover:bg-blue-600 hover:text-white transition-colors"
						>
							<Facebook size={18} />
						</a>
						<a 
							href="https://instagram.com/kronostech" 
							target="_blank" 
							rel="noopener noreferrer"
							aria-label="Instagram"
							class="p-2 rounded-full bg-slate-100 dark:bg-slate-700 text-slate-600 dark:text-slate-400 hover:bg-gradient-to-br hover:from-purple-600 hover:to-pink-600 hover:text-white transition-colors"
						>
							<Instagram size={18} />
						</a>
						<a 
							href="https://twitter.com/kronostech" 
							target="_blank" 
							rel="noopener noreferrer"
							aria-label="Twitter"
							class="p-2 rounded-full bg-slate-100 dark:bg-slate-700 text-slate-600 dark:text-slate-400 hover:bg-black hover:text-white transition-colors"
						>
							<Twitter size={18} />
						</a>
					</div>
				</div>
			</div>
			
			<!-- Enlaces adicionales -->
			<div class="flex flex-wrap justify-center gap-4 text-xs text-slate-500 dark:text-slate-400">
				<a href="/informacion#terminos" class="hover:text-primary transition-colors">Términos y Condiciones</a>
				<span>•</span>
				<a href="/informacion#privacidad" class="hover:text-primary transition-colors">Política de Privacidad</a>
				<span>•</span>
				<a href="/informacion#envios" class="hover:text-primary transition-colors">Política de Envíos</a>
			</div>
		</div>
	</div>
	
	<div class="fixed bottom-6 right-6 z-50 flex flex-col items-end gap-4">
		<ChatbotWidget />
		<!-- WhatsApp Float Button -->
		<a
			href="https://wa.me/51900480093?text=Hola,%20tengo%20una%20consulta%20sobre%20KronosTech"
			target="_blank"
			rel="noopener noreferrer"
			aria-label="Contactar por WhatsApp"
			class="p-4 bg-green-500 hover:bg-green-600 text-white rounded-full shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-110 group"
		>
			<MessageCircle size={28} />
			<span class="absolute right-full mr-3 top-1/2 -translate-y-1/2 bg-slate-900 dark:bg-slate-700 text-white text-sm font-medium px-3 py-2 rounded-lg opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none">
				¿Necesitas ayuda?
			</span>
		</a>
	</div>
</footer>

