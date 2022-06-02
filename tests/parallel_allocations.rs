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

    run_with_two_threads(
        commons::simple_box_allocation,
        commons::simple_box_allocation,
    );

    commons::end_test();
}

#[test]
fn multiple_boxes_allocation() {
    commons::init_test();

    run_with_two_threads(
        commons::multiple_boxes_allocation,
        commons::multiple_boxes_allocation,
    );

    commons::end_test();
}

#[test]
fn intertwined_box_allocation() {
    commons::init_test();

    run_with_two_threads(
        commons::intertwined_box_allocation,
        commons::intertwined_box_allocation,
    );

    commons::end_test();
}

#[test]
fn vec_allocation() {
    commons::init_test();

    run_with_two_threads(
        || {
            commons::vec_allocation(128);
        },
        || {
            commons::vec_allocation(256);
        },
    );

    commons::end_test();
}

#[test]
fn mixed_allocation() {
    commons::init_test();

    run_with_two_threads(
        || {
            commons::vec_allocation(128);
            useless_compute();
            commons::intertwined_box_allocation();
            commons::vec_allocation(64);
            useless_compute();
            commons::vec_allocation(2);
            commons::simple_box_allocation();
            commons::intertwined_box_allocation();
            useless_compute();
        },
        || {
            commons::vec_allocation(2);
            useless_compute();
            commons::vec_allocation(32);
            commons::simple_box_allocation();
            useless_compute();
            commons::intertwined_box_allocation();
            useless_compute();
            commons::vec_allocation(16);
        },
    );

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

#[test]
fn many_small_allocations() {
    commons::init_test();

    run_with_two_threads(
        commons::many_small_allocations,
        commons::many_small_allocations,
    );

    commons::end_test();
}

#[test]
fn many_bins_allocations() {
    commons::init_test();

    run_with_two_threads(
        commons::many_bins_allocations,
        commons::many_bins_allocations,
    );

    commons::end_test();
}

#[test]
fn large_allocations() {
    commons::init_test();

    run_with_two_threads(commons::large_allocations, commons::large_allocations);

    commons::end_test();
}

#[test]
fn mixed_large_small_allocations() {
    commons::init_test();

    run_with_two_threads(
        || {
            commons::many_bins_allocations();
            commons::large_allocations();
            commons::large_allocations();
            commons::many_bins_allocations();
        },
        || {
            commons::simple_box_allocation();
            commons::large_allocations();
            commons::many_bins_allocations();
        },
    );

    commons::end_test();
}

fn run_with_two_threads(thread1task: fn(), thread2task: fn()) {
    let t1 = thread::spawn(move || {
        thread1task();
    });
    let t2 = thread::spawn(move || {
        thread2task();
    });
    t1.join().unwrap();
    t2.join().unwrap();
}

#[inline(always)]
fn useless_compute() {
    (1..1_000_000).sum::<usize>();
}
