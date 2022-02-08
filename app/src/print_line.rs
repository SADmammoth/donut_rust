use term_printer_2d::{figures::*, Intensity, Point, Printer, append_debug_string, update_debug_string};

pub fn run() {
  let mut printer = Printer::new();
  let line1 = line(
    Point::new(1.0, 0.0),
    Point::new(0.2, 0.9)
  ,
    Intensity::new(4),
    printer.relative(1),
  );
  let path1 = path(vec!{
    Point::new(1.0, 0.0),
    Point::new(0.2, 0.9),
    Point::new(0.0, 1.0)
  },
    Intensity::new(4),
    printer.relative(1),
);
  
  append_debug_string(format!("{:?}", (line1.origin(), path1.origin())));
    printer.print_figure(Box::new(path1));
}