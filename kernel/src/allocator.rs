// ==========================================
// ASTRA-OS SLAB MEMORY ALLOCATOR (THE HEAP)
// ==========================================
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

pub struct AstraAllocator;

unsafe impl GlobalAlloc for AstraAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // A simple bump allocator mapping into a safe heap region
        // Starting at a safe space right above our kernel stack
        let heap_start = 0x40800000 as *mut u8;
        
        // For now, return a raw pointer to the heap space
        // We will expand this with a buddy-system algorithm next!
        heap_start
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Freeing memory back to the system pool
    }
}

#[global_allocator]
static ALLOCATOR: AstraAllocator = AstraAllocator;

