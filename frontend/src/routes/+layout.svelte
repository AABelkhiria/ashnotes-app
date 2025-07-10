<script lang="ts">
	import '../app.css';
	import { theme } from '$lib/themeStore';
	import { onMount } from 'svelte';
	import NoteTree from '$lib/NoteTree.svelte';
	import Settings from '$lib/Settings.svelte';

	onMount(() => {
		const savedTheme = localStorage.getItem('theme') || 'dark';
		theme.set(savedTheme);
	});
</script>

<div class="app-container" data-theme={$theme}>
	<aside class="sidebar">
		<NoteTree />
		<div class="settings-container">
			<Settings />
		</div>
	</aside>
	<main class="content">
		<slot></slot>
	</main>
</div>

<style>
	.app-container {
		display: flex;
		height: 100vh;
		font-family: 'Inter', sans-serif;
		background-color: var(--bg-color);
		color: var(--text-color);
		transition:
			background-color 0.3s ease,
			color 0.3s ease;
	}

	.sidebar {
		width: 250px;
		background-color: var(--surface-color);
		padding: 1rem;
		border-right: 1px solid var(--border-color);
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		transition:
			background-color 0.3s ease,
			border-color 0.3s ease;
	}

	.content {
		flex-grow: 1;
		padding: 1rem;
		overflow-y: auto;
		background-color: var(--bg-color);
		transition: background-color 0.3s ease;
	}

	.settings-container {
		margin-top: auto;
	}
</style>
