#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

mod mmu;
mod exceptions;
mod keyboard;
mod framebuffer;
mod timer;
mod gic; 
mod allocator;

use core::panic::PanicInfo;
use core::fmt::{self, Write};
use core::arch::{global_asm, asm};

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
    
    // ==========================================
    // PHASE 3: ACTIVATE THE HARDWARE FIREWALL
    // ==========================================
    unsafe {
        // 1. Populate the Page Tables with your Physical Addresses
        mmu::build_identity_map();

        // 2. The Final Hardware Switch & EVT Registration
        asm!(
            // --- MMU Activation ---
            "ldr {tmp}, =0x00", 
            "msr mair_el1, {tmp}",
            "ldr {tmp}, =0x19", 
            "msr tcr_el1, {tmp}",
            "adrp {tmp}, {table}",
            "add {tmp}, {tmp}, :lo12:{table}",
            "msr ttbr0_el1, {tmp}",
            "tlbi vmalle1is",
            "dsb ish",
            "isb",
            "mrs {tmp}, sctlr_el1",
            "orr {tmp}, {tmp}, #1",
            "msr sctlr_el1, {tmp}",
            "isb", 
            
            // --- NEW: Load the Exception Vector Table ---
            "adrp {tmp}, {vector}",
            "add {tmp}, {tmp}, :lo12:{vector}",
            "msr vbar_el1, {tmp}",
            "isb",
            
            // --- Register Mappings ---
            tmp = out(reg) _,
            table = sym mmu::L1_TABLE,
            vector = sym exceptions::vector_table, 
        );
    }
    // ==========================================

    let os_name = "Astra-OS";
    let version = "0.1";
    let memory_address = 0x40080000;
    
    println!("{} v{} Initialized...", os_name, version);
    println!("Booted at secure memory address: {:#X}", memory_address);
        println!("Kernel sandbox active. Welcome to the Generative Future.");
    
    // ==========================================
    // WAKE UP THE MOBILE DISPLAY
    // ==========================================
    let display = framebuffer::Framebuffer {
        width: 1080,             // Standard mobile width
        height: 1920,            // Standard mobile height
        pitch: 1080 * 4,         // 4 bytes per pixel (RGBA)
        base_address: 0x40100000 as *mut u32, // Safe memory zone for video RAM
    };

    // Paint the screen Astra Blue!
    display.clear_screen(0x000000FF);
    println!("Visual Frontend engaged. Screen painted Astra Blue!");

        // ==========================================
    // POWER MANAGEMENT & TIMERS
    // ==========================================
    // 1. Boot up the Interrupt Switchboard
    gic::init();
    println!("GIC Switchboard active. Routing hardware signals...");

    // 2. Get the physical frequency of the device's crystal oscillator
    let cpu_freq = timer::get_frequency();
    
    // 3. Set the hardware countdown for exactly 1 second
    timer::set_countdown(cpu_freq);
    
    // 4. Turn the physical timer on
    timer::enable();
    println!("Hardware clock enabled running at {} Hz.", cpu_freq);
    println!("CPU entering low-power sleep mode to save battery...");

        // ==========================================
    // THE MOBILE EVENT LOOP
    // ==========================================
    loop {
        // Put the CPU into a deep sleep. 
        // It uses almost zero battery while waiting on this line!
        unsafe {
            core::arch::asm!("wfi"); 
        }
    }
}
// ==========================================
// SYSTEM ERROR HANDLERS
// ==========================================

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    println!("ALLOCATION ERROR: Failed to allocate {} bytes.", layout.size());
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("SYSTEM PANIC: {}\r\n", info);
    loop {}
}


