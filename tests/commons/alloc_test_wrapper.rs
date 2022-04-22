use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use libc;
use libc_print::std_name::*;

pub struct AllocTestWrapper<T: GlobalAlloc> {
    pub tested_alloc: T,
    use_alloc: RefCell<AllocatorType>,
}

enum AllocatorType {
    Tested,
    Standard,
}

impl<T: GlobalAlloc> AllocTestWrapper<T> {
    pub const fn new(allocator: T) -> Self {
        Self {
            tested_alloc: allocator,
            use_alloc: RefCell::new(AllocatorType::Standard),
        }
    }

    // SAFETY: only call this function from a single thread, panic! will occur otherwise
    pub unsafe fn use_tested_allocator(&self) {
        *self.use_alloc.borrow_mut() = AllocatorType::Tested;
    }

    // SAFETY: only call this function from a single thread, panic! will occur otherwise
    pub unsafe fn use_standard_allocator(&self) {
        *self.use_alloc.borrow_mut() = AllocatorType::Standard;
    }
}

unsafe impl<T: GlobalAlloc> GlobalAlloc for AllocTestWrapper<T> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match *self.use_alloc.borrow() {
            AllocatorType::Tested => self.tested_alloc.alloc(layout),
            AllocatorType::Standard => libc::malloc(layout.size() as libc::size_t) as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        match *self.use_alloc.borrow() {
            AllocatorType::Tested => self.tested_alloc.dealloc(ptr, layout),
            AllocatorType::Standard => libc::free(ptr as *mut libc::c_void)
        };
    }
}
