//! The `kernel` binary.

#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![no_main]
#![no_std]

mod bsp;
mod console;
mod cpu;
mod panic_wait;
mod print;
mod synchronization;

unsafe fn kernel_init() -> ! {
    use console::console;

    println!("[0] Booting on: {}", bsp::board_name());

    println!("[1] Hello world from the kernel!");

    println!("[2] Chars written: {}", console().chars_written());

    println!("[3] Stopping here.");

    cpu::wait_forever()
}
