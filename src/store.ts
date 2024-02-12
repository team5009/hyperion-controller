import { writable } from "svelte/store";
import { PreviewAppState, type Point, AppState, type CommandPath, ErrorType, type Settings, FTCSEASON } from "$lib";



export const mousePosition = writable({ x: 0, y: 0 });
export const BotPosition = writable({ x: 0, y: 0, rot: 0 });
export const BotSpeed = writable(0);
export const appPreviewState = writable(0 as PreviewAppState);
export const appState = writable(AppState.PREVIEW as AppState);
export const pathCommands = writable([] as CommandPath[]);
export const BotSocketConnected = writable(true);
export const NotificationState = writable({type: ErrorType.NOTHING} as { type: ErrorType, message: string });
export const SettingsModalState = writable(false);
export const SettingsState = writable({
    ip: "127.0.0.1",
    port: 43,
    speed: 0,
    width: 0,
    length: 0,
    measurements: "in",
    year: FTCSEASON.CENTERSTAGE,
    color: "#ff9812"
} as Settings);