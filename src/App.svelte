<script lang="ts">
	import Connection from './components/extra/Connection.svelte';
  import Preview from './components/preview/Preview.svelte';
  import MenuTemp from './components/layout/MenuTemp.svelte';
  import { BotPosition, SettingsState, appState, mousePosition } from '$store';
  import { AppState, canvasToField, radToDeg, type Settings } from '$lib';
  import NavBar from './components/layout/NavBar.svelte';
  import {type FadeParams} from 'svelte/transition';
  import FileInput from './components/layout/FileInput.svelte';
  import Ping from './components/extra/Ping.svelte';
  import Menu from './components/extra/Menu.svelte';
  import Notifications from './components/extra/Notifications.svelte';
  import SettingsModal from './components/settings/SettingsModal.svelte';
  import WindowBar from './components/layout/WindowBar.svelte';
  import { onMount } from 'svelte';
  import { Store } from 'tauri-plugin-store-api';
  import { get } from 'svelte/store';

  onMount(async () => {
    let store: Store = new Store(".settings.json");

    if (!(await store.has("settings"))) {
      await store.set("settings", get(SettingsState));
      await store.save();
    }
    const settings = (await store.get("settings")) as Settings;
    SettingsState.set(settings);
  })

    let state: AppState;
    const mousePos = {
        x: "0",
        y: "0"
    };
    const botPos = {
        x: "0",
        y: "0",
        rot: "0"
    };
    appState.subscribe((value) => {
      state = value;
    });

    mousePosition.subscribe((value) => {
        const point = canvasToField(value);
        mousePos.x = point.x.toFixed(1);
        mousePos.y = point.y.toFixed(1);
    })

    BotPosition.subscribe((value) => {
        const point = canvasToField(value);
        botPos.x = point.x.toFixed(1);
        botPos.y = point.y.toFixed(1);
        botPos.rot = radToDeg(value.rot).toFixed(1);
    })

  const fadeOption: FadeParams = {
    delay: 0,
    duration: 300,

  }
</script>
<!-- <WindowBar /> -->
<Notifications />
<SettingsModal />
<div class="bottom-right">
  <Menu/>
</div>
<main>
  {#if state == AppState.SETTINGS}
    <h1>Settings</h1>
  {:else}
    <div class="grid-container">
      <section class="field">
      {#if state == AppState.PREVIEW}
        <Preview transition={fadeOption}/>
      {:else if state == AppState.CREATOR}
        <h1>Creator</h1>
      {/if}
      </section>

      <section class="info">
        <p>Mouse position: {mousePos.x}, {mousePos.y}</p>
        <p>Bot position: {botPos.x}, {botPos.y}, {botPos.rot}</p>
      </section>

      <section class="files">
        <FileInput />
      </section>
    </div>
  {/if}
</main>

<style>
  main {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    margin: 1rem 0;
  }

  main .grid-container {
    display: grid; 
    grid-auto-columns: 1fr; 
    grid-template-columns: 20em 1fr; 
    grid-template-rows: 2fr 0.9fr; 
    gap: 0px 0px; 
    grid-template-areas: 
      "files field"
      "info field"; 
  }
  main .grid-container .field { grid-area: field; }
  main .grid-container .info { grid-area: info; }
  main .grid-container .files { grid-area: files; }

  .bottom-right {
    position: fixed;
    bottom: 1rem;
    right: 1rem;
    height: 5rem;
    width: 5rem;
    z-index: 100;
  }
</style>