import { writable } from 'svelte/store';
import { browser } from '$app/environment';

const MAX_HISTORY = 10;

function createSearchHistory() {
  const initialHistory = browser ? JSON.parse(localStorage.getItem('searchHistory') || '[]') : [];
  const { subscribe, set, update } = writable<string[]>(initialHistory);

  return {
    subscribe,
    addSearch: (term: string) => {
      if (!term.trim()) return;
      
      update(history => {
        const newHistory = [term, ...history.filter(t => t !== term)].slice(0, MAX_HISTORY);
        if (browser) {
          localStorage.setItem('searchHistory', JSON.stringify(newHistory));
        }
        return newHistory;
      });
    },
    clearHistory: () => {
      set([]);
      if (browser) {
        localStorage.removeItem('searchHistory');
      }
    }
  };
}

export const searchHistory = createSearchHistory();
