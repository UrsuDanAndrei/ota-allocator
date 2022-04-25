#![feature(allocator_api)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(core_panic)]
#![no_std]

mod metadata;
pub mod mman_wrapper;
pub mod utils;

use buddy_system_allocator::LockedHeap;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::marker::Sync;
use core::mem;
use core::panicking::panic;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;
use hashbrown::HashMap;
use libc_print::std_name::*;
use metadata::{AddrMeta, MetaAllocWrapper, Metadata, ThreadMeta};
use spin::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use utils::consts;

// reexports
pub use consts::{META_ADDR_SPACE_START, META_ADDR_SPACE_MAX_SIZE};

// reexports to be used for testing
pub use consts::{TEST_ADDR_SPACE_START, TEST_ADDR_SPACE_MAX_SIZE};

// TODO maybe type OtaAllocator with MA too, so the user can easily give an metadata allocator
pub struct OtaAllocator<'a, MA: GlobalAlloc> {
    use_meta_allocator: AtomicUsize,

    // TODO make your own wrapper or find a better one instead of using Option
    //  we need option here because HashMap::new can't be called from a constant function

    // TODO maybe give Metadata directly an MetaAllocWrapper !!!!!!!!!!!!!!!!!!!
    meta: Option<RwLock<Metadata<'a, MA>>>,

    meta_alloc: MetaAllocWrapper<MA>,
}

impl<'a, MA: GlobalAlloc> OtaAllocator<'a, MA> {
    pub const fn new(meta_alloc: MA) -> Self {
        OtaAllocator {
            use_meta_allocator: AtomicUsize::new(0),
            meta: None,
            meta_alloc: MetaAllocWrapper::new(meta_alloc),
        }
    }

    // this function must be called EXACTLY once before using the allocator
    pub fn init(&'a mut self) {
        self.meta = Some(RwLock::new(Metadata::new_in(&self.meta_alloc)));
    }

    pub fn meta_alloc(&self) -> &MA {
        self.meta_alloc.wrapped_allocator()
    }

    pub fn read_meta(
        &self,
    ) -> RwLockReadGuard<Metadata<'a, MA>> {
        self.meta.as_ref().unwrap().read()
    }

    pub fn write_meta(
        &self,
    ) -> RwLockWriteGuard<Metadata<'a, MA>> {
        // getting the write lock trough an upgradeable read lock to avoid write starvation
        self.meta.as_ref().unwrap().upgradeable_read().upgrade()
    }
}

unsafe impl<'a, MA: GlobalAlloc> GlobalAlloc for OtaAllocator<'a, MA> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let tid = utils::get_current_tid();
        let mut read_meta = self.read_meta();

        let tmeta = match read_meta.get_tmeta(tid) {
            None => {
                // dropping the read lock earlier, so we can get the write lock
                mem::drop(read_meta);

                let mut write_meta = self.write_meta();
                write_meta.add_new_thread(tid);

                // dropping the write lock earlier, to release waiting reading threads
                mem::drop(write_meta);

                // regaining the read lock
                read_meta = self.read_meta();
                read_meta.get_tmeta(tid).unwrap()
            }

            Some(tmeta) => tmeta,
        };

        let addr = tmeta.lock().next_addr(layout); addr as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let addr = ptr as usize;
        let read_meta = self.read_meta();
        let addr_tmeta = read_meta.get_addr_tmeta(addr);

        match addr_tmeta {
            None => {
                eprintln!("Invalid or double free!");
                panic!("");
            },

            Some(addr_tmeta) => addr_tmeta.lock().free_addr(addr)
        };
    }
}
