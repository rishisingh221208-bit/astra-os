// ==========================================
// ASTRA-OS ARM64 GENERIC TIMER DRIVER
// ==========================================
use core::arch::asm;

// Enable the physical timer
pub fn enable() {
    unsafe {
        // Write '1' to the Control Register (CNTP_CTL_EL0) to enable the timer
        asm!("msr cntp_ctl_el0, {0}", in(reg) 1u64);
    }
}

// Set a countdown timer (in ticks)
pub fn set_countdown(ticks: u64) {
    unsafe {
        // Write the countdown value to the Timer Value Register (CNTP_TVAL_EL0)
        asm!("msr cntp_tval_el0, {0}", in(reg) ticks);
    }
}

// Read the current physical frequency of the hardware clock
pub fn get_frequency() -> u64 {
    let freq: u64;
    unsafe {
        // Read the Counter-timer Frequency Register (CNTFRQ_EL0)
        asm!("mrs {0}, cntfrq_el0", out(reg) freq);
    }
    freq
}

