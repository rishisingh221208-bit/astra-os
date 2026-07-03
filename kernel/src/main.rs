#![no_std]
#![no_main]

use core::panic::PanicInfo;
// Import Rust's core formatting tools
use core::fmt::{self, Write}; 

const UART_BASE: *mut u8 = 0x0900_0000 as *mut u8;

// 1. Create a struct to represent our hardware
struct Uart;

// 2. Implement Rust's 'Write' trait for our hardware
impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            unsafe {
                // This is the ONLY unsafe block needed for printing
                core::ptr::write_volatile(UART_BASE, byte);
            }
        }
        Ok(())
    }
}

// 3. Create a hidden printing engine that the macros will safely use
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut uart = Uart;
    // This tells Rust to format the text and pipe it securely to the UART
    uart.write_fmt(args).unwrap();
}

// 4. Define the standard 'print!' macro
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

// 5. Define the standard 'println!' macro (adds a newline at the end)
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// 6. The main entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // We can now format variables safely without risking buffer overflows!
    let os_name = "Astra-OS";
    let version = 0.1;
    let memory_address = 0x40080000;
    
    println!("{} v{} Initialized...", os_name, version);
    println!("Booted at secure memory address: {:#X}", memory_address);
    println!("Kernel sandbox active. Welcome to the Generative Future.");

    loop {}
}

// 7. Upgraded Panic Handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // If the system detects a security fault, it will print the exact location
    println!("SYSTEM PANIC: {}", info);
    loop {}
}
