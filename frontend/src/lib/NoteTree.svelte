<script lang="ts">
	import { onMount } from 'svelte';
	import { refreshTrigger, triggerRefresh } from './noteStore';
	import NoteTreeItem from './NoteTreeItem.svelte';
	import Icon from './Icon.svelte';

	import { listNotes, createNote } from './api';

	interface NoteItem {
		name: string;
		path: string;
		type: 'file' | 'dir';
		children: NoteItem[] | null;
	}

	let notes: NoteItem[] = [];
	let errorMessage: string | null = null;
	let loading: boolean = true;

	async function handleNewRootNote() {
		const newNoteName = prompt('Enter the name for the new root note:', 'new-root-note');
		if (!newNoteName) return;

		const newNotePath = `${newNoteName}/README.md`;
		const newNoteContent = `# ${newNoteName}`;

		try {
			await createNote(newNotePath, newNoteContent);
			triggerRefresh();
		} catch (error) {
			console.error('Failed to create root note:', error);
			alert(`Failed to create root note: ${(error as Error).message}`);
		}
	}

	/**
	 * Recursively processes notes from the API to add a 'type' field
	 * and map the 'id' field to 'path'.
	 * @param notesRaw The raw notes array from the API.
	 * @returns A properly typed array of NoteItems.
	 */
	function processNotes(notesRaw: any[]): NoteItem[] {
		if (!Array.isArray(notesRaw)) {
			return [];
		}

		return notesRaw
			.map((note) => {
				const isDir = Array.isArray(note.children);
				return {
					...note,
					path: note.id,
					type: isDir ? 'dir' : 'file',
					children: isDir ? processNotes(note.children) : null
				};
			})
			.filter((note) => note.name !== 'README.md');
	}

	async function fetchNotes() {
		loading = true;
		errorMessage = null;
		try {
			const rawNotes = await listNotes();
			notes = processNotes(rawNotes);
		} catch (error: any) {
			errorMessage = `Failed to fetch or process notes: ${error.message}`;
			console.error(errorMessage);
		} finally {
			loading = false;
		}
	}

	onMount(() => {
		fetchNotes();
		const unsubscribe = refreshTrigger.subscribe(() => {
			fetchNotes();
		});
		return unsubscribe;
	});
</script>

<div class="note-tree">
	<div class="note-tree-header">
		<h2><a href="/notes/README.md">Notes</a></h2>
		<button class="icon-button" on:click={handleNewRootNote} title="New Root Note">
			<Icon name="plus" />
		</button>
	</div>
	{#if loading}
		<p>Loading...</p>
	{:else if errorMessage}
		<p class="error">{errorMessage}</p>
	{:else if notes.length === 0}
		<p>No notes found. Create your first note!</p>
	{:else}
		<ul>
			{#each notes as note}
				<li>
					<NoteTreeItem item={note} level={0} />
				</li>
			{/each}
		</ul>
	{/if}
</div>

<style>
	.note-tree h2 a {
		text-decoration: none;
		color: inherit;
	}

	.note-tree h2 {
		color: var(--text-color);
		margin-top: 0;
		margin-bottom: 0;
		transition: color 0.3s ease;
	}

	.note-tree-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.note-tree ul {
		list-style: none;
		padding-left: 0;
	}

	.note-tree li {
		margin-bottom: 0.5rem;
	}

	.error {
		color: var(--bs-danger-text-emphasis);
		margin-top: 1rem;
	}

	.note-tree p {
		color: var(--text-color);
		transition: color 0.3s ease;
	}
</style>
