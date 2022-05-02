mod addr_meta;
mod pool_allocator;

use crate::consts;
use crate::metadata::thread_meta::pool_allocator::{Pool, PoolAllocator};
use crate::utils::mman_wrapper;
use crate::utils::rc_alloc::RcAlloc;
use addr_meta::AddrMeta;
use core::alloc::Allocator;
use core::cell::RefCell;
use hashbrown::hash_map::DefaultHashBuilder;
use hashbrown::HashMap;
use libc_print::std_name::eprintln;

pub struct ThreadMeta<'a, A: Allocator> {
    addr2ameta: HashMap<usize, AddrMeta<'a, A>, DefaultHashBuilder, &'a A>,
    pool: RcAlloc<RefCell<Pool>, &'a A>,
    pool_alloc: PoolAllocator,
    i: usize,
}

impl<'a, A: Allocator> ThreadMeta<'a, A> {
    pub fn new_in(first_addr: usize, allocator: &'a A) -> Self {
        let mut pool_alloc = PoolAllocator::new(first_addr);
        let current_pool = pool_alloc.next_pool();

        ThreadMeta {
            addr2ameta: HashMap::with_capacity_in(consts::RESV_ADDRS_NO, allocator),
            pool: RcAlloc::new_in(RefCell::new(current_pool), allocator),
            pool_alloc,
            i: 0,
        }
    }

    pub fn next_addr(&mut self, size: usize) -> usize {
        // TODO change this when implementing big allocation management
        if size > consts::POOL_SIZE {
            return self.pool_alloc.next_region(size);
        }

        let addr = self.pool.borrow_mut().next_addr(size);
        self.i += 1;
        let (addr, addr_meta) = match addr {
            Ok(addr) => (addr, AddrMeta::new_single_pool(size, self.pool.clone())),

            Err(alloc_err) => {
                let new_pool = RcAlloc::new_in(
                    RefCell::new(self.pool_alloc.next_pool()),
                    *self.addr2ameta.allocator(),
                );

                let addr_meta = if alloc_err.allocated != 0 {
                    AddrMeta::new_double_pool(size, self.pool.clone(), new_pool.clone())
                } else {
                    AddrMeta::new_single_pool(size, new_pool.clone())
                };

                self.pool = new_pool;

                (alloc_err.addr, addr_meta)
            }
        };

        self.addr2ameta.insert(addr, addr_meta);

        addr
    }

    pub fn free_addr(&mut self, addr: usize) {
        let ameta = self.addr2ameta.get(&addr).unwrap();

        // TODO change this when implementing big allocation management
        if ameta.size > consts::POOL_SIZE {
            if let Err(err) = unsafe { mman_wrapper::munmap(addr, ameta.size) } {
                // TODO maybe handle mmap errors
                eprintln!("Error with code: {}, when calling munmap!", err);
                panic!("");
            }
        }

        self.addr2ameta.remove(&addr);
    }
}
