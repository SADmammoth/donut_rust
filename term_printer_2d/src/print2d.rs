use crate::{figures::Printable, Canvas, Intensity, Pixel, Point};

#[derive(Debug)]
pub struct CanvasPoint {
    pub x: usize,
    pub y: usize,
    pub intensity: Intensity,
}

pub fn format_matrix(mat: Vec<Vec<char>>) -> String {
    mat.iter()
        .map(|row| format!("{:}\n", &row.iter().collect::<String>()))
        .collect::<String>()
}

pub fn convert_to_canvas_points(
    pixels: &Vec<Pixel>,
    canvas_width: usize,
    canvas_height: usize,
) -> Vec<CanvasPoint> {
    pixels
        .iter()
        .map(|pixel| CanvasPoint {
            x: mul_usize_f64(&canvas_width, &pixel.get_x()),
            y: mul_usize_f64(&canvas_height, &pixel.get_y()),
            intensity: pixel.get_intensity(),
        })
        .collect()
}

pub fn convert_figure_to_canvas_points(
    figure: Box<dyn Printable>,
    canvas_width: usize,
    canvas_height: usize,
) -> Vec<CanvasPoint> {
    let mut steps_x = mul_usize_f64_ceil(&canvas_width, &figure.width());
    if steps_x <= 0 {
        steps_x = 1;
    }

    let step_x = 1f64 / steps_x as f64;
    let mut steps_y = mul_usize_f64_ceil(&canvas_height, &figure.height());
    if steps_y <= 0 {
        steps_y = 1;
    }

    let step_y = 1f64 / steps_y as f64;

    let mut pixels = vec![];

    for y in 0..steps_y {
        for x in 0..steps_x {
            if let Some(pixel) = figure.map(x, y, Point::new(x as f64 * step_x, y as f64 * step_y)) {
                pixels.push(pixel);
            }
        }
    }

    convert_to_canvas_points(&pixels, canvas_width, canvas_height)
}

fn mul_usize_f64(a: &usize, b: &f64) -> usize {
    ((*a as f64) * b).round() as usize
}

// fn mul_usize_f64_floor(a: &usize, b: &f64) -> usize {
//     ((*a as f64) * b).floor() as usize
// }

fn mul_usize_f64_ceil(a: &usize, b: &f64) -> usize {
    ((*a as f64) * b).ceil() as usize
}

pub fn get_print_matrix(mut prev_matrix: Canvas, canvas_points: &Vec<CanvasPoint>) -> Canvas {
    canvas_points.iter().for_each(|point| {
        let mut y = point.y;
        if y >= prev_matrix.len() {
            y = prev_matrix.len() - 1;
        }
        let mut x = point.x;
        if x >= prev_matrix[y].len() {
            x = prev_matrix[y].len() - 1;
        }
        if prev_matrix[y][x] != 0 {
            let current_value = prev_matrix[y][x];
            prev_matrix[y][x] =
                ((current_value + point.intensity.get_value()) as f64 / 2.0).round() as u8;
            return;
        }
        prev_matrix[y][x] = point.intensity.get_value();
    });

    prev_matrix
}
