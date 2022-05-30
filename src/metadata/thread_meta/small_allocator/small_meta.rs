use crate::metadata::thread_meta::small_allocator::pool::Pool;
use crate::utils::rc_alloc::RcAlloc;
use core::alloc::Allocator;
use core::cell::RefCell;

pub struct SmallMeta<'a, A: Allocator> {
    // this is the requested size, the allocated size might be larger due to alignment
    size: usize,

    // prevent the drop of the pool until this addr is freed
    _pool: RcAlloc<RefCell<Pool>, &'a A>,
}

impl<'a, A: Allocator> SmallMeta<'a, A> {
    pub fn new(size: usize, pool: RcAlloc<RefCell<Pool>, &'a A>) -> Self {
        SmallMeta{ size, _pool: pool }
    }
}
