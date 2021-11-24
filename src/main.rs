mod printer;
use printer::*;

fn main() {
    let points = vec![Point::new(0.5f32, 0.5f32)];
    print2d(&points);
}
