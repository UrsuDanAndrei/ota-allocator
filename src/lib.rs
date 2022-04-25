#![feature(allocator_api)]
#![feature(nonnull_slice_from_raw_parts)]
#![no_std]

mod metadata;
pub mod mman_wrapper;
pub mod utils;

use buddy_system_allocator::LockedHeap;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::marker::Sync;
use core::mem;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;
use hashbrown::HashMap;
use libc_print::std_name::*;
use metadata::{AddrMeta, MetaAllocWrapper, Metadata, ThreadMeta};
use spin::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use utils::consts;

// TODO maybe type OtaAllocator with MA too, so the user can easily give an metadata allocator
pub struct OtaAllocator<'a> {
    use_meta_allocator: AtomicUsize,

    // TODO make your own wrapper or find a better one instead of using Option
    //  we need option here because HashMap::new can't be called from a constant function
    meta: Option<RwLock<Metadata<'a, LockedHeap<{ consts::BUDDY_ALLOCATOR_ORDER }>>>>,

    meta_alloc: MetaAllocWrapper<LockedHeap<{ consts::BUDDY_ALLOCATOR_ORDER }>>,
}

impl<'a> OtaAllocator<'a> {
    pub const fn new() -> Self {
        OtaAllocator {
            use_meta_allocator: AtomicUsize::new(0),
            meta: None,
            meta_alloc: MetaAllocWrapper::new(LockedHeap::new()),
        }
    }

    // this function must be called EXACTLY once before using the allocator
    pub fn init(&'a mut self) {
        unsafe {
            self.init_meta_alloc();
        }
        self.meta = Some(RwLock::new(Metadata::new_in(&self.meta_alloc)));
    }

    unsafe fn init_meta_alloc(&mut self) {
        if let Err(err) = mman_wrapper::mmap(
            consts::META_ADDR_SPACE_START as *mut u8,
            consts::META_ADDR_SPACE_SIZE,
        ) {
            eprintln!(
                "Error with code: {}, when calling mmap for allocating heap memory!",
                err
            );
            panic!("");
        }

        self.meta_alloc
            .allocator
            .lock()
            .init(consts::META_ADDR_SPACE_START, consts::META_ADDR_SPACE_SIZE);
    }

    pub fn read_meta(
        &self,
    ) -> RwLockReadGuard<Metadata<'a, LockedHeap<{ consts::BUDDY_ALLOCATOR_ORDER }>>> {
        self.meta.as_ref().unwrap().read()
    }

    pub fn write_meta(
        &self,
    ) -> RwLockWriteGuard<Metadata<'a, LockedHeap<{ consts::BUDDY_ALLOCATOR_ORDER }>>> {
        // getting the write lock trough an upgradeable read lock to avoid write starvation
        self.meta.as_ref().unwrap().upgradeable_read().upgrade()
    }
}

unsafe impl<'a> GlobalAlloc for OtaAllocator<'a> {
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

        //
        // if *tmeta.use_meta_alloc.borrow() {
        //     return self.meta_alloc.alloc(layout);
        // }
        //
        // // this call isn't protected by a lock, because it only reads/writes thread local data
        // let next_addr = tmeta.next_addr(layout);
        //
        // // start of the critical region protected by addr2meta lock
        // let mut locked_addr2meta = tmeta.addr2meta.lock();
        //
        // let addr2meta = locked_addr2meta.get_or_insert(HashMap::new());
        //
        // // insert might trigger a call for alloc and dealloc, handled by the meta allocator
        // *tmeta.use_meta_alloc.borrow_mut() = true;
        // addr2meta.insert(next_addr, AddrMeta::new(2));
        // *tmeta.use_meta_alloc.borrow_mut() = false;
        //
        // next_addr
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // let tid = utils::get_current_tid();
        //
        // if self
        //     .use_meta_allocator
        //     .compare_exchange(tid, tid, Ordering::Relaxed, Ordering::Relaxed)
        //     .is_ok()
        // {
        //     self.meta_alloc.dealloc(ptr, layout);
        // }
        //
        // let rlocked_meta = self.meta.read();
        // let tmeta = rlocked_meta.get_tmeta_for_tid(tid).unwrap();
        //
        // if utils::is_meta_addr(ptr) {
        //     if *tmeta.use_meta_alloc.borrow() {
        //         self.meta_alloc.dealloc(ptr, layout);
        //         return;
        //     } else {
        //         eprintln!("Invalid metadata free attempt! This might be a security issue!");
        //         panic!("");
        //     }
        // }
        //
        // // the metadata of the thread that allocated this address (might be different from tid)
        // // let alloc_tid =
        // let alloc_tmeta = rlocked_meta.get_tmeta_for_addr(ptr as usize).unwrap();
        //
        // // TODO maybe move this someplace else, like we did with next_addr method
        // //  make the call to munmap, and find the size of the memory, don't use layout.size()
        // //  since it is not compatible with the c free api
        // if let Err(err) = mman_wrapper::munmap(ptr, layout.size()) {
        //     eprintln!("Error with code: {}, when calling unmap!", err);
        //     panic!("");
        // }
        //
        // // start of the critical region protected by addr2meta lock
        // let mut locked_addr2meta = alloc_tmeta.addr2meta.lock();
        //
        // // TODO handle the None case
        // let addr2meta = locked_addr2meta.as_mut().unwrap();
        //
        // // remove might trigger a call for alloc and dealloc, handled by the meta allocator
        // *tmeta.use_meta_alloc.borrow_mut() = true;
        // addr2meta.remove(&ptr);
        // *tmeta.use_meta_alloc.borrow_mut() = false;
    }
}
