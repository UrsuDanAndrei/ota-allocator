mod pool;

use core::cmp;
use libc_print::std_name::eprintln;

// reexport
pub use pool::Pool;

use crate::{consts, utils, utils::mman_wrapper};

// TODO maybe make POOL_SIZE and MAPPED_MEMORY_EXTENSION_SIZE const type parameters
pub struct PoolAllocator {
    last_mapped_addr: usize, // open endpoint
    next_pool_addr: usize,
}

impl PoolAllocator {
    pub fn new(first_pool_addr: usize) -> Self {
        PoolAllocator {
            next_pool_addr: first_pool_addr,
            last_mapped_addr: first_pool_addr,
        }
    }

    pub fn next_pool(&mut self) -> Pool {
        let pool = Pool::new(self.next_pool_addr);
        self.next_pool_addr += consts::POOL_SIZE;

        if self.next_pool_addr > self.last_mapped_addr {
            self.extend_mapped_region(consts::MAPPED_MEMORY_EXTENSION_SIZE);
        }

        pool
    }

    // TODO remove this when implementing big allocation management
    // this method should only be called if the size of the allocation exceeds the size of the pool
    // this method will probably be moved or deleted when the management of big size allocation
    // is completed
    pub fn next_region(&mut self, size: usize) -> usize {
        let region = self.next_pool_addr;
        let size = utils::align_up(size, consts::POOL_SIZE);

        self.next_pool_addr += size;

        if self.next_pool_addr > self.last_mapped_addr {
            self.extend_mapped_region(cmp::max(consts::MAPPED_MEMORY_EXTENSION_SIZE, size));
        }

        region
    }

    fn extend_mapped_region(&mut self, size: usize) {
        if let Err(err) = unsafe { mman_wrapper::mmap(self.last_mapped_addr, size) } {
            // TODO maybe handle mmap errors
            eprintln!("Error with code: {}, when calling mmap!", err);
            panic!("");
        }

        self.last_mapped_addr += size;
    }
}
