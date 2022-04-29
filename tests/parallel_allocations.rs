// TODO find a way to make this test no_std, if you can't only enable it when the std feature flag
//  is used

#![feature(default_alloc_error_handler)]
#![feature(core_panic)]
#![feature(allocator_api)]

mod commons;

extern crate alloc;

use alloc::vec::Vec;
use std::thread;

#[test]
fn simple_box_allocation() {
    commons::init_test();
    {
        let t1 = thread::spawn(|| {
            commons::simple_box_allocation();
        });

        let t2 = thread::spawn(|| {
            commons::simple_box_allocation();
        });

        t1.join().unwrap();
        t2.join().unwrap();
    }
    commons::end_test();
}

#[test]
fn multiple_boxes_allocation() {
    commons::init_test();
    {
        let t1 = thread::spawn(|| {
            commons::multiple_boxes_allocation();
        });

        let t2 = thread::spawn(|| {
            commons::multiple_boxes_allocation();
        });

        t1.join().unwrap();
        t2.join().unwrap();
    }
    commons::end_test();
}

#[test]
fn intertwined_box_allocation() {
    commons::init_test();
    {
        let t1 = thread::spawn(|| {
            commons::intertwined_box_allocation();
        });

        let t2 = thread::spawn(|| {
            commons::intertwined_box_allocation();
        });

        t1.join().unwrap();
        t2.join().unwrap();
    }
    commons::end_test();
}

#[test]
fn vec_allocation() {
    commons::init_test();
    {
        let t1 = thread::spawn(|| {
            commons::vec_allocation(256);
        });

        let t2 = thread::spawn(|| {
            commons::vec_allocation(256);
        });

        t1.join().unwrap();
        t2.join().unwrap();
    }
    commons::end_test();
}

#[test]
fn mixed_allocation() {
    commons::init_test();
    {
        let t1 = thread::spawn(|| {
            commons::vec_allocation(128);
            useless_compute();
            commons::intertwined_box_allocation();
            commons::vec_allocation(64);
            useless_compute();
            commons::vec_allocation(2);
            commons::simple_box_allocation();
            commons::intertwined_box_allocation();
            useless_compute();
        });

        let t2 = thread::spawn(|| {
            commons::vec_allocation(2);
            useless_compute();
            commons::vec_allocation(32);
            commons::simple_box_allocation();
            useless_compute();
            commons::intertwined_box_allocation();
            useless_compute();
            commons::vec_allocation(16);
        });

        t1.join().unwrap();
        t2.join().unwrap();
    }
    commons::end_test();
}

#[test]
fn many_threads() {
    commons::init_test();
    {
        let mut handles = Vec::new();

        for _ in 0..16 {
            handles.push(thread::spawn(|| {
                commons::vec_allocation(32);
                commons::simple_box_allocation();
                useless_compute();
                commons::intertwined_box_allocation();
                commons::vec_allocation(64);
                useless_compute();
                commons::intertwined_box_allocation();
                useless_compute();
            }))
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
    commons::end_test();
}

#[inline(always)]
pub fn useless_compute() {
    (1..1_000_000).sum::<usize>();
}
