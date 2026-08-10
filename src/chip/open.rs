use crate::gpiod_chip;
use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "gpiod")]
unsafe extern "C" {
    /// Open a chip by path.
    ///
    /// # Parameters
    ///  * `path` - Path to the gpiochip device file.
    ///
    /// # Returns
    /// GPIO chip object or [`null_mut`] if an error occurred.
    pub fn gpiod_chip_open(path: *const c_char) -> *mut gpiod_chip;
}
