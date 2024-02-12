import { Point } from "$lib";
import { resolution } from ".";
import type { Measurement } from "../types";

export function canvasToField(point: {x: number, y:number}) : {x: number, y: number} {
    return {
        x: (point.x * 144 / resolution) - 72,
        y: (point.y * 144 / resolution) - 72
    }
}

export function fieldToCanvas(point: {x: number, y:number}) : {x: number, y: number} {
    return {
        x: (point.x + 72) * resolution / 144,
        y: (point.y + 72) * resolution / 144
    }
}

export const canvasResolutionConvert = (x: number) => x / 1080;

export function degToRad(deg: number) {
    return deg * Math.PI / 180;
}

export function radToDeg(rad: number) {
    return rad * 180 / Math.PI;
}

export function refRad(theta: number) {
    if (theta < 0) {
        return -((-theta + Math.PI) % (2 * Math.PI)) + Math.PI;
    }
    return (theta + Math.PI) % (2 * Math.PI) - Math.PI
}

export function refDeg(theta: number) {
    if (theta < 0) {
        return -((-theta + 180) % 360) + 180;
    }
    return (theta + 180) % 360 - 180;
}

/**
 * Converts value to inches if it is in a different unit
 * @param value number to convert
 * @param unit unit of value
 */
export function convertToUnit(value: number, prevUnit: Measurement, unit: Measurement) {
    if (prevUnit === unit) {
        return value;
    }
    switch (prevUnit) {
        case "in":
            return unit === "cm" ? value * 2.54 : value * 25.4;
        case "cm":
            return unit === "in" ? value / 2.54 : value * 10;
        case "mm":
            return unit === "in" ? value / 25.4 : value / 10;
    }
    
}


export function Lerp(a: number, b: number, t: number): number {
    return a + (b - a) * t;
}

export function BezierGen(p0: Point, c0: Point, c1: Point, p1: Point): Point[] {
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
        
        points.push(new Point(x, y, rot));
    }
    return points;
}