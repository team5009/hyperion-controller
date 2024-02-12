use crate::{Event, Point};
use serde_json::{json, Value};

pub fn bezier_curve(points: Vec<Point>, resolution: u32) -> Value {
    let mut temp_points: Vec<Point> = vec![];

    for i in 0..(resolution + 1) {
        let t = i as f64 / resolution as f64;

        let x = (1.0 - t).powi(3) * points[0].x  // X Point
            + 3.0 * (1.0 - t).powi(2) * t * points[1].x
            + 3.0 * (1.0 - t) * t.powi(2) * points[2].x
            + t.powi(3) * points[3].x;

        let y = (1.0 - t).powi(3) * points[0].y  // Y point
            + 3.0 * (1.0 - t).powi(2) * t * points[1].y
            + 3.0 * (1.0 - t) * t.powi(2) * points[2].y
            + t.powi(3) * points[3].y;

        if (i as f64 / resolution as f64) == 0.0 {
            // For the first point, use the first event
            temp_points.push(Point::new(x, y, 0.0, points[0].event.clone()));
        } else if (i as f64 / resolution as f64) == 1.0 {
            // For the last point, use the last event
            temp_points.push(Point::new(x, y, 0.0, points[3].event.clone()));
        } else if (i as f64 / resolution as f64) == (1.0 / 4.0) {
            // For 1/4 of the way through, use the first control point event
            temp_points.push(Point::new(x, y, 0.0, points[1].event.clone()));
        } else if (i as f64 / resolution as f64) == (3.0 / 4.0) {
            // For 3/4 of the way through, use the second control point event
            temp_points.push(Point::new(x, y, 0.0, points[2].event.clone()));
        } else {
            // For all other points, use the "nothing" event (which is just an empty string)
            temp_points.push(Point::new(x, y, 0.0, Event::new("nothing".to_string())));
        }
    }

    json!(temp_points)
}
