use term_printer_2d::{figures::line, Intensity, Point, Printer};

pub fn run() {
  let mut printer = Printer::new();

    printer.print_figure(Box::new(line(
            Point::new(1.0, 0.0),
            Point::new(0.0, 0.0),
            Intensity::new(4),
            printer.relative(1),
        )));
}