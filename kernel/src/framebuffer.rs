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
    // ==========================================
    // PREMIUM UI: ROUNDED CORNERS
    // ==========================================
    
    // Draw a rectangle with smooth rounded corners (Squircles)
    pub fn draw_rounded_rect(&self, start_x: u32, start_y: u32, width: u32, height: u32, radius: u32, color: u32) {
        let r_squared = radius * radius;

        for y in start_y..(start_y + height) {
            for x in start_x..(start_x + width) {
                // Critical Security Check
                if x >= self.width || y >= self.height { continue; }

                let mut draw_pixel = true;

                // Calculate distances from the nearest corner center
                let mut dx = 0;
                let mut dy = 0;

                // Top-Left Corner
                if x < start_x + radius && y < start_y + radius {
                    dx = (start_x + radius) - x;
                    dy = (start_y + radius) - y;
                }
                // Top-Right Corner
                else if x >= start_x + width - radius && y < start_y + radius {
                    dx = x - (start_x + width - radius) + 1;
                    dy = (start_y + radius) - y;
                }
                // Bottom-Left Corner
                else if x < start_x + radius && y >= start_y + height - radius {
                    dx = (start_x + radius) - x;
                    dy = y - (start_y + height - radius) + 1;
                }
                // Bottom-Right Corner
                else if x >= start_x + width - radius && y >= start_y + height - radius {
                    dx = x - (start_x + width - radius) + 1;
                    dy = y - (start_y + height - radius) + 1;
                }

                // If the pixel is inside a corner zone, check if it falls outside the radius
                if dx > 0 || dy > 0 {
                    if dx * dx + dy * dy > r_squared {
                        draw_pixel = false; // Outside the curve, skip this pixel!
                    }
                }

                // If the pixel is inside the main body OR inside the curve, draw it
                if draw_pixel {
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
    // ==========================================
    // THE ALPHA BLENDING ENGINE
    // ==========================================
    
    // Draw a single pixel with transparency support (ARGB format: 0xAARRGGBB)
    pub fn draw_pixel_alpha(&self, x: u32, y: u32, color_argb: u32) {
        if x >= self.width || y >= self.height { return; } // Security check

        // 1. Extract the Alpha (transparency) channel (0 to 255)
        let alpha = (color_argb >> 24) & 0xFF;
        let pixel_offset = (y * self.width) + x;

        unsafe {
            let target_address = self.base_address.offset(pixel_offset as isize);

            if alpha == 255 {
                // Fully solid: Skip the heavy math and just overwrite
                core::ptr::write_volatile(target_address, color_argb);
            } else if alpha > 0 {
                // Transparent: We must blend with the background
                
                // Read the existing background color from RAM
                let bg_color = core::ptr::read_volatile(target_address);

                // Extract Foreground RGB channels
                let fg_r = (color_argb >> 16) & 0xFF;
                let fg_g = (color_argb >> 8) & 0xFF;
                let fg_b = color_argb & 0xFF;

                // Extract Background RGB channels
                let bg_r = (bg_color >> 16) & 0xFF;
                let bg_g = (bg_color >> 8) & 0xFF;
                let bg_b = bg_color & 0xFF;

                // Calculate the Inverse Alpha
                let inv_alpha = 255 - alpha;

                // Apply the Blending Formula to each channel
                let out_r = ((fg_r * alpha) + (bg_r * inv_alpha)) / 255;
                let out_g = ((fg_g * alpha) + (bg_g * inv_alpha)) / 255;
                let out_b = ((fg_b * alpha) + (bg_b * inv_alpha)) / 255;

                // Recombine into a single 32-bit color and write to screen
                let final_color = (out_r << 16) | (out_g << 8) | out_b;
                core::ptr::write_volatile(target_address, final_color);
            }
        }
    }

    // Draw a rectangle using the new alpha blending pixel engine
    pub fn draw_rect_alpha(&self, start_x: u32, start_y: u32, width: u32, height: u32, color_argb: u32) {
        for y in start_y..(start_y + height) {
            for x in start_x..(start_x + width) {
                self.draw_pixel_alpha(x, y, color_argb);
            }
        }
    }
}

