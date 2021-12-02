mod printer;
use printer::figures::*;
use printer::Printer;
use printer::*;
use printer::transformations::*;
use printer::animation::*;
use std::{thread, time};

fn main() {
    let mut printer = Printer::new();
    let mut animator = Animator::new_with_printer_background(printer);

    Animator::by_frame(animator);
}
