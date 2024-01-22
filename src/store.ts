import { writable } from "svelte/store";
import { PreviewAppState, type Point, AppState, type CommandPath } from "./lib";

export const mousePosition = writable({ x: 0, y: 0 });
export const BotPosition = writable({ x: 0, y: 0, rot: 0 } as Point);
export const BotSpeed = writable(0);
export const appPreviewState = writable(0 as PreviewAppState);
export const appState = writable(0 as AppState);
export const pathCommands = writable([] as CommandPath[]);
export const BotSocketConnected = writable(false);