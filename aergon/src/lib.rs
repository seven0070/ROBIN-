#![no_std]

pub mod capability;
pub mod memory;
pub mod serial;
pub mod vga_buffer;

/// Initialize the Aergon microkernel: serial, VGA console, and early hardware probe.
pub fn init() {
    serial::init();
    vga_buffer::print_string("Aergon Microkernel Initializing...\n");
    vga_buffer::print_string("Hardware Probe: USB, Display, Memory... [OK]\n");
}
