#![no_std]
#![no_main]

use aergon::capability::Manifest;
use aergon::vga_buffer;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    aergon::init();

    vga_buffer::print_string("Carry OS Loaded into RAM.\n");

    // 1. Verify Manifest
    vga_buffer::print_string("Verifying Identity and Manifest...\n");
    let manifest = Manifest::load_from_secure_enclave();

    // 2. Start the Intelligence Engine (never returns)
    vga_buffer::print_string("Waking Robin Engine...\n");
    engine::start(manifest);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga_buffer::print_string("KERNEL PANIC: ");
    if let Some(location) = info.location() {
        vga_buffer::print_string(location.file());
    }
    // In a real panic, trigger the atomic save to eMMC here
    loop {
        x86_64::instructions::hlt();
    }
}
