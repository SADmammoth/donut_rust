const MAX_INTENSITY:u8 = 8;

#[derive(Debug)]
pub struct Point {
  pub x: f32,
  pub y: f32,
  pub intensity: u8,
}

impl Point {
  pub fn new(x:f32, y:f32, intensity: u8) -> Point {
    if (x > 1.0 || y > 1.0) || (x < 0.0 || y < 0.0)  {
      panic!("Point coordinates must be between 1 and 0")
    }

    if intensity > MAX_INTENSITY {
      panic!("Intensity mustn't be higher than ".to_string() + &MAX_INTENSITY.to_string())
    }

    Point{x, y, intensity}
  }
}