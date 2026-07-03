#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::{self, Write};
// Import global_asm to give the CPU its first instructions
use core::arch::global_asm; 

// 1. Set up the Stack Pointer in pure assembly
global_asm!(
    ".section .text._start",
    ".globl _start",
    "_start:",
    "adrp x0, STACK",             // Find the memory we reserved below
    "add x0, x0, :lo12:STACK",
    "add x0, x0, 8192",           // Move to the top of our 8KB stack
    "mov sp, x0",                 // Set the CPU's Stack Pointer!
    "bl kmain",                   // Jump safely to our Rust kernel main
    "b ."                         // If it returns, freeze safely
);

// 2. Reserve 8 Kilobytes of memory specifically for the macro to use
#[no_mangle]
static mut STACK: [u8; 8192] = [0; 8192];

const UART_BASE: *mut u8 = 0x0900_0000 as *mut u8;

struct Uart;

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            unsafe {
                core::ptr::write_volatile(UART_BASE, byte);
            }
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut uart = Uart;
    uart.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// 3. Renamed to 'kmain' (Kernel Main) since assembly handles the actual start
#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let os_name = "Astra-OS";
    let version = 0.1;
    let memory_address = 0x40080000;
    
    println!("{} v{} Initialized...", os_name, version);
    println!("Booted at secure memory address: {:#X}", memory_address);
    println!("Kernel sandbox active. Welcome to the Generative Future.");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("SYSTEM PANIC: {}", info);
    loop {}
}
