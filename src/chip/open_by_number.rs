use crate::gpiod_chip;
use std::ffi::c_uint;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "gpiod")]
unsafe extern "C" {
    /// Open a gpiochip by number.
    ///
    /// # Parameters
    ///  * `num` - Number of the gpiochip.
    ///
    /// # Returns
    /// GPIO chip handle or [`null_mut`] if an error occurred.
    ///
    /// # Description
    /// This routine appends `num` to '/dev/gpiochip' to create the path.
    pub fn gpiod_chip_open_by_number(num: c_uint) -> *mut gpiod_chip;
}
