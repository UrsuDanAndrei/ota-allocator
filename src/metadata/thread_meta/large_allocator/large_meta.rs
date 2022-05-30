use crate::metadata::thread_meta::small_allocator::pool::Pool;
use crate::utils::rc_alloc::RcAlloc;
use core::alloc::Allocator;
use core::cell::RefCell;

pub struct LargeMeta {
    // this is the requested size, the allocated size might be larger due to alignment
    size: usize,
}

impl LargeMeta {
    pub fn new(size: usize) -> Self {
        LargeMeta { size }
    }
}
