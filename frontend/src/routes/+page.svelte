<script lang="ts">
	import NoteEditor from '../lib/NoteEditor.svelte';
	import { triggerRefresh } from '../lib/noteStore';
	import { onMount } from 'svelte';
	import { getNote, updateNote, deleteNote, isInitialized } from '../lib/api';

	let content: string = '';
	let notePath: string = 'README.md';
	let successMessage: string | null = null;
	let loading: boolean = true;

	onMount(() => {
		if (import.meta.env.VITE_BUILD_TARGET === 'desktop') {
			const unsubscribe = isInitialized.subscribe((initialized) => {
				if (initialized) {
					fetchNoteContent();
				}
			});

			return () => {
				unsubscribe();
			};
		} else {
			(async () => {
				await fetchNoteContent();
			})();
		}
	});


	async function fetchNoteContent() {
		loading = true;
		try {
			const note = await getNote(notePath);
			content = note.content || '';
		} catch (error: any) {
			console.error('Error fetching note content:', error);
			content = `Error fetching note: ${error.message || error}`;
		} finally {
			loading = false;
		}
	}

	async function handleSave(newContent: string) {
		try {
			await updateNote(notePath, newContent);
			successMessage = 'Note saved successfully!';
			setTimeout(() => (successMessage = null), 3000);
			triggerRefresh();
		} catch (error: any) {
			console.error('Error saving note:', error);
			successMessage = 'Failed to save note.';
			setTimeout(() => (successMessage = null), 3000);
		}
	}

	async function handleDelete() {
		if (!confirm(`Are you sure you want to delete ${notePath}?`)) {
			return;
		}
		try {
			await deleteNote(notePath);
			successMessage = 'Note deleted successfully!';
			setTimeout(() => (successMessage = null), 3000);
			triggerRefresh();
			content = '';
			notePath = 'README.md';
			fetchNoteContent();
		} catch (error: any) {
			console.error('Error deleting note:', error);
			successMessage = 'Failed to delete note.';
			setTimeout(() => (successMessage = null), 3000);
		}
	}
</script>

<svelte:head>
	<title>Ash Notes</title>
</svelte:head>

{#if loading}
	<p>Loading note...</p>
{:else}
	<NoteEditor
		bind:content
		{notePath}
		onSave={handleSave}
		onDelete={handleDelete}
		{successMessage}
	/>
{/if}
