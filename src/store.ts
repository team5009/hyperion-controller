import { writable } from "svelte/store";
import { PreviewAppState, type Point } from "./lib";

export const mousePosition = writable({ x: 0, y: 0 });
export const BotPosition = writable({ x: 0, y: 0, rot: 0 } as Point);
export const BotSpeed = writable(0);
export const appPreviewState = writable(0 as PreviewAppState);