use std::{thread, time};
use term_printer_2d::{animation::*, Printer, figures::rect, Point, transformations::* };
use std::sync::{Arc};

fn main() {
    let mut printer = Printer::new();

    let mut pos = -0.33;
    let mut angle = 0;
    let mut scale = 1.0;

    let mut frames: Vec<Frame> = vec![];

    while pos < 1.0 && angle < 360 {
      
        let mut rectangle = rect(Point::new(0.33, 0.66, 0), Point::new(0.66, 0.33, 0), 3);
        rectangle = Transform::new(rectangle)
                    .rotate(Angle { value: angle })
                    .offset(Point { x: pos, y: pos, intensity: 0 })
                    .scale(Scale { x: scale, y: scale})
                    .apply();

        frames.push(Frame {
            matrix: printer.get_figure_matrix(&rectangle),
        });
        pos += 0.01;
        angle += 5;
        if scale >= 2.0 && scale > 0.0
        {
          scale -= 0.01
        } else {
          scale += 0.01;
        }
    }

    let frames = Arc::new(frames);

    let animator = Animator::new_with_printer_background(printer);

    printer = animator
        .by_frame(Arc::clone(&frames), AnimationTime::Milliseconds(10000))
        .return_printer();
    
    printer.wipe();
    thread::sleep(time::Duration::from_millis(1000));

    let animator2 = Animator::new_with_printer_background(printer);

    animator2.by_frame(Arc::clone(&frames), AnimationTime::Milliseconds(10000));
}
