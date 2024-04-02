//! Conditional reexporting of Board Support Packages.

#[cfg(any(feature = "bsp_rpi3"))]
mod raspberrypi;

#[cfg(any(feature = "bsp_rpi3"))]
#[allow(unused_imports)]
pub use raspberrypi::*;

pub fn board_name() -> &'static str {
    #[cfg(feature = "bsp_rpi3")]
    {
        "Raspberry Pi 3"
    }

    #[cfg(not(feature = "bsp_rpi3"))]
    {
        "Unknown board"
    }
}
