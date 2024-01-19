use ndarray::arr2;

use crate::Point;
use serde_json::{json, Value};

pub fn bezier_curve(points: Vec<Point>, resolution: u32) -> Value {
    let matrix_a = arr2(&[
        [1.0, -3.0, 3.0, -1.0],
        [0.0, 3.0, -6.0, 3.0],
        [0.0, 0.0, 3.0, -3.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let matrix_b = arr2(&[
        [points[0].x, points[0].y],
        [points[1].x, points[1].y],
        [points[2].x, points[2].y],
        [points[3].x, points[3].y],
    ]);

    let matrix = matrix_a.dot(&matrix_b);

    let mut points: Vec<Point> = vec![];

    for i in 0..(resolution + 1) {
        let t = i as f32 / resolution as f32;
        let t2 = t * t;
        let t3 = t2 * t;

        let x = matrix[[0, 0]] * t3 + matrix[[1, 0]] * t2 + matrix[[2, 0]] * t + matrix[[3, 0]];
        let y = matrix[[0, 1]] * t3 + matrix[[1, 1]] * t2 + matrix[[2, 1]] * t + matrix[[3, 1]];

        points.push(Point::new(x, y, 0.0));
    }

    json!(points)
}
