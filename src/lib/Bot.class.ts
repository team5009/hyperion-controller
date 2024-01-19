import { Point, degToRad, Square, Line, Circle, refDeg, resolution } from ".";
import { fieldToCanvas, refRad } from "./math";

export class Bot {
    dx: number;
    dy: number;
    drot: number;
    x: number;
    y: number;
    rot: number;
    finalPoint: Point = Point(0, 0, 0);
    constructor(
        point: Point,
        private speed: number,
    ) {
        const convertedPoint = fieldToCanvas(point);
        this.dx = 1 * speed;
        this.dy = 1 * speed;
        this.drot = 1 * speed;
        this.x = convertedPoint.x;
        this.y = convertedPoint.y;
        this.rot = degToRad(point.rot);
    }
    draw(context: CanvasRenderingContext2D) {
        const radius = 62.5 * resolution / 1080;
        const referencePoint = {
            x: this.x,
            y: this.y
        }
        context.beginPath();
        context.strokeStyle = "pink";
        context.lineWidth = 4;
        context.translate(referencePoint.x, referencePoint.y);
        context.rotate(this.rot);
        context.translate(-referencePoint.x, -referencePoint.y);
        // context.strokeRect(this.x - radius, this.y - radius, radius * 2, radius * 2);
        new Square(this.x - radius, this.y - radius, radius * 2, "pink", 4* resolution / 1080).draw(context);
        
        new Line(this.x, this.y, this.x + radius, this.y , "pink", 4* resolution / 1080).draw(context);
        new Circle(this.x, this.y, 4, "pink", 4* resolution / 1080).draw(context);
        context.setTransform(1, 0, 0, 1, 0, 0);

    }

    updateNextPoint(point: Point) {
        this.finalPoint = {
            ...fieldToCanvas(point),
            rot: degToRad(point.rot)
        }
    }

    update(context: CanvasRenderingContext2D, max: {x: number, y: number}, point: Point) {
        context.clearRect(0, 0, max.x, max.y);
        this.draw(context);
        const nextPoint = fieldToCanvas(point);
        const deltaX = nextPoint.x - this.x;
        const deltaY = nextPoint.y - this.y;
        const distanceX = this.finalPoint.x - this.x;
        const distanceY = this.finalPoint.y - this.y;
        const deltaRot = refRad(degToRad(point.rot)) - this.rot;
        const theta = Math.atan2(deltaY, deltaX);
        const sinVal = Math.sin(theta);
        const cosVal = Math.cos(theta);

        // get the current postion a devide by the final position distance

        if (Math.abs(deltaRot) > 0) { 
            this.drot = deltaRot * this.speed/150;
        } else {
            this.drot = 0;
        }

        if (Math.abs(deltaX) > 0) {
            this.dx = cosVal * (Math.abs(distanceX/2) / this.speed);
        } else {
            this.dx = 0;
        }

        if (Math.abs(deltaY) > 0) {
            this.dy = sinVal * (Math.abs(distanceY/2)/this.speed);
        } else {
            this.dy = 0;
        }

        this.x += this.dx;
        this.y += this.dy;
        this.rot += this.drot;

    }

    resetPosition(context: CanvasRenderingContext2D, max: {x: number, y: number},point: Point) {
        context.clearRect(0, 0, max.x, max.y);
        const convertedPoint = fieldToCanvas(point);
        this.x = convertedPoint.x;
        this.y = convertedPoint.y;
        this.rot = degToRad(point.rot);

        this.draw(context);
    }
}