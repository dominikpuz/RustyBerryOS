//! Processor code.

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/cpu.rs"]
mod arch_cpu;

mod boot;

// Re-export the `arch_cpu` functions and types.
pub use arch_cpu::{nop, wait_forever};

#[cfg(feature = "bsp_rpi3")]
pub use arch_cpu::spin_for_cycles;