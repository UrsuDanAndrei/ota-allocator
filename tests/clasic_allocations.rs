#![no_std]

#![feature(const_fn_trait_bound)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]

mod commons;

extern crate alloc;

use commons::AllocTestWrapper;
use ota_allocator::OtaAllocator;
use alloc::{boxed::Box, vec::Vec};
use libc_print::std_name::*;
use lazy_static::lazy_static;

#[global_allocator]
static mut ALLOCATOR: AllocTestWrapper<OtaAllocator> =
    AllocTestWrapper::new(OtaAllocator::new());

pub fn test_runner(tests: &[&dyn Fn()]) {
    unsafe {
        ALLOCATOR.tested_alloc.init();
        commons::test_runner(tests, &ALLOCATOR);
    }
}

#[test_case]
fn simple_box_allocation() {
    print!("testing simple_box_allocation... ");

    let x = Box::new(2);
    assert_eq!(*x, 2);

    println!("OK");
}

#[test_case]
fn multiple_boxes_allocation() {
    print!("testing multiple_boxes_allocation... ");

    let x = Box::new(2);
    assert_eq!(*x, 2);

    let y = Box::new(3);
    assert_eq!(*y, 3);

    let z = Box::new(4);
    assert_eq!(*z, 4);

    println!("OK");
}

#[test_case]
fn intertwined_box_allocation() {
    print!("testing intertwined_box_allocation... ");

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

    println!("OK");
}

#[test_case]
fn vec_allocation() {
    print!("testing vec_allocation... ");

    let mut v = Vec::new();
    let max_size = 256;

    for i in 0..max_size {
        v.push(2 * i);
    }

    assert_eq!(v.iter().sum::<usize>(), max_size * (max_size - 1));

    println!("OK");
}
