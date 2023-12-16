import { Point } from ".";

export function Lerp(a: number, b: number, t: number): number {
    return a + (b - a) * t;
}
export function Bezier(p0: Point, c0: Point, c1: Point, p1: Point): Point[] {
    const points: Point[] = [];
    for (let i = 0; i <= 1; i+=0.05) {
        let x1 = Lerp(p0.x, c0.x, i);
        let y1 = Lerp(p0.y, c0.y, i);
        let x2 = Lerp(x1, c1.x, i);
        let y2 = Lerp(y1, c1.y, i);
        
        let x3 = Lerp(x2, p1.x, i);
        let y3 = Lerp(y2, p1.y, i);

        let rot: number
        if (points.length > 0) {
            const lastPoint = points[points.length - 1];
            rot = Math.atan2(y3 - lastPoint.y, x3 - lastPoint.x);
        } else {
            rot = Math.atan2(y3 - p0.y, x3 - p0.x);
        }
        
        if (rot < Math.PI / 4) {
            rot = Lerp(p0.rot, p1.rot, i);
        } else if (i >= 1) {
            rot = Lerp(p0.rot, p1.rot, i);
        } else {
            rot *= 180 / Math.PI;
        }
        
        points.push(Point(x3, y3, rot));
    }
    return points;
}