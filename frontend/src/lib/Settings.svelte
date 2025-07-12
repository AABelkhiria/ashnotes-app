<script lang="ts">
	import { theme, toggleTheme } from './themeStore';
	import { githubToken, noteRepo, appIdentifier } from './settingsStore';
	import Icon from './Icon.svelte';

	let showSettings = false;
	let githubTokenInput = '';
	let noteRepoInput = '';
	let appIdentifierInput = '';

	githubToken.subscribe((value) => {
		githubTokenInput = value;
	});

	noteRepo.subscribe((value) => {
		noteRepoInput = value;
	});

	appIdentifier.subscribe((value) => {
		appIdentifierInput = value;
	});

	function saveSettings() {
		githubToken.set(githubTokenInput);
		noteRepo.set(noteRepoInput);
		appIdentifier.set(appIdentifierInput);
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
				<label for="githubToken">GitHub Token</label>
				<input type="password" id="githubToken" bind:value={githubTokenInput} />
			</div>
			<div class="setting">
				<label for="noteRepo">Note Repository</label>
				<input type="text" id="noteRepo" bind:value={noteRepoInput} />
			</div>
			<div class="setting">
				<label for="appIdentifier">App Identifier</label>
				<input type="text" id="appIdentifier" bind:value={appIdentifierInput} />
			</div>
			<hr class="divider" />
			<div class="setting theme-toggle">
				<label for="theme-toggle-button">Theme</label>
				<button id="theme-toggle-button" class="icon-button" on:click={toggleTheme}>
					<Icon name={$theme === 'dark' ? 'sun' : 'moon'} />
				</button>
			</div>
			<div class="save-button-container">
				<button on:click={saveSettings}>Save</button>
			</div>
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
	.divider {
		border: none;
		border-top: 1px solid var(--border-color);
		margin: 1.5rem 0;
	}
	.theme-toggle {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.theme-toggle label {
		margin-bottom: 0;
	}
	.save-button-container {
		display: flex;
		justify-content: flex-end;
		margin-top: 2rem;
	}
</style>
