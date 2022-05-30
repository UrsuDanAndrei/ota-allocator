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

pub fn vec_allocation(size: usize) {
    let mut v = Vec::new();

    for i in 0..size {
        v.push(2 * i);
    }

    assert_eq!(v.iter().sum::<usize>(), size * (size - 1));
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

pub fn many_bins_allocations() {
    let bytes6 = [2_u8; 6];
    let bytes16 = [2_u8; 16];
    let bytes48 = [2_u8; 48];
    let bytes64 = [2_u8; 64];
    let bytes160 = [2_u8; 160];
    let bytes180 = [2_u8; 180];
    let bytes2048 = [2_u8; 2048];
    let bytes2400 = [2_u8; 2400];
    let bytes4096 = [2_u8; 4096];

    let mut x1 = Box::new(bytes6);
    let mut x2 = Box::new(bytes16);

    {
        let _x3 = Box::new(bytes16);
    }

    x1[2] = 1;
    x2[8] = 2;
    assert_eq!(x1[2], 1);
    assert_eq!(x2[2], 2);

    let _x4 = Box::new(bytes48);
    let _x5 = Box::new(bytes64);
    let mut x6 = Box::new(bytes64);

    let _x7 = Box::new(bytes160);

    {
        let _x8 = Box::new(bytes160);
        let _x9 = Box::new(bytes180);
        let _x10 = Box::new(bytes160);
    }

    let mut x11 = Box::new(bytes160);
    let mut x12 = Box::new(bytes180);

    x6[32] = 1;
    x11[48] = 2;
    x11[140] = 3;
    x12[170] = 4;
    assert_eq!(x6[32], 1);
    assert_eq!(x11[48], 2);
    assert_eq!(x11[140], 3);
    assert_eq!(x12[170], 4);

    let mut x13 = Box::new(bytes2048);
    let mut x14 = Box::new(bytes2400);
    let mut x15 = Box::new(bytes4096);

    {
        let mut x16 = Box::new(bytes2400);
        let mut x17 = Box::new(bytes4096);
        let _x18 = Box::new(bytes2048);
        let _x19 = Box::new(bytes2400);

        x16[32] = 1;
        x16[60] = 2;
        x16[1100] = 3;
        x16[2000] = 4;
        x17[32] = 1;
        x17[60] = 2;
        x17[1100] = 3;
        x17[2000] = 4;

        assert_eq!(x16[32], 1);
        assert_eq!(x16[60], 2);
        assert_eq!(x16[1100], 3);
        assert_eq!(x16[2000], 4);

        assert_eq!(x17[32], 1);
        assert_eq!(x17[60], 2);
        assert_eq!(x17[1100], 3);
        assert_eq!(x17[2000], 4);
    }

    x13[32] = 1;
    x13[60] = 2;
    x13[1100] = 3;
    x13[2000] = 4;
    x14[32] = 1;
    x14[60] = 2;
    x14[1100] = 3;
    x14[2000] = 4;
    x15[32] = 1;
    x15[60] = 2;
    x15[1100] = 3;
    x15[2000] = 4;

    assert_eq!(x13[32], 1);
    assert_eq!(x13[60], 2);
    assert_eq!(x13[1100], 3);
    assert_eq!(x13[2000], 4);

    assert_eq!(x14[32], 1);
    assert_eq!(x14[60], 2);
    assert_eq!(x14[1100], 3);
    assert_eq!(x14[2000], 4);

    assert_eq!(x15[32], 1);
    assert_eq!(x15[60], 2);
    assert_eq!(x15[1100], 3);
    assert_eq!(x15[2000], 4);

    let _x20 = Box::new(bytes2400);
}

pub fn large_allocations() {
    // trigger a large allocation
    let mut v1 = Vec::new();
    large_vec_add_test(&mut v1, ota_allocator::POOL_SIZE / 16 + 1);

    // trigger a jumbo allocation
    let mut v2 = Vec::new();
    large_vec_add_test(&mut v2, ota_allocator::TANK_SIZE / 16 + 1);

    // trigger multiple large allocations
    for i in 1..4 {
        let mut v3 = Vec::new();
        large_vec_add_test(&mut v3, i * ota_allocator::POOL_SIZE / 16 + i * 16);
    }

    // trigger a jumbo allocation
    let mut v4 = Vec::new();
    large_vec_add_test(&mut v4, 2 * ota_allocator::TANK_SIZE / 16 + 1);

    // trigger a large allocation
    let mut v5 = Vec::new();
    large_vec_add_test(&mut v5, ota_allocator::POOL_SIZE / 16 + 1);
}

fn large_vec_add_test(v: &mut Vec<u64>, size: usize) {
    let size = size as u64;

    for i in 0..size {
        v.push(2_u64 * i);
    }

    assert_eq!(v.iter().sum::<u64>(), size * (size - 1));
}
