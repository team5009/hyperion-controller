import { Line } from "$lib";

export class Square {
    constructor(
        private x: number,
        private y: number,
        private size: number,
        private color: string,
        private lineWidth: number
    ) {

    }
    draw(context: CanvasRenderingContext2D) {
        new Line(this.x, this.y, this.x + this.size, this.y, this.color, this.lineWidth).draw(context);
        new Line(this.x + this.size, this.y, this.x + this.size, this.y + this.size, this.color, this.lineWidth).draw(context);
        new Line(this.x + this.size, this.y + this.size, this.x, this.y + this.size, this.color, this.lineWidth).draw(context);
        new Line(this.x, this.y + this.size, this.x, this.y, this.color, this.lineWidth).draw(context);
    }

    
}