import { writable } from 'svelte/store';
import { browser } from '$app/environment';

// Detectar tema del sistema o del localStorage
const getInitialTheme = (): 'light' | 'dark' => {
  if (!browser) return 'light';
  
  const stored = localStorage.getItem('theme');
  if (stored === 'dark' || stored === 'light') {
    // Aplicar inmediatamente al HTML para evitar flash
    if (stored === 'dark') {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
    return stored;
  }
  
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  if (prefersDark) {
    document.documentElement.classList.add('dark');
  }
  return prefersDark ? 'dark' : 'light';
};

export const theme = writable<'light' | 'dark'>(getInitialTheme());

// Sincronizar con localStorage y DOM
if (browser) {
  theme.subscribe((value) => {
    localStorage.setItem('theme', value);
    if (value === 'dark') {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  });
}

export const toggleTheme = () => {
  theme.update((current) => (current === 'light' ? 'dark' : 'light'));
};
