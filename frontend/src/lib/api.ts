import { get, writable } from 'svelte/store';
import { backendUrl } from '$lib/noteStore';
import { githubToken, noteRepo, appIdentifier } from '$lib/settingsStore';

// const isDesktop = typeof window !== 'undefined' && window.__TAURI__;
const isDesktop = import.meta.env.VITE_BUILD_TARGET === 'desktop';

export const isInitialized = writable(isDesktop ? false : true);

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
        log('Setting credentials for desktop app.');
        const { invoke } = await import('@tauri-apps/api/core');
        await invoke('set_credentials', {
            githubToken: get(githubToken),
            notesRepo: get(noteRepo),
            appIdentifier: get(appIdentifier),
        });
        isInitialized.set(true);
        log('Credentials set.');
    }
}

export async function listNotes(): Promise<Note[]> {
	if (settingsAreEmpty()) {
		log('Settings are empty, returning empty list of notes.');
		return [];
	}
    if (isDesktop && !get(isInitialized)) {
        log('Desktop app not initialized, returning empty list of notes.');
        return [];
    }
    if (isDesktop) {
        const { invoke } = await import('@tauri-apps/api/core');
        log('Invoking list_notes');
        const notes = await invoke('list_notes');
        log('Received notes:', notes);
        return notes as Note[];
    } else {
        log('Fetching notes from backend server.');
        const res = await fetch(`${get(backendUrl)}/api/notes`, {
            headers: getHeaders()
        });
        if (!res.ok) throw new Error(await res.text());
        const notes = await res.json();
        log('Received notes:', notes);
        return notes;
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
        log(`Invoking get_note for path: ${path}`);
        const note = await invoke('get_note', { path });
        log('Received note:', note);
        if (note) {
            return note as Note;
        } else {
            throw new Error("Note not found");
        }
    } else {
        log(`Fetching note for path: ${path}`);
        const res = await fetch(`${get(backendUrl)}/api/notes/${path}`, {
            headers: getHeaders()
        });
        if (!res.ok) throw new Error(await res.text());
        const note = await res.json();
        log('Received note:', note);
        return note;
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
        log(`Invoking create_note for path: ${path}`);
        const { invoke } = await import('@tauri-apps/api/core');
        await invoke('create_note', { path, content });
        log('Note created.');
    } else {
        log(`Creating note for path: ${path}`);
        const res = await fetch(`${get(backendUrl)}/api/notes`, {
            method: 'POST',
            headers: getHeaders(),
            body: JSON.stringify({ path, content }),
        });
        if (!res.ok) throw new Error(await res.text());
        log('Note created.');
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
        log(`Invoking update_note for path: ${path}`);
        const { invoke } = await import('@tauri-apps/api/core');
        await invoke('update_note', { path, content });
        log('Note updated.');
    } else {
        log(`Updating note for path: ${path}`);
        const res = await fetch(`${get(backendUrl)}/api/notes/${path}`, {
            method: 'PUT',
            headers: getHeaders(),
            body: JSON.stringify({ content }),
        });
        if (!res.ok) throw new Error(await res.text());
        log('Note updated.');
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
        log(`Invoking delete_note for path: ${path}`);
        const { invoke } = await import('@tauri-apps/api/core');
        await invoke('delete_note', { path });
        log('Note deleted.');
    } else {
        log(`Deleting note for path: ${path}`);
        const res = await fetch(`${get(backendUrl)}/api/notes/${path}`, {
            method: 'DELETE',
            headers: getHeaders(),
        });
        if (!res.ok) throw new Error(await res.text());
        log('Note deleted.');
    }
}

import { addLog } from './logStore';

export async function log(message: string): Promise<void> {
    addLog(message); // Add message to our Svelte store
    if (isDesktop) {
        const { invoke } = await import('@tauri-apps/api/core');
        await invoke('log_message', { message });
    } else {
        // In a browser environment, we can just log to the console.
        console.log(message);
    }
}
