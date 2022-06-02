// TODO figure out the proper value to use here
pub const RESV_THREADS_NO: usize = 8;

// TODO figure out the proper value to use here
pub const RESV_ADDRS_NO: usize = 16;

// TODO make take this from the OS
pub const PAGE_SIZE: usize = 4096;

// TODO manage address spaces
// 0x0000_0000_0000_0000
// 64 bits for memory addressing, only 48 bits can be currently used,
// only addresses from 0x0000_0000_0000_0000 to 0x0000_7FFF_FFFF_FFFF are valid, so we can use only
// use 47 bits from 64 for addressing
//
// current design:
// - I chose to use addresses from 0x0000_0010_0000_0000 to 0x0000_7FFF_FFFF_FFFF
// - each address space has a size of 2 ^ 36 (64 GB), so we have a 11 bits left for identifying the
//   address space itself => a maximum of 2047 address spaces (we don't use address space 0)
// - address space 0x001 is used for testing purposes
// - address space 0x002 is used for metadata store
// - each thread that allocates memory is assign an address spaces from 0x003 to 0x7FF
//
// limitations:
// - a thread can only allocate a maximum of 64GB of memory (even if it frees it)
// - there can be only a maximum of 2045 threads that allocate memory
// - the 2 limitations above directly influence each other, if one improves the other worsens
// - TODO find the sweet spot to use as default, but give the user the option to adjust
//
// pub const FIRST_ADDR_SPACE_START: usize = 0x0000_0030_0000_0000;
// pub const ADDR_SPACE_MAX_SIZE: usize = 0x0000_0010_0000_0000;
// pub const ADDR_SPACE_MASK: usize = 0xFFFF_FFF0_0000_0000;
// pub const LARGE_ADDR_SPACE_OFFSET: usize = 0x0000_0008_0000_0000;
// pub const LARGE_ADDR_SPACE_MASK: usize = 0x0000_0008_0000_0000;
//
// pub const META_ADDR_SPACE_START: usize = 0x0000_0020_0000_0000;
// pub const META_ADDR_SPACE_MAX_SIZE: usize = 32 * PAGE_SIZE;

// values for 1TB max memory allocation, 128 max threads
pub const FIRST_ADDR_SPACE_START: usize = 0x0000_0300_0000_0000;
pub const ADDR_SPACE_MAX_SIZE: usize = 0x0000_0100_0000_0000;
pub const ADDR_SPACE_MASK: usize = 0xFFFF_FF00_0000_0000;
pub const LARGE_ADDR_SPACE_OFFSET: usize = 0x0000_0080_0000_0000;
pub const LARGE_ADDR_SPACE_MASK: usize = 0x0000_0080_0000_0000;

pub const META_ADDR_SPACE_START: usize = 0x0000_0200_0000_0000;
pub const META_ADDR_SPACE_MAX_SIZE: usize = TANK_SIZE;

// TODO make this values configurable from the user
pub const POOL_SIZE: usize = 2 * PAGE_SIZE;
pub const TANK_SIZE: usize = 512 * POOL_SIZE;

// TODO research how to do custom alignment per allocation instead of always using this const
pub const STANDARD_ALIGN: usize = 16;

pub const BINS_NO: usize = 10;

#[cfg(feature = "integration-test")]
pub const TEST_ADDR_SPACE_START: usize = 0x0000_0100_0000_0000;

#[cfg(feature = "integration-test")]
pub const TEST_ADDR_SPACE_MAX_SIZE: usize = 32 * PAGE_SIZE;
