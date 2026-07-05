use core::arch::global_asm;

// ==========================================
// THE EXCEPTION VECTOR TABLE (EVT)
// ==========================================

// The AArch64 processor strictly requires this table to be aligned to 2048 bytes
global_asm!(
    ".section .text",
    ".align 11",
    ".globl vector_table",
    "vector_table:",
    
    // --- Current Exception Level with SP_EL0 ---
    ".align 7", "b unhandled_trap", // Synchronous
    ".align 7", "b unhandled_trap", // IRQ
    ".align 7", "b unhandled_trap", // FIQ
    ".align 7", "b unhandled_trap", // SError

    // --- Current Exception Level with SP_ELx (Where AstraOS lives) ---
    ".align 7", "b unhandled_trap", // Synchronous
    ".align 7", "b irq_trap",       // IRQ (This is where the Keyboard sends its signal!)
    ".align 7", "b unhandled_trap", // FIQ
    ".align 7", "b unhandled_trap", // SError
);

// A secure safety net for interrupts we aren't ready for yet
#[no_mangle]
pub extern "C" fn unhandled_trap() {
    crate::println!("SYSTEM HALTED: Unknown hardware exception caught!");
    loop {}
}

// The dedicated trapdoor for our physical hardware signals
#[no_mangle]
pub extern "C" fn irq_trap() {
    crate::println!("HARDWARE PING: An interrupt was caught!");
    loop {} // We will expand this to read the actual keyboard key later
}
// The Bridge: Tell the Rust compiler that our Assembly label exists
extern "C" {
    pub fn vector_table();
}

