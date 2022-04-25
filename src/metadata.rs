pub mod addr_meta;
pub mod metadata_allocator_wrapper;
pub mod thread_meta;

use buddy_system_allocator::LockedHeap;
use core::alloc::{Allocator, GlobalAlloc};
use hashbrown::hash_map::DefaultHashBuilder;

// reexports
pub use addr_meta::AddrMeta;
pub use metadata_allocator_wrapper::AllocatorWrapper;
pub use thread_meta::ThreadMeta;

use crate::{consts, OtaAllocator};
use hashbrown::HashMap;
use spin::{Mutex, MutexGuard};

pub struct Metadata<'a, A: Allocator> {
    next_addr_space: usize,
    addr2tid: HashMap<usize, usize, DefaultHashBuilder, &'a A>,
    tid2tmeta:
        HashMap<usize, Mutex<ThreadMeta<'a, A>>, DefaultHashBuilder, &'a A>,
}

impl<'a, A: Allocator> Metadata<'a, A> {
    pub fn new_in(meta_alloc: &'a A) -> Self {
        Metadata {
            next_addr_space: consts::FIRST_ADDR_SPACE_START,
            addr2tid: HashMap::with_capacity_in(consts::INIT_THREADS_NO, meta_alloc),
            tid2tmeta: HashMap::with_capacity_in(consts::INIT_THREADS_NO, meta_alloc),
        }
    }

    // TODO maybe make this only &self instead of &mut self
    pub fn add_new_thread(&mut self, tid: usize) {
        // new thread => new associated ThreadMeta structure
        self.tid2tmeta.insert(
            tid,
            Mutex::new(ThreadMeta::new_in(
                self.next_addr_space,
                *self.tid2tmeta.allocator(),
            )),
        );

        // new thread => new address space
        self.next_addr_space -= consts::ADDR_SPACE_MAX_SIZE;

        // new thread => new (address space -> thread) mapping
        self.addr2tid.insert(self.next_addr_space, tid);
    }

    pub fn get_tmeta(&self, tid: usize) -> Option<&Mutex<ThreadMeta<'a, A>>> {
        self.tid2tmeta.get(&tid)
    }

    pub fn get_addr_tmeta(&self, addr: usize) -> Option<&Mutex<ThreadMeta<'a, A>>> {
        self.addr2tid
            // TODO maybe make this &(addr & consts::ADDR_SPACE_MASK) a separate function in utils
            .get(&(addr & consts::ADDR_SPACE_MASK))
            .and_then(|tid| self.get_tmeta(*tid))
    }
}
