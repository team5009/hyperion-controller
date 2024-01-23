export class Circle {
    constructor(
        private x: number,
        private y: number,
        private radius: number,
        private color: string,
        private lineWidth: number
    ) {

    }
    draw(context: CanvasRenderingContext2D) {
        context.beginPath();
        context.arc(this.x, this.y, this.radius, 0, 2 * Math.PI);
        context.strokeStyle = this.color;
        context.lineWidth = this.lineWidth;
        context.stroke();
    }

}