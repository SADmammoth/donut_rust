use crate::{figures::Figure, Canvas, Point};

#[derive(Debug)]
pub struct CanvasPoint {
    pub x: usize,
    pub y: usize,
    pub intensity: u8,
}

pub fn format_matrix(mat: Vec<Vec<char>>) -> String {
    mat.iter()
        .map(|row| format!("{:}\n", &row.iter().collect::<String>()))
        .collect::<String>()
}

pub fn convert_to_canvas_points(
    points: &Vec<Point>,
    canvas_width: usize,
    canvas_height: usize,
) -> Vec<CanvasPoint> {
    points
        .iter()
        .map(|point| CanvasPoint {
            x: mul_usize_f32(&canvas_width, &point.x),
            y: mul_usize_f32(&canvas_height, &point.y),
            intensity: point.intensity,
        })
        .collect()
}

pub fn convert_figure_to_canvas_points(
    figure: &Figure,
    canvas_width: usize,
    canvas_height: usize,
) -> Vec<CanvasPoint> {
    let steps_x = mul_usize_f32_ceil(&canvas_width, &figure.width);
    let step_x = 1f32 / steps_x as f32;
    let steps_y = mul_usize_f32_ceil(&canvas_height, &figure.height);
    let step_y = 1f32 / steps_y as f32;

    let mut points = vec![];

    for y in 0..steps_y {
        for x in 0..steps_x {
            if let Some(point) = (*figure.point_mapper)(Point {
                x: x as f32 * step_x,
                y: y as f32 * step_y,
                intensity: 0,
            }) {
                points.push(point);
            }
        }
    }

    convert_to_canvas_points(&points, canvas_width, canvas_height)
}

fn mul_usize_f32(a: &usize, b: &f32) -> usize {
    ((*a as f32) * b).round() as usize
}

fn mul_usize_f32_floor(a: &usize, b: &f32) -> usize {
    ((*a as f32) * b).floor() as usize
}

fn mul_usize_f32_ceil(a: &usize, b: &f32) -> usize {
    ((*a as f32) * b).ceil() as usize
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
            prev_matrix[y][x] = ((current_value + point.intensity) as f32 / 2.0).round() as u8;
            return;
        }
        prev_matrix[y][x] = point.intensity;
    });

    prev_matrix
}
