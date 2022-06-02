mod allocator_wrapper;
mod thread_meta;

use core::alloc::Allocator;
use core::hash::BuildHasherDefault;

// reexports
pub use allocator_wrapper::AllocatorWrapper;

use crate::consts;
use crate::utils::get_addr_space;
use hashbrown::HashMap;
use rustc_hash::FxHasher;
use spin::Mutex;
use thread_meta::ThreadMeta;

pub struct Metadata<'a, A: Allocator> {
    next_addr_space: usize,
    addr2tid: HashMap<usize, usize, BuildHasherDefault<FxHasher>, &'a A>,
    tid2tmeta: HashMap<usize, Mutex<ThreadMeta<'a, A>>, BuildHasherDefault<FxHasher>, &'a A>,
    meta_alloc: &'a A,
}

impl<'a, A: Allocator> Metadata<'a, A> {
    pub fn new_in(first_addr_space: usize, meta_alloc: &'a A) -> Self {
        Metadata {
            next_addr_space: first_addr_space,
            addr2tid: HashMap::with_capacity_and_hasher_in(
                consts::RESV_THREADS_NO,
                BuildHasherDefault::<FxHasher>::default(),
                meta_alloc,
            ),
            tid2tmeta: HashMap::with_capacity_and_hasher_in(
                consts::RESV_THREADS_NO,
                BuildHasherDefault::<FxHasher>::default(),
                meta_alloc,
            ),
            meta_alloc,
        }
    }

    pub fn add_new_thread(&mut self, tid: usize) {
        // new thread => new associated ThreadMeta structure
        self.tid2tmeta.insert(
            tid,
            Mutex::new(ThreadMeta::new_in(self.next_addr_space, self.meta_alloc)),
        );

        // new thread => new (address space -> thread) mapping
        self.addr2tid.insert(self.next_addr_space, tid);

        // new thread => new address space
        self.next_addr_space += consts::ADDR_SPACE_MAX_SIZE;
    }

    pub fn get_tmeta(&self, tid: usize) -> Option<&Mutex<ThreadMeta<'a, A>>> {
        self.tid2tmeta.get(&tid)
    }

    pub fn get_addr_tmeta(&self, addr: usize) -> Option<&Mutex<ThreadMeta<'a, A>>> {
        self.addr2tid
            .get(&get_addr_space(addr))
            .and_then(|tid| self.get_tmeta(*tid))
    }
}
