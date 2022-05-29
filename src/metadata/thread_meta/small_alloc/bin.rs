use core::alloc::Allocator;
use core::cell::RefCell;
use crate::metadata::thread_meta::small_alloc::pool::Pool;
use crate::utils::rc_alloc::RcAlloc;

pub struct Bin<'a, A: Allocator> {
    max_size: usize,
    pub pool: RcAlloc<RefCell<Pool>, &'a A>,
}

impl<'a, A: Allocator> Bin<'a, A> {
    pub fn new(max_size: usize, pool: RcAlloc<RefCell<Pool>, &'a A>) -> Self {
        Bin {
            max_size,
            pool
        }
    }
}
