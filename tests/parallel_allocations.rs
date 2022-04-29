#![no_std]
#![feature(default_alloc_error_handler)]
#![feature(core_panic)]
#![feature(allocator_api)]

mod commons;

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};

#[test]
fn simple_box_allocation() {
    commons::init_test();

    let x = Box::new(2);
    assert_eq!(*x, 2);

    commons::end_test();
}
