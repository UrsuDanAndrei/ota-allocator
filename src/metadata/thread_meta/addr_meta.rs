use crate::metadata::thread_meta::pool_allocator::Pool;
use crate::utils::rc_alloc::RcAlloc;
use core::alloc::Allocator;
use core::cell::RefCell;

// TODO when managing big allocation make this en enum BigAddrMeta, SmallAddrMeta
pub struct AddrMeta<'a, A: Allocator> {
    // this is the requested size, the allocated size might be larger due to alignment
    // TODO maybe make size private
    pub size: usize,

    // the purpose of this field is to keep the pool alive at lest while this addr is still valid
    _pool: RcAlloc<RefCell<Pool>, &'a A>,
}

impl<'a, A: Allocator> AddrMeta<'a, A> {
    pub fn new(size: usize, pool: RcAlloc<RefCell<Pool>, &'a A>) -> Self {
        AddrMeta {
            size,
            _pool: pool,
        }
    }
}
