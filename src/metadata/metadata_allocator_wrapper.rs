use core::alloc::{Allocator, AllocError, GlobalAlloc, Layout};
use core::ptr::NonNull;

// this wrapper is needed because of E0117
pub struct MetaAllocWrapper<MA: GlobalAlloc> {
    pub allocator: MA
}

impl<MA: GlobalAlloc> MetaAllocWrapper<MA> {
    pub(crate) const fn new(allocator: MA) -> Self {
        MetaAllocWrapper {
            allocator
        }
    }
}

unsafe impl<MA: GlobalAlloc> Allocator for MetaAllocWrapper<MA> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let addr = unsafe { self.allocator.alloc(layout) };

        match NonNull::new(addr) {
            None => Err(AllocError),
            Some(addr) => Ok(NonNull::slice_from_raw_parts(addr, layout.size()))
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        self.allocator.dealloc(ptr.as_ptr(), layout);
    }
}
