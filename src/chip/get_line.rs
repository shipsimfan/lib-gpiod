use crate::{gpiod_chip, gpiod_line};
use std::ffi::c_uint;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "gpiod")]
unsafe extern "C" {
    /// Get the handle to the GPIO line at given offset.
    ///
    /// # Parameters
    ///  * `chip` - The GPIO chip object.
    ///  * `offset` - The offset of the GPIO line.
    ///
    /// # Returns
    /// Pointer to the GPIO line handle or [`null_mut`] if an error occured.
    pub fn gpiod_chip_get_line(chip: *mut gpiod_chip, offset: c_uint) -> *mut gpiod_line;
}
