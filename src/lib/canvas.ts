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