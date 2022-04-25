use core::alloc::{AllocError, Allocator, GlobalAlloc, Layout};
use core::ptr::NonNull;

// this wrapper is needed because of E0117
pub struct AllocatorWrapper<GA: GlobalAlloc> {
    pub allocator: GA,
}

impl<GA: GlobalAlloc> AllocatorWrapper<GA> {
    pub(crate) const fn new(allocator: GA) -> Self {
        AllocatorWrapper { allocator }
    }

    pub fn wrapped_allocator(&self) -> &GA {
        &self.allocator
    }
}

unsafe impl<GA: GlobalAlloc> Allocator for AllocatorWrapper<GA> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let addr = unsafe { self.allocator.alloc(layout) };

        match NonNull::new(addr) {
            None => Err(AllocError),
            Some(addr) => Ok(NonNull::slice_from_raw_parts(addr, layout.size())),
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        self.allocator.dealloc(ptr.as_ptr(), layout);
    }
}
