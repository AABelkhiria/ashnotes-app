<script lang="ts">
	import { onMount } from 'svelte';
	import { refreshTrigger, triggerRefresh } from './noteStore';
	import NoteTreeItem from './NoteTreeItem.svelte';
	import Icon from './Icon.svelte';

	import { listNotes, createNote, isInitialized } from './api';

	export let isCollapsed: boolean = false;

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
		if (import.meta.env.VITE_BUILD_TARGET === 'desktop') {
			const unsubscribeInitialized = isInitialized.subscribe((initialized) => {
				if (initialized) {
					fetchNotes();
				}
			});

			const unsubscribeRefresh = refreshTrigger.subscribe(() => {
				fetchNotes();
			});

			return () => {
				unsubscribeInitialized();
				unsubscribeRefresh();
			};
		} else {
			const unsubscribe = refreshTrigger.subscribe(() => {
				fetchNotes();
			});
			return unsubscribe;
		}
	});
</script>

<div class="note-tree">
	<div class="note-tree-header" class:collapsed={isCollapsed}>
		<a href="/notes/README.md" class="logo-link">
			<img src="/favicon.svg" alt="Logo" class="logo-icon" />
		</a>
		{#if !isCollapsed}
			<button class="icon-button" on:click={handleNewRootNote} title="New Root Note">
				<Icon name="plus" />
			</button>
		{/if}
	</div>
	{#if !isCollapsed}
		{#if loading}
			<p>Loading...</p>
		{:else if errorMessage}
			<p class="error">{errorMessage}</p>
		{:else if notes.length === 0}
			<p>Settings are empty or invalid. Please configure your settings.</p>
		{:else}
			<ul>
				{#each notes as note}
					<li>
						<NoteTreeItem item={note} level={0} />
					</li>
				{/each}
			</ul>
		{/if}
	{/if}
</div>

<style>
	.logo-link {
		display: inline-block;
		line-height: 0;
	}

	.logo-icon {
		height: 32px;
		width: 32px;
		transition: transform 0.3s ease;
	}

	.logo-link:hover .logo-icon {
		transform: scale(1.1);
	}

	.note-tree-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.note-tree-header.collapsed {
		justify-content: center;
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
