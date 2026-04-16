# Rust x86_64 Kernel Development

This project is my first major step into systems programming and operating systems development. Based on the "Writing an OS in Rust" series by Philipp Oppermann, I have implemented a fundamental kernel to understand how hardware and software interact at the lowest level.

## Key Features Implemented
* **VGA Text Mode Driver:** Direct memory manipulation to display text on the screen.
* **CPU Exception Handling:** Implementing the Interrupt Descriptor Table (IDT) to handle hardware and software exceptions.
* **Double Faults & Stack Management:** Proper handling of kernel panics and stack overflows.
* **Paging & Memory Management:** Direct mapping of physical memory and implementing a heap allocator.
* **Custom Shell & Command Line:** Developed a standalone shell environment from scratch.
    * Features a command buffer and basic string parsing.
    * Supports multiple built-in commands (type `help` to see the full list).
    * Provides a foundation for future user-space interaction.
  
## Tech Stack
* **Language:** Rust (`no_std` environment)
* **Architecture:** x86_64
* **Tools:** Bootloader, QEMU for emulation

## What's Next?
Now that I have mastered the basics of x86_64 kernel development, I am moving towards designing my own **Capability-based microkernel** for the **RISC-V** architecture. 

My goal is to build a **Non-allocation RTOS** focused on:
1. **Memory Safety** without a garbage collector or dynamic heap.
2. **Deterministic Latency** for real-time applications.
3. **Capability-based Security** to ensure resource isolation.
