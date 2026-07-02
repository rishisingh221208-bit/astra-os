#![no_std]
#![no_main]

use core::panic::PanicInfo;

// 1. Define the exact memory address of the QEMU UART hardware
const UART_BASE: *mut u8 = 0x0900_0000 as *mut u8;

// 2. Create our custom print function
pub fn print_text(text: &str) {
    for byte in text.bytes() {
        unsafe {
            // Write each byte directly to the hardware address
            core::ptr::write_volatile(UART_BASE, byte);
        }
    }
}

// 3. The entry point where the bootloader hands over control
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Send our very first message to the outside world!
    print_text("Astra-OS Core Initialized...\n");
    print_text("Welcome to the Generative Future.\n");

    // Enter a safe infinite loop so the OS stays awake
    loop {}
}

// 4. Required panic handler for bare-metal Rust
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print_text("KERNEL PANIC!\n");
    loop {}
}
