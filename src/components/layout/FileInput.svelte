<script lang="ts">
    import {open} from '@tauri-apps/api/dialog'
    import {invoke} from '@tauri-apps/api/tauri'
    import { appPreviewState, pathCommands } from '../../store';
    import { AppState, PreviewAppState, type CommandPath, type Point } from '../../lib';
    let fileInput: HTMLInputElement;
    const commands: CommandPath[] = []
    const clickHandler = async () => {
        commands.length = 0
        const file = await open({
            multiple: false,
            directory: false,
            filters: [
                {
                    name: 'Text',
                    extensions: ['csv']
                }
            ],
            title: 'Select a path instance file'
        })
        if (!file) return
        const rust_input = await invoke('load_file', {path: file}) as CommandPath[]

        commands.push(...rust_input)
    }

    const confirmPath = async () => {
        pathCommands.set(commands)
        appPreviewState.set(PreviewAppState.RESETING)
    }
</script>

<section>
    <div class="file_input">
        <input type="button" value="Browse..." on:click={clickHandler} />
        <span>Upload a file</span>
    </div>
    <button on:click={confirmPath}>
        Confirm
    </button>
</section>

<style>
    section {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 1rem;
    }
    .file_input {
        position: relative;
        background-color: black;
        width: 10rem;
        height: 10rem;
        border: 0.5rem dashed white;
        border-radius: 1rem;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .file_input input[type="button"] {
        position: absolute;
        width: 100%;
        height: 100%;
        opacity: 0;
        cursor: pointer;
        box-sizing: border-box;
    }
</style>