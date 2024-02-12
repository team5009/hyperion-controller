<script lang="ts">
    import type { Measurement } from "$lib";
    import { SettingsState } from "$store";
    import { get } from "svelte/store";
    let currentMeasurement: Measurement = get(SettingsState).measurements;

    SettingsState.subscribe((value) => {
        currentMeasurement = value.measurements;
    })

    const measurement: {name: string, value: Measurement}[] = [
        {
            name: "mm",
            value: "mm"
        },
        {
            name: "cm",
            value: "cm"
        },
        {
            name: "in",
            value: "in"
        }
    ]

    const changeMeasurement = (m: Measurement) => {
        SettingsState.set({
            ...get(SettingsState),
            measurements: m
        })
    }

</script>

<div>
    {#each measurement as mesure}
        {#if mesure.value === currentMeasurement}
            <button disabled>
                {mesure.name}
            </button>
        {:else}
            <button on:click={() => changeMeasurement(mesure.value)}>
                {mesure.name}
            </button>
        {/if}
    {/each}
</div>

<style lang="scss">
  div {
    position: relative;
    display: flex;
    flex-direction: row;
    height: 3rem;
    width: 100%;
    margin: 1rem 0;

    button {
        width: 100%;
        height: 100%;
        border: 0.1rem solid var(--secondary);
        border-radius: 1rem;
        background-color: var(--primary);
        color: var(--secondary);
        font-size: 1.2rem;
        transition: all 0.2s ease-in-out;
        
    }
    button:not(:disabled):hover {
        background-color: var(--secondary);
        color: var(--primary);
        cursor: pointer;
        scale: 1.1;
    }

    button:not(:disabled):active {
        scale: 1.05;
    }

    button:nth-child(1) {
        border-radius: 1rem 0 0 1rem;
    }

    button:nth-child(2) {
        border-radius: 0;
    }

    button:nth-child(3) {
        border-radius: 0 1rem 1rem 0;
    }

    button:disabled {
        background-color: var(--secondary);
        color: var(--primary);
        cursor: auto;
        transition: all 0.2s ease-in-out;
    }
  }
</style>