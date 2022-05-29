use alloc::{boxed::Box, vec::Vec};

pub fn simple_box_allocation() {
    let x = Box::new(2);
    assert_eq!(*x, 2);
}

pub fn multiple_boxes_allocation() {
    let x = Box::new(2);
    assert_eq!(*x, 2);

    let y = Box::new(3);
    assert_eq!(*y, 3);

    let z = Box::new(4);
    assert_eq!(*z, 4);
}

pub fn intertwined_box_allocation() {
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
}

pub fn vec_allocation(max_size: usize) {
    let mut v = Vec::new();

    for i in 0..max_size {
        v.push(2 * i);
    }

    assert_eq!(v.iter().sum::<usize>(), max_size * (max_size - 1));
}

pub fn many_small_allocations() {
    let to_box = (2_u64, 2_u64);

    // consume the first pool
    for _ in 0..(ota_allocator::POOL_SIZE / 16 + 1) {
        let _ = Box::new(to_box);
    }

    // consume multiple pools
    for _ in 0..4 {
        for _ in 0..(ota_allocator::POOL_SIZE / 16 + 1) {
            let _ = Box::new(to_box);
        }
    }

    // trigger a second mmap call
    for _ in 0..(ota_allocator::TANK_SIZE / 16 + 1) {
        let _ = Box::new(to_box);
    }

    // trigger multiple mmap calls
    for _ in 0..4 {
        for _ in 0..(ota_allocator::TANK_SIZE / 16) {
            let _ = Box::new(to_box);
        }
    }
}
