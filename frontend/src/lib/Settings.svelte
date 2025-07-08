<script lang="ts">
	import { theme, toggleTheme } from './themeStore';
	import { backendUrl } from './noteStore';
	import Icon from './Icon.svelte';

	let showSettings = false;
	let urlInput = '';

	backendUrl.subscribe((value) => {
		urlInput = value;
	});

	function saveSettings() {
		backendUrl.set(urlInput);
		showSettings = false;
	}
</script>

<div class="settings-container">
	<button class="icon-button" on:click={() => (showSettings = !showSettings)}>
		<Icon name="settings" />
	</button>

	{#if showSettings}
		<div
			class="modal-overlay"
			on:click={() => (showSettings = false)}
			on:keydown={(e) => e.key === 'Escape' && (showSettings = false)}
			role="button"
			tabindex="0"
		></div>
		<div class="modal">
			<h2>Settings</h2>
			<div class="setting">
				<label for="backendUrl">Backend URL</label>
				<input type="text" id="backendUrl" bind:value={urlInput} />
			</div>
			<div class="setting">
				<label for="darkMode">Dark Mode</label>
				<button on:click={toggleTheme}>
					{`Switch to ${$theme === 'dark' ? 'Light' : 'Dark'} Mode`}
				</button>
			</div>
			<button on:click={saveSettings}>Save</button>
		</div>
	{/if}
</div>

<style>
	.settings-container {
		position: relative;
	}
	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background: rgba(0, 0, 0, 0.5);
		z-index: 999;
	}
	.modal {
		position: fixed;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		background: var(--surface-color);
		padding: 2rem;
		border-radius: 8px;
		z-index: 1000;
		box-shadow: 0 4px 15px var(--shadow-color);
	}
	.setting {
		margin-bottom: 1rem;
	}
	.setting label {
		display: block;
		margin-bottom: 0.5rem;
	}
	.setting input {
		width: 100%;
		padding: 0.5rem;
		border-radius: 4px;
		border: 1px solid var(--border-color);
	}
</style>
