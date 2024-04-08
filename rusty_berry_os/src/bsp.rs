//! Conditional reexporting of Board Support Packages.

mod device_driver;

#[cfg(any(feature = "bsp_rpi3"))]
mod raspberrypi;

#[cfg(any(feature = "bsp_rpi3"))]
#[allow(unused_imports)]
pub use raspberrypi::*;
