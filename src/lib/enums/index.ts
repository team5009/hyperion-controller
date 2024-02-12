export enum PreviewAppState {
    STOPPED,
    PAUSED,
    RUNNING,
    RESETING
}

export enum AppState {
    MENU,
    CONTROLLER,
    CREATOR,
    PREVIEW,
    SETTINGS
}

export enum ConnectionStatus {
    Connected,
    Disconnected,
    Pending,
    Error
}

export enum NotificationType {
    Success,
    Info,
    Warning,
    Error
}

export enum EventType {
    //Status
	GET_ROBOT_STATUS,
	RECEIVE_ROBOT_STATUS,
//Op Mode Management
	INIT_OP_MODE,
	START_OP_MODE,
	STOP_OP_MODE,
	RECEIVE_OP_MODE_LIST,
//Telemetry
	RECEIVE_TELEMETRY,
//Camera
	GET_CAMERA_FRAME,
//Path
	STORE_GENERATED_PATH,
	DELETE_GENERATED_PATH,
	GET_GENERATED_PATHS,
//Health
	PING

}

export enum ErrorType {
    SUCCESS,
    INFO,
    WARNING,
    ERROR,
    CRITICAL,
    NOTHING,
}

export enum AppColor {
    SECONDARY = "#ff9812"
}

export enum FTCSEASON {
    ULTIMATE_GOAL = 2021,
    FREIGHT_FRENZY = 2022,
    POWERPLAY = 2023,
    CENTERSTAGE = 2024,
}