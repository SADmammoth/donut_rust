use super::super::*;
use std::{thread, time};
use super::super::transformations::*;
use super::super::figures::*;

pub fn frame_animation(printer: &mut Printer, background: &Canvas/*, frames: &Frame[]*/) {
    let mut angle = 0;
    let delay = time::Duration::from_millis(100);

    loop {
        angle += 5;
        let rectangle = rect(Point::new(0.33, 0.66, 0), Point::new(0.66, 0.33, 0), 3);
        printer.print_figure(&affine_transform(rectangle, Angle{ value: angle }));
        thread::sleep(delay);
        printer.wipe(); 
    }
}