mod printer;
use printer::figures::*;
use printer::Printer;
use printer::*;
use printer::transformations::*;

fn main() {
    let mut printer = Printer::new();
    printer.print_figure(&rect(Point::new(0.0, 1.0, 0), Point::new(1.0, 0.0, 0), 1));
    printer.print_figure(&rect(
        Point::new(0.33, 0.63, 0),
        Point::new(0.63, 0.33, 0),
        3,
    ));
    let rectangle = rect(Point::new(0.2, 0.5, 0), Point::new(0.5, 0.2, 0), 5);
    printer.print_figure(&affine_transform(rectangle, Angle{ value: 45 }));
}
