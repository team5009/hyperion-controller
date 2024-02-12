<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
    import { listen } from "@tauri-apps/api/event";
    import { BotSocketConnected, NotificationState } from '$store';
    import { ConnectionStatus, ErrorType } from '$lib';

    let connStatus = ConnectionStatus.Disconnected
    let connMessage = "Connect to Bot"
    let retryCounter = 5
    let disabled = false
    let color = "red"

    onMount(async () => {
        await listen("bot_connect", (response) => {
            const {event, payload} = response as {event: string, payload: {status: ConnectionStatus, message?: string}}

            switch (payload.status) {
                case ConnectionStatus.Connected:
                    connStatus = payload.status
                    BotSocketConnected.set(true)
                    NotificationState.set({type: ErrorType.SUCCESS, message: "Connected to Bot"})
                    connMessage = "Disconnect from Bot"
                    disabled = false
                    break;
                
                case ConnectionStatus.Disconnected:
                    connStatus = payload.status
                    BotSocketConnected.set(false)
                    NotificationState.set({type: ErrorType.SUCCESS, message: "Disconnected from Bot"})
                    connMessage = "Connect to Bot"
                    disabled = false
                    break;

                case ConnectionStatus.Pending:
                    connStatus = payload.status
                    connMessage = payload.message?? ""
                    disabled = true
                    break;

                case ConnectionStatus.Error:
                    connStatus = payload.status
                    connMessage = payload.message?? ""
                    NotificationState.set({type: ErrorType.ERROR, message: connMessage})

                    const interval = setInterval(() => {
                        retryCounter--
                        if (retryCounter <= 0) {
                            clearInterval(interval)
                            retryCounter = 5
                            connStatus = ConnectionStatus.Disconnected
                            BotSocketConnected.set(false)
                            disabled = false
                            connMessage = "Connect to Bot"

                        }
                    }, 1000)

                    break;
                
                default:
                    break;
            }
        })

    })
    

    const connectToBot = async () => {
        const port = 443
        await invoke("connect_to_bot", { port: Math.abs(port) })
    }

    const disconnectFromBot = async () => {
        await invoke("disconnect_bot")
    }

    const handleConnection = async () => {
        if (connStatus == ConnectionStatus.Disconnected) {
            await connectToBot()
        } else if (connStatus == ConnectionStatus.Connected) {
            await disconnectFromBot()
        }
    }

</script>
<div>
    <button class={color} on:click={handleConnection} disabled={disabled}>
        {connMessage}
    </button>
    {#if connStatus == ConnectionStatus.Error}
        <span>
            Retry in {retryCounter}s
        </span>
    {/if}

    <!-- {#if connStatus == ConnectionStatus.Connected}
        <button class={color} on:click={disconnectFromBot}>
            Disconnect from Bot
        </button>
    {:else if connStatus == ConnectionStatus.Disconnected}
        <button class={color} on:click={connectToBot}>
            Connect to Bot
        </button>
    {:else if connStatus == ConnectionStatus.Pending || connStatus == ConnectionStatus.Error}
        <button class={color} disabled>
            {connMessage}
        </button>
        {#if connStatus == ConnectionStatus.Error}
            <span>
                Retry in {retryCounter}s
            </span>
        {/if}
    {/if} -->
</div>

<style>
    div {
        position: relative;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
</style>