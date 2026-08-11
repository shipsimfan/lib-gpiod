use crate::gpiod_line;
use std::ffi::{c_char, c_int};

#[link(name = "gpiod")]
unsafe extern "C" {
    /// Reserve a single line, set the direction to input.
    ///
    /// # Parameters
    ///  * `line` - GPIO line object.
    ///  * `consumer` - Name of the consumer.
    ///
    /// # Returns
    /// 0 if the line was properly reserved, -1 on failure.
    pub fn gpiod_line_request_input(line: *mut gpiod_line, consumer: *const c_char) -> c_int;
}
