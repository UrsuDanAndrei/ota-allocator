pub const MAX_THREADS_NO: usize = 3;
// pub const MAX_THREADS_NO: usize = 32768;
// pub const BUDDY_ALLOCATOR_ORDER: usize = 16;

pub const PAGE_SIZE: usize = 4096;

// TODO manage address spaces
pub const META_ADDR_START: usize = 0x00007FFF_00000000;
pub const META_HEAP_SIZE: usize = 16 * PAGE_SIZE;
pub const META_CHECK_MASK: usize = 0xFFFFFFFF_00000000;

/*
2 ^ 15

0x00007FFF_FF000000

0x7FFFF____

32768 -> 0x8000

FFFF8000_00000000
FFFFFFFF_FFF00000



FFFF_FFFF_FFF0_FFFF

0xFFFF_8000_0000_0000


 */
