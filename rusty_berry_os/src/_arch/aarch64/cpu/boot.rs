//! Architectural boot code.
//!
//! This module is reeexported with path attr
//!
//! Correct usage:
//! crate::cpu::boot::arch_boot
use core::arch::global_asm;

global_asm!(include_str!("boot.s"), CONST_CORE_ID_MASK = const 0b11);

/// Rust entry point.
/// Called from assembly inside `_start` function.
#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    crate::kernel_init()
}
