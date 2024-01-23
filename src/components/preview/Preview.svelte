<script lang="ts">
    import { fade, type FadeParams } from "svelte/transition";
    import { PreviewAppState, canvasToField, radToDeg, resolution } from "$lib";
    import { BotPosition, appPreviewState, mousePosition } from "$store";
    // import FieldMap from "../FieldCanvas/FieldMap.svelte";
    import BotCanvas from "./BotCanvas.svelte";
    import FieldCanvas from "../FieldCanvas/FieldCanvas.svelte";
    import PathCanvas from "./PathCanvas.svelte";
    const fieldResolution = resolution;
    export let transition: FadeParams
    
</script>

<section transition:fade={transition}>
    <div class="game">
        <FieldCanvas resolution={fieldResolution}/>
        <BotCanvas resolution={fieldResolution}/>
        <PathCanvas resolution={fieldResolution}/>
        <!-- <FieldMap/> -->
    </div>
    <div>
        <button on:click={() => appPreviewState.set(PreviewAppState.RESETING)}>Reset</button>
        <button on:click={() => appPreviewState.set(PreviewAppState.RUNNING)}>Start</button>
        <button on:click={() => appPreviewState.set(PreviewAppState.STOPPED)}>Stop</button>
        <button on:click={() => appPreviewState.set(PreviewAppState.PAUSED)}>Pause</button>
    </div>
</section>

<style>
    .game {
      position: relative;
      display: flex;
      justify-content: center;
      align-items: center;
    }
    .status {
      display: flex;
      justify-content: center;
      gap: 1rem;
    }
  </style>