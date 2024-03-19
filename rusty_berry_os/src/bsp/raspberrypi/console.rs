//! BSP console facilities.

use crate::console;
use core::fmt;

/// Console write function.
struct QEMUOutput;
static GPIO_BASE: u32 = 0x3F20_1000;

/// Write a formatted string to the console.
impl fmt::Write for QEMUOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            unsafe {
                core::ptr::write_volatile(GPIO_BASE as *mut u8, c as u8);
            }
        }

        Ok(())
    }
}

/// Return a reference to the console.
pub fn console() -> impl console::interface::Write {
    QEMUOutput {}
}
