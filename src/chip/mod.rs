use std::ffi::c_void;

mod close;
mod get_line;
mod num_lines;
mod open;
mod open_by_number;

pub use close::*;
pub use get_line::*;
pub use num_lines::*;
pub use open::*;
pub use open_by_number::*;

#[allow(non_camel_case_types, missing_docs)]
pub type gpiod_chip = c_void;
