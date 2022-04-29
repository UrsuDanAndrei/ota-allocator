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

#[test]
fn multiple_boxes_allocation() {
    commons::init_test();

    let x = Box::new(2);
    assert_eq!(*x, 2);

    let y = Box::new(3);
    assert_eq!(*y, 3);

    let z = Box::new(4);
    assert_eq!(*z, 4);

    commons::end_test();
}

#[test]
fn intertwined_box_allocation() {
    commons::init_test();

    let x = Box::new(2);
    let y = Box::new(3);

    assert_eq!(*x, 2);
    assert_eq!(*y, 3);

    let z = Box::new(4);

    {
        let u = Box::new(5);
        assert_eq!(*u, 5);
    }

    assert_eq!(*z, 4);

    commons::end_test();
}

#[test]
fn vec_allocation() {
    commons::init_test();

    let mut v = Vec::new();
    let max_size = 2;

    for i in 0..max_size {
        v.push(2 * i);
    }

    assert_eq!(v.iter().sum::<usize>(), max_size * (max_size - 1));

    commons::end_test();
}


