import type { NotificationType, Point } from "$lib";

export interface Notification {
    show: boolean,
    type: NotificationType,
    message: string
}

export interface CommandPath {
    "Start" : Point,
    "Wait" : String,
    "Goto" : Point,
    "Spline" : Point[][],
};
