//! The `kernel` binary.

#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![feature(unchecked_math)]
#![feature(const_option)]
#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod panic_wait;
mod print;
mod synchronization;
mod driver;
mod console;
mod time;

unsafe fn kernel_init() -> ! {
    // Initialize the BSP driver subsystem.
    if let Err(x) = bsp::driver::init() {
        panic!("Error initializing BSP driver subsystem: {}", x);
    }

    // Initialize all device drivers.
    driver::driver_manager().init_drivers();
    // println! is usable from here on.

    // Transition from unsafe to safe.
    kernel_main()
}

/// The main function running after init.
fn kernel_main() -> ! {
    use console::console;
    use core::time::Duration;

    info!("[1] Booting on: {}", bsp::board_name());
    info!(
        "Architectural timer resolution: {} ns",
        time::time_manager().resolution().as_nanos()
    );

    info!("[2] Drivers loaded:");
    driver::driver_manager().enumerate();

    info!("Spinning for 1 second");
    time::time_manager().spin_for(Duration::from_secs(1));

    info!("[3] Chars written: {}", console().chars_written());
    info!("[4] Echoing input now");

    // Discard any spurious received characters before going into echo mode.
    console().clear_rx();
    loop {
        let c = console().read_char();
        print!("{}", c);
    }
}
