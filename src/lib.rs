#![feature(allocator_api)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(core_panic)]
#![feature(panic_info_message)]
#![no_std]

extern crate alloc;

#[cfg(not(test))]
pub mod c_glue;

mod metadata;
mod utils;

use alloc::boxed::Box;
// reexports
pub use consts::{META_ADDR_SPACE_MAX_SIZE, META_ADDR_SPACE_START};

#[cfg(feature = "integration-test")]
pub use consts::{POOL_SIZE, TANK_SIZE, TEST_ADDR_SPACE_MAX_SIZE, TEST_ADDR_SPACE_START};

#[cfg(feature = "integration-test")]
pub use utils::mman_wrapper;

use core::alloc::{GlobalAlloc, Layout};
use core::{mem, panic};
use libc_print::std_name::*;
use metadata::{AllocatorWrapper, Metadata};
use spin::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use utils::consts;

// TODO as you are able to pass a reference to Box::new_in and HashMap::new_in instead of the
//  actual allocator, maybe you can also try to require your GA to be Clone as well and call
//  OtaAllocator::new_in with a reference instead of the actual value.
//  -
//  This would eliminate the need for lifetimes, so the code will be simpler and various lifetime
//  restrictions, such as the one we had with Once::call_once might be mitigated
//  -
//  Sample code for this usage:
//  -
//  static META_ALLOC = LockedHeap::new();
//  static ALLOCATOR = OtaAllocator::new_in(&META_ALLOC)
//  -
//  the 2 lines can be hidden behind a macro:
//  -
//  define_ota_allocator!(ALLOCATOR, LockedHeap::new());
//  -
pub struct OtaAllocator<'a, GA: GlobalAlloc> {
    // TODO find a way out of using Option here, this is the only thing that makes use of
    //  the init method, it would be great if we could get rid of it
    //
    // we need option here because HashMap::new can't be called from a constant function
    meta: Option<RwLock<Metadata<'a, AllocatorWrapper<GA>>>>,
    meta_alloc: AllocatorWrapper<GA>,
}

// FIXME maybe add malloc_usable_size !!!!!!!
impl<'a, GA: GlobalAlloc> OtaAllocator<'a, GA> {
    pub const fn new_in(meta_alloc: GA) -> Self {
        OtaAllocator {
            meta: None,
            meta_alloc: AllocatorWrapper::new(meta_alloc),
        }
    }

    // this function must be called EXACTLY once before using the allocator
    pub fn init(&'a mut self) {

        // let i = 0;

        // while i == 0 {
        // unsafe { libc::usleep(15_000_000); }
        // }

        self.meta = Some(RwLock::new(Metadata::new_in(
            consts::FIRST_ADDR_SPACE_START,
            &self.meta_alloc,
        )));
    }

    pub fn meta_alloc(&self) -> &GA {
        self.meta_alloc.wrapped_allocator()
    }

    pub fn usable_size(&self, ptr: *mut u8) -> usize {
        let addr = ptr as usize;
        let read_meta = self.read_meta();

        // TODO move this into a function
        match read_meta.get_addr_tmeta(addr) {
            None => {

                // if addr == 0 {
                //     return;
                // }

                eprintln!("Invalid or double free! addr: {}", addr);
                // panic!("");
                0
            }

            Some(addr_tmeta) => { eprintln!("BEFORE 1!!!!: {}", utils::get_current_tid()); let x = addr_tmeta.lock().usable_size(addr); eprintln!("AFTER 1!!!!: {}", utils::get_current_tid()); x},
        }
    }

    fn read_meta(&self) -> RwLockReadGuard<Metadata<'a, AllocatorWrapper<GA>>> {
        self.meta.as_ref().unwrap().read()
    }

    fn write_meta(&self) -> RwLockWriteGuard<Metadata<'a, AllocatorWrapper<GA>>> {
        // getting the write lock trough an upgradeable read lock to avoid write starvation
        self.meta.as_ref().unwrap().upgradeable_read().upgrade()
    }

    // TODO do a reset method, that brings the frees all memory and brings the allocator in the
    //  state is was right before init for testing purposes
}

unsafe impl<'a, GA: GlobalAlloc> GlobalAlloc for OtaAllocator<'a, GA> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // eprintln!("Allocated: {}", layout.size());
        let size = layout.size();

        if size == 1 {
            let x = 1;
        }

        let tid = utils::get_current_tid();
        let mut read_meta = self.read_meta();

        // TODO move this into a function
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

        eprintln!("BEFORE 2!!!!: {}", utils::get_current_tid());
        let addr = tmeta.lock().next_addr(size);
        eprintln!("AFTER 2!!!!: {}", utils::get_current_tid());
        addr as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        // eprintln!("Free: {}", ptr as usize);
        let addr = ptr as usize;
        let read_meta = self.read_meta();

        // TODO move this into a function
        match read_meta.get_addr_tmeta(addr) {
            None => {

                // if addr == 0 {
                //     return;
                // }

                eprintln!("Invalid or double free! addr: {}", addr);
                // panic!("");
                return;
            }

            Some(addr_tmeta) => { eprintln!("BEFORE 3!!!!:"); let x = addr_tmeta.lock().free(addr); eprintln!("AFTER 3!!!!:"); x}
        };
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.alloc(layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        self.dealloc(ptr, layout);
        self.alloc(Layout::from_size_align_unchecked(new_size, layout.size()))
    }
}
