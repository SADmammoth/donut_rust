use super::figures::*;
use super::point::*;
use super::print2d::*;
use super::Canvas;

pub struct Printer {
    mat: Canvas,
    intensity_map: Vec<char>,
    width: usize,
    height: usize,
}

impl Printer {
    pub fn new() -> Printer {
        let (term_w, _) = term_size::dimensions_stdout().unwrap();
        let (canvas_w, canvas_y) = (term_w, (term_w as f32 * 0.4).round() as usize);

        Printer {
            mat: vec![vec![0; canvas_w]; canvas_y],
            intensity_map: vec![' ', '.', '+', '*', '#', '@'],
            width: canvas_w,
            height: canvas_y,
        }
    }

    pub fn wipe(self: &mut Self) {
      self.mat = vec![vec![0; self.width]; self.height];
    }

    fn canvas_to_string(self: &mut Self, canvas: Canvas) -> String {
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

        self.mat = canvas;

        print_mat
    }

    fn print(self: &mut Self, points: Vec<CanvasPoint>) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{:}", self.canvas_to_string get_print_matrix(self.mat.clone(), &points)));
    }

    pub fn print_points(self: &mut Self, points: &Vec<Point>) {
        let canvas_points = convert_to_canvas_points(points, self.width, self.height);
        self.print(canvas_points);
    }

    pub fn print_figure(self: &mut Self, figure: &Figure) {
        let canvas_points = convert_figure_to_canvas_points(figure, self.width, self.height);
      
        self.print(canvas_points);
    }

    pub fn get_figure_matrix(self: &Self, figure: &Figure) -> Canvas {
         let canvas_points = convert_figure_to_canvas_points(figure, self.width, self.height);
         get_print_matrix(self.mat.clone(), &canvas_points)
    }

    pub fn print_matrix(self: &Self, matrix: &Canvas) -> Canvas {
        println!("{:}", self.canvas_to_string(matrix));
    }

    pub fn get_current_mat(self: &Self) -> Canvas {
      self.mat.clone()
    }
}
