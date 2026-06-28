#![no_std]
#![no_main]

// We need this feature to write raw assembly code in our entry point
#![feature(asm_const)] 

mod uart;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    uart::print_str("KERNEL PANIC!\n");
    loop {}
}

// Ensure this function is placed exactly where the linker expects it
#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! {
    // 1. SET THE STACK POINTER
    // We use raw assembly to point the CPU to the 16KB stack space we reserved in linker.ld
    unsafe {
        core::arch::asm!(
            "ldr x30, =__stack_top", // Load the stack top address from the linker script
            "mov sp, x30",           // Move that address into the Stack Pointer (sp) register
        );
    }

    // 2. RUN THE RUST CODE
    // Now that the CPU has a stack, it can safely execute our printing functions!
    uart::print_str("\n===============================\n");
    uart::print_str(" Astra-OS successfully booted! \n");
    uart::print_str("===============================\n\n");
    
    // 3. STAY ALIVE
    loop {}
}


