mod geometry;
mod utils;

use geometry::Point;
use geometry::shapes::Polygon;

fn main() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(3.0, 4.0);
    println!("Distance a-b: {:.2}", a.distance(&b));

    let square = Polygon {
        vertices: vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
        ],
    };
    println!("perimeter: {:.2}", square.perimeter());
    println!("is closed: {}", square.is_closed());
}