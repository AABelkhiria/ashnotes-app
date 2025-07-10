import { writable } from 'svelte/store';

function createStoredWritable<T>(key: string, defaultValue: T) {
	const initialValue =
		typeof window !== 'undefined' ? JSON.parse(localStorage.getItem(key) || 'null') ?? defaultValue : defaultValue;

	const { subscribe, set, update } = writable<T>(initialValue);

	return {
		subscribe,
		set: (value: T) => {
			if (typeof window !== 'undefined') {
				localStorage.setItem(key, JSON.stringify(value));
			}
			set(value);
		},
		update
	};
}

export const githubToken = createStoredWritable<string>('githubToken', '');
export const noteRepo = createStoredWritable<string>('noteRepo', '');
export const appIdentifier = createStoredWritable<string>('appIdentifier', '');
