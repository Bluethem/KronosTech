<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { ChevronRight, Shield, Truck, HelpCircle, FileText, Building2, Award } from 'lucide-svelte';
	
	let activaSeccion = 'nosotros';
	
	const secciones = [
		{ id: 'nosotros', titulo: 'Sobre Nosotros', icono: Building2 },
		{ id: 'garantia', titulo: 'Garantía Oficial', icono: Award },
		{ id: 'envios', titulo: 'Política de Envíos', icono: Truck },
		{ id: 'privacidad', titulo: 'Privacidad', icono: Shield },
		{ id: 'terminos', titulo: 'Términos y Condiciones', icono: FileText },
		{ id: 'preguntas', titulo: 'Preguntas Frecuentes', icono: HelpCircle }
	];
	
	function scrollToSection(id: string) {
		activaSeccion = id;
		if (browser) {
			const element = document.getElementById(id);
			if (element) {
				const offset = 100;
				const elementPosition = element.getBoundingClientRect().top;
				const offsetPosition = elementPosition + window.pageYOffset - offset;
				
				window.scrollTo({
					top: offsetPosition,
					behavior: 'smooth'
				});
			}
		}
	}
	
	onMount(() => {
		const handleScroll = () => {
			const sections = secciones.map(s => document.getElementById(s.id)).filter(Boolean);
			const scrollPos = window.scrollY + 150;
			
			for (const section of sections) {
				if (section) {
					const sectionTop = section.offsetTop;
					const sectionHeight = section.offsetHeight;
					
					if (scrollPos >= sectionTop && scrollPos < sectionTop + sectionHeight) {
						activaSeccion = section.id;
						break;
					}
				}
			}
		};
		
		window.addEventListener('scroll', handleScroll);
		return () => window.removeEventListener('scroll', handleScroll);
	});
</script>

<svelte:head>
	<title>Información Legal - KronosTech</title>
	<meta name="description" content="Información sobre KronosTech, políticas, términos y condiciones" />
</svelte:head>

<div class="w-full max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
	<nav class="flex mb-6 text-sm">
		<ol class="inline-flex items-center space-x-2">
			<li><a href="/" class="text-slate-500 dark:text-slate-400 hover:text-primary">Inicio</a></li>
			<li class="flex items-center">
				<span class="mx-2 text-slate-400">/</span>
				<span class="text-text-light dark:text-text-dark font-medium">Información</span>
			</li>
		</ol>
	</nav>
	
	<div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
		<aside class="lg:col-span-1">
			<div class="sticky top-24 rounded-lg border border-border-light dark:border-border-dark p-4">
				<h3 class="font-bold text-lg mb-4">Contenido</h3>
				<nav class="space-y-2">
					{#each secciones as seccion}
						<button
							on:click={() => scrollToSection(seccion.id)}
							class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-left transition-colors {activaSeccion === seccion.id 
								? 'bg-primary text-white' 
								: 'hover:bg-slate-100 dark:hover:bg-slate-700'}"
						>
							<svelte:component this={seccion.icono} size={18} />
							<span class="text-sm font-medium">{seccion.titulo}</span>
						</button>
					{/each}
				</nav>
			</div>
		</aside>
		
		<div class="lg:col-span-3 space-y-12 prose dark:prose-invert max-w-none">
			<!-- Sobre Nosotros -->
			<section id="nosotros" class="scroll-mt-24">
				<div class="flex items-center gap-3 mb-6 not-prose">
					<Building2 size={32} class="text-primary" />
					<h1 class="text-3xl font-black">Sobre Nosotros</h1>
				</div>
				
				<h2>¿Quiénes Somos?</h2>
				<p><strong>KronosTech</strong> es una plataforma de e-commerce especializada en componentes de PC para el mercado peruano.</p>
				
				<h3>Nuestra Misión</h3>
				<p>Facilitar el acceso a tecnología de última generación con precios justos y servicio de calidad.</p>
				
				<h3>¿Por Qué Elegirnos?</h3>
				<ul>
					<li>Productos originales con garantía oficial</li>
					<li>Precios competitivos</li>
					<li>Envío rápido (24-48h en Lima)</li>
					<li>Soporte técnico especializado</li>
				</ul>
			</section>
			
			<section id="garantia" class="scroll-mt-24">
				<div class="flex items-center gap-3 mb-6 not-prose">
					<Award size={32} class="text-primary" />
					<h1 class="text-3xl font-black">Garantía Oficial</h1>
				</div>
				
				<p>Todos nuestros productos cuentan con garantía oficial del fabricante (12-36 meses según el producto).</p>
				
				<h3>¿Qué Cubre?</h3>
				<ul>
					<li>Defectos de fábrica</li>
					<li>Fallas técnicas en uso normal</li>
					<li>Reparación o reemplazo sin costo</li>
				</ul>
			</section>
			
			<section id="envios" class="scroll-mt-24">
				<div class="flex items-center gap-3 mb-6 not-prose">
					<Truck size={32} class="text-primary" />
					<h1 class="text-3xl font-black">Política de Envíos</h1>
				</div>
				
				<p>Enviamos a todo el Perú:</p>
				<ul>
					<li>Lima: 24-48 horas (Gratis > S/. 200)</li>
					<li>Provincias: 3-7 días (Gratis > S/. 300-400)</li>
				</ul>
			</section>
			
			<section id="privacidad" class="scroll-mt-24">
				<div class="flex items-center gap-3 mb-6 not-prose">
					<Shield size={32} class="text-primary" />
					<h1 class="text-3xl font-black">Política de Privacidad</h1>
				</div>
				
				<p>Protegemos tus datos según la Ley N° 29733. No vendemos tu información a terceros.</p>
				
				<h3>Seguridad</h3>
				<ul>
					<li>Cifrado SSL/TLS</li>
					<li>Cumplimiento PCI DSS</li>
					<li>Servidores protegidos</li>
				</ul>
			</section>
			
			<section id="terminos" class="scroll-mt-24">
				<div class="flex items-center gap-3 mb-6 not-prose">
					<FileText size={32} class="text-primary" />
					<h1 class="text-3xl font-black">Términos y Condiciones</h1>
				</div>
				
				<p>Al usar KronosTech, aceptas estos términos.</p>
				
				<h3>Puntos Clave</h3>
				<ul>
					<li>Precios en Soles (incluye IGV)</li>
					<li>Pedido confirmado al recibir pago</li>
					<li>Puedes cancelar antes del envío</li>
					<li>Reembolso en 7-15 días hábiles</li>
				</ul>
			</section>
			
			<section id="preguntas" class="scroll-mt-24">
				<div class="flex items-center gap-3 mb-6 not-prose">
					<HelpCircle size={32} class="text-primary" />
					<h1 class="text-3xl font-black">Preguntas Frecuentes</h1>
				</div>
				
				<div class="space-y-4 not-prose">
					<details class="group rounded-lg border border-border-light dark:border-border-dark">
						<summary class="px-6 py-4 cursor-pointer font-semibold flex items-center justify-between">
							¿Los productos son originales?
							<ChevronRight size={20} class="group-open:rotate-90 transition-transform" />
						</summary>
						<div class="px-6 pb-4 text-slate-600 dark:text-slate-400">
							Sí, 100% originales con garantía oficial. Trabajamos con distribuidores autorizados.
						</div>
					</details>
					
					<details class="group rounded-lg border border-border-light dark:border-border-dark">
						<summary class="px-6 py-4 cursor-pointer font-semibold flex items-center justify-between">
							¿Cuánto demora el envío?
							<ChevronRight size={20} class="group-open:rotate-90 transition-transform" />
						</summary>
						<div class="px-6 pb-4 text-slate-600 dark:text-slate-400">
							Lima: 24-48h. Provincias: 3-7 días hábiles.
						</div>
					</details>
					
					<details class="group rounded-lg border border-border-light dark:border-border-dark">
						<summary class="px-6 py-4 cursor-pointer font-semibold flex items-center justify-between">
							¿Qué métodos de pago aceptan?
							<ChevronRight size={20} class="group-open:rotate-90 transition-transform" />
						</summary>
						<div class="px-6 pb-4 text-slate-600 dark:text-slate-400">
							Tarjetas (Visa, Mastercard), Yape, Plin, transferencia bancaria.
						</div>
					</details>
				</div>
			</section>
		</div>
	</div>
</div>
