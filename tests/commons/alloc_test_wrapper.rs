use buddy_system_allocator::LockedHeap;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::sync::atomic::AtomicBool;
use libc;
use libc_print::std_name::*;
use spin::Once;

pub struct AllocTestWrapper<T: GlobalAlloc> {
    pub tested_alloc: T,
    pub use_tested_alloc: bool,
    buddy_alloc: Once<LockedHeap<{ super::BUDDY_ALLOCATOR_ORDER }>>,
}

// FIXME, malloc allocations, even though small, can interfere with the address space of the tested
//  allocator. This might cause tests to fail even though there are no problems with the tested
//  allocator. We might need to address this problem, even though it is highly unlikely to happen
impl<T: GlobalAlloc> AllocTestWrapper<T> {
    pub const fn new(allocator: T) -> Self {
        Self {
            tested_alloc: allocator,
            use_tested_alloc: false,
            buddy_alloc: Once::new(),
        }
    }
}

unsafe impl<T: GlobalAlloc> GlobalAlloc for AllocTestWrapper<T> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if self.use_tested_alloc {
            self.tested_alloc.alloc(layout)
        } else {
            let buddy_alloc = self.buddy_alloc.call_once(|| {
                let buddy_alloc = LockedHeap::new();

                super::init_buddy_allocator(
                    &buddy_alloc,
                    ota_allocator::TEST_ADDR_SPACE_START,
                    ota_allocator::TEST_ADDR_SPACE_MAX_SIZE,
                );

                buddy_alloc
            });

            buddy_alloc.alloc(layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if self.use_tested_alloc {
            self.tested_alloc.dealloc(ptr, layout);
        } else {
            let buddy_alloc = self.buddy_alloc.get().unwrap();
            buddy_alloc.dealloc(ptr, layout);
        }
    }
}
