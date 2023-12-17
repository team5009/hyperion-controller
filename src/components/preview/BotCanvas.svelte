<script lang="ts">
    import { onMount } from "svelte";
    import { Bezier, Bot, Point, PreviewAppState, canvasToField, degToRad, fieldToCanvas } from "../../lib";
    import { BotPosition, appPreviewState, mousePosition } from "../../store";
    export let resolution: number;

    let c: HTMLCanvasElement;
    let AppState: PreviewAppState;

    appPreviewState.subscribe((value) => {
        AppState = value;
    })

    onMount(() => {
        c.width = resolution;
        c.height = c.width;
        const ctx = c.getContext("2d");
        const maxWidth = c.width;
        const maxHeight = c.height;
        const speed = 10;

        if (!ctx) {
            throw new Error("Could not get context");
        }

        
        let pointStep = 0;
        const bezier1: Point[] = [
            Point(-36, -50, 90),
            Point(30, -80, 0),
            Point(-10, -70, 0),
            Point(-12, 60, 90),
        ]
        const bezier2: Point[] = [
            Point(-36, -50, 90),
            Point(-48, -30, 0),
            Point(-48, -30, 0),
            Point(-63, -62, 90),
        ]

        const pixelBezierPoints = Bezier(bezier1[0], bezier1[1], bezier1[2], bezier1[3]);
        const parkBezierPoints = Bezier(bezier2[0], bezier2[1], bezier2[2], bezier2[3]);
        const mapPoints: Point[] = [
            Point(-35, -12, 0),
            Point(-36, -50, 90),
            ...pixelBezierPoints,
            ...pixelBezierPoints.reverse(),
            ...parkBezierPoints
        ]

        const startPos = Point(-63, -12, 0)

        const bot = new Bot(startPos, speed);
        BotPosition.set(bot);
        bot.draw(ctx);
        const updateBot = () => {
            requestAnimationFrame(updateBot);

            if (AppState !== PreviewAppState.RUNNING) {

                if (AppState === PreviewAppState.RESETING) {
                    bot.resetPosition(ctx,{x: maxWidth, y: maxHeight},startPos);
                    BotPosition.set(bot);
                    pointStep = 0;
                }

                return;
            }

            const convertBot = canvasToField(bot);

            if (pointStep >= mapPoints.length || mapPoints[pointStep] === undefined) {
                appPreviewState.set(PreviewAppState.STOPPED);
            } else if (
                Math.abs(convertBot.x - mapPoints[pointStep].x) <= 1080/resolution * 0.8 &&
                Math.abs(convertBot.y - mapPoints[pointStep].y) <= 1080/resolution * 0.8 &&
                Math.abs(bot.rot - degToRad(mapPoints[pointStep].rot)) <= Math.PI / 3
            ) {

                // if (pointStep == mapPoints.length - 1 ||
                //     Math.abs(bot.rot - degToRad(mapPoints[pointStep].rot)) <= Math.PI / 3
                // ) {
                //     bot.update(ctx, {x: maxWidth, y: maxHeight}, mapPoints[pointStep]);
                //     BotPosition.set(bot);
                //     return;
                // }
                pointStep++;
            } else if (pointStep < mapPoints.length) {
                bot.update(ctx, {x: maxWidth, y: maxHeight}, mapPoints[pointStep]);
                BotPosition.set(bot);
            }
        }
        
        setTimeout(() => {
            updateBot();
        }, 1000);
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