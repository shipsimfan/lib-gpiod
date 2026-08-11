use crate::gpiod_line;

#[link(name = "gpiod")]
unsafe extern "C" {
    /// Release a previously reserved line.
    ///
    /// # Parameters
    ///  * `line` - GPIO line object.
    pub fn gpiod_line_release(line: *mut gpiod_line);
}
