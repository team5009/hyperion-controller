<script lang="ts">
  import { get } from "svelte/store";

    import { type Measurement, convertToUnit } from "$lib";
    import { SettingsState } from "$store";
    // import { convertToUnit } from '../../lib'

    export let label: string
    export let value: number
    const id = label.toLowerCase().replace(" ", "_")
    let unit: Measurement = 'in'
    let units: Measurement[] = [
        'mm',
        'cm',
        'in',
    ]


    SettingsState.subscribe(({measurements}) => {
        const prevUnit = unit;
        unit = measurements;
        value = convertToUnit(value, prevUnit,unit);
    })

    const handleUnitChange = (e: Event) => {
        const target = e.target as HTMLSelectElement;
        const prevUnit = unit;
        unit = target.value as Measurement;
        SettingsState.set({
            ...get(SettingsState),
            measurements: unit
        
        })
        value = convertToUnit(value,prevUnit, unit);
    }

    const handleInput = (e: Event) => {
        const target = e.target as HTMLInputElement;
        const temp = parseFloat(target.value);
        value = temp
    }

</script>

<div>
    <input id={id} type="number" on:change={handleInput} value={value} placeholder={label} required/>
    <label for={id}>{label}</label>
    <select on:change={handleUnitChange} value={unit}>
        {#each units as unit}
            <option value={unit}>{unit}</option>
        {/each}
    </select>
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
            transform: translateY(50%);
            transition: all 0.5s ease-in-out;
        }
        input {
            width: 100%;
            padding: 0.5rem 1rem;
            padding-top: 1rem;
            border-radius: 1rem;
            font-size: 1.2rem;
            transition: all 0.5s ease-in-out;
        }
        select {
            position: absolute;
            right: 0;


            height: 100%;
            width: 5rem;
            border-radius: 0 1rem 1rem 0;

            background: var(--secondary);
            color: #fff;
            border: none;
            font-weight: bold;
            font-size: 1.5rem;
            text-align: center;

            transition: all 0.5s ease-in-out;

            option {
                background: var(--secondary);
                color: #fff;
                border: none;
                font-weight: bold;
                font-size: 1.5rem;
            
            }

            option:hover {
                background: var(--primary);
                color: #fff;
                border: none;
                font-weight: bold;
                font-size: 1.5rem;
            }
        }

        input::placeholder {
            opacity: 0;
        }

        input:focus {
            outline: 0.1rem solid var(--secondary);
            border: 0.1rem solid var(--secondary);
        }

        input:focus ~ label {
            transform: translateY(-0.8rem);
            padding: 0.1rem 0.5rem;
            outline: 0.1rem solid var(--secondary);
            background-color: rgb(27, 27, 34);
            border-radius: 1rem;
            box-sizing: border-box;
            color: var(--secondary);
        }

        input:not(:placeholder-shown) ~ label {
            transform: translateY(-0.8rem);
            padding: 0.1rem 0.5rem;
            outline: 0.1rem solid var(--secondary);
            background-color: rgb(27, 27, 34);
            border-radius: 1rem;
            box-sizing: border-box;
            color: var(--secondary);
        }

        select:focus {
            width: 50%;
            border-radius: 1rem;

        }
    }
</style>