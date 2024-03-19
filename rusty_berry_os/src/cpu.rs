//! Processor code.

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/cpu.rs"]
mod arch_cpu;

mod boot;

// Re-export the `arch_cpu` functions and types.
pub use arch_cpu::wait_forever;
