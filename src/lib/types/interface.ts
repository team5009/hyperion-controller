import type { Event, FTCSEASON, Measurement, NotificationType, Point } from "$lib";

export interface Notification {
    show: boolean,
    type: NotificationType,
    message: string
}

export interface CommandPath {
    "Start" : Point,
    "Wait" : Wait,
    "Line" : Point,
    "Spline" : Spline[],
    "End" : Event
};

export interface Spline {
    Bezier: Bezier,
    Wait: Wait
}

export interface Wait {
    wait_type: WaitType;
    event: Event;
}

export interface Bezier {
    start: Point,
    control: Point[],
    end: Point
}

export interface WaitType  {
    Time: number;
    Event: Event;
}

export interface Message {
    type: string,
    data: any
}

export interface Settings {
    ip: string,
    port: number,
    speed: number,
    width: number,
    length: number,
    measurements: Measurement,
    year: FTCSEASON,
    color: string
}