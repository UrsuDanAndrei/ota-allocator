mod alloc_test_wrapper;
mod test_scenarios;

// reexport
pub use test_scenarios::*;

use alloc_test_wrapper::AllocTestWrapper;

#[global_allocator]
static mut ALLOCATOR: AllocTestWrapper = AllocTestWrapper::new();

pub fn init_test() {
    unsafe {
        ALLOCATOR.try_init();
        ALLOCATOR.use_ota_allocator();
    }
}

pub fn end_test() {
    unsafe {
        ALLOCATOR.use_cargo_allocator();
    }
}
