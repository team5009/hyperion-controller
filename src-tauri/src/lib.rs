pub mod construction;

#[derive(PartialEq)]
pub enum State {
    Bezier,
    // Line,
    Spline,
    Start,
    Check,
    // End,
}

#[derive(serde::Serialize)]
pub struct Point {
    x: f32,
    y: f32,
    rot: f32,
}
impl Point {
    fn new(x: f32, y: f32, rot: f32) -> Point {
        Point { x, y, rot }
    }
    fn new_from_vec(vec: Vec<f32>) -> Point {
        Point {
            x: vec[0],
            y: vec[1],
            rot: vec[2],
        }
    }
}

#[derive(serde::Serialize)]
pub enum Command {
    Start(Point),
    Bezier(Vec<Point>),
    Goto(Point),
    // Line(Point, Point),
    Spline(Vec<Vec<Point>>),
    Wait(String),
    // End,
}
