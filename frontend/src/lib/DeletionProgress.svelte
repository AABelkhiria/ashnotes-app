<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { deleteNote } from './api';

	export let directoryName: string;
	export let filesToDelete: string[];
	export let subDirPaths: string[];

	const dispatch = createEventDispatcher();

	let isDeleting = false;
	let deletionStatus: { path: string; status: 'pending' | 'deleting' | 'deleted' | 'failed' }[] =
		filesToDelete.map((path) => ({ path, status: 'pending' }));

	async function startDeletion() {
		isDeleting = true;
		await new Promise((resolve) => setTimeout(resolve, 0));

		for (let i = 0; i < filesToDelete.length; i++) {
			const file = filesToDelete[i];
			deletionStatus[i].status = 'deleting';
			deletionStatus = [...deletionStatus];

			await new Promise((resolve) => setTimeout(resolve, 50));

			try {
				await deleteNote(file);
				deletionStatus[i].status = 'deleted';
			} catch (error) {
				deletionStatus[i].status = 'failed';
				console.error(`Failed to delete ${file}:`, error);
			}
			deletionStatus = [...deletionStatus];
		}
	}

	function handleClose() {
		dispatch('close');
	}

	function getDisplayPath(path: string): string {
		if (path.endsWith('README.md')) {
			const dirPath = path.substring(0, path.length - '/README.md'.length);
			// If path was 'README.md', dirPath is ''. In that case, show './'
			if (dirPath === '') {
				return './';
			}
			return dirPath;
		}
		return path;
	}
</script>

<div
	class="modal-overlay"
	role="dialog"
	aria-modal="true"
	tabindex="-1"
	on:keydown={(event) => {
		if (event.key === 'Enter') handleClose();
	}}
>
	<div class="modal-content">
		{#if !isDeleting}
			<h2>Confirm Deletion</h2>
			<p>Are you sure you want to delete the directory "{directoryName}" and all its contents?</p>
			{#if subDirPaths.length > 0}
				<p>This will also delete the following subdirectories:</p>
				<ul>
					{#each subDirPaths as path}
						<li>{path}</li>
					{/each}
				</ul>
			{/if}
			<div class="modal-actions">
				<button on:click={startDeletion}>Confirm</button>
				<button on:click={handleClose}>Cancel</button>
			</div>
		{:else}
			<h2>Deleting...</h2>
			<ul>
				{#each deletionStatus as item}
					<li>
						{#if item.status === 'deleted'}
							✅
						{:else if item.status === 'deleting'}
							⏳
						{:else if item.status === 'failed'}
							❌
						{:else}
							⚪
						{/if}
						{getDisplayPath(item.path)}
					</li>
				{/each}
			</ul>
			<div class="modal-actions">
				<button on:click={handleClose}>Close</button>
			</div>
		{/if}
	</div>
</div>

<style>
	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background-color: rgba(0, 0, 0, 0.5);
		display: flex;
		justify-content: center;
		align-items: center;
		z-index: 1000;
	}
	.modal-content {
		background-color: var(--surface-color);
		padding: 2rem;
		border-radius: 8px;
		color: var(--text-color);
		width: 500px;
		max-width: 90%;
	}
	.modal-actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		margin-top: 1rem;
	}

	.modal-content > .modal-actions {
		margin-top: 1.5rem;
		justify-content: center;
	}
</style>
