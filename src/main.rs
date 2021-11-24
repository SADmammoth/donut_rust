mod printer;
use printer::Printer;
use printer::*;
use printer::figures::*;

fn main() {
    let mut printer = Printer::new();
    printer.print_figure(rect(Point::new(0.0,1.0, 0), Point::new(1.0, 0.0, 0), 1));
    printer.print_figure(rect(Point::new(0.2, 0.5, 0), Point::new(0.5, 0.0, 0), 3));
}
