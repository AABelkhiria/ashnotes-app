import { writable } from 'svelte/store';
import { setCredentials } from '$lib/api';

function createStoredWritable<T>(key: string, defaultValue: T) {
	const initialValue =
		typeof window !== 'undefined' ? JSON.parse(localStorage.getItem(key) || 'null') ?? defaultValue : defaultValue;

	const { subscribe, set } = writable<T>(initialValue);

	subscribe(async (value) => {
		if (typeof window !== 'undefined') {
			localStorage.setItem(key, JSON.stringify(value));
			await setCredentials();
		}
	});

	return {
		subscribe,
		set
	};
}

export const githubToken = createStoredWritable<string>('githubToken', '');
export const noteRepo = createStoredWritable<string>('noteRepo', '');
export const appIdentifier = createStoredWritable<string>('appIdentifier', 'note-app-v1');
