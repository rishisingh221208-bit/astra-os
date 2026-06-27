// The memory address where QEMU's virtual serial port lives
const UART0: *mut u8 = 0x0900_0000 as *mut u8;

pub fn print_str(s: &str) {
    for byte in s.bytes() {
        unsafe {
            // Write each byte directly to the hardware memory address
            core::ptr::write_volatile(UART0, byte);
        }
    }
}

