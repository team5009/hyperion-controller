<script lang="ts">
    import { AppState } from "../../lib";
  import app from "../../main";
    import { appState } from "../../store";

    let state: AppState;
    const buttons = [
        {
            name: "Creator",
            state: AppState.CREATOR
        },
        {
            name: "Preview",
            state: AppState.PREVIEW
        },
        {
            name: "Settings",
            state: AppState.SETTINGS
        }
    ]

    appState.subscribe((value) => {
        state = value;
    });
</script>

<nav>
    <div class="container">
        {#each buttons as button}
            {#if state == button.state}
                <button class="active">
                    {button.name}
                </button>
            {:else}
                <button on:click={() => appState.set(button.state)}>
                    {button.name}
                </button>
            {/if}
        {/each}
    </div>
</nav>

<style>
    nav {
        position: relative;
    }

    nav .container  {
        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: row;
    }

    nav .container button {
        margin: 0 1rem;
        background: none;
        border: none;
        font-size: 1rem;
        font-weight: bold;
        color: #fff;
        cursor: pointer;
        border-radius: 0;
        border-bottom: #fff 2px solid;
        transition: all 0.1s ease-in-out; 

    }

    nav .container button:hover {
        transform: translateY(-0.3rem);
    }
    nav .container button:active {
        transform: translateY(-0.1rem);
    }

    nav .container button.active {
        transform: translateY(-0.3rem);
        background-color: #fff;
        color: #000;
    }

</style>