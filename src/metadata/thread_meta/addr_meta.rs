use crate::metadata::thread_meta::pool_allocator::Pool;
use crate::utils::rc_alloc::RcAlloc;
use core::alloc::Allocator;
use core::cell::RefCell;

// TODO when managing big allocation make this en enum BigAddrMeta, SmallAddrMeta
pub struct AddrMeta<'a, A: Allocator> {
    // this is the requested size, the allocated size might be larger due to alignment
    // TODO maybe make size private
    pub size: usize,

    // the purpose of these fields is to keep the pool alive at lest while this addr is still valid
    _pool1: RcAlloc<RefCell<Pool>, &'a A>,
    _pool2: Option<RcAlloc<RefCell<Pool>, &'a A>>,
}

impl<'a, A: Allocator> AddrMeta<'a, A> {
    pub fn new_single_pool(size: usize, pool1: RcAlloc<RefCell<Pool>, &'a A>) -> Self {
        AddrMeta {
            size,
            _pool1: pool1,
            _pool2: None,
        }
    }

    pub fn new_double_pool(
        size: usize,
        pool1: RcAlloc<RefCell<Pool>, &'a A>,
        pool2: RcAlloc<RefCell<Pool>, &'a A>,
    ) -> Self {
        AddrMeta {
            size,
            _pool1: pool1,
            _pool2: Some(pool2),
        }
    }
}
