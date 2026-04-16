#![feature(abi_x86_interrupt)]
#![no_std]
extern crate alloc;

pub mod vga_buffer;
pub mod shell;
pub mod gdt;
pub mod commands;
pub use core::panic;
pub mod allocator;

pub mod interrupts;
pub mod memory;

pub fn hlt_loop() -> ! {
    loop{
        x86_64::instructions::hlt();
    }
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe {interrupts::PICS.lock().initialize()};
    x86_64::instructions::interrupts::enable();
}
#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

/// Entry point for `cargo test`
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    // like before
    init();
    test_main();
    hlt_loop();
}
