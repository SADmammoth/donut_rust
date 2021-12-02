mod printer;
use printer::figures::*;
use printer::Printer;
use printer::*;
use printer::transformations::*;
use std::{thread, time};

fn main() {
    let mut printer = Printer::new();

    let mut angle = 0;
    let delay = time::Duration::from_millis(100);

    loop{   
        printer.print_figure(&rect(Point::new(0.0, 1.0, 0), Point::new(1.0, 0.0, 0), 1));
        angle += 5;
        let rectangle = rect(Point::new(0.33, 0.66, 0), Point::new(0.66, 0.33, 0), 3);
        printer.print_figure(&affine_transform(rectangle, Angle{ value: angle }));
        thread::sleep(delay);
        printer.wipe(); 
    }
}
