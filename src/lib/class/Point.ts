export class Point {
    x: number;
    y: number;
    rot: number;
    event: Event;
    constructor(x: number, y: number, rot: number, event: Event | undefined = undefined) {
        this.x = x;
        this.y = y;
        this.rot = rot;
        this.event = event ? event : new Event("");
    }

}

export class Event {
    message: string;
    constructor(message: string) {
        this.message = message;
    }
}