import { writable, type Writable } from 'svelte/store';

export const refreshTrigger = writable(0);

export function triggerRefresh() {
	refreshTrigger.update((n) => n + 1);
}

export const activeMenu = writable<string | null>(null);

export const backendUrl = writable<string>('');
