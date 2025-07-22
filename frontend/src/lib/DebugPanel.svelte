<script lang="ts">
    import { logs } from './logStore';
    import { onMount } from 'svelte';

    let showLogs = false;
    let logContainer: HTMLElement;

    // Scroll to bottom when logs update
    $: if (logContainer) {
        logContainer.scrollTop = logContainer.scrollHeight;
    }

    onMount(() => {
        // Initial scroll to bottom if logs are already present
        if (logContainer) {
            logContainer.scrollTop = logContainer.scrollHeight;
        }
    });
</script>

<div class="debug-panel-container">
    <button class="debug-toggle-button" on:click={() => (showLogs = !showLogs)}>
        {showLogs ? 'Hide Logs' : 'Show Logs'}
    </button>

    {#if showLogs}
        <div class="log-display" bind:this={logContainer}>
            {#each $logs as logEntry}
                <p>{logEntry}</p>
            {/each}
        </div>
    {/if}
</div>

<style>
    .debug-panel-container {
        position: fixed;
        bottom: 10px;
        right: 10px;
        z-index: 1000;
    }

    .debug-toggle-button {
        background-color: #4CAF50; /* Green */
        border: none;
        color: white;
        padding: 10px 15px;
        text-align: center;
        text-decoration: none;
        display: inline-block;
        font-size: 14px;
        margin: 4px 2px;
        cursor: pointer;
        border-radius: 5px;
    }

    .log-display {
        background-color: #222;
        color: #0f0;
        border: 1px solid #555;
        padding: 10px;
        width: 300px;
        height: 200px;
        overflow-y: scroll;
        font-family: monospace;
        font-size: 12px;
        border-radius: 5px;
        margin-top: 5px;
    }

    .log-display p {
        margin: 0;
        padding: 2px 0;
        border-bottom: 1px solid #333;
    }

    .log-display p:last-child {
        border-bottom: none;
    }
</style>
