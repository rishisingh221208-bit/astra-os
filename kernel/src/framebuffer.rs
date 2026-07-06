// ==========================================
// ASTRA-OS VISUAL FRONTEND (FRAMEBUFFER)
// ==========================================

// In a QEMU ARM64 "virt" mobile setup, the screen memory is dynamically
// assigned, but we will start by defining the structure of the screen.

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub pitch: u32, // How many bytes per row of pixels
    pub base_address: *mut u32, // The physical memory address of the first pixel
}

impl Framebuffer {
    // A function to paint the entire screen a single solid color
    pub fn clear_screen(&self, color: u32) {
        let total_pixels = self.width * self.height;
        
        unsafe {
            for i in 0..total_pixels {
                // Write the color code directly into the video memory
                core::ptr::write_volatile(self.base_address.offset(i as isize), color);
            }
        }
    }
}

