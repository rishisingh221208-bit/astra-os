#![no_std]
#![no_main]

mod mmu;

use core::panic::PanicInfo;
use core::fmt::{self, Write};
use core::arch::global_asm; 

// 1. HARDWARE PING: Print 'A' using pure assembly before Rust even wakes up!
global_asm!(
    ".section .text._start",
    ".globl _start",
    "_start:",
    "ldr x1, =0x09000000",        // Load UART hardware address
    "mov w2, 65",                 // ASCII code for 'A'
    "strb w2, [x1]",              // Force print 'A'
    "mov w2, 10",                 // ASCII code for newline
    "strb w2, [x1]",              // Force print newline
    
    // Normal Stack Setup
    "adrp x0, STACK",             
    "add x0, x0, :lo12:STACK",
    "add x0, x0, 8192",           
    "mov sp, x0",                 
    "bl kmain",                   
    "b ."                         
);

// 1. Create a custom "blueprint" (struct) and align it to 16 bytes
#[repr(align(16))]
struct AlignedStack([u8; 8192]);

// 2. Build our stack using that perfectly aligned blueprint
#[no_mangle]
static mut STACK: AlignedStack = AlignedStack([0; 8192]);



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

// 2. BUFFER FIX: Added '\r\n' to force the QEMU terminal to print immediately
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => ($crate::print!("{}\r\n", format_args!($($arg)*)));
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let os_name = "Astra-OS";
    let version = ("0.1");
    let memory_address = 0x40080000;
    
    println!("{} v{} Initialized...", os_name, version);
    println!("Booted at secure memory address: {:#X}", memory_address);
    println!("Kernel sandbox active. Welcome to the Generative Future.");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("SYSTEM PANIC: {}\r\n", info);
    loop {}
}
