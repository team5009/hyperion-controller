<script lang="ts">
  import { onMount } from "svelte";
  import { NotificationState, SettingsModalState, SettingsState } from "$store";
  import { Store } from "tauri-plugin-store-api";
  import Connection from "../extra/Connection.svelte";
  import SettingsInput from "./SettingsInput.svelte";
  import { ErrorType, type Settings } from "$lib";
  import SizeInput from "./SizeInput.svelte";
  import SeasonSelect from "./SeasonSelect.svelte";
  import Measurements from "./Measurements.svelte";
  import { get } from "svelte/store";

    let background: HTMLDivElement;
    let main: HTMLElement;
    let displaySection: 'show-section' | 'hide-section' = "show-section"
    let displayBg: 'show-bg' | 'hide-bg' = "show-bg";

    let showModal = true;
    let currentSettings: Settings
    let store: Store = new Store(".settings.json");
    SettingsModalState.subscribe((value) => {
        if (value) {
            showModal = true;
            setTimeout(() => {
                displaySection = "show-section";
                displayBg = "show-bg";
            }, 1);
        } else {
            displaySection = "hide-section";
            displayBg = "hide-bg";
            setTimeout(() => {
                showModal = false;
            }, 300);
        }
    });

    SettingsState.subscribe((value) => {
        currentSettings = value;
    });


    const closeSettings = () => {
        SettingsModalState.set(false);
    }

    const saveSettings = async () => { 
        await store.set("settings", currentSettings);
        await store.save();
        NotificationState.set({
            type: ErrorType.SUCCESS,
            message: "Settings saved"
        });
    }

</script>

{#if showModal}
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class={`settings_modal_bg ${displayBg}`} on:click={closeSettings} bind:this={background}/>
    <section class={`${displaySection}`} bind:this={main}>
        
        <div class="title">
            <h2>Settings</h2>
        </div>
        <div class="app">
            <SizeInput label="Track Width" bind:value={currentSettings.width}/>
            <SizeInput label="Track Length" bind:value={currentSettings.length} />
            <SeasonSelect/>
            <Measurements/>
        </div>
        <div class="bot">
            <div class="bot_connection">
                <Connection />
            </div>
        </div>

        <button class="save_button" on:click={saveSettings}>
            Save
        </button>
    </section>
{/if}


<style lang="scss">
    .settings_modal_bg {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.5);
        z-index: 1000;
        cursor: pointer;
        transition: all 0.3s ease-in-out;
    }
    section {
        display: grid;
        grid-auto-columns: 1fr;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: 4rem 1fr;
        gap: 0px 0px;
        grid-template-areas:
            "title title"
            "app bot";


        background-color: rgb(27, 27, 34);
        position: fixed;
        top: 50%;
        left: 50%;
        box-shadow: 0rem 1rem 2rem rgba(0, 0, 0, 0.5);

        transform: translate(-50%, -50%);

        width: 80%;
        height: 90%;
        min-width: 30rem;
        min-height: 30rem;
        max-width: 50rem;
        max-height: 30rem;


        z-index: 1000;

        border-radius: 1rem;
        padding: 1rem;

        transition: all 0.3s ease-in-out;

        .bot_connection {
            position: relative;
            width: 100%;
            height: 3rem;
            display: flex;
            justify-content: center;
            align-items: center;
            bottom: 1rem;
        }

        .title {
            grid-area: title;
            display: flex;
            justify-content: center;
            align-items: center;
            h2 {
                color: white;
                font-size: 2rem;
            }
        }

        .app {
            grid-area: app;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
        }

        .bot {
            grid-area: bot;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
        }
    }

    .show-section {
        opacity: 1;
        transform: translate(-50%, -50%) scale(1);
    }

    .hide-section {
        opacity: 0;
        transform: translate(-50%, -50%) scale(0);
    }

    .show-bg {
        opacity: 1;
        backdrop-filter: blur(1rem);
    }

    .hide-bg {
        opacity: 0;
        backdrop-filter: blur(0);
    }

    .save_button {
        background-color: green;
        color: white;
        border: none;
        padding: 1rem;
        width: 7rem;
        border-radius: 1rem;
        transition: all 0.1s ease-in-out;
    }
    .save_button:hover {
        background-color: darkgreen;
        scale: 1.1;
    }
</style>