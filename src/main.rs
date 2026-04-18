#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(abi_x86_interrupt)]

pub mod vga_buffer;
mod serial;
use vga_buffer::PROMPT;
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;
// use os::memory::active_level_4_table;
use core::panic::PanicInfo;

extern crate alloc;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}", info);
    exit_qemu(QemuExitCode::Failed);
    os::hlt_loop();
}
// static HELLO: &[u8] = b"Hello World!";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }    
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // use os::memory;

    // use x86_64::{structures::paging::Page, VirtAddr};
    use os::memory::{self,BootInfoFrameAllocator};
    use os::allocator;

    os::init(); 

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    use x86_64::structures::paging::Size4KiB;
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    use alloc::boxed::Box;
    let b = Box::new(41);


    
    print!(" Hello, root.\n");
    // print!("{}", PROMPT);
    println!("[Enter] to continue");
    #[cfg(test)]
    test_kernel_main();
    os::hlt_loop();
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests,", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 0);
    serial_println!("[ok]");
}
