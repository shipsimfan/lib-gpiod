use crate::gpiod_chip;

#[link(name = "gpiod")]
unsafe extern "C" {
    /// Close the chip and release all associated resources.
    ///
    /// # Parameters
    ///  * `chip` - Chip to close.
    pub fn gpiod_chip_close(chip: *mut gpiod_chip);
}
