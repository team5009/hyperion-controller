<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window'
    import { onMount } from 'svelte';

    let minimizeButton: HTMLDivElement;
    let maximizeButton: HTMLDivElement;
    let closeButton: HTMLDivElement;
    let windowMaximized = false;

    onMount(async () => {
        minimizeButton.addEventListener('click', async () => {
            await appWindow.minimize()
        })
        maximizeButton.addEventListener('click', async () => {
            if (windowMaximized) {
                await appWindow.unmaximize()
                windowMaximized = false
            } else {
                await appWindow.maximize()
                windowMaximized = true
            }

            console.log(windowMaximized)
        })
        closeButton.addEventListener('click', async () => {
            await appWindow.close()
        })
    })
</script>

<div data-tauri-drag-region class="titlebar" bind:this={minimizeButton}>
    <div class="titlebar-button" id="titlebar-minimize">
        <img
        src="https://api.iconify.design/mdi:window-minimize.svg"
        alt="minimize"
        />
    </div>
    <div class="titlebar-button" id="titlebar-maximize" bind:this={maximizeButton}>
        <img
        src="https://api.iconify.design/mdi:window-maximize.svg"
        alt="maximize"
        />
    </div>
    <div class="titlebar-button" id="titlebar-close" bind:this={closeButton}>
        <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
    </div>
</div>