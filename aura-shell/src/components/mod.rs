#![no_std]
extern crate alloc;
use alloc::vec::Vec;

pub const GRAPHICS_PAGE_SIZE: usize = 4046;

pub struct SerializedRenderNode {
    pub payload: Vec<u8>,
}

impl SerializedRenderNode {
    pub fn serialize_panel_bounds(width: u32, height: u32, bg_color: u32) -> Self {
        let mut buffer = Vec::with_capacity(12);
        buffer.extend_from_slice(&width.to_be_bytes());
        buffer.extend_from_slice(&height.to_be_bytes());
        buffer.extend_from_slice(&bg_color.to_be_bytes());
        SerializedRenderNode { payload: buffer }
    }
}

