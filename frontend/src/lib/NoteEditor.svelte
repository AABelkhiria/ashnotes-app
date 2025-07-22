<script lang="ts">
	import { marked } from 'marked';
	import Icon from './Icon.svelte';

	export let content: string;
	export let notePath: string;
	export let onSave: (content: string) => void;
	export let onDelete: () => void;
	export let successMessage: string | null;
	export let isCreating: boolean = false;

	let editMode = false;
	let showSuccessMessage = false;

	$: if (successMessage) {
		showSuccessMessage = true;
		setTimeout(() => {
			successMessage = null;
			showSuccessMessage = false;
		}, 5000);
	}

	$: cleanPath = notePath.endsWith('/README.md')
		? notePath.slice(0, -'/README.md'.length)
		: notePath === 'README.md'
			? ''
			: notePath;
	$: pathSegments = cleanPath.split('/').filter(Boolean);

	function getPath(index: number) {
		return `/notes/${pathSegments.slice(0, index + 1).join('/')}`;
	}
</script>

<div class="note-editor">
	<div class="note-header">
		<div class="title-container">
			<button class="icon-button" on:click={() => (editMode = !editMode)}>
				<Icon name={editMode ? 'pencil' : 'eye'} />
			</button>
			<h2>
				<a href="/notes/README.md">/</a>
				{#each pathSegments as segment, i}
					<a href={getPath(i)}>{segment}</a>
					{#if i < pathSegments.length - 1}
						<span>/</span>
					{/if}
				{/each}
			</h2>
		</div>
		<div class="note-actions">
			<button class="icon-button" on:click={() => onSave(content)}><Icon name="save" /></button>
			{#if !isCreating}
				<button class="icon-button delete-button" on:click={onDelete}><Icon name="trash" /></button>
			{/if}
		</div>
	</div>
	{#if editMode}
		<textarea bind:value={content}></textarea>
	{:else}
		<div class="markdown-preview">
			{@html marked(content)}
		</div>
	{/if}
	{#if showSuccessMessage}
		<p class="success-message-centered" class:hidden={!showSuccessMessage}>{successMessage}</p>
	{/if}
</div>

<style>
	.note-editor {
		display: flex;
		flex-direction: column;
		height: 100%;
	}
	.success-message-centered {
		text-align: center;
		margin-top: 1rem;
		color: #28a745;
		opacity: 1;
		transition: opacity 3s ease-in-out;
	}

	.success-message-centered.hidden {
		opacity: 0;
	}
	.note-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.5rem;
	}

	.title-container {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.icon-button {
		background: none;
		border: none;
		cursor: pointer;
		padding: 0;
		display: flex;
		align-items: center;
	}

	.note-editor h2 {
		color: var(--text-color);
		margin-top: 0;
		margin-bottom: 0;
		transition: color 0.3s ease;
	}

	.note-editor h2 a {
		text-decoration: none;
		color: inherit;
	}

	.note-editor h2 span {
		margin: 0 0.25rem;
	}

	.note-editor textarea {
		width: 100%;
		flex-grow: 1;
		padding: 1rem;
		border: 1px solid var(--border-color);
		border-radius: 8px;
		font-family: 'Inter', sans-serif;
		font-size: 1rem;
		background-color: var(--surface-color);
		color: var(--text-color);
		box-shadow: 0 2px 5px var(--shadow-color);
		resize: vertical;
		transition:
			background-color 0.3s ease,
			color 0.3s ease,
			border-color 0.3s ease,
			box-shadow 0.3s ease;
	}

	.markdown-preview {
		padding: 1rem;
		border: 1px solid var(--border-color);
		border-radius: 8px;
		background-color: var(--surface-color);
		color: var(--text-color);
		flex-grow: 1;
		overflow-y: auto;
	}

	.note-actions {
		display: flex;
		gap: 1rem;
	}

	
</style>
