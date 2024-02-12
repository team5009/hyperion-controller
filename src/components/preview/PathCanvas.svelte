<script lang="ts">
  import { onMount } from "svelte";
  import {Circle, Point, PreviewAppState, fieldToCanvas, type CommandPath, Line, BezierGen} from "$lib";
  import { BotPosition, appPreviewState, pathCommands } from "$store";
  import { invoke } from "@tauri-apps/api";

    export let resolution: number;
    const botPos = new Point(-100, -100, 0);

    let c: HTMLCanvasElement;
    let AppState: PreviewAppState;
    let commandPath: CommandPath[] = [];
    let prevPoint = {
        x: 0,
        y: 0,
    };
    let pathloaded = false;

    appPreviewState.subscribe((value) => {
        AppState = value;
    })
    BotPosition.subscribe((value) => {
        botPos.x = value.x;
        botPos.y = value.y;
        botPos.rot = Math.round(value.rot);
    })


    pathCommands.subscribe((value) => {
        if (value.length === 0) {
            commandPath = [];
            return;
        }
        commandPath.length = 0;
        commandPath.push(...value);
        pathloaded = false;
    })

    onMount(async () => {
        c.width = resolution;
        c.height = c.width;
        
        const ctx = c.getContext("2d");
        
        if (!ctx) {
            throw new Error("Could not get context");
        }
        const maxWidth = c.width;
        const maxHeight = c.height;

        const update = async () => {
            requestAnimationFrame(update);

            if (!pathloaded && commandPath.length > 0) {
                ctx.clearRect(0, 0, maxWidth, maxHeight);
                
                for (let i = 0; i < commandPath.length; i++) {
                    const command = commandPath[i];

                    const dataTitle = Object.keys(command)[0].toLowerCase();

                    if (dataTitle === "start") {
                        const start = command.Start;
                        const convertedStart = fieldToCanvas(start);
                        new Circle(convertedStart.x, convertedStart.y, 3 * resolution / 1080, "green",5* resolution / 1080).draw(ctx);
                        prevPoint = convertedStart;
                    } else if (dataTitle === "line") {
                        const line = command.Line;
                        const convertedLine = fieldToCanvas(line);
                        new Line(prevPoint.x, prevPoint.y, convertedLine.x, convertedLine.y, "green", 5 * resolution / 1080).draw(ctx);
                        prevPoint = convertedLine;
                    } else if (dataTitle === "spline") {
                        const splineArray = command.Spline;
                        for(let j = 0; j < splineArray.length; j++) {
                            const spline = splineArray[j];
                            const splineTitle = Object.keys(spline)[0].toLowerCase();
                            if (splineTitle === "bezier") {
                                const startBezier = spline.Bezier.start;
                                const endBezier = spline.Bezier.end;
                                const controlBezier = spline.Bezier.control;
                                const bezierToConvert = [startBezier, ...controlBezier, endBezier];
                                const tempBezier = await invoke('gen_bezier_points', {points: bezierToConvert, resolution: 20}) as Point[];
                                for (let k = 0; k < tempBezier.length; k++) {
                                    const point = tempBezier[k];
                                    const convertedPoint = fieldToCanvas(point);
                                    new Line(prevPoint.x, prevPoint.y, convertedPoint.x, convertedPoint.y, "green", 5 * resolution / 1080).draw(ctx);
                                    prevPoint = convertedPoint;
                                }
                            }
                        }
                    } else if (dataTitle === "end") {
                        const end = prevPoint
                        new Circle(end.x, end.y, 3 * resolution / 1080, "green",5* resolution / 1080).draw(ctx);
                        prevPoint = new Point(0, 0, 0);
                    }
                }
                pathloaded = true;
            }
                


            // if (AppState !== PreviewAppState.RUNNING) {
            //     if (AppState === PreviewAppState.RESETING) {
            //         ctx.clearRect(0, 0, maxWidth, maxHeight);
            //         ctx.beginPath();
            //         ctx.moveTo(botPos.x, botPos.y);
            //     }
            // };
            // ctx.clearRect(0, 0, maxWidth, maxHeight);
        }

        await update();
    })
    
</script>

<canvas bind:this={c} ></canvas>

<style>
    canvas {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 500px;
        height: 500px;
        z-index: 4;
    }
</style>