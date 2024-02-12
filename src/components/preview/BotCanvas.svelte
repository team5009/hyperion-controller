<script lang="ts">
    import { onMount } from "svelte";
    import { Bot, PreviewAppState, canvasToField, degToRad, type CommandPath, Point, Event } from "$lib";
    import { BotPosition, appPreviewState, pathCommands } from "$store";
    import { writable } from "svelte/store";
    import { invoke } from "@tauri-apps/api/tauri";
    export let resolution: number;

    let c: HTMLCanvasElement;
    let AppState: PreviewAppState;
    
    let commandPath: CommandPath[] = [];
    $: commandPath;

    appPreviewState.subscribe((value) => {
        AppState = value;
    })

    let botStateString = ""
    let botState = writable("")
    botState.subscribe((value) => {
        botStateString = value;
        if (value === "wait") {
            setTimeout(() => {
                botState.set("complete")
            }, 4000)
        }
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
        let splineStep = 0;
        let isSpline = false;
        let bezierStep = 0;
        let bezierPoints: Point[] = [];
        const currentPath = []

        if (!ctx) {
            throw new Error("Could not get context");
        }

        let startPos = {x: 0, y: 0, rot: 0, event: new Event("nothing")};
        
        const bot = new Bot(startPos, speed);
        BotPosition.set(bot);
        bot.draw(ctx);
        const updateBot = async () => {
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
                case "line": {
                    if (pointStep >= commandPath.length) {
                        appPreviewState.set(PreviewAppState.STOPPED);
                        return;
                    }
                    const point = commandPath[pointStep].Line;
                    if (
                        Math.abs(convertBot.x - point.x) <= 1080/resolution * 0.8 &&
                        Math.abs(convertBot.y - point.y) <= 1080/resolution * 0.8 &&
                        Math.abs(bot.rot - degToRad(point.rot)) <= Math.PI / 3
                    ) {
                        pointStep++;
                    } else {
                        bot.updateNextPoint(point)
                        bot.update(ctx, {x: maxWidth, y: maxHeight}, point);
                    }
                    break;
                }
                case "wait": {
                    console.log(`Waiting for ${commandPath[pointStep].Wait} to succeed`);
                    // console.log(botStateString)
                    if (botStateString === "") botState.set("wait");
                    if (botStateString === "complete") {
                        pointStep++
                        botState.set("")};
                    break;
                }
                case "spline": {
                    const spline = commandPath[pointStep].Spline;
                    if (splineStep >= spline.length) {
                        pointStep++;
                        splineStep = 0;
                        bezierPoints = [];
                        isSpline = false;
                        break;
                    }

                    switch(Object.keys(spline[splineStep])[0].toLowerCase()) {
                        case "bezier" : {
                            if (!isSpline) {
                                const startBezier = spline[splineStep].Bezier.start;
                                const endBezier = spline[splineStep].Bezier.end;
                                const controlBezier = spline[splineStep].Bezier.control;
                                const bezierToConvert = [startBezier, ...controlBezier, endBezier];
                                const tempBezier = await invoke('gen_bezier_points', {points: bezierToConvert, resolution: 20}) as Point[]
                                bezierPoints.push(...tempBezier);
                                bot.updateNextPoint(bezierPoints[bezierPoints.length - 1])
                                isSpline = true;
                            }

                            if (isSpline) {
                                if (bezierStep + 1 > bezierPoints.length - 1) {
                                    bezierStep = 0;
                                    splineStep++;
                                    isSpline = false;
                                    break;
                                }
                                const point = bezierPoints[bezierStep + 1];
                                console.table({
                                    bot: convertBot,
                                    point: point,
                                    diff: 5
                                })
                                if (
                                    (Math.abs(convertBot.x - point.x) <= 55 &&
                                    Math.abs(convertBot.y - point.y) <= 5) &&
                                    Math.abs(bot.rot - degToRad(point.rot)) >= Math.PI / 3
                                ) {
                                    console.log(bezierStep)
                                    bezierStep++;
                                } else {
                                    console.log(`Updating bot ${point.x}, ${point.y}`)
                                    console.log(bezierPoints[bezierPoints.length - 1])
                                    bot.update(ctx, {x: maxWidth, y: maxHeight}, point);
                                }
                            }
                            break;
                        }
                        case "wait" : {
                            console.log(`Waiting for ${commandPath[pointStep].Wait} to succeed`);
                            // console.log(botStateString)
                            if (botStateString === "") botState.set("wait");
                            if (botStateString === "complete") {
                                splineStep++
                                botState.set("")};
                            break;
                        }
                    }
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