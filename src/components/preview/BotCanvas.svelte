<script lang="ts">
    import { onMount } from "svelte";
    import { Bezier, Bot, Point, PreviewAppState, canvasToField, degToRad, fieldToCanvas, type CommandPath } from "../../lib";
    import { BotPosition, appPreviewState, mousePosition, pathCommands } from "../../store";
    export let resolution: number;

    let c: HTMLCanvasElement;
    let AppState: PreviewAppState;
    
    let commandPath: CommandPath[] = [];
    $: commandPath;

    appPreviewState.subscribe((value) => {
        AppState = value;
    })

    pathCommands.subscribe((value) => {

        if (value.length === 0) {
            commandPath = [];
            return;
        }
        commandPath.push(...value);
    })

    onMount(() => {
        c.width = resolution;
        c.height = c.width;
        const ctx = c.getContext("2d");
        const maxWidth = c.width;
        const maxHeight = c.height;
        const speed = 10;
        let pointStep = 0;
        const currentPath = []

        if (!ctx) {
            throw new Error("Could not get context");
        }

        let startPos = {x: 0, y: 0, rot: 0};
        

        const bot = new Bot(startPos, speed);
        BotPosition.set(bot);
        bot.draw(ctx);
        const updateBot = () => {
            requestAnimationFrame(updateBot);

            
            if (AppState !== PreviewAppState.RUNNING) {
                switch (AppState) {
                    case PreviewAppState.PAUSED:
                        return;
                    case PreviewAppState.RESETING:
                        bot.resetPosition(ctx,{x: maxWidth, y: maxHeight}, commandPath[0].Start);
                        pointStep = 0;
                        break;
                    case PreviewAppState.STOPPED:
                        return;
                }
                
                return;
            }
            
            const convertBot = canvasToField(bot);
            switch (Object.keys(commandPath[pointStep])[0].toLowerCase()) {
                case "start" : {
                    bot.resetPosition(ctx,{x: maxWidth, y: maxHeight}, commandPath[pointStep].Start);
                    pointStep++;
                    break;
                }
                case "goto": {
                    if (pointStep >= commandPath.length) {
                        appPreviewState.set(PreviewAppState.STOPPED);
                        return;
                    }
                    const point = commandPath[pointStep].Goto;
                    if (
                        Math.abs(convertBot.x - point.x) <= 1080/resolution * 0.8 &&
                        Math.abs(convertBot.y - point.y) <= 1080/resolution * 0.8 &&
                        Math.abs(bot.rot - degToRad(point.rot)) <= Math.PI / 3
                    ) {
                        pointStep++;
                    } else {
                        bot.update(ctx, {x: maxWidth, y: maxHeight}, point);
                    }
                    break;
                }
                case "wait": {
                    console.log(`Waiting for ${commandPath[pointStep].Wait} to succeed`);
                    pointStep++;
                    break;
                }
                case "spline": {
                    const spline = commandPath[pointStep].Spline;
                    console.log(spline);
                    break;
                }
            }

            // if (pointStep >= mapPoints.length || mapPoints[pointStep] === undefined) {
            //     appPreviewState.set(PreviewAppState.STOPPED);
            // } else if (
            //     Math.abs(convertBot.x - mapPoints[pointStep].x) <= 1080/resolution * 0.8 &&
            //     Math.abs(convertBot.y - mapPoints[pointStep].y) <= 1080/resolution * 0.8 &&
            //     Math.abs(bot.rot - degToRad(mapPoints[pointStep].rot)) <= Math.PI / 3
            // ) {

            //     // if (pointStep == mapPoints.length - 1 ||
            //     //     Math.abs(bot.rot - degToRad(mapPoints[pointStep].rot)) <= Math.PI / 3
            //     // ) {
            //     //     bot.update(ctx, {x: maxWidth, y: maxHeight}, mapPoints[pointStep]);
            //     //     BotPosition.set(bot);
            //     //     return;
            //     // }
            //     pointStep++;
            // } else if (pointStep < mapPoints.length) {
            //     bot.update(ctx, {x: maxWidth, y: maxHeight}, mapPoints[pointStep]);
            //     BotPosition.set(bot);
            // }
        }
        
        updateBot();
    })




</script>

<canvas bind:this={c}></canvas>

<style>
    canvas {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 500px;
        height: 500px;
        z-index: 2;
    }
</style>