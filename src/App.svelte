<script lang="ts">
  import FieldCanvas from './components/preview/FieldCanvas.svelte';
  import BotCanvas from './components/preview/BotCanvas.svelte';
  import { BotPosition, appPreviewState, mousePosition } from './store';
  import { Point, PreviewAppState, canvasToField, radToDeg, resolution } from './lib';
  import PathCanvas from './components/preview/PathCanvas.svelte';
  import FieldMap from './components/layout/FieldMap.svelte';

  const fieldResolution = resolution;
  const mousePos = {
    x: "0",
    y: "0"
  }
  const botPos = {
    x: "0",
    y: "0",
    rot: "0"
  };

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
</script>

<main class="container">

  <div class="game">
    <FieldCanvas resolution={fieldResolution}/>
    <BotCanvas resolution={fieldResolution}/>
    <PathCanvas resolution={fieldResolution}/>
    <FieldMap/>
  </div>
  <div class="status">
    <p>Mouse position: {mousePos.x}, {mousePos.y}</p>
    <p>Bot position: {botPos.x}, {botPos.y}, {botPos.rot}</p>
  </div>
  <div>
    <button on:click={() => appPreviewState.set(PreviewAppState.RESETING)}>Reset</button>
    <button on:click={() => appPreviewState.set(PreviewAppState.RUNNING)}>Start</button>
    <button on:click={() => appPreviewState.set(PreviewAppState.STOPPED)}>Stop</button>
    <button on:click={() => appPreviewState.set(PreviewAppState.PAUSED)}>Pause</button>
  </div>

</main>

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