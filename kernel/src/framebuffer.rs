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
    // ==========================================
    // UI RENDER METHODS
    // ==========================================
    
    // Draw a solid rectangle (for NavBars, Buttons, and Panels)
    pub fn draw_rect(&self, start_x: u32, start_y: u32, width: u32, height: u32, color: u32) {
        for y in start_y..(start_y + height) {
            for x in start_x..(start_x + width) {
                // Critical Security Check: Prevent drawing outside the screen glass!
                // If we don't do this, the OS will overwrite memory and crash.
                if x < self.width && y < self.height {
                    let pixel_offset = (y * self.width) + x;
                    unsafe {
                        core::ptr::write_volatile(
                            self.base_address.offset(pixel_offset as isize), 
                            color
                        );
                    }
                }
            }
        }
    }
    
}

