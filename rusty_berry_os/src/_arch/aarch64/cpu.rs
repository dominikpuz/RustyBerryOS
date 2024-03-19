//! Architectural processor code.
//!
//! File importing path is:
//!
//! create::cpu::arch_cpu

use aarch64_cpu::asm;

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe()
    }
}
