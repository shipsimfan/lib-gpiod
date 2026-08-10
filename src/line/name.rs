use crate::gpiod_line;
use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "gpiod")]
unsafe extern "C" {
    /// Read the GPIO line name.
    ///
    /// # Parameters
    ///  * `line` - GPIO line object.
    ///
    /// # Returns
    /// Name of the GPIO line as it is represented in the kernel. This routine returns a pointer to
    /// a null-terminated string or [`null`] if the line is unnamed.
    pub fn gpiod_line_name(line: *mut gpiod_line) -> *const c_char;
}
