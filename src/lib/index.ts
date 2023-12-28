import { BotPosition } from '../store';

export const resolution = 1920;

export type Point = {
    x: number,
    y: number,
    rot: number
}

export enum PreviewAppState {
    STOPPED, // The preview is stopped
    PAUSED, // The preview is paused
    RUNNING, // The preview is running
    RESETING // The preview is reseting
}

export enum AppState {
    MENU, // The app is in the menu
    CREATOR, // The app is in the editor
    PREVIEW, // The app is in the preview
    SETTINGS // The app is in the settings
}

export function Point(x: number, y: number, rot: number): Point {
    return { x, y: y, rot };
}

export type Command = "Start" | "Wait" | "Goto" | "Spline";
export interface CommandPath {
    "Start" : Point,
    "Wait" : String,
    "Goto" : Point,
    "Spline" : Point[],
};

export {
    canvasToField,
    fieldToCanvas,
    degToRad,
    radToDeg,
    refRad,
    refDeg
} from "./math";

export {
    Line,
    Circle,
    Square
} from "./canvas";

export {
    Bot
} from "./Bot.class";

export {
    Bezier
} from "./BezierAlgo";