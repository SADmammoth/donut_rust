use std::fmt::Debug;

pub static mut DEBUG_STRING: String = String::new();

#[allow(dead_code)]
pub fn update_debug_string(debug_object: Box<dyn Debug>) {
  unsafe {
    DEBUG_STRING = format!("{:?}", debug_object);
  }
}

#[allow(dead_code)]
pub fn get_debug_string() -> &'static str {
  unsafe {
    return &DEBUG_STRING;
  }
}

#[allow(dead_code)]
pub fn append_debug_string(string: String)  {
  unsafe {
    DEBUG_STRING = format!("{}\n{}", DEBUG_STRING, string);
  }
}