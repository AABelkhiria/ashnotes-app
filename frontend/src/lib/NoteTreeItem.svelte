<script lang="ts">
	import { onMount } from 'svelte';
	import { triggerRefresh, backendUrl } from './noteStore';
	import DeletionProgress from './DeletionProgress.svelte';
	import Icon from './Icon.svelte';
	import { createNote } from './api';
	export let item: NoteItem;
	export let level: number;

	let showDeletionProgress = false;

	interface NoteItem {
		name: string;
		path: string;
		type: 'file' | 'dir';
		children?: NoteItem[] | null;
	}

	function getNoteRoute(apiPath: string): string {
		return apiPath.startsWith('notes/') ? apiPath.substring(6) : apiPath;
	}

	const readme =
		item.type === 'dir' && Array.isArray(item.children)
			? item.children.find((child) => child.name === 'README.md')
			: undefined;

	const displayChildren = Array.isArray(item.children)
		? item.children.filter((child) => child.name !== 'README.md')
		: [];

	async function handleNewNote() {
		const newNoteName = prompt('Enter the name for the new note:', 'new-note');
		if (!newNoteName) return;

		const newNotePath = `${getNoteRoute(item.path)}/${newNoteName}/README.md`;
		const newNoteContent = `# ${newNoteName}`;

		try {
			await createNote(newNotePath, newNoteContent);
			triggerRefresh();
		} catch (error: any) {
			if (error.message.includes('409')) {
				alert(`A note with the name "${newNoteName}" already exists in this directory.`);
			} else {
				console.error('Failed to create note:', error);
				alert(`Failed to create note: ${error.message}`);
			}
		}
	}

	let filesToDelete: string[] = [];
	let subDirPaths: string[] = [];

	async function handleDeleteDirectory() {
		function getDeletionTargets(directoryItem: NoteItem): {
			filesToDelete: string[];
			subDirPaths: string[];
		} {
			const filesToDelete: string[] = [];
			const subDirPaths: string[] = [];

			function recurse(currentItem: NoteItem, currentRelativePath: string) {
				if (currentItem.type === 'dir') {
					const relativePath = currentRelativePath
						? `${currentRelativePath}/${currentItem.name}`
						: currentItem.name;
					if (currentItem !== directoryItem) {
						subDirPaths.push(relativePath);
					}

					const readmePath = `${getNoteRoute(currentItem.path)}/README.md`;
					if (!filesToDelete.includes(readmePath)) {
						filesToDelete.push(readmePath);
					}

					if (currentItem.children && currentItem.children.length > 0) {
						for (const child of currentItem.children) {
							recurse(child, relativePath);
						}
					}
				} else {
					filesToDelete.push(getNoteRoute(currentItem.path));
				}
			}

			recurse(directoryItem, '');

			const uniqueFiles = [...new Set(filesToDelete)];
			uniqueFiles.sort((a, b) => (b.match(/\//g) || []).length - (a.match(/\//g) || []).length);

			return { filesToDelete: uniqueFiles, subDirPaths };
		}

		const targets = getDeletionTargets(item);
		filesToDelete = targets.filesToDelete;
		subDirPaths = targets.subDirPaths;

		showDeletionProgress = true;
	}

	function handleDeletionComplete() {
		showDeletionProgress = false;
		triggerRefresh();
	}
</script>

{#if showDeletionProgress}
	<DeletionProgress
		directoryName={item.name}
		{filesToDelete}
		{subDirPaths}
		on:close={handleDeletionComplete}
	/>
{/if}

<div class="note-item-container" style="padding-left: {level * 0.125}rem;">
	{#if item.type === 'dir'}
		<details class="note-item-details" open>
			<summary>
				<div class="summary-content">
					<div class="name-container">
						<a
							class="note-item-link"
							href="/notes/{getNoteRoute(readme ? readme.path : item.path + '/README.md')}"
							>{item.name}</a
						>
					</div>
					<div class="actions-icons">
						<button class="icon-button" on:click|stopPropagation={handleNewNote} title="New Note">
							<Icon name="plus" />
						</button>
						<button
							class="icon-button"
							on:click|stopPropagation={handleDeleteDirectory}
							title="Delete Directory"
						>
							<Icon name="trash" />
						</button>
					</div>
				</div>
			</summary>
			<ul>
				{#each displayChildren as child}
					<li>
						<svelte:self item={child} level={level + 1} />
					</li>
				{/each}
			</ul>
		</details>
	{:else}
		<a class="note-item-link" href="/notes/{getNoteRoute(item.path)}">{item.name}</a>
	{/if}
</div>

<style>
	.note-item-container {
		position: relative;
	}
	.summary-content {
		display: flex;
		justify-content: space-between;
		align-items: center;
		width: 100%;
		position: relative; /* Establish positioning context for absolute children */
	}
	.name-container {
		display: flex;
		align-items: center;
	}
	.actions-icons {
		display: flex;
		gap: 0.2rem; /* Reduced gap between icons */
		position: absolute;
		right: 0;
		top: 50%;
		transform: translateY(-50%);
	}

	.icon-button {
		background: none;
		border: none;
		cursor: pointer;
		font-size: 1rem; /* Smaller icons */
		padding: 0 0.1rem; /* Reduced padding around icons */
		opacity: 0.7;
		transition: opacity 0.2s ease-in-out;
	}

	.icon-button:hover {
		opacity: 1;
	}

	.note-item-details {
		background-color: var(--surface-color);
		/* Removed border-radius, margin-bottom, box-shadow, and border */
		transition:
			background-color 0.3s ease,
			box-shadow 0.3s ease;
	}

	.note-item-details > summary {
		padding: 0.25rem 0; /* Removed horizontal padding */
		padding-left: 0; /* Ensure no space for marker */
		cursor: pointer;
		font-weight: bold;
		color: var(--primary-color);
		list-style: none; /* Remove default arrow */
		list-style-type: none; /* Explicitly remove list style type */
		transition: color 0.3s ease;
	}

	.note-item-details > summary::-webkit-details-marker {
		font-size: 0 !important;
		width: 0 !important;
		color: transparent !important;
	}

	.note-item-details > summary::marker {
		font-size: 0 !important;
		width: 0 !important;
		color: transparent !important;
	}

	.note-item-details > ul {
		list-style: none; /* Remove bullet points */
		display: block;
		padding: 0; /* Removed all padding */
		text-decoration: none;
		color: var(--text-color);
		border-radius: 4px;
		transition:
			background-color 0.2s ease-in-out,
			color 0.3s ease;
	}

	.note-item-details > ul > li {
		margin-bottom: 2px; /* Add 2 pixels between lines */
	}

	.note-item-link:hover {
		background-color: var(--bg-color);
	}
</style>
