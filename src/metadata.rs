mod allocator_wrapper;
mod thread_meta;

use core::alloc::Allocator;
use core::hash::BuildHasherDefault;

// reexports
pub use allocator_wrapper::AllocatorWrapper;

use crate::utils::rc_alloc::RcAlloc;
use crate::{consts, utils};
use arr_macro;
use hashbrown::HashMap;
use rustc_hash::FxHasher;
use spin::Mutex;
use thread_meta::ThreadMeta;

pub struct Metadata<'a, A: Allocator> {
    next_addr_space_id: usize,
    // TODO erase this value
    addr_space2tmeta: [Option<RcAlloc<Mutex<ThreadMeta<'a, A>>, &'a A>>; consts::MAX_THREADS_NO],
    tid2tmeta: HashMap<
        usize,
        RcAlloc<Mutex<ThreadMeta<'a, A>>, &'a A>,
        BuildHasherDefault<FxHasher>,
        &'a A,
    >,
    meta_alloc: &'a A,
}

impl<'a, A: Allocator> Metadata<'a, A> {
    pub fn new_in(first_addr_space: usize, meta_alloc: &'a A) -> Self {
        // this is for assuring consistency, arr! only accepts a literal
        assert_eq!(consts::MAX_THREADS_NO, 128);

        Metadata {
            next_addr_space_id: consts::FIRST_ADDR_SPACE_ID,
            addr_space2tmeta: arr_macro::arr![None; 128],
            tid2tmeta: HashMap::with_capacity_and_hasher_in(
                consts::RESV_THREADS_NO,
                BuildHasherDefault::<FxHasher>::default(),
                meta_alloc,
            ),
            meta_alloc,
        }
    }

    pub fn add_new_thread(&mut self, tid: usize) {
        // new thread => new address space
        let addr_space_id = self.next_addr_space_id;
        let addr_space = utils::get_address_space_for_id(addr_space_id);
        self.next_addr_space_id += 1;

        // new thread => new associated ThreadMeta structure
        let tmeta = RcAlloc::new_in(
            Mutex::new(ThreadMeta::new_in(addr_space, self.meta_alloc)),
            self.meta_alloc,
        );

        // new thread => new mappings
        self.tid2tmeta.insert(tid, tmeta.clone());
        self.addr_space2tmeta[addr_space_id] = Some(tmeta);
    }

    pub fn get_tmeta(&self, tid: usize) -> Option<&RcAlloc<Mutex<ThreadMeta<'a, A>>, &A>> {
        self.tid2tmeta.get(&tid)
    }

    pub fn get_addr_tmeta(&self, addr: usize) -> Option<&RcAlloc<Mutex<ThreadMeta<'a, A>>, &A>> {
        self.addr_space2tmeta[utils::get_address_space_id(addr)].as_ref()
    }
}
