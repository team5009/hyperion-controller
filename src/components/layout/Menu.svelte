<script lang="ts">
    import { fade, type FadeParams } from "svelte/transition";
    import { AppState } from "../../lib";
    import { appState } from "../../store";

    export let transition: FadeParams

</script>

<section transition:fade={transition}>
    <button on:click={() => appState.set(AppState.CONTROLLER)}>
        Controller
    </button>
    <button on:click={() => appState.set(AppState.CREATOR)}>
        Creator
    </button>
    <button on:click={() => appState.set(AppState.PREVIEW)}>
        Preview
    </button>
</section>

<style>

</style>