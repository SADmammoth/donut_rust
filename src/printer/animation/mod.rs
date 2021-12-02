mod frame_animation;
use super::Canvas;
use super::Printer;

pub struct Frame {
  matrix: Canvas,
}

pub struct Animator {
  background: Canvas,
  printer: Printer,
}

impl Animator {
  pub fn new_with_printer_background(printer: Printer) -> Animator {
    Animator {
      background: printer.get_current_mat(),
      printer
    }
  }

  pub fn new(background: Canvas, printer: Printer) -> Animator {
    Animator {
      background,
      printer
    }
  }

  pub fn by_frame(mut self: Self/*, frames: &Frame[]*/){
    frame_animation::frame_animation(&mut self.printer, &self.background/*, frames*/);
  }

  pub fn takeout_printer(self: Self) -> Printer {
    self.printer
  }
}