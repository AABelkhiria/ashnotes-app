import { get, writable } from 'svelte/store';
import { backendUrl } from '$lib/noteStore';
import { githubToken, noteRepo, appIdentifier } from '$lib/settingsStore';

const isDesktop = import.meta.env.VITE_BUILD_TARGET === 'desktop';

export const isInitialized = writable(false);

export interface Note {
    path: string;
    sha: string;
    note_type: 'file' | 'dir';
    content?: string;
    children?: Note[];
}

export async function checkInitializedApi() {
    if (isDesktop) {
        const { invoke } = await import('@tauri-apps/api/core');
        const initialized = await invoke('is_initialized');
        isInitialized.set(initialized as boolean);
        return initialized;
    }
    return true;
}

// Check on startup
checkInitializedApi();

function getHeaders() {
    return {
        'Content-Type': 'application/json',
        'GITHUB_TOKEN': get(githubToken),
        'NOTES_REPO': get(noteRepo),
        'APP_IDENTIFIER': get(appIdentifier),
    };
}

function settingsAreEmpty() {
	return !get(githubToken) || !get(noteRepo);
}

export async function setCredentials() {
    if (isDesktop) {
        const { invoke } = await import('@tauri-apps/api/core');
        await invoke('set_credentials', {
            githubToken: get(githubToken),
            notesRepo: get(noteRepo),
            appIdentifier: get(appIdentifier),
        });
        isInitialized.set(true);
    }
}

export async function listNotes(): Promise<Note[]> {
	if (settingsAreEmpty()) {
		return [];
	}
    if (isDesktop && !get(isInitialized)) {
        return [];
    }
    if (isDesktop) {
        const { invoke } = await import('@tauri-apps/api/core');
        return await invoke('list_notes');
    } else {
        const res = await fetch(`${get(backendUrl)}/api/notes`, {
            headers: getHeaders()
        });
        if (!res.ok) throw new Error(await res.text());
        return await res.json();
    }
}

export async function getNote(path: string): Promise<Note> {
	if (settingsAreEmpty()) {
		throw new Error('Settings are not configured: GitHub token or notes repository is empty.');
	}
    if (isDesktop && !get(isInitialized)) {
        throw new Error('Application is not initialized.');
    }
    if (isDesktop) {
        const { invoke } = await import('@tauri-apps/api/core');
        const note = await invoke('get_note', { path });
        if (note) {
            return note as Note;
        } else {
            throw new Error("Note not found");
        }
    } else {
        const res = await fetch(`${get(backendUrl)}/api/notes/${path}`, {
            headers: getHeaders()
        });
        if (!res.ok) throw new Error(await res.text());
        return await res.json();
    }
}

export async function createNote(path: string, content: string): Promise<void> {
	if (settingsAreEmpty()) {
		throw new Error('Settings are not configured: GitHub token or notes repository is empty.');
	}
    if (isDesktop && !get(isInitialized)) {
		throw new Error('Application is not initialized.');
	}
    if (isDesktop) {
        const { invoke } = await import('@tauri-apps/api/core');
        await invoke('create_note', { path, content });
    } else {
        const res = await fetch(`${get(backendUrl)}/api/notes`, {
            method: 'POST',
            headers: getHeaders(),
            body: JSON.stringify({ path, content }),
        });
        if (!res.ok) throw new Error(await res.text());
    }
}

export async function updateNote(path: string, content: string): Promise<void> {
	if (settingsAreEmpty()) {
		throw new Error('Settings are not configured: GitHub token or notes repository is empty.');
	}
    if (isDesktop && !get(isInitialized)) {
		throw new Error('Application is not initialized.');
	}
    if (isDesktop) {
        const { invoke } = await import('@tauri-apps/api/core');
        await invoke('update_note', { path, content });
    } else {
        const res = await fetch(`${get(backendUrl)}/api/notes/${path}`, {
            method: 'PUT',
            headers: getHeaders(),
            body: JSON.stringify({ content }),
        });
        if (!res.ok) throw new Error(await res.text());
    }
}

export async function deleteNote(path: string): Promise<void> {
	if (settingsAreEmpty()) {
		throw new Error('Settings are not configured: GitHub token or notes repository is empty.');
	}
    if (isDesktop && !get(isInitialized)) {
		throw new Error('Application is not initialized.');
	}
    if (isDesktop) {
        const { invoke } = await import('@tauri-apps/api/core');
        await invoke('delete_note', { path });
    } else {
        const res = await fetch(`${get(backendUrl)}/api/notes/${path}`, {
            method: 'DELETE',
            headers: getHeaders(),
        });
        if (!res.ok) throw new Error(await res.text());
    }
}
