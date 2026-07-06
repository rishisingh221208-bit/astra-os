// ==========================================
// ASTRA-OS KEYBOARD DRIVER
// ==========================================

pub fn read_key() -> u8 {
    // The physical hardware addresses for the terminal
    let uart_data_register = 0x09000000 as *mut u8;
    let uart_flag_register = 0x09000018 as *mut u32;
    
    unsafe {
        // Step 1: Wait for a key to be pressed
        // (Bit 4 of the flag register tells us if the buffer is empty)
        while (core::ptr::read_volatile(uart_flag_register) & (1 << 4)) != 0 {
            core::hint::spin_loop(); // Do nothing until the user types
        }
        
        // Step 2: Grab the physical key data and return it
        core::ptr::read_volatile(uart_data_register)
    }
}

