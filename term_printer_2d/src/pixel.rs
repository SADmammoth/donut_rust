use crate::Point;

#[derive(Debug, Clone, Copy)]
pub struct Intensity {
    value: u8,
}

impl Intensity {
    const MAX: u8 = 8;

    pub fn new(value: u8) -> Intensity {
        if value > Self::MAX {
            panic!(
                "Intensity mustn't be higher than {:}",
                &Self::MAX.to_string()
            )
        }

        Intensity { value }
    }

    pub fn get_value(self: &Self) -> u8 {
      self.value
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    intensity: Intensity,
    point: Point,
}

impl Pixel {
    pub fn new(x: f32, y: f32, intensity: Intensity) -> Pixel {
        Pixel {
            point: Point::new(x, y),
            intensity,
        }
    }

    pub fn get_x(self: &Self) -> f32 {
        self.point.get_x()
    }

    pub fn get_y(self: &Self) -> f32 {
        self.point.get_y()
    }

    pub fn get_intensity(self: &Self) -> Intensity {
        self.intensity
    }
}
