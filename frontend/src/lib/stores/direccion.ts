import { writable } from 'svelte/store';
import type { Direccion } from '$lib/services/direccion';

// Store para las direcciones del usuario
export const direcciones = writable<Direccion[]>([]);

// Store para la dirección seleccionada en el checkout
export const direccionSeleccionada = writable<Direccion | null>(null);

// Función helper para marcar una dirección como seleccionada
export function seleccionarDireccion(direccion: Direccion | null) {
	direccionSeleccionada.set(direccion);
}

// Función helper para actualizar la lista de direcciones
export function setDirecciones(lista: Direccion[]) {
	direcciones.set(lista);

	// Si hay una dirección predeterminada y no hay ninguna seleccionada, seleccionarla automáticamente
	const predeterminada = lista.find(d => d.es_predeterminada);
	if (predeterminada) {
		direccionSeleccionada.update(current => current ?? predeterminada);
	}
}
