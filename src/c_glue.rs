use core::alloc::{GlobalAlloc, Layout};
use buddy_system_allocator::LockedHeap;
use libc_print::std_name::eprintln;
use crate::{consts, utils::mman_wrapper, OtaAllocator};

// FIXME, figure out the proper ORDER value here, using 32 for now
const BUDDY_ALLOCATOR_ORDER: usize = 32;
type MetaAlloc = LockedHeap<{ BUDDY_ALLOCATOR_ORDER }>;

static mut ALLOCATOR: OtaAllocator<'static, MetaAlloc> = OtaAllocator::new_in(MetaAlloc::new());

#[no_mangle]
pub extern "C" fn ota_init() {
    unsafe {
        if let Err(err) = mman_wrapper::mmap(consts::META_ADDR_SPACE_START, consts::META_ADDR_SPACE_MAX_SIZE) {
            eprintln!(
                "Error with code: {}, when calling mmap for allocating heap memory!",
                err
            );
            panic!("");
        }
        ALLOCATOR.meta_alloc().lock().init(consts::META_ADDR_SPACE_START, consts::META_ADDR_SPACE_MAX_SIZE);
        ALLOCATOR.init();
    }
}

#[no_mangle]
pub extern "C" fn ota_malloc(size: usize) -> *mut u8 {
    unsafe {
        // the align field is used to conform to the function signature, it is not used
        ALLOCATOR.alloc(Layout::from_size_align_unchecked(size, consts::STANDARD_ALIGN))
    }
}

#[no_mangle]
pub extern "C" fn ota_free(addr: *mut u8) {
    unsafe {
        // the layout field is used to conform to the function signature, it is not used
        ALLOCATOR.dealloc(addr, Layout::from_size_align_unchecked(consts::STANDARD_ALIGN, consts::STANDARD_ALIGN));
    }
}
