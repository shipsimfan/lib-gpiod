use crate::gpiod_line;
use std::ffi::c_int;

#[link(name = "gpiod")]
unsafe extern "C" {
    /// Read current value of a single GPIO line.
    ///
    /// # Parameters
    ///  * `line` - GPIO line object.
    ///
    /// # Returns
    /// 0 or 1 if the operation succeeds. On error this routine returns -1 and sets the last error
    /// number.
    pub fn gpiod_line_get_value(line: *mut gpiod_line) -> c_int;
}
