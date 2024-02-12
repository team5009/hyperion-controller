use std::fmt;

use serde::{Deserialize, Serialize};

pub mod commands;
pub mod connection;
pub mod construction;
pub mod controller;
pub mod math;
pub mod misc;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Event {
    message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Point {
    x: f64,
    y: f64,
    rot: f64,
    event: Event,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Bezier {
    start: Point,
    end: Point,
    control: Vec<Point>,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Wait {
    wait_type: WaitType,
    event: Event,
}

pub struct Error {
    error_type: ErrorType,
    message: String,
}

impl Point {
    fn new(x: f64, y: f64, rot: f64, event: Event) -> Point {
        Point { x, y, rot, event }
    }

    fn new_from_vec(vec: Vec<f64>, event: Event) -> Point {
        Point {
            x: vec[0],
            y: vec[1],
            rot: vec[2],
            event,
        }
    }
}

impl Bezier {
    fn new() -> Bezier {
        Bezier {
            start: Point::new(0.0, 0.0, 0.0, Event::new("".to_string())),
            end: Point::new(0.0, 0.0, 0.0, Event::new("".to_string())),
            control: vec![],
        }
    }

    fn start(&mut self, start: Point) {
        self.start = start;
    }

    fn end(&mut self, end: Point) {
        self.end = end;
    }

    fn add_control(&mut self, control: Point) {
        self.control.push(control);
    }
}

impl Event {
    fn new(message: String) -> Event {
        Event { message }
    }
}

impl Wait {
    fn new(wait_type: WaitType, event: Event) -> Wait {
        Wait { wait_type, event }
    }
}

impl Error {
    fn new(error_type: ErrorType, message: String) -> Error {
        Error {
            error_type,
            message,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {:?} - {}", self.error_type, self.message)
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Command {
    Start(Point),
    End(Event),
    Line(Point),
    Bezier(Bezier),
    Spline(Vec<Spline>),
    Wait(Wait),
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Spline {
    Bezier(Bezier),
    Wait(Wait),
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum WaitType {
    Time(u64),
    Event(Event),
}

#[derive(PartialEq)]
pub enum CommandState {
    Start,
    Reading,
    End,
    None,
}

#[derive(Debug)]
pub enum ErrorType {
    InvalidCommand,
    InvalidEvent,
    InvalidPoint,
    InvalidWait,
    InvalidStart,
    InvalidEnd,
    InvalidTime,
    InvalidSpline,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorType::InvalidCommand => write!(f, "Invalid Command"),
            ErrorType::InvalidEvent => write!(f, "Invalid Event"),
            ErrorType::InvalidPoint => write!(f, "Invalid Point"),
            ErrorType::InvalidWait => write!(f, "Invalid Wait"),
            ErrorType::InvalidStart => write!(f, "Invalid Start"),
            ErrorType::InvalidEnd => write!(f, "Invalid End"),
            ErrorType::InvalidTime => write!(f, "Invalid Time"),
            ErrorType::InvalidSpline => write!(f, "Invalid Spline"),
        }
    }
}

pub enum Call {
    NewLine(String),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Pending,
    Error,
}
