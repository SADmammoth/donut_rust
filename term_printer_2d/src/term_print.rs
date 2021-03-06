use crate::{figures, print2d, Canvas, Pixel};
use figures::Printable;
use print2d::{
    convert_figure_to_canvas_points, convert_to_canvas_points, format_matrix, get_print_matrix,
    CanvasPoint,
};
use std::process::Command;
use crate::get_debug_string;

pub const CHAR_ASPECT_RATIO: f64 = 0.4;

#[derive(Debug)]
pub struct Printer {
    mat: Canvas,
    intensity_map: Vec<char>,
    width: usize,
    height: usize,
}

impl Printer {
    pub fn new() -> Printer {
        let (term_w, _) = term_size::dimensions_stdout().unwrap();
        let (canvas_w, canvas_y) = (term_w, (term_w as f64 * CHAR_ASPECT_RATIO).round() as usize);

        Printer {
            mat: vec![vec![0; canvas_w]; canvas_y],
            intensity_map: vec![' ', '.', '+', '*', '#', '@'],
            width: canvas_w,
            height: canvas_y,
        }
    }

    pub fn relative(self: &Self, canvas_points: usize) -> (f64, f64) {
      let x_point = canvas_points as f64 / self.width as f64;
      let y_point = canvas_points as f64 / self.height as f64;

      (x_point, y_point)
    }

    pub fn wipe(self: &mut Self) {
        self.mat = vec![vec![0; self.width]; self.height];
        Self::clear_term();
    }

    pub fn clear_term() {
        print!("\r\x1b[0J");
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "cls"])
                .status()
                .expect("failed to execute process \"cmd /C cls\"");
        } else {
            Command::new("sh")
                .args(["-c", "clear"])
                .status()
                .expect("failed to execute process \"sh -c clear\"");
        };
    }

    fn canvas_to_string(self: &mut Self, canvas: &Canvas) -> String {
        let print_mat = format_matrix(
            canvas
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|i| self.intensity_map[*i as usize])
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>(),
        );

        self.mat = canvas.clone();

        print_mat
    }

    fn print(self: &mut Self, points: &Vec<CanvasPoint>) {
        Self::clear_term();
        print!(
            "{:}",
            self.canvas_to_string(&get_print_matrix(self.mat.clone(), &points))
        );
        println!("{}", get_debug_string());
    }

    pub fn print_points(self: &mut Self, points: &Vec<Pixel>) {
        let canvas_points = convert_to_canvas_points(points, self.width, self.height);
        self.print(&canvas_points);
    }

    pub fn print_figure(self: &mut Self, figure: Box<dyn Printable>) {
        let canvas_points = convert_figure_to_canvas_points(figure, self.width, self.height);

        self.print(&canvas_points);
    }

    pub fn get_figure_matrix(self: &Self, figure: Box<dyn Printable>) -> Canvas {
        let canvas_points = convert_figure_to_canvas_points(figure, self.width, self.height);
        get_print_matrix(self.mat.clone(), &canvas_points)
    }

    pub fn print_matrix(self: &mut Self, matrix: &Canvas) {
        Self::clear_term();
        print!("{:}", self.canvas_to_string(matrix));
        println!("{}", get_debug_string());
    }

    pub fn get_current_mat(self: &Self) -> Canvas {
        self.mat.clone()
    }
}
