use crate::gpiod_chip;
use std::ffi::c_uint;

#[link(name = "gpiod")]
unsafe extern "C" {
    /// Get the number of GPIO lines exposed by this chip.
    ///
    /// # Parameters
    ///  * `chip` - The GPIO chip object.
    ///
    /// # Returns
    /// Number of GPIO lines.
    pub fn gpiod_chip_num_lines(chip: *mut gpiod_chip) -> c_uint;
}
