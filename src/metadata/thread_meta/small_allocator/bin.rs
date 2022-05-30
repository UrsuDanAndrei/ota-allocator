use crate::metadata::thread_meta::small_allocator::pool::Pool;
use crate::utils::rc_alloc::RcAlloc;
use core::alloc::Allocator;
use core::cell::RefCell;

pub struct Bin<'a, A: Allocator> {
    pub pool: RcAlloc<RefCell<Pool>, &'a A>,
}

impl<'a, A: Allocator> Bin<'a, A> {
    pub fn new(pool: RcAlloc<RefCell<Pool>, &'a A>) -> Self {
        Bin { pool }
    }
}
