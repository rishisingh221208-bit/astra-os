// ==========================================
// ASTRA-OS UI ENGINE & WINDOW MANAGER
// ==========================================
use alloc::vec::Vec;
use alloc::string::String;

// Define a standard UI Component
pub struct Component {
    pub id: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub is_active: bool,
}

// The Window Manager holds our dynamic component tree
pub struct WindowManager {
    // Vec is a dynamic array. It uses our new Heap to grow automatically!
    pub elements: Vec<Component>, 
}

impl WindowManager {
    // Boot up an empty Window Manager
    pub fn new() -> Self {
        WindowManager {
            elements: Vec::new(), 
        }
    }

    // Register a new component into dynamic memory
    pub fn add_component(&mut self, id: String, x: u32, y: u32, width: u32, height: u32) {
        let new_comp = Component {
            id,
            x,
            y,
            width,
            height,
            is_active: true,
        };
        
        // Push the component onto the Heap
        self.elements.push(new_comp);
    }
}
