use crate::{consts, utils::mman_wrapper, OtaAllocator};
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicBool, Ordering};
use lazy_static::lazy_static;
use libc_print::std_name::eprintln;

// use buddy_system_allocator::LockedHeap;
//
// // FIXME, figure out the proper ORDER value here, using 32 for now
// const BUDDY_ALLOCATOR_ORDER: usize = 32;
// type MetaAlloc = LockedHeap<{ BUDDY_ALLOCATOR_ORDER }>;
//
// pub static mut ALLOCATOR: OtaAllocator<'static, MetaAlloc> = OtaAllocator::new_in(MetaAlloc::new());
// pub static IS_INIT: AtomicBool = AtomicBool::new(false);
// pub static DONE_INIT: AtomicBool = AtomicBool::new(false);
//
// #[no_mangle]
// #[inline(always)]
// pub extern "C" fn ota_init() {
//     if !DONE_INIT.load(Ordering::Relaxed) {
//         if !IS_INIT.swap(true, Ordering::Relaxed) {
//             unsafe {
//                 if let Err(err) = mman_wrapper::mmap(
//                     consts::META_ADDR_SPACE_START,
//                     consts::META_ADDR_SPACE_MAX_SIZE,
//                 ) {
//                     eprintln!(
//                         "Error with code: {}, when calling mmap for allocating heap memory!",
//                         err
//                     );
//                     panic!("");
//                 }
//                 ALLOCATOR.meta_alloc().lock().init(
//                     consts::META_ADDR_SPACE_START,
//                     consts::META_ADDR_SPACE_MAX_SIZE,
//                 );
//                 ALLOCATOR.init();
//             }
//
//             DONE_INIT.store(true, Ordering::Relaxed);
//         }
//
//         while !DONE_INIT.load(Ordering::Relaxed) {}
//     }
// }

// use snmalloc_rs::SnMalloc;
//
// type MetaAlloc = SnMalloc;
//
// pub(crate) static mut ALLOCATOR: OtaAllocator<'static, MetaAlloc> = OtaAllocator::new_in(SnMalloc);
// pub static IS_INIT: AtomicBool = AtomicBool::new(false);
// pub static DONE_INIT: AtomicBool = AtomicBool::new(false);
//
// #[no_mangle]
// #[inline(always)]
// pub extern "C" fn ota_init() {
//     if !DONE_INIT.load(Ordering::Relaxed) {
//         if !IS_INIT.swap(true, Ordering::Relaxed) {
//             unsafe {
//                 ALLOCATOR.init();
//             }
//             DONE_INIT.store(true, Ordering::Relaxed);
//         }
//
//         while !DONE_INIT.load(Ordering::Relaxed) {}
//     }
// }

use mimalloc::MiMalloc;

type MetaAlloc = MiMalloc;

pub static mut ALLOCATOR: OtaAllocator<'static, MetaAlloc> = OtaAllocator::new_in(MiMalloc);
pub static IS_INIT: AtomicBool = AtomicBool::new(false);
pub static DONE_INIT: AtomicBool = AtomicBool::new(false);

#[no_mangle]
#[inline(always)]
pub extern "C" fn ota_init() {
    if !IS_INIT.load(Ordering::Relaxed) {
        if !IS_INIT.swap(true, Ordering::Relaxed) {
            unsafe {
                ALLOCATOR.init();
            }
            DONE_INIT.store(true, Ordering::Relaxed);
        }

        while !DONE_INIT.load(Ordering::Relaxed) {}
    }
}

#[no_mangle]
pub extern "C" fn malloc(size: usize) -> *mut u8 {
    unsafe {
        ota_init();

        // the align field is used to conform to the function signature, it is not used
        ALLOCATOR.alloc(Layout::from_size_align_unchecked(
            size,
            consts::STANDARD_ALIGN,
        ))
    }
}

#[no_mangle]
pub extern "C" fn calloc(number: usize, size: usize) -> *mut u8 {
    unsafe {
        ota_init();

        // the align field is used to conform to the function signature, it is not used
        ALLOCATOR.alloc_zeroed(Layout::from_size_align_unchecked(
            size * number,
            consts::STANDARD_ALIGN,
        ))
    }
}

#[no_mangle]
pub extern "C" fn realloc(addr: *mut u8, size: usize) -> *mut u8 {
    unsafe {
        ota_init();

        // the align field is used to conform to the function signature, it is not used
        ALLOCATOR.realloc(
            addr,
            Layout::from_size_align_unchecked(ALLOCATOR.usable_size(addr), consts::STANDARD_ALIGN),
            size,
        )
    }
}

#[no_mangle]
pub extern "C" fn free(addr: *mut u8) {
    unsafe {
        // the layout field is used to conform to the function signature, it is not used
        ALLOCATOR.dealloc(
            addr,
            Layout::from_size_align_unchecked(consts::STANDARD_ALIGN, consts::STANDARD_ALIGN),
        );
    }
}

#[no_mangle]
pub extern "C" fn malloc_usable_size(addr: *mut u8) -> usize {
    unsafe { ALLOCATOR.usable_size(addr) }
}
