// ==========================================
// ASTRA-OS SERIAL INPUT DRIVER (UART)
// ==========================================

const UART0: *mut u32 = 0x0900_0000 as *mut u32;

pub fn read_char() -> Option<char> {
    unsafe {
        // Read the UART Flag Register (FR) located 24 bytes (6 * 4) past the base address
        let fr = core::ptr::read_volatile(UART0.offset(6));
        
        // Check the RXFE (Receive FIFO Empty) bit. 
        // If it is NOT set to 1, data is waiting in the buffer!
        if (fr & (1 << 4)) == 0 {
            // Read the keystroke from the Data Register (DR)
            let dr = core::ptr::read_volatile(UART0);
            return Some((dr as u8) as char);
        }
        
        None // No key was pressed
    }
}

