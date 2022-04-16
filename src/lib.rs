mod align;
mod mman_wrapper;
mod spinlock_wrapper;

use core::{
    alloc::{GlobalAlloc, Layout},
    cmp, ptr,
};
use spinlock_wrapper::Spinlock;

pub struct OtaAllocator {
    last_addr: usize,
}

pub type GlobalOtaAlloc = Spinlock<OtaAllocator>;

impl GlobalOtaAlloc {
    pub const fn new_global_alloc() -> GlobalOtaAlloc {
        GlobalOtaAlloc::new(OtaAllocator {
            last_addr: 0x00006FFF_FFFF0000,
        })
    }
}

unsafe impl GlobalAlloc for GlobalOtaAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();

        let next_addr = match allocator.last_addr.checked_sub(layout.size()) {
            None => return ptr::null_mut(),
            Some(next_addr) => align::align_down(next_addr, cmp::max(layout.align(), 4096)),
        } as *mut u8;

        match mman_wrapper::mmap(next_addr, layout.size()) {
            Err(e) => return ptr::null_mut(),
            Ok(_) => (),
        };

        allocator.last_addr = next_addr as usize;

        next_addr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        mman_wrapper::munmap(ptr, layout.size());
    }
}
