use std::ffi::c_void;

mod get_value;
mod name;

pub use get_value::*;
pub use name::*;

#[allow(non_camel_case_types, missing_docs)]
pub type gpiod_line = c_void;
