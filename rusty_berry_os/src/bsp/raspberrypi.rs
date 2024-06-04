//! Bsp file for raspberry pi

pub mod cpu;
pub mod driver;
pub mod memory;
pub mod exception;

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