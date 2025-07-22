import { writable } from 'svelte/store';

export const logs = writable<string[]>([]);

export function addLog(message: string) {
    logs.update(currentLogs => {
        const timestamp = new Date().toLocaleTimeString();
        return [...currentLogs, `[${timestamp}] ${message}`];
    });
}
