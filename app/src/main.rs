use term_printer_2d::{ Printer, figures::line, Point };

fn main() {
    let mut printer = Printer::new();

    printer.print_figure(&line(Point::new(0.0, 0.0, 4), Point::new(1.0, 1.0, 4), 0.02));
    printer.print_figure(&line(Point::new(1.0, 1.0, 4), Point::new(0.0, 1.0, 4), 1.0));
    printer.print_figure(&line(Point::new(0.0, 1.0, 4), Point::new(0.0, 0.0, 4), 1.0));
    
    printer.print_figure(&line(Point::new(0.0, 0.0, 4), Point::new(0.2, 0.6, 4), 0.04));
    printer.print_figure(&line(Point::new(1.0, 1.0, 4), Point::new(0.2, 0.6, 4), 0.04));
    printer.print_figure(&line(Point::new(0.0, 1.0, 4), Point::new(0.2, 0.6, 4), 0.04));
}
