<script lang="ts">
    import { FTCSEASON } from "$lib";  
    import { SettingsState } from "$store";
    import { get } from "svelte/store";   

    let currentYear = get(SettingsState).year;

    SettingsState.subscribe((value) => {
        currentYear = value.year;
    })


    const seasons: {name: string, value: FTCSEASON}[] = [
        {
            name: "Centerstage",
            value: FTCSEASON.CENTERSTAGE
        },
        {
            name: "Power Play",
            value: FTCSEASON.POWERPLAY
        },
        {
            name: "Freight Frenzy",
            value: FTCSEASON.FREIGHT_FRENZY
        },
        {
            name: "Ultimate Goal",
            value: FTCSEASON.ULTIMATE_GOAL
        }
    ]

    const handleChange = (e: Event) => {
        const target = e.target as HTMLSelectElement;
        seasons.forEach(season => {
            if (season.value.toString() === target.value) {
                SettingsState.set({
                    ...get(SettingsState),
                    year: season.value
                })
                console.log(get(SettingsState))
            }
        })
    }

</script>
<div>
    <select id="season" on:change={handleChange} value={currentYear}>
        {#each seasons as season}
        <option value={season.value}>{season.name}</option>
        {/each}
    </select>
    <label for="season">Season</label>
</div>

<style lang="scss">
    div {
        position: relative;
        display: flex;
        flex-direction: row;
        height: 3rem;
        width: 100%;
        margin: 1rem 0;
        label {
            position: absolute;
            left: 1rem;
            transform: translateY(-0.8rem);
            padding: 0.1rem 0.5rem;
            outline: 0.1rem solid var(--secondary);
            background-color: rgb(27, 27, 34);
            border-radius: 1rem;
            box-sizing: border-box;
            color: var(--secondary);
        }
        select {
            width: 100%;
            padding: 0.5rem 1rem;
            padding-top: 1rem;
            border-radius: 1rem;
            font-size: 1.2rem;
            transition: all 0.5s ease-in-out;
            background-color: #0f0f0f98;
            border: none;
            color: white;
            option {
                background: var(--secondary);
                color: #fff;
                border: none;
                font-weight: bold;
                font-size: 1.5rem;
            }
        }

        select:focus {
            outline: 0.1rem solid var(--secondary);
            border: 0.1rem solid var(--secondary);
        }
    }
</style>