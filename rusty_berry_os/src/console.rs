//! System console

use crate::bsp;

/// Console interface.

pub mod interface {
    pub use core::fmt::Write;
}

/// Return a reference to the console.
///
/// Global instance used by all println macros.

pub fn console() -> impl interface::Write {
    bsp::console::console()
}
