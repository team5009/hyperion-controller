<script lang="ts">
    import Icon from '@iconify/svelte'
    import menuIcon from '@iconify/icons-majesticons/menu'
    import editPen2 from '@iconify/icons-majesticons/edit-pen-2';
    import bookOpen from '@iconify/icons-majesticons/book-open';
    import robotIcon from '@iconify/icons-majesticons/robot';
    import settingsCog from '@iconify/icons-majesticons/settings-cog';
    import Ping from './Ping.svelte';
    import { BotSocketConnected, appState } from '../../store';
    import { AppState } from '../../lib';

    let botConnected = false

    BotSocketConnected.subscribe(value => {
        botConnected = value
    })

    const menuOptions = [
        {
            name: 'Controller',
            icon: robotIcon,
            action: () => appState.set(AppState.CONTROLLER)
        },
        {
            name: 'Creator',
            icon: editPen2,
            action: () => appState.set(AppState.CREATOR)
        },
        {
            name: 'Preview',
            icon: bookOpen,
            action: () => appState.set(AppState.PREVIEW)
        },
        {
            name: 'Settings',
            icon: settingsCog,
            action: () => appState.set(AppState.SETTINGS)
        }
    ]
</script>

<div class="container_menu">
    <input id="menu" type="checkbox"/>
    <label for="menu">
        <Icon icon={menuIcon} style="width: 75%; height: 75%"/>
    </label>
    <ul class="menu">
        {#each menuOptions as option}
            <li>
                <button on:click={option.action}>
                    <Icon icon={option.icon} style="width: 75%; height: 75%"/>
                </button>
            </li>
        {/each}
    </ul>
    <ul class="extra">
        {#if !botConnected}

        {:else} 
            <li>
                <Ping/>
            </li>
        {/if}
        
    </ul>
</div>


<style>
    .container_menu {
        position: relative;
        display: flex;
        width: 100%;
        height: 100%;
        border-radius: 50%;
    }

    .container_menu  [type="checkbox"] {
        display: none;
    }
    .container_menu  [type="checkbox"] + label {
        width: 5rem;
        height: 5rem;
        position: relative;
        border-radius: 100%;
        background-color: blue;
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 100;
        cursor: pointer;
        transition: all 0.5s ease;
    }
    .container_menu  [type="checkbox"]:checked + label {
        background-color: red;
        transform: scale(0.75);
    }
    .container_menu  [type="checkbox"]:not(:checked) + label:hover {
        transform: scale(1.1);
    }
    .container_menu  [type="checkbox"]:checked + label:hover {
        transform: scale(0.9);
    }

    .container_menu ul {
        position: fixed;
        height: 5rem;
        width: 5rem;
        padding: 0;
        margin: 0;
    }

    .container_menu ul li {
        position: absolute;
        list-style: none;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        height: 2.5rem;
        display: flex;
        justify-content: center;
        align-items: center;
        transition: all 0.5s ease;
        opacity: 0;
        
    }

    .container_menu .menu li {
        width: 2.5rem;
    }

    .container_menu .menu li button {
        color: white;
        background-color: blue;
        border: none;
        border-radius: 100%;
        padding: 0;
        height: 100%;
        width: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
        box-shadow: 1px 1px 5px rgba(0, 0, 0, 0.5);
    }

    .container_menu input:checked ~ ul li {
        opacity: 1;
    }

    .container_menu .menu li:nth-child(1) {
        transition-delay: 0.1s;
    }

    .container_menu .menu li:nth-child(2) {
        transition-delay: 0.2s;
    }

    .container_menu .menu li:nth-child(3) {
        transition-delay: 0.3s;
    }

    .container_menu .menu li:nth-child(4) {
        transition-delay: 0.4s;
    }

    .container_menu input:checked ~ .menu li:nth-child(1) {
        transform: translate(calc(-50% + 1.5rem), calc(-50% - 4.25rem));
    }

    .container_menu input:checked ~ .menu li:nth-child(2) {
        transform: translate(calc(-50% - 1.5rem), calc(-50% - 3.75rem));
    }

    .container_menu input:checked ~ .menu li:nth-child(3) {
        transform: translate(calc(-50% - 3.75rem), calc(-50% - 1.5rem));
    }

    .container_menu input:checked ~ .menu li:nth-child(4) {
        transform: translate(calc(-50% - 4.25rem), calc(-50% + 1.5rem));
    }




    .container_menu .extra li:nth-child(1) {
        transition-delay: 0.5s;
        width: 5rem;
        padding: 0.5rem;
        box-sizing: border-box;
        background-color: blue;
        border-radius: 100rem;
        box-shadow: 1px 1px 5px rgba(0, 0, 0, 0.5);

    }

    .container_menu input:checked ~ .extra li:nth-child(1) {
        transform: translate(calc(-50% - 8.75rem), calc(-50% + 1.5rem));
    }

</style>