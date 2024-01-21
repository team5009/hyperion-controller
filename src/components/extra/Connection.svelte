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
    let retryCounter = 5

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

                    const interval = setInterval(() => {
                        retryCounter--
                        if (retryCounter <= 0) {
                            clearInterval(interval)
                            retryCounter = 5
                            connStatus = ConnectionStatus.Disconnected
                        }
                    }, 1000)

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
<div>
    {#if connStatus == ConnectionStatus.Connected}
        <button on:click={disconnectFromBot}>
            Disconnect from Bot
        </button>
    {:else if connStatus == ConnectionStatus.Disconnected}
        <button on:click={connectToBot}>
            Connect to Bot
        </button>
    {:else if connStatus == ConnectionStatus.Pending || connStatus == ConnectionStatus.Error}
        <button disabled>
            {connMessage}
        </button>
        {#if connStatus == ConnectionStatus.Error}
            <span>
                Retry in {retryCounter}s
            </span>
        {/if}
    {/if}
</div>

<style>
    div {
        position: fixed;
        top: 1rem;
        left: 1rem;
        z-index: 100;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
</style>