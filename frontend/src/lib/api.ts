// src/lib/api.ts

import { get } from 'svelte/store';
import { backendUrl } from '$lib/noteStore';

const isDesktop = import.meta.env.VITE_BUILD_TARGET === 'desktop';

export interface Note {
    path: string;
    sha: string;
    note_type: 'file' | 'dir';
    content?: string;
    children?: Note[];
}

export async function listNotes(): Promise<Note[]> {
    if (isDesktop) {
        const { invoke } = await import('@tauri-apps/api/tauri');
        return await invoke('list_notes');
    } else {
        const res = await fetch(`${get(backendUrl)}/api/notes`);
        if (!res.ok) throw new Error(await res.text());
        return await res.json();
    }
}

export async function getNote(path: string): Promise<Note> {
    if (isDesktop) {
        const { invoke } = await import('@tauri-apps/api/tauri');
        const note = await invoke('get_note', { path });
        if (note) {
            return note as Note;
        } else {
            throw new Error("Note not found");
        }
    } else {
        const res = await fetch(`${get(backendUrl)}/api/notes/${path}`);
        if (!res.ok) throw new Error(await res.text());
        return await res.json();
    }
}

export async function createNote(path: string, content: string): Promise<void> {
    if (isDesktop) {
        const { invoke } = await import('@tauri-apps/api/tauri');
        await invoke('create_note', { path, content });
    } else {
        const res = await fetch(`${get(backendUrl)}/api/notes`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ path, content }),
        });
        if (!res.ok) throw new Error(await res.text());
    }
}

export async function updateNote(path: string, content: string): Promise<void> {
    if (isDesktop) {
        const { invoke } = await import('@tauri-apps/api/tauri');
        await invoke('update_note', { path, content });
    } else {
        const res = await fetch(`${get(backendUrl)}/api/notes/${path}`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ content }),
        });
        if (!res.ok) throw new Error(await res.text());
    }
}

export async function deleteNote(path: string): Promise<void> {
    if (isDesktop) {
        const { invoke } = await import('@tauri-apps/api/tauri');
        await invoke('delete_note', { path });
    } else {
        const res = await fetch(`${get(backendUrl)}/api/notes/${path}`, {
            method: 'DELETE',
        });
        if (!res.ok) throw new Error(await res.text());
    }
}
