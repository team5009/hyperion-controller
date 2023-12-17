<script lang="ts">
  import { onMount } from "svelte";
  import {Bezier, Circle, Point, PreviewAppState, fieldToCanvas} from "../../lib";
  import { BotPosition, appPreviewState } from "../../store";

    export let resolution: number;
    const botPos = Point(-100, -100, 0);

    let c: HTMLCanvasElement;
    let AppState: PreviewAppState;

    appPreviewState.subscribe((value) => {
        AppState = value;
    })
    BotPosition.subscribe((value) => {
        botPos.x = value.x;
        botPos.y = value.y;
        botPos.rot = Math.round(value.rot);
    })


    onMount(() => {
        c.width = resolution;
        c.height = c.width;

        const ctx = c.getContext("2d");

        if (!ctx) {
            throw new Error("Could not get context");
        }

        const mapPoints: Point[] = [
            Point(-36, -50, 90),
            Point(-48, -40, 0),
            Point(-48, -40, 0),
            Point(-65, -60, 90),
        ]
        
        for (let i = 0; i < mapPoints.length; i++) {
            mapPoints[i] = Point(fieldToCanvas(mapPoints[i]).x, fieldToCanvas(mapPoints[i]).y, mapPoints[i].rot);
        }

        const bezierPoints = Bezier(mapPoints[0], mapPoints[1], mapPoints[2], mapPoints[3]);

        for (const point of bezierPoints) {
            new Circle(point.x, point.y, 3, "green",5).draw(ctx);
        }

        const update = () => {
            requestAnimationFrame(update);
            if (AppState !== PreviewAppState.RUNNING) {
                if (AppState === PreviewAppState.RESETING) {
                    ctx.clearRect(0, 0, c.width, c.height);
                    ctx.beginPath();
                    ctx.moveTo(botPos.x, botPos.y);
                }
            };
            new Circle(botPos.x, botPos.y, 3 * resolution / 1080, "green",5* resolution / 1080).draw(ctx);
        }

        update();
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