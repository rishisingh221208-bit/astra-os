#![no_std]
#![no_main]

mod uart; // Load our new driver

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    uart::print_str("KERNEL PANIC!\n");
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // This is the very first thing Astra-OS will do!
    uart::print_str("\n===============================\n");
    uart::print_str(" Astra-OS successfully booted! \n");
    uart::print_str("===============================\n\n");
    
    // Then sit in the infinite loop to stay alive
    loop {}
}

