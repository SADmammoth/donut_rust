#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        if (x > 1.0 || y > 1.0) || (x < 0.0 || y < 0.0) {
            panic!("Point coordinates must be between 1 and 0")
        }
        Point { x, y }
    }

    pub fn is_higher_from(self: &Self, point: &Point) -> bool {
        self.y > point.y
    }

    pub fn is_to_right_from(self: &Self, point: &Point) -> bool {
        self.x > point.x
    }

    pub fn get_x(self: &Self) -> f64 {
        self.x
    }

    pub fn get_y(self: &Self) -> f64 {
        self.y
    }
}