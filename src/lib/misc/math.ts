import { resolution } from ".";

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

