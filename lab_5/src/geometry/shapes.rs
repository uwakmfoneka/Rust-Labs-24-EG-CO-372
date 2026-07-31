pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

pub struct Polygon {pub vertices: Vec<Point>}

impl Polygon {
    pub fn perimeter(&self) -> f64 {
        let mut perimeter = 0.0;
        let n = self.vertices.len();
        for i in 0..n {
            let next_index = (i + 1) % n;
            perimeter += self.vertices[i].distance(&self.vertices[next_index]);
        }
        perimeter
    }

    pub fn is_closed(&self) -> bool {
        if self.vertices.len() < 3 {
            return false;
        }
        let first = &self.vertices[0];
        let last = &self.vertices[self.vertices.len() - 1];
        first.x == last.x && first.y == last.y
    }
}

