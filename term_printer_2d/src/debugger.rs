use std::fmt::Debug;

pub static mut debug_string: String = String::new();

pub fn update_debug_string(debug_object: Box<dyn Debug>) {
  unsafe {
    debug_string = format!("{:?}", debug_object);
  }
}

pub fn get_debug_string() -> &'static str {
  unsafe {
    return &debug_string;
  }
}

pub fn append_debug_string(string: String)  {
  unsafe {
    debug_string = format!("{}\n{}", debug_string, string);
  }
}