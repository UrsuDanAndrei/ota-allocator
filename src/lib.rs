#![no_std]

mod metadata;
mod mman_wrapper;
mod utils;

use buddy_system_allocator::LockedHeap;
use core::alloc::{GlobalAlloc, Layout};
use hashbrown::HashMap;
use metadata::{AddrMeta, ThreadMeta};
use utils::consts;

pub struct OtaAllocator {
    tid2meta: [ThreadMeta; consts::MAX_THREADS_NO],
    // FIXME, figure out the proper ORDER value here, using 16 for now
    //  also figure out why using consts::BUDDY_ALLOCATOR_ORDER doesn't work
    meta_alloc: LockedHeap<16>,
}

impl OtaAllocator {
    pub fn new() -> Self {
        // this is needed because arr! doesn't accept MAX_THREADS_NO
        if consts::MAX_THREADS_NO != 32768 {
            panic!();
        }

        let mut tid = 0;
        OtaAllocator {
            tid2meta: arr_macro::arr![ThreadMeta::new({ tid += 1; tid - 1 }); 32768],
            meta_alloc: LockedHeap::new(),
        }
    }

    // this function must be call EXACTLY once before using the allocator
    pub fn init(&mut self) {
        unsafe {
            if let Err(err) = mman_wrapper::mmap(consts::META_ADDR_START as *mut u8,
                                                 consts::META_HEAP_SIZE) {
                panic!("Error with code: {}, when calling mmap for allocating heap memory!", err);
            }

            self.meta_alloc
                .lock()
                .init(consts::META_ADDR_START, consts::META_HEAP_SIZE);
        }
    }
}

unsafe impl GlobalAlloc for OtaAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let tid = utils::get_current_tid();
        let tmeta = &self.tid2meta[tid];

        if *tmeta.use_meta_alloc.borrow() {
            return self.meta_alloc.alloc(layout);
        }

        // this call isn't protected by a lock, because it only reads/writes thread local data
        let next_addr = tmeta.next_addr(layout);

        // start of the critical region protected by addr2meta lock
        let mut locked_addr2meta = tmeta.addr2meta.lock();
        let addr2meta = locked_addr2meta.get_or_insert(HashMap::new());

        // insert might trigger a call for alloc and dealloc, handled by the meta allocator
        *tmeta.use_meta_alloc.borrow_mut() = true;
        addr2meta.insert(next_addr, AddrMeta::new(2));
        *tmeta.use_meta_alloc.borrow_mut() = false;

        next_addr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let tid = utils::get_current_tid();
        let tmeta = &self.tid2meta[tid];

        if utils::is_meta_addr(ptr) {
            if *tmeta.use_meta_alloc.borrow() {
                self.meta_alloc.dealloc(ptr, layout);
                return;
            } else {
                panic!("Invalid metadata free attempt! This might be a security issue!");
            }
        }

        // the metadata of the thread that allocated this address (might be different from tid)
        let alloc_tid = utils::get_tid_for_addr(ptr as usize);
        let alloc_tmeta = &self.tid2meta[alloc_tid];

        // TODO maybe move this someplace else, like we did with next_addr method
        //  make the call to munmap, and find the size of the memory, don't use layout.size()
        //  since it is not compatible with the c free api
        if let Err(err) = mman_wrapper::munmap(ptr, layout.size()) {
            panic!("Error with code: {}, when calling unmap!", err);
        }

        // start of the critical region protected by addr2meta lock
        let mut locked_addr2meta = alloc_tmeta.addr2meta.lock();

        // TODO handle the None case
        let addr2meta = locked_addr2meta.as_mut().unwrap();

        // remove might trigger a call for alloc and dealloc, handled by the meta allocator
        *tmeta.use_meta_alloc.borrow_mut() = true;
        addr2meta.remove(&ptr);
        *tmeta.use_meta_alloc.borrow_mut() = false;
    }
}
