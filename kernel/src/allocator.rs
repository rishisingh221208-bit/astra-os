// ==========================================
// ASTRA-OS DYNAMIC MEMORY (BUMP ALLOCATOR)
// ==========================================
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};

// Define our safe memory region for the UI and Data Structures
const HEAP_START: usize = 0x40800000;
const HEAP_SIZE: usize = 1024 * 1024; // 1 Megabyte of RAM for the Heap

// An atomic tracker that safely counts how many bytes we've used
static HEAP_OFFSET: AtomicUsize = AtomicUsize::new(0);

pub struct AstraAllocator;

unsafe impl GlobalAlloc for AstraAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // 1. Get the current memory offset
        let mut current_offset = HEAP_OFFSET.load(Ordering::Relaxed);

        // 2. Fix Alignment (Computers like memory addresses to be even multiples)
        let align_remainder = current_offset % align;
        if align_remainder != 0 {
            current_offset += align - align_remainder;
        }

        // 3. Prevent Memory Leaks / Crashes (Out of Memory Check)
        if current_offset + size > HEAP_SIZE {
            return core::ptr::null_mut(); // Tell the OS we are out of RAM!
        }

        // 4. "Bump" the pointer forward for the next time we need memory
        HEAP_OFFSET.store(current_offset + size, Ordering::Relaxed);

        // 5. Return the exact starting address for this specific component
        (HEAP_START + current_offset) as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // A simple bump allocator never frees individual memory bits!
        // It's blazing fast, but the heap only resets on a reboot.
    }
}

#[global_allocator]
static ALLOCATOR: AstraAllocator = AstraAllocator;
