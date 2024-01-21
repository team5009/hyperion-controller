<script lang="ts">
	import Connection from './components/extra/Connection.svelte';
  import Preview from './components/preview/Preview.svelte';
  import Menu from './components/layout/Menu.svelte';
  import { BotPosition, appState, mousePosition } from './store';
  import { AppState, canvasToField, radToDeg } from './lib';
  import NavBar from './components/layout/NavBar.svelte';
  import {type FadeParams} from 'svelte/transition';
  import FileInput from './components/layout/FileInput.svelte';
  import Ping from './components/extra/Ping.svelte';

  let state: AppState;
  const mousePos = {
        x: "0",
        y: "0"
    }
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

<Connection />
<Ping />
{#if state !== AppState.MENU}
  <NavBar />
{/if}
<main>
  {#if state == AppState.MENU}
  <Menu transition={fadeOption}/>
  {:else if state == AppState.SETTINGS}
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
</style>