pub mod alloc_test_wrapper;

// reexports
pub use alloc_test_wrapper::AllocTestWrapper;

use core::alloc::GlobalAlloc;
use libc_print::std_name::*;

pub fn test_runner<T: GlobalAlloc>(tests: &[&dyn Fn()], allocator: &AllocTestWrapper<T>) {
    println!("\nRunning {} tests...\n", tests.len());
    for test in tests {
        unsafe { allocator.use_tested_allocator(); }
        test();
        unsafe { allocator.use_standard_allocator() }
    }
    println!();
}
