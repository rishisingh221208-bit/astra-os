// The MMU requires Page Tables to be perfectly aligned to a 4KB (4096 byte) grid.
#[repr(C, align(4096))]
pub struct PageTable {
    // AArch64 hardware strictly expects exactly 512 entries per table
    pub entries: [u64; 512],
}

impl PageTable {
    // A secure initialization function to zero out the map
    pub const fn empty() -> Self {
        PageTable { entries: [0; 512] }
    }
}

// We need three levels of tables to route a 64-bit address space safely
#[no_mangle]
pub static mut L1_TABLE: PageTable = PageTable::empty();

#[no_mangle]
pub static mut L2_TABLE: PageTable = PageTable::empty();

#[no_mangle]
pub static mut L3_TABLE: PageTable = PageTable::empty();

// ==========================================
// SECURITY FLAGS (The Hardware Clearances)
// ==========================================

// Bit 0: Tells the hardware this memory entry is active and secure
pub const FLAG_VALID: u64 = 1 << 0; 

// Bit 1: Tells the MMU to route the signal down to the next Table
pub const FLAG_TABLE: u64 = 1 << 1; 

// Bit 1: (Same bit, different level) Tells the MMU we reached the final physical memory
pub const FLAG_PAGE: u64  = 1 << 1; 

// Bit 10: Access Flag (The AArch64 processor requires this to be 1, or it crashes)
pub const FLAG_ACCESS: u64 = 1 << 10;

