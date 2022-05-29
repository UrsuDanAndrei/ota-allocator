use crate::metadata::thread_meta::small_allocator::pool::Pool;
use crate::utils::rc_alloc::RcAlloc;
use core::alloc::Allocator;
use core::cell::RefCell;

pub enum AddrMeta<'a, A: Allocator> {
    SmallMeta {
        // this is the requested size, the allocated size might be larger due to alignment
        size: usize,

        // prevent the drop of the pool until this addr is freed
        _pool: RcAlloc<RefCell<Pool>, &'a A>,
    },

    LargeMeta {
        // this is the requested size, the allocated size might be larger due to alignment
        size: usize,
    },
}

impl<'a, A: Allocator> AddrMeta<'a, A> {
    pub fn new_small(size: usize, pool: RcAlloc<RefCell<Pool>, &'a A>) -> Self {
        AddrMeta::SmallMeta { size, _pool: pool }
    }

    pub fn new_large(size: usize) -> Self {
        AddrMeta::LargeMeta { size }
    }
}
