<script lang="ts">
    import { fade, type FadeParams } from "svelte/transition";
    import { PreviewAppState, canvasToField, radToDeg, resolution } from "$lib";
    import { BotPosition, appPreviewState, mousePosition } from "$store";
    import FieldMap from "../FieldCanvas/FieldMap.svelte";
    import BotCanvas from "./BotCanvas.svelte";
    import FieldCanvas from "../FieldCanvas/FieldCanvas.svelte";
    import PathCanvas from "./PathCanvas.svelte";

    import PlayButton from '@iconify/icons-majesticons/play-circle'
    import RewindButton from '@iconify/icons-majesticons/backward-start-circle'
    import StopButton from '@iconify/icons-majesticons/stop-circle'
    import PauseButton from '@iconify/icons-majesticons/pause-circle'
  import Icon from "@iconify/svelte";

    const fieldResolution = resolution;
    export let transition: FadeParams
    
  const sequenceButtons = [
    {
      name: "Reset",
      action: PreviewAppState.RESETING,
      icon: RewindButton
    },
    {
      name: "Start",
      action: PreviewAppState.RUNNING,
      icon: PlayButton
    },
    {
      name: "Stop",
      action: PreviewAppState.STOPPED,
      icon: StopButton
    },
    {
      name: "Pause",
      action: PreviewAppState.PAUSED,
      icon: PauseButton
    }
  ]

</script>

<section transition:fade={transition}>
    <div class="game">
        <FieldCanvas resolution={fieldResolution}/>
        <!-- <BotCanvas resolution={fieldResolution}/> -->
        <PathCanvas resolution={fieldResolution}/>
        <FieldMap/>
    </div>
    <div>
      {#each sequenceButtons as sequence}
        <button on:click={() => appPreviewState.set(sequence.action)} class={sequence.name.toLowerCase()}>
          <Icon icon={sequence.icon} style="width: 75%; height: 75%"/>
        </button>
      {/each}
    </div>
</section>

<style>
  section {
    display: flex;
    flex-direction: row-reverse;
    gap: 1rem;
  }
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
    button {
      width: 3rem;
      height: 3rem;
      border-radius: 50%;
      background-color: var(--secondary);
      display: flex;
      justify-content: center;
      align-items: center;
      cursor: pointer;
      transition: all 0.5s ease;
      padding: 0;
      margin: 0.5rem 0.25rem;
    }
    .start {
      background-color: green;
    }
    .stop {
      background-color: red;
    }
    .pause {
      background-color: yellow;
    }
    .reset {
      background-color: blue;
    }
  </style>