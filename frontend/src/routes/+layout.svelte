<script lang="ts">
	import '../app.css';
	import { theme } from '$lib/themeStore';
	import { onMount } from 'svelte';
	import NoteTree from '$lib/NoteTree.svelte';
	import Settings from '$lib/Settings.svelte';
	import { checkInitializedApi } from '$lib/api';
	import { isSidebarCollapsed } from '$lib/sidebarStore';
	import Icon from '$lib/Icon.svelte';
	import DebugPanel from '$lib/DebugPanel.svelte';

	let isHovered = false;
	let hoverTimeout: number;
	let windowWidth: number;

	$: isSmallScreen = windowWidth < 600;

	$: if (isSmallScreen) {
		isSidebarCollapsed.set(true);
	}

	$: isCollapsed = $isSidebarCollapsed && !isHovered;

	onMount(async () => {
		const savedTheme = localStorage.getItem('theme') || 'dark';
		theme.set(savedTheme);
		await checkInitializedApi();
	});

	function handleMouseEnter() {
		if ($isSidebarCollapsed) {
			hoverTimeout = setTimeout(() => {
				isHovered = true;
			}, 1000);
		}
	}

	function handleMouseLeave() {
		clearTimeout(hoverTimeout);
		isHovered = false;
	}
</script>

<svelte:window bind:innerWidth={windowWidth} />

<div class="app-container" data-theme={$theme}>
	<aside
		class="sidebar"
		class:collapsed={isCollapsed}
		on:mouseenter={handleMouseEnter}
		on:mouseleave={handleMouseLeave}
	>
		<div class="sidebar-content">
			<NoteTree {isCollapsed} />
		</div>
		<div class="footer">
			{#if !isCollapsed}
				<Settings />
			{/if}
			<div class="collapse-button-wrapper">
				<button
					class="icon-button"
					on:click={() => isSidebarCollapsed.update((v) => !v)}
					disabled={isSmallScreen}
				>
					<Icon name={isCollapsed ? 'arrowRight' : 'arrowLeft'} size={24} />
				</button>
			</div>
		</div>
	</aside>
	<main class="content">
		<slot />
	</main>
	<DebugPanel />
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
		min-width: 250px;
		background-color: var(--surface-color);
		padding: 1rem;
		border-right: 1px solid var(--border-color);
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		transition: all 0.3s ease-in-out;
	}

	.sidebar.collapsed {
		width: 48px;
		min-width: 48px;
		padding: 1rem 12px;
	}

	.content {
		flex-grow: 1;
		padding: 0.5rem;
		overflow-y: auto;
		background-color: var(--bg-color);
		transition: background-color 0.3s ease;
	}

	.footer {
		margin-top: auto;
		display: flex;
		align-items: center;
	}

	.collapse-button-wrapper {
		margin-left: auto;
	}
</style>
