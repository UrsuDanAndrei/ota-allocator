#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]

mod commons;

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use buddy_system_allocator::LockedHeap;
use commons::AllocTestWrapper;
use lazy_static::lazy_static;
use libc_print::std_name::*;
use ota_allocator::OtaAllocator;

type MetaTestAlloc = LockedHeap<{ commons::BUDDY_ALLOCATOR_ORDER }>;

#[global_allocator]
static mut ALLOCATOR: AllocTestWrapper<OtaAllocator<MetaTestAlloc>> =
    AllocTestWrapper::new(OtaAllocator::new(MetaTestAlloc::new()));

pub fn test_runner(tests: &[&dyn Fn()]) {
    unsafe {
        init_meta_alloc();
        ALLOCATOR.tested_alloc.init();

        commons::test_runner(tests, &mut ALLOCATOR);
    }
}

unsafe fn init_meta_alloc() {
    commons::init_buddy_allocator(
        ALLOCATOR.tested_alloc.meta_alloc(),
        ota_allocator::META_ADDR_SPACE_START,
        ota_allocator::META_ADDR_SPACE_MAX_SIZE,
    );
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
