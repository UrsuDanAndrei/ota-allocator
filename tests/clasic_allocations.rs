#![no_std]
#![feature(default_alloc_error_handler)]
#![feature(core_panic)]
#![feature(allocator_api)]

mod commons;

extern crate alloc;

#[test]
fn simple_box_allocation() {
    commons::init_test();

    commons::simple_box_allocation();

    commons::end_test();
}

#[test]
fn multiple_boxes_allocation() {
    commons::init_test();

    commons::multiple_boxes_allocation();

    commons::end_test();
}

#[test]
fn intertwined_box_allocation() {
    commons::init_test();

    commons::intertwined_box_allocation();

    commons::end_test();
}

#[test]
fn vec_allocation() {
    commons::init_test();

    commons::vec_allocation(256);

    commons::end_test();
}

#[test]
fn mixed_allocation() {
    commons::init_test();

    commons::vec_allocation(2);
    commons::intertwined_box_allocation();
    commons::vec_allocation(128);
    commons::simple_box_allocation();
    commons::simple_box_allocation();
    commons::vec_allocation(64);
    commons::intertwined_box_allocation();

    commons::end_test();
}

#[test]
fn many_small_allocations() {
    commons::init_test();

    commons::many_small_allocations();

    commons::end_test();
}
