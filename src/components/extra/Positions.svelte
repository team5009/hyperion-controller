<script lang="ts">
    import { canvasToField, radToDeg } from "$lib";
    import { BotPosition, mousePosition } from "$store";

    const mousePos = {
        x: "0",
        y: "0"
    };
    const botPos = {
        x: "0",
        y: "0",
        rot: "0"
    };
    mousePosition.subscribe((value) => {
        const point = canvasToField(value);
        mousePos.x = point.x.toFixed(1);
        mousePos.y = point.y.toFixed(1);
    })

    BotPosition.subscribe((value) => {
        const point = canvasToField(value);
        botPos.x = point.x.toFixed(1);
        botPos.y = point.y.toFixed(1);
        botPos.rot = radToDeg(value.rot).toFixed(1);
    })

</script>

