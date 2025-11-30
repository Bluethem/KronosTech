<div class="relative w-full flex flex-col group/design-root" style='font-family: Inter, "Noto Sans", sans-serif;'>
<div class="flex flex-col">
<header class="p-4 sm:p-6 lg:px-20 lg:py-8 sticky top-0 z-10 bg-background-light/80 dark:bg-background-dark/80 backdrop-blur-sm border-b border-[#DEE2E6] dark:border-gray-700">
<div class="flex flex-wrap justify-between items-center gap-4">
<p class="text-[#111418] dark:text-white text-3xl md:text-4xl font-black leading-tight tracking-[-0.033em]">Crear Nuevo Producto</p>
</div>
</header>
<main class="px-4 sm:px-6 lg:px-20 py-8 grid grid-cols-1 lg:grid-cols-3 gap-8 mb-28">
<!-- Columna Izquierda -->
<div class="lg:col-span-2 flex flex-col gap-6">
<!-- Información Básica -->
<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[#DEE2E6] dark:border-gray-700">
<h2 class="text-[#111418] dark:text-white text-xl font-bold p-5 border-b border-[#DEE2E6] dark:border-gray-700">Información Básica</h2>
<div class="p-5 grid grid-cols-1 md:grid-cols-2 gap-6">
<label class="flex flex-col md:col-span-2">
<p class="text-sm font-medium leading-normal pb-2">Nombre del Producto *</p>
<input class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 focus:outline-0 focus:ring-0 h-12 p-3 text-base" placeholder="Ej: Laptop Pro 15 pulgadas" value=""/>
</label>
<label class="flex flex-col md:col-span-2">
<p class="text-sm font-medium leading-normal pb-2">Descripción</p>
<textarea class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 focus:outline-0 focus:ring-0 min-h-32 p-3 text-base" placeholder="Describe las características principales de tu producto..."></textarea>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Modelo</p>
<input class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 focus:outline-0 focus:ring-0 h-12 p-3 text-base" placeholder="LP-15-PRO" value=""/>
</label>
<div class="relative flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">SKU *</p>
<input class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#198754] focus:border-[#198754] h-12 p-3 text-base pr-10" placeholder="LP-PRO-15-BLK-512" value="LP-PRO-15-BLK-512"/>
<span class="material-symbols-outlined absolute right-3 top-[43px] text-green-600">check_circle</span>
<p class="text-xs text-green-600 mt-1">SKU disponible</p>
</div>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Código de Barras (EAN, UPC)</p>
<input class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 focus:outline-0 focus:ring-0 h-12 p-3 text-base" placeholder="Ej: 7891234567890" value=""/>
</label>
</div>
</div>
<!-- Categorización -->
<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[#DEE2E6] dark:border-gray-700">
<h2 class="text-[#111418] dark:text-white text-xl font-bold p-5 border-b border-[#DEE2E6] dark:border-gray-700">Categorización</h2>
<div class="p-5 grid grid-cols-1 md:grid-cols-2 gap-6">
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Familia *</p>
<select class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base">
<option>Electrónica</option>
<option>Ropa</option>
</select>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Categoría *</p>
<select class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base">
<option>Computadoras</option>
<option>Smartphones</option>
</select>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Subcategoría (Opcional)</p>
<select class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base">
<option>Laptops</option>
<option>Desktops</option>
</select>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Marca *</p>
<select class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base">
<option>Apple</option>
<option>Dell</option>
</select>
<a class="text-sm text-primary hover:underline mt-2" href="#">+ Agregar nueva marca</a>
</label>
</div>
</div>
<!-- Precios -->
<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[#DEE2E6] dark:border-gray-700">
<h2 class="text-[#111418] dark:text-white text-xl font-bold p-5 border-b border-[#DEE2E6] dark:border-gray-700">Precios</h2>
<div class="p-5 grid grid-cols-1 md:grid-cols-2 gap-6">
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Precio Base (PEN) *</p>
<input class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" placeholder="4500.00" type="number" value=""/>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Precio de Venta (PEN) *</p>
<input class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" placeholder="5200.00" type="number" value=""/>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Costo (Opcional)</p>
<input class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" placeholder="4000.00" type="number" value=""/>
</label>
<div class="flex flex-col justify-end">
<p class="text-sm font-medium leading-normal pb-2">Margen</p>
<div class="bg-accent-light dark:bg-primary/20 h-12 rounded-lg flex items-center px-4">
<p class="text-[#212529] dark:text-white font-semibold">23.08%</p>
</div>
</div>
</div>
</div>
<!-- Inventario -->
<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[#DEE2E6] dark:border-gray-700">
<h2 class="text-[#111418] dark:text-white text-xl font-bold p-5 border-b border-[#DEE2E6] dark:border-gray-700">Inventario Inicial</h2>
<div class="p-5 grid grid-cols-1 md:grid-cols-3 gap-6">
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Cantidad Inicial</p>
<input class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" placeholder="100" type="number"/>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Stock Mínimo</p>
<input class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" type="number" value="5"/>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Ubicación Física</p>
<input class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" placeholder="Almacén A, Pasillo 3"/>
</label>
</div>
</div>
<!-- Físico y Garantía -->
<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[#DEE2E6] dark:border-gray-700">
<h2 class="text-[#111418] dark:text-white text-xl font-bold p-5 border-b border-[#DEE2E6] dark:border-gray-700">Físico y Garantía</h2>
<div class="p-5 grid grid-cols-1 md:grid-cols-3 gap-6">
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Peso (kg)</p>
<input class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" placeholder="1.8" type="number"/>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Dimensiones (cm)</p>
<input class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" placeholder="30x20x10"/>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Garantía (meses)</p>
<input class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" type="number" value="12"/>
</label>
</div>
</div>
</div>
<!-- Columna Derecha -->
<div class="lg:col-span-1 flex flex-col gap-6">
<!-- Imágenes del Producto -->
<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[#DEE2E6] dark:border-gray-700">
<h2 class="text-[#111418] dark:text-white text-xl font-bold p-5 border-b border-[#DEE2E6] dark:border-gray-700">Imágenes del Producto</h2>
<div class="p-5 flex flex-col gap-4">
<div class="border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg p-6 text-center cursor-pointer hover:border-primary dark:hover:border-primary transition">
<span class="material-symbols-outlined text-4xl text-gray-400 dark:text-gray-500 mx-auto">upload_file</span>
<p class="mt-2 text-sm text-[#6C757D] dark:text-gray-400">Arrastra imágenes o haz clic para seleccionar</p>
<button class="mt-4 bg-primary/10 text-primary font-semibold py-2 px-4 rounded-lg text-sm">Seleccionar Imágenes</button>
<p class="text-xs text-[#6C757D] dark:text-gray-500 mt-2">Soporta: JPG, PNG, WEBP. Máx 5MB c/u</p>
</div>
<div class="grid grid-cols-3 gap-3">
<div class="relative group">
<img class="w-full h-24 object-cover rounded-lg" alt="Laptop plateada sobre una mesa de madera" src="https://lh3.googleusercontent.com/aida-public/AB6AXuAzZeRtL0pG5gucImZHcQoVsQQNPOqZ0MunmIbCyJR98KZbQ2JajaaKZZLDgP-ViE2NwovWM11_2K5j9Tboui_iE0jFsJUUvnQpLU8vh52jD6iXboUMYlYtUErq8aZRQXc9i93ZsWfsKoCun0wENz0TwiHOuWFJ-gd1aCTAnVaxsk_SjF7hmhUXNAwGcgSmccrkLkmRU0lCA-K72T5u7zbPDZ_sYREl8HnA49XHg9ALeUkVKdwVJKOVLjIuytdaS_fMBvmsfs3CVS8"/>
<div class="absolute inset-0 bg-black/50 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity rounded-lg">
<button class="text-white"><span class="material-symbols-outlined text-xl">delete</span></button>
</div>
<input checked="" class="absolute top-2 left-2" name="main-image" type="radio"/>
</div>
<div class="relative group">
<img class="w-full h-24 object-cover rounded-lg" alt="Vista frontal de una laptop con pantalla iluminada" src="https://lh3.googleusercontent.com/aida-public/AB6AXuDRzji9NGBkMCbFEaMA4q8Q8cPCTvJtBaNINohx4E5uVkBYYptMTvktLvL--Ki8qgYqjUCWWk77EayCLR-UqYZi18x6puaCaLmUGFEatMvBf4kbG-0KKPs9dw_x04EwlXuruxJ1Lr31rXlHq2lMCWJPDizXEl5p8PBYpaAkaVSX4j93YlgdYECk9OnSdSVn4q1QXPq283fdbJ_2aNraVUgh4MTB18-_zsvWrVF2dO17BS-8rgX0_BwCYVCqzEAS5BdU3sdnF63Cj-4"/>
<div class="absolute inset-0 bg-black/50 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity rounded-lg">
<button class="text-white"><span class="material-symbols-outlined text-xl">delete</span></button>
</div>
<input class="absolute top-2 left-2" name="main-image" type="radio"/>
</div>
</div>
</div>
</div>
<!-- Estado -->
<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[#DEE2E6] dark:border-gray-700">
<h2 class="text-[#111418] dark:text-white text-xl font-bold p-5 border-b border-[#DEE2E6] dark:border-gray-700">Estado</h2>
<div class="p-5 flex flex-col gap-2">
<div class="flex items-center justify-between">
<label class="font-medium text-[#212529] dark:text-white" for="product-status">Estado del Producto</label>
<label class="relative inline-flex items-center cursor-pointer">
<input checked="" class="sr-only peer" id="product-status" type="checkbox"/>
<div class="w-14 h-7 bg-gray-200 peer-focus:outline-none rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:left-[4px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-6 after:w-6 after:transition-all dark:border-gray-600 peer-checked:bg-primary"></div>
<span class="ml-3 text-sm font-medium text-gray-900 dark:text-gray-300">Activo</span>
</label>
</div>
<p class="text-sm text-[#6C757D] dark:text-gray-400">Los productos inactivos no aparecen en la tienda.</p>
</div>
</div>
<!-- Destacados -->
<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[#DEE2E6] dark:border-gray-700">
<h2 class="text-[#111418] dark:text-white text-xl font-bold p-5 border-b border-[#DEE2E6] dark:border-gray-700">Destacados</h2>
<div class="p-5 flex flex-col gap-4">
<label class="flex items-center gap-3">
<input class="h-5 w-5 rounded border-gray-300 text-primary focus:ring-primary" type="checkbox"/>
<span class="text-base text-[#212529] dark:text-white">Es Destacado</span>
</label>
<label class="flex items-center gap-3">
<input class="h-5 w-5 rounded border-gray-300 text-primary focus:ring-primary" type="checkbox"/>
<span class="text-base text-[#212529] dark:text-white">Es Nuevo</span>
</label>
<label class="flex items-center gap-3">
<input class="h-5 w-5 rounded border-gray-300 text-primary focus:ring-primary" type="checkbox"/>
<span class="text-base text-[#212529] dark:text-white">Es Oferta</span>
</label>
</div>
</div>
<!-- SEO -->
<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-[#DEE2E6] dark:border-gray-700">
<details>
<summary class="text-[#111418] dark:text-white text-xl font-bold p-5 flex justify-between items-center cursor-pointer list-none">
Configuración SEO
<span class="material-symbols-outlined transition-transform duration-300 transform">expand_more</span>
</summary>
<div class="p-5 border-t border-[#DEE2E6] dark:border-gray-700 flex flex-col gap-4">
<label class="flex flex-col">
<div class="flex justify-between">
<p class="text-sm font-medium leading-normal pb-2">Meta Título</p>
<p class="text-sm text-[#6C757D]">23/60</p>
</div>
<input class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" value="Laptop Pro 15 pulgadas"/>
</label>
<label class="flex flex-col">
<div class="flex justify-between">
<p class="text-sm font-medium leading-normal pb-2">Meta Descripción</p>
<p class="text-sm text-[#6C757D]">45/160</p>
</div>
<textarea class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 min-h-28 p-3 text-base">La mejor laptop para profesionales creativos.</textarea>
</label>
<label class="flex flex-col">
<p class="text-sm font-medium leading-normal pb-2">Slug URL</p>
<input class="form-input w-full rounded-lg text-[#111418] dark:text-white dark:bg-gray-700 border-[#DEE2E6] dark:border-gray-600 h-12 p-3 text-base" value="laptop-pro-15-pulgadas"/>
<p class="text-xs text-[#6C757D] mt-1">tutienda.com/productos/<strong>laptop-pro-15-pulgadas</strong></p>
</label>
</div>
</details>
</div>
</div>
</main>
</div>
<footer class="fixed bottom-0 left-0 right-0 bg-white dark:bg-gray-800 border-t border-[#DEE2E6] dark:border-gray-700 shadow-lg p-4 z-20">
<div class="max-w-screen-2xl mx-auto px-4 sm:px-6 lg:px-20 flex justify-between items-center">
<button class="px-5 py-2.5 rounded-lg text-base font-semibold text-[#212529] dark:text-white bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600">Cancelar</button>
<div class="flex items-center gap-4">
<button class="px-5 py-2.5 rounded-lg text-base font-semibold text-primary bg-primary/10 hover:bg-primary/20 dark:bg-primary/20 dark:hover:bg-primary/30">Guardar como Borrador</button>
<button class="px-5 py-2.5 rounded-lg text-base font-semibold text-white bg-primary hover:bg-primary/90">Guardar y Publicar</button>
</div>
</div>
</footer>
</div>
