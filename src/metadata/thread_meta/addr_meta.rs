use crate::metadata::thread_meta::pool_allocator::Pool;
use crate::utils::rc_alloc::RcAlloc;
use core::alloc::Allocator;
use core::cell::RefCell;

pub struct AddrMeta<'a, A: Allocator> {
    // this is the requested size, the allocated size might be larger due to alignment
    // TODO maybe make size private
    pub size: usize,
    pool1: RcAlloc<RefCell<Pool>, &'a A>,
    pool2: Option<RcAlloc<RefCell<Pool>, &'a A>>,
}

impl<'a, A: Allocator> AddrMeta<'a, A> {
    pub fn new_single_pool(size: usize, pool1: RcAlloc<RefCell<Pool>, &'a A>) -> Self {
        AddrMeta {
            size,
            pool1,
            pool2: None,
        }
    }

    pub fn new_double_pool(
        size: usize,
        pool1: RcAlloc<RefCell<Pool>, &'a A>,
        pool2: RcAlloc<RefCell<Pool>, &'a A>,
    ) -> Self {
        AddrMeta {
            size,
            pool1,
            pool2: Some(pool2),
        }
    }
}
