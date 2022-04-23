pub mod addr_meta;
pub mod metadata_allocator_wrapper;
pub mod thread_meta;

use buddy_system_allocator::LockedHeap;
use core::alloc::{Allocator, GlobalAlloc};
use hashbrown::hash_map::DefaultHashBuilder;
// reexports
pub use addr_meta::AddrMeta;
pub use metadata_allocator_wrapper::MetaAllocWrapper;
pub use thread_meta::ThreadMeta;

use crate::{consts, OtaAllocator};
use hashbrown::HashMap;

pub struct Metadata<'ma, MA: GlobalAlloc> {
    next_addr_space: usize,

    // TODO make your own wrapper or find a better one instead of using Option
    //  we need option here because HashMap::new can't be called from a constant function
    addr2tid: Option<HashMap<usize, usize, DefaultHashBuilder, &'ma MetaAllocWrapper<MA>>>,

    tid2tmeta: Option<HashMap<usize, ThreadMeta<'ma, MA>, DefaultHashBuilder, &'ma MetaAllocWrapper<MA>>>,

    meta_alloc: MetaAllocWrapper<MA>,
}

impl<'ma, MA: GlobalAlloc> Metadata<'ma, MA> {
    pub const fn new_in(allocator: MA) -> Self {
        Metadata {
            next_addr_space: consts::FIRST_ADDR_SPACE_START,
            addr2tid: None,
            tid2tmeta: None,
            meta_alloc: MetaAllocWrapper::new(allocator),
        }
    }

    pub fn init(&'ma mut self) {
        // TODO consts::MAX_THREADS_NO might not be necessary!
        self.addr2tid = Some(HashMap::with_capacity_in(
            consts::MAX_THREADS_NO,
            &self.meta_alloc,
        ));

        self.tid2tmeta = Some(HashMap::with_capacity_in(
            consts::MAX_THREADS_NO,
            &self.meta_alloc,
        ));
    }

    pub fn add_new_thread(&'ma mut self, tid: usize) {
        // new thread => new associated ThreadMeta structure
        let tid2meta = self.tid2tmeta.as_mut().unwrap();
        tid2meta.insert(tid, ThreadMeta::new_in(self.next_addr_space, &self.meta_alloc));

        // new thread => new address space
        self.next_addr_space -= consts::ADDR_SPACE_SIZE;

        // new mapping: new address space -> new thread
        let addr2tid = self.addr2tid.as_mut().unwrap();
        addr2tid.insert(self.next_addr_space, tid);
    }

    pub fn get_tmeta_for_tid(&self, tid: usize) -> Option<&ThreadMeta<'ma, MA>> {
        self.tid2tmeta.as_ref().unwrap().get(&tid)
    }

    pub fn get_tmeta_for_addr(&self, addr: usize) -> Option<&ThreadMeta<'ma, MA>> {
        let tid = *self
            .addr2tid
            .as_ref()
            .unwrap()
            .get(&(addr & consts::ADDR_SPACE_MASK))
            .unwrap();
        self.get_tmeta_for_tid(tid)
    }
}
