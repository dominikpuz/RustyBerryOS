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

const MINILOAD_LOGO: &str = r#"
 __  __ _      _ _                 _
|  \/  (_)_ _ (_) |   ___  __ _ __| |
| |\/| | | ' \| | |__/ _ \/ _` / _` |
|_|  |_|_|_||_|_|____\___/\__,_\__,_|
"#;

/// The main function running after init.
fn kernel_main() -> ! {
    use console::console;

    info!("{}", MINILOAD_LOGO);
    info!("{:^37}", bsp::board_name());
    println!();
    info!("[ML] Requesting binary");
    console().flush();
    info!("Architecture timer resolution: {} ns", time::time_manager().resolution().as_nanos());

    console().clear_rx();

    for _ in 0..3 {
        console().write_char(3 as char);
    }

    let mut size: u32 = u32::from(console().read_char() as u8);
    size |= u32::from(console().read_char() as u8) << 8;
    size |= u32::from(console().read_char() as u8) << 16;
    size |= u32::from(console().read_char() as u8) << 24;

    console().write_char('O');
    console().write_char('K');

    let kernel_addr: *mut u8 = bsp::memory::board_default_load_addr() as *mut u8;
    unsafe {
        // Read the kernel byte by byte.
        for i in 0..size {
            core::ptr::write_volatile(kernel_addr.offset(i as isize), console().read_char() as u8)
        }
    }

    info!("[ML] Loaded! Executing the payload now\n");
    console().flush();

    // Use black magic to create a function pointer.
    let kernel: fn() -> ! = unsafe { core::mem::transmute(kernel_addr) };

    // Jump to loaded kernel!
    kernel()
}
