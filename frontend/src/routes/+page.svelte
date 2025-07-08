<script lang="ts">
	import NoteEditor from '../lib/NoteEditor.svelte';
	import { triggerRefresh } from '../lib/noteStore';
	import { onMount } from 'svelte';

	let content: string = '';
	let notePath: string = 'README.md';
	let successMessage: string | null = null;

	onMount(async () => {
		await fetchNoteContent();
	});

	async function fetchNoteContent() {
		try {
			const response = await fetch(`/api/notes/${notePath}`);
			if (response.ok) {
				content = await response.text();
			} else if (response.status === 404) {
				content = '';
			} else {
				throw new Error(`Failed to fetch note: ${response.statusText}`);
			}
		} catch (error) {
			console.error('Error fetching note content:', error);
			content = '# Error loading note\n\nCould not load the note content.';
		}
	}

	async function handleSave(newContent: string) {
		try {
			const response = await fetch(`/api/notes/${notePath}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'text/plain' },
				body: newContent
			});
			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			}
			successMessage = 'Note saved successfully!';
			setTimeout(() => (successMessage = null), 3000);
			triggerRefresh();
		} catch (error) {
			console.error('Error saving note:', error);
			successMessage = 'Failed to save note.';
			setTimeout(() => (successMessage = null), 3000);
		}
	}

	function handleNoteDeleted() {
		triggerRefresh();
		content = '';
		notePath = 'README.md';
		successMessage = 'Note deleted successfully!';
		setTimeout(() => (successMessage = null), 3000);
	}
</script>

<svelte:head>
	<title>Note App</title>
</svelte:head>

<NoteEditor
	bind:content
	{notePath}
	onSave={handleSave}
	onDelete={handleNoteDeleted}
	{successMessage}
/>
