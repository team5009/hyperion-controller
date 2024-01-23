export class Line {
    constructor(
        private x1: number,
        private y1: number,
        private x2: number,
        private y2: number,
        private color: string,
        private lineWidth: number
    ) {

    }
    draw(context: CanvasRenderingContext2D) {
        context.beginPath();
        context.moveTo(this.x1, this.y1);
        context.lineTo(this.x2, this.y2);
        context.strokeStyle = this.color;
        context.lineWidth = this.lineWidth;
        context.stroke();
    }
}