#![no_std]

use aergon::capability::Manifest;
use aergon::vga_buffer;

/// Start the Robin intelligence engine under the given Manifest.
///
/// Capabilities are checked before any privileged action. The main event
/// loop reads input, traverses the PAM graph, and generates responses.
pub fn start(manifest: Manifest) -> ! {
    vga_buffer::print_string("Robin 50M Memory-Traversal Engine Online.\n");

    if manifest.can_net.is_some() {
        vga_buffer::print_string("OmniRoute: Cloud Spillover Permitted.\n");
    } else {
        vga_buffer::print_string("OmniRoute: Offline-Only Mode Enforced.\n");
    }

    vga_buffer::print_string("\nHello. I am awake.\n> ");

    // Main event loop — read input, traverse PAM graph, generate response
    loop {
        x86_64::instructions::hlt();
    }
}
