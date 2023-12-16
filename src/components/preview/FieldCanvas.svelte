<script lang="ts">
    import { onMount } from "svelte";
    import { Line, canvasToField } from "../../lib";
    import { mousePosition } from "../../store";
    export let resolution: number;

    let c: HTMLCanvasElement;

    onMount(() => {
        c.width = resolution;
        c.height = c.width;
        const ctx = c.getContext("2d");
        const maxWidth = c.width;
        const maxHeight = c.height;
        const poleThickness = 10;
        const tapeThickness = 10;
        const gridThickness = 2;
        const gridTapeRatio = tapeThickness / gridThickness;
        if (!ctx) {
            throw new Error("Could not get context");
        }
        const splitIntoSix = (max: number) => max / 6

        for (let i = splitIntoSix(maxWidth); i < maxWidth; i+=splitIntoSix(maxWidth)) {
            if (i === splitIntoSix(maxWidth) * 6) {
                continue;
            }
            new Line(i, 5, i, maxHeight - 5, "#ffffff40", gridThickness).draw(ctx);
            ctx.font = "60px Arial";
            ctx.textAlign = "center";
            ctx.fillStyle = "white";
            ctx.textBaseline = "top";
            ctx.fillText(canvasToField({x: i, y: i}).x.toString(), i, 10);
        }
        for (let i = splitIntoSix(maxHeight); i < maxHeight; i+=splitIntoSix(maxHeight)) {
            if (i === splitIntoSix(maxHeight) * 6) {
                continue;
            }
            new Line(5, i, maxWidth - 5, i, "#ffffff40", gridThickness).draw(ctx);
            ctx.font = "60px Arial";
            ctx.textAlign = "left";
            ctx.fillStyle = "white";
            ctx.textBaseline = "top";
            ctx.fillText(canvasToField({x: i, y: i}).x.toString(), 10, i);
        }
    });

    function handleMouseDown(e: MouseEvent) {
        const bounds = c.getBoundingClientRect();
        const mouse = {
            x: e.pageX - bounds.left - scrollX,
            y: e.pageY - bounds.top - scrollY
        }

        mouse.x /= bounds.width;
        mouse.y /= bounds.height;

        mouse.x *= c.width;
        mouse.y *= c.height;

        mousePosition.set({
            x: Math.round(mouse.x),
            y: Math.round(mouse.y)
        
        });
    }
    
</script>

<canvas bind:this={c} style="border:2px solid #fff;" on:mousedown={handleMouseDown}></canvas>

<style>
    canvas {
        position: relative;
        width: 500px;
        height: 500px;
        z-index: 5;
    }
</style>