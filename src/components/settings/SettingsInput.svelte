<script lang="ts">
    export let label: string
    export let value: string
    const id = label.toLowerCase().replace(" ", "_")

    const handleInput = (e: Event) => {
        const target = e.target as HTMLInputElement;
    }
</script>
<div>
    <input id={id} type="text" on:change={handleInput} value={value} placeholder={label} required/>
    <label for={id}>{label}</label>
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
    }
</style>