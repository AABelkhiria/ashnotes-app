<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import NoteEditor from '$lib/NoteEditor.svelte';
	import { triggerRefresh, backendUrl } from '$lib/noteStore';
	import { goto } from '$app/navigation';

	let noteContent: string | null = null;
	let notePathForSave: string = '';
	let currentUrlPath: string = '';
	let errorMessage: string | null = null;
	let successMessage: string | null = null;
	let loading: boolean = true;
	let isCreating = false;

	function getParentPath(fullPath: string): string {
		const parts = fullPath.split('/');
		const lastPart = parts[parts.length - 1];
		if (lastPart.includes('.') || lastPart === 'README.md') {
			return parts.slice(0, -1).join('/');
		}
		return fullPath;
	}

	async function fetchNoteContent(path: string) {
		loading = true;
		errorMessage = null;
		noteContent = null;
		isCreating = false;

		try {
			const response = await fetch(`${$backendUrl}/api/notes/${path}`);
			if (response.status === 404) {
				noteContent = '';
				const parentPath = getParentPath(path);
				notePathForSave = `${parentPath}/README.md`;
				isCreating = true;
			} else if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			} else {
				const note = await response.json();
				noteContent = note.content;
				// If the path doesn't contain a file extension, assume it's a directory
				// and the content fetched is for its README.md
				if (!path.includes('.')) {
					notePathForSave = `${path}/README.md`;
				} else {
					notePathForSave = path;
				}
			}
		} catch (error: any) {
			errorMessage = `Failed to fetch note content: ${error.message}`;
			console.error(errorMessage);
			noteContent = null;
		} finally {
			loading = false;
		}
	}

	async function handleSave(content: string) {
		errorMessage = null;
		successMessage = null;
		try {
			const url = isCreating
				? `${$backendUrl}/api/notes`
				: `${$backendUrl}/api/notes/${notePathForSave}`;
			const method = isCreating ? 'POST' : 'PUT';
			const body = isCreating
				? JSON.stringify({ path: notePathForSave, content })
				: JSON.stringify({ content });

			const response = await fetch(url, {
				method,
				headers: {
					'Content-Type': 'application/json'
				},
				body
			});

			if (!response.ok) {
				const errorText = await response.text();
				throw new Error(`HTTP error! status: ${response.status} - ${errorText}`);
			}

			successMessage = `Note ${isCreating ? 'created' : 'updated'} successfully!`;
			triggerRefresh();

			if (isCreating) {
				goto(`/notes/${notePathForSave}`);
			}
		} catch (error: any) {
			errorMessage = `Failed to save note: ${error.message}`;
			console.error(errorMessage);
		}
	}

	async function handleDelete() {
		if (!confirm(`Are you sure you want to delete ${notePathForSave}?`)) {
			return;
		}
		errorMessage = null;
		successMessage = null;
		try {
			const response = await fetch(`${$backendUrl}/api/notes/${notePathForSave}`, {
				method: 'DELETE'
			});
			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			}
			successMessage = 'Note deleted successfully!';
			triggerRefresh();
			goto('/');
		} catch (error: any) {
			errorMessage = `Failed to delete note: ${error.message}`;
			console.error(errorMessage);
		}
	}

	onMount(() => {
		const unsubscribe = backendUrl.subscribe(() => {
			if ($page.params.path) {
				fetchNoteContent($page.params.path);
			}
		});

		return unsubscribe;
	});

	$: if (
		typeof window !== 'undefined' &&
		$page.params.path &&
		$page.params.path !== currentUrlPath
	) {
		currentUrlPath = $page.params.path;
		fetchNoteContent(currentUrlPath);
	}
</script>

<div class="note-page">
	{#if loading}
		<p>Loading note...</p>
	{:else if errorMessage}
		<div class="error-message">
			<p>{errorMessage}</p>
		</div>
	{:else if noteContent !== null}
		<NoteEditor
			bind:content={noteContent}
			notePath={notePathForSave}
			onSave={handleSave}
			onDelete={handleDelete}
			{successMessage}
			{isCreating}
		/>
	{:else}
		<div class="welcome-message">
			<h2>Welcome to Notes</h2>
			<p>Select a note from the list to view or edit it.</p>
		</div>
	{/if}
</div>

<style>
	.note-page {
		padding: 2rem;
		height: 100%;
		overflow-y: auto;
	}

	.error-message,
	.welcome-message {
		text-align: center;
		margin-top: 4rem;
		color: var(--text-color);
	}

	.error-message p {
		color: var(--bs-danger-text-emphasis);
	}
</style>
