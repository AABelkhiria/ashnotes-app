import { writable } from 'svelte/store';

export const logs = writable<string[]>([]);

export function addLog(message: string) {
    if (import.meta.env.DEBUG_BUILD) {
        logs.update(currentLogs => {
            const timestamp = new Date().toLocaleTimeString();
            return [...currentLogs, `[${timestamp}] ${message}`];
        });
    }
}
