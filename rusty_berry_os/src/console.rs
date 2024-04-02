//! System console

use crate::bsp;

/// Console interface.

pub mod interface {
    pub use core::fmt;

    pub trait Write {
        fn write_fmt(&self, args: fmt::Arguments) -> core::fmt::Result;
    }

    pub trait Statistics {
        fn chars_written(&self) -> usize {
            0
        }
    }

    pub trait All: Write + Statistics {}
}

/// Return a reference to the console.
///
/// Global instance used by all println macros.

pub fn console() -> &'static dyn interface::All {
    bsp::console::console()
}
