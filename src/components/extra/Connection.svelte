<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
    import { emit, listen } from "@tauri-apps/api/event";
    import { writable } from "svelte/store";

    enum ConnectionStatus {
        Connected,
        Disconnected,
        Pending,
        Error
    }

    let connStatus = ConnectionStatus.Disconnected
    let connMessage = ""

    onMount(async () => {
        await listen("bot_connect", (response) => {
            const {event, payload} = response as {event: string, payload: {status: ConnectionStatus, message?: string}}

            switch (payload.status) {
                case ConnectionStatus.Connected:
                    connStatus = payload.status
                    break;
                
                case ConnectionStatus.Disconnected:
                    connStatus = payload.status
                    break;

                case ConnectionStatus.Pending:
                    connStatus = payload.status
                    connMessage = payload.message?? ""
                    break;

                case ConnectionStatus.Error:
                    connStatus = payload.status
                    connMessage = payload.message?? ""
                    break;
                
                default:
                    break;
            }
        })

        // unlisten()
    })
    

    const connectToBot = async () => {
        const port = 443
        await invoke("connect_to_bot", { port: Math.abs(port) })
    }

    const disconnectFromBot = async () => {
        await invoke("disconnect_bot")
    }

</script>

{#if connStatus == ConnectionStatus.Connected}
    <button on:click={disconnectFromBot}>
        Disconnect from Bot
    </button>
{:else if connStatus == ConnectionStatus.Disconnected}
    <button on:click={connectToBot}>
        Connect to Bot
    </button>
{:else if connStatus == ConnectionStatus.Pending}
    <button disabled>
        {connMessage}
    </button>
{/if}

<style>
    button {
        position: fixed;
        top: 1rem;
        left: 1rem;
        z-index: 100;
    }
</style>