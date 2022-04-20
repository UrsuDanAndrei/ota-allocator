pub mod allocator_test_wrapper;

// reexports
pub use allocator_test_wrapper::AllocatorTestWrapper;

use core::alloc::GlobalAlloc;
use libc_print::std_name::*;

pub fn test_runner<T: GlobalAlloc>(tests: &[&dyn Fn()], allocator: &mut AllocatorTestWrapper<T>) {
    println!("\nRunning {} tests...\n", tests.len());
    for test in tests {
        allocator.use_wrapped_allocator = true;
        test();
        allocator.use_wrapped_allocator = false;
    }
    println!();
}
