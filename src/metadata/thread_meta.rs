mod addr_meta;
mod bin;
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
use crate::metadata::thread_meta::bin::Bin;
use arr_macro::arr;
use crate::consts::BINS_NO;

pub struct ThreadMeta<'a, A: Allocator> {
    addr2ameta: HashMap<usize, AddrMeta<'a, A>, DefaultHashBuilder, &'a A>,
    pool_alloc: PoolAllocator,
    bins: [Bin<'a, A>; consts::BINS_NO]
}

impl<'a, A: Allocator> ThreadMeta<'a, A> {
    pub fn new_in(first_addr: usize, allocator: &'a A) -> Self {
        let mut pool_alloc = PoolAllocator::new(first_addr);
        let mut size = consts::STANDARD_ALIGN / 2;

        // this is for assuring consistency, arr! only accepts a literal
        assert_eq!(consts::BINS_NO, 10);

        ThreadMeta {
            addr2ameta: HashMap::with_capacity_in(consts::RESV_ADDRS_NO, allocator),
            bins: arr![
                Bin::new(
                    { size <<= 1; size },
                    RcAlloc::new_in(
                        RefCell::new(pool_alloc.next_pool()),
                        allocator
                    )
                );
                10
            ],
            pool_alloc,
        }
    }

    pub fn next_addr(&mut self, size: usize) -> usize {
        // TODO change this when implementing big allocation management
        if size > consts::POOL_SIZE {
            return self.pool_alloc.next_region(size);
        }

        let bin_index = self.size2bin_index(size);
        let bin = &mut self.bins[bin_index];

        let addr = bin.pool.borrow_mut().next_addr(size);
        let addr = addr.unwrap_or_else(|| {
            bin.pool = RcAlloc::new_in(
                RefCell::new(self.pool_alloc.next_pool()),
                *self.addr2ameta.allocator(),
            );

            bin.pool.borrow_mut().next_addr(size).unwrap()
        });

        self.addr2ameta
            .insert(addr, AddrMeta::new(size, bin.pool.clone()));

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

    // TODO optimise this
    fn size2bin_index(&self, size: usize) -> usize {
        if size <= consts::STANDARD_ALIGN {
            0
        } else if size >= consts::PAGE_SIZE {
            consts::BINS_NO - 1
        } else {
            (size.next_power_of_two().trailing_zeros() - 4) as usize
        }
    }
}
