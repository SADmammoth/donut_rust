use std::{thread, time};
use term_printer_2d::animation::*;
use term_printer_2d::figures::*;
use term_printer_2d::transformations::*;
use term_printer_2d::Printer;
use term_printer_2d::*;

fn main() {
    let mut printer = Printer::new();
    let animator = Animator::new_with_printer_background(printer);

    printer = animator
        .by_frame(AnimationTime::Milliseconds(10000))
        .return_printer();
    printer.wipe();
    thread::sleep(time::Duration::from_millis(1000));

    let animator2 = Animator::new_with_printer_background(printer);

    animator2.by_frame(AnimationTime::Milliseconds(10000));
}
