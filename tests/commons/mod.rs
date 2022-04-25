pub mod alloc_test_wrapper;

// reexports
pub use alloc_test_wrapper::AllocTestWrapper;

use buddy_system_allocator::LockedHeap;
use core::alloc::GlobalAlloc;
use libc_print::std_name::*;
use ota_allocator::mman_wrapper;

// FIXME, figure out the proper ORDER value here, using 16 for now
pub const BUDDY_ALLOCATOR_ORDER: usize = 16;

pub fn test_runner<T: GlobalAlloc>(tests: &[&dyn Fn()], allocator: &mut AllocTestWrapper<T>) {
    println!("\nRunning {} tests...\n", tests.len());
    for test in tests {
        allocator.use_tested_alloc = true;
        test();
        allocator.use_tested_alloc = false;
    }
    println!();
}

// TODO once you can use another metadata allocator than buddy for OtaAllocator move this function
//  in alloc_test_wrapper
pub unsafe fn init_buddy_allocator(
    buddy_alloc: &LockedHeap<{ BUDDY_ALLOCATOR_ORDER }>,
    addr_space_start: usize,
    addr_space_size: usize,
) {
    if let Err(err) =
        mman_wrapper::mmap(addr_space_start, addr_space_size)
    {
        eprintln!(
            "Error with code: {}, when calling mmap for allocating heap memory!",
            err
        );
        panic!("");
    }

    buddy_alloc
        .lock()
        .init(addr_space_start, addr_space_size);
}
