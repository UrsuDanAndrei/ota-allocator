use crate::metadata::thread_meta::small_allocator::pool::Pool;
use crate::utils::rc_alloc::RcAlloc;
use core::alloc::Allocator;
use core::cell::RefCell;

pub struct SmallMeta<'a, A: Allocator> {
    // prevent the drop of the pool until this addr is freed
    _pool: RcAlloc<RefCell<Pool>, &'a A>,
}

impl<'a, A: Allocator> SmallMeta<'a, A> {
    pub fn new(pool: RcAlloc<RefCell<Pool>, &'a A>) -> Self {
        SmallMeta { _pool: pool }
    }
}
