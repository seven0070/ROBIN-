//! UART 16550 serial port for early debug output.

use spin::Mutex;
use uart_16550::SerialPort;

pub static SERIAL1: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x3F8) });

/// Initialize COM1 (0x3F8) for debug logging.
pub fn init() {
    SERIAL1.lock().init();
}

/// Write a string to the serial port.
pub fn write_string(s: &str) {
    use core::fmt::Write;
    SERIAL1.lock().write_str(s).ok();
}
