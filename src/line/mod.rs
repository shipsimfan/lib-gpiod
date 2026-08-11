use std::ffi::c_void;

mod get_value;
mod name;
mod release;
mod request_input;

pub use get_value::*;
pub use name::*;
pub use release::*;
pub use request_input::*;

#[allow(non_camel_case_types, missing_docs)]
pub type gpiod_line = c_void;
