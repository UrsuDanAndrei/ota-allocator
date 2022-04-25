// FIXME, figure out the proper ORDER value here, using 16 for now
pub const BUDDY_ALLOCATOR_ORDER: usize = 16;

// TODO figure out the proper value to use here
pub const INIT_THREADS_NO: usize = 0;

// TODO make this configurable
pub const PAGE_SIZE: usize = 4096;

// TODO manage address spaces
pub const FIRST_ADDR_SPACE_START: usize = 0x0000_7FFD_0000_0000;
pub const ADDR_SPACE_MAX_SIZE: usize = 0x0000_0001_0000_0000;
pub const ADDR_SPACE_MASK: usize = 0xFFFF_FFFF_0000_0000;

pub const META_ADDR_SPACE_START: usize = 0x0000_7FFE_0000_0000;
pub const META_ADDR_SPACE_MAX_SIZE: usize = 2 * PAGE_SIZE;

pub const TEST_ADDR_SPACE_START: usize = 0x0000_7FFF_0000_0000;
pub const TEST_ADDR_SPACE_MAX_SIZE: usize = 2 * PAGE_SIZE;


/*
0 -> 0000_7FFF_FFFF_FFFF
FFFF_8000_0000_0000 -> FFFF_FFFF_FFFF_FFFF

32768 = 0x8000
65536 = 0x10000 (0xFFFF + 1)
 */
