import { writable, type Writable } from 'svelte/store';

export const refreshTrigger = writable(0);

export function triggerRefresh() {
	refreshTrigger.update((n) => n + 1);
}

export const activeMenu = writable<string | null>(null);

function createBackendUrlStore(): Writable<string> {
	const initialUrl =
		typeof window !== 'undefined'
			? localStorage.getItem('backendUrl') || 'http://localhost:3000'
			: 'http://localhost:3000';

	const { subscribe, set, update } = writable<string>(initialUrl);

	return {
		subscribe,
		set: (value: string) => {
			if (typeof window !== 'undefined') {
				localStorage.setItem('backendUrl', value);
			}
			set(value);
		},
		update
	};
}

export const backendUrl = createBackendUrlStore();
