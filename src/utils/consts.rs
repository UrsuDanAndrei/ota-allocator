// pub const MAX_THREADS_NO: usize = 3;
// pub const MAX_THREADS_NO: usize = 32768;
// pub const BUDDY_ALLOCATOR_ORDER: usize = 16;

pub const FIRST_ADDR_SPACE: usize = 0x0000_7FFE_0000_0000;
pub const ADDR_SPACE_SIZE: usize = 0x0000_0001_0000_0000;
pub const ADDR_SPACE_MASK: usize = 0xFFFF_FFFF_0000_0000;

pub const PAGE_SIZE: usize = 4096;

// TODO manage address spaces
pub const META_ADDR_SPACE: usize = 0x0000_7FFF_0000_0000;
pub const META_SPACE_SIZE: usize = 65536; // 16 * PAGE_SIZE;

/*

32768 -> 0x8000


==========================================
0 -> 0000_7FFF_FFFF_FFFF

FFFF_8000_0000_0000 -> FFFF_FFFF_FFFF_FFFF

32768 = 0x8000
65536 = 0x10000 (0xFFFF + 1)

2 ^ 31 allocations

2 ^ 32

 */
