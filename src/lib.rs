#![no_std]

mod mman_wrapper;
mod spinlock_wrapper;
mod metadata;
mod utils;

use core::{
    alloc::{GlobalAlloc, Layout},
    cmp, ptr,
};
use spin::{Mutex, MutexGuard};
use metadata::Metadata;
use hashbrown::HashMap;
use buddy_system_allocator::LockedHeap;
use utils::consts;


fn get_meta_for_addr(addr: *mut u8) -> ThreadMeta {
    ThreadMeta::new()
}

pub struct OtaAllocator {
    tid2meta: [ThreadMeta; MAX_THREADS_NO],
    // FIXME, figure out the proper ORDER value here, using 16 for now
    meta_alloc: LockedHeap<consts::BUDDY_ALLOCATOR_ORDER>
}

impl OtaAllocator {
    pub const fn new() -> Self {
        // TODO assign each thread a different address space
        //  use this trick: https://www.joshmcguigan.com/blog/array-initialization-rust/

        // this is needed because arr! doesn't accept MAX_THREADS_NO
        if MAX_THREADS_NO != 32768 {
            panic!();
        }

        OtaAllocator {
            tid2meta: arr_macro::arr![Metadata::new(); 32768],
            meta_alloc: LockedHeap::new()
        }
    }

    // this function must be call EXACTLY once before using the allocator
    pub fn init(&mut self) {
        unsafe {
            // mman_wrapper::mmap(0x00007FFF_00000000 as *mut u8, 4096 * 10);
            self.meta_alloc.lock().init(0x00007FFF_00000000 as usize, 4096 * 10);
        }
    }
}

unsafe impl GlobalAlloc for OtaAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let tid = get_tid();
        let tmeta = &self.tid2meta[tid];

        if tmeta.use_meta_alloc.borrow() {
            return self.meta_alloc.alloc(layout);
        }

        // this call isn't protected by a lock, because it only reads/writes thread local data
        let next_addr = tmeta.next_addr();

        // start of the critical region protected by addr2info lock
        let addr2info = tmeta.addr2meta.lock()
            .get_or_insert(HashMap::new());

        // insert might trigger a call for alloc and dealloc, handled by the meta allocator
        *tmeta.use_meta_alloc.borrow_mut() = true;
        addr2info.insert(next_addr, AddrMeta::new(2));
        *tmeta.use_meta_alloc.borrow_mut() = false;

        next_addr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let tid = get_tid();
        let tmeta = &self.tid2meta[tid];

        if is_meta_addr(ptr) {
            if tmeta.use_meta_alloc.borrow() {
                self.meta_alloc.dealloc(ptr, layout);
                return;
            } else {
                // attempt from the outside of the allocator to free a metadata address
                seg_fault();
            }
        }

        // the metadata of the thread that allocated this address (might be different from tid)
        let alloc_tmeta = get_meta_for_addr(ptr);

        // TODO handle the None case
        let addr2meta = alloc_tmeta.addr2meta.lock().as_mut().unwrap();

        // remove might trigger a call for alloc and dealloc, handled by the meta allocator
        *tmeta.use_meta_alloc.borrow_mut() = true;
        addr2meta.remove(&ptr);
        *tmeta.use_meta_alloc.borrow_mut() = false;
    }
}

// TODO
fn get_tid() -> usize {
    2
}



// TODO
fn seg_fault() {

}
