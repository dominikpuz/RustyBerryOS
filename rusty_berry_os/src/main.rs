//! From a namespace perspective, **memory** subsystem code lives in:
//!
//! - `crate::memory::*`
//! - `crate::bsp::memory::*`
//!
//! # Boot flow
//!
//! 1. The kernel's entry point is the function `cpu::boot::arch_boot::_start()`.
//!     - It is implemented in `src/_arch/__arch_name__/cpu/boot.s`.
//! 2. Once finished with architectural setup, the arch code calls `kernel_init()`.

#![allow(clippy::upper_case_acronyms)]
#![allow(incomplete_features)]
#![feature(asm_const)]
#![feature(const_option)]
#![feature(core_intrinsics)]
#![feature(format_args_nl)]
#![feature(generic_const_exprs)]
#![feature(int_roundings)]
#![feature(is_sorted)]
#![feature(nonzero_min_max)]
#![feature(panic_info_message)]
#![feature(step_trait)]
#![feature(trait_alias)]
#![feature(unchecked_math)]
#![no_main]
#![no_std]

mod bsp;
mod common;
mod console;
mod cpu;
mod driver;
mod exception;
mod memory;
mod panic_wait;
mod print;
mod synchronization;
mod time;
mod state;

/// Early init code.
///
/// When this code runs, virtual memory is already enabled.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
/// - Printing will not work until the respective driver's MMIO is remapped.
#[no_mangle]
unsafe fn kernel_init() -> ! {
    exception::handling_init();
    memory::init();

    // Initialize the BSP driver subsystem.
    if let Err(x) = bsp::driver::init() {
        panic!("Error initializing BSP driver subsystem: {}", x);
    }

    // Initialize all device drivers.
    driver::driver_manager().init_drivers_and_irqs();

    bsp::memory::mmu::kernel_add_mapping_records_for_precomputed();

    // Unmask interrupts on the boot CPU core.
    exception::asynchronous::local_irq_unmask();

    // Announce conclusion of the kernel_init() phase.
    state::state_manager().transition_to_single_core_main();

    // Transition from unsafe to safe.
    kernel_main()
}
/// Version of the kernel.
pub fn version() -> &'static str {
    concat!(
    env!("CARGO_PKG_NAME"),
    " version ",
    env!("CARGO_PKG_VERSION")
    )
}

/// The main function running after the early init.
fn kernel_main() -> ! {
    info!("{}", version());
    info!("Booting on: {}", bsp::board_name());

    info!("MMU online:");
    memory::mmu::kernel_print_mappings();

    let (_, privilege_level) = exception::current_privilege_level();
    info!("Current privilege level: {}", privilege_level);

    info!("Exception handling state:");
    exception::asynchronous::print_state();

    info!(
        "Architectural timer resolution: {} ns",
        time::time_manager().resolution().as_nanos()
    );

    info!("Drivers loaded:");
    driver::driver_manager().enumerate();

    info!("Registered IRQ handlers:");
    exception::asynchronous::irq_manager().print_handler();

    info!("Echoing input now");
    cpu::wait_forever();
}