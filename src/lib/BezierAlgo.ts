import { Point } from ".";

export function Lerp(a: number, b: number, t: number): number {
    return a + (b - a) * t;
}

export function Bezier(p0: Point, c0: Point, c1: Point, p1: Point): Point[] {
    const points: Point[] = [];
    for (let i = 0; i <= 1; i+=0.05) {
        let constant = 1 - i
        let x =  Math.pow(constant, 2) * p0.x + 3 * Math.pow(constant, 2)* i * c0.x + 3 * constant * Math.pow(i, 2) * c1.x + Math.pow(i, 3) * p1.x
        let y =  Math.pow(constant, 2) * p0.y + 3 * Math.pow(constant, 2)* i * c0.y + 3 * constant * Math.pow(i, 2) * c1.y + Math.pow(i, 3) * p1.y

        let rot: number
        if (points.length > 0) {
            const lastPoint = points[points.length - 1];
            rot = Math.atan2(y - lastPoint.y, x - lastPoint.x);
        } else {
            rot = Math.atan2(y - p0.y, x - p0.x);
        }
        
        if (rot > Math.PI / 4) {
            rot = Lerp(p0.rot, p1.rot, i);
        } else if (i >= 1) {
            rot = Lerp(p0.rot, p1.rot, i);
        } else {
            rot *= 180 / Math.PI;
        }
        
        points.push(Point(x, y, rot));
    }
    return points;
}