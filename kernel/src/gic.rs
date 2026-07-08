// ==========================================
// ASTRA-OS GENERIC INTERRUPT CONTROLLER (GIC)
// ==========================================

// Memory addresses for the GIC on a standard ARM64 virt board
const GICD_BASE: *mut u32 = 0x08000000 as *mut u32; // Distributor
const GICC_BASE: *mut u32 = 0x08010000 as *mut u32; // CPU Interface

// The hardware ID for the ARM Generic Timer
const TIMER_INTERRUPT_ID: u32 = 30; 

pub fn init() {
    unsafe {
        // 1. Enable the GIC Distributor
        core::ptr::write_volatile(GICD_BASE, 1);
        
        // 2. Enable the CPU Interface to accept signals
        core::ptr::write_volatile(GICC_BASE, 1);
        
        // 3. Unmask (enable) the Timer Interrupt specifically
        let target_register = GICD_BASE.offset(0x100 / 4 + (TIMER_INTERRUPT_ID / 32) as isize);
        core::ptr::write_volatile(target_register, 1 << (TIMER_INTERRUPT_ID % 32));
    }
}

