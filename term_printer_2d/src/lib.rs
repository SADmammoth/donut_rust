mod point;
mod term_print;

pub mod animation;
pub mod figures;
pub mod print2d;
pub mod transformations;

pub use point::*;
pub use term_print::*;

pub type Canvas = Vec<Vec<u8>>;
