//! Architectural boot code.
//!
//! This module is reeexported with path attr
//!
//! Correct usage:
//! crate::cpu::boot::arch_boot
use core::arch::global_asm;

global_asm!(include_str!("boot.s"));
