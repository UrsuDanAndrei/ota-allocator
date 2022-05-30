use libc_print::std_name::eprintln;

use crate::metadata::thread_meta::small_allocator::pool::Pool;
use crate::{consts, utils::mman_wrapper};

// TODO maybe make POOL_SIZE and MAPPED_MEMORY_EXTENSION_SIZE const type parameters
pub struct PoolAllocator {
    last_mapped_addr: usize, // open endpoint
    next_pool_addr: usize,
}

impl PoolAllocator {
    pub fn new(first_pool_addr: usize) -> Self {
        let mut pool_alloc = PoolAllocator {
            next_pool_addr: first_pool_addr,
            last_mapped_addr: first_pool_addr,
        };

        // preallocate a tank
        pool_alloc.map_new_tank();

        pool_alloc
    }

    pub fn next_pool(&mut self) -> Pool {
        let pool = Pool::new(self.next_pool_addr);
        self.next_pool_addr += consts::POOL_SIZE;

        if self.next_pool_addr > self.last_mapped_addr {
            self.map_new_tank();
        }

        pool
    }

    fn map_new_tank(&mut self) {
        if let Err(err) = unsafe { mman_wrapper::mmap(self.last_mapped_addr, consts::TANK_SIZE) } {
            // TODO maybe handle mmap errors
            eprintln!("Error with code: {}, when calling mmap!", err);
            panic!("");
        }

        self.last_mapped_addr += consts::TANK_SIZE;
    }
}
