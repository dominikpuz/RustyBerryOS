//! Architectural processor code.
//!
//! File importing path is:
//!
//! create::cpu::arch_cpu

use aarch64_cpu::asm;

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

pub use asm::nop;

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe()
    }
}
