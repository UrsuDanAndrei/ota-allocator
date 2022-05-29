mod bin;
mod pool_allocator;
pub mod pool;

use core::alloc::Allocator;
use core::cell::RefCell;
use hashbrown::hash_map::DefaultHashBuilder;
use hashbrown::HashMap;
use libc_print::std_name::eprintln;
use crate::{consts, utils::mman_wrapper};
use crate::metadata::thread_meta::addr_meta::AddrMeta;
use crate::metadata::thread_meta::small_alloc::bin::Bin;
use crate::metadata::thread_meta::small_alloc::pool_allocator::PoolAllocator;
use crate::utils::rc_alloc::RcAlloc;

pub struct SmallAlloc<'a, A: Allocator> {
    pool_alloc: PoolAllocator,
    bins: [Bin<'a, A>; consts::BINS_NO],
    meta_alloc: &'a A
}

impl<'a, A: Allocator> SmallAlloc<'a, A> {
    pub fn new_in(first_addr: usize, meta_alloc: &'a A) -> Self {
        let mut pool_alloc = PoolAllocator::new(first_addr);
        let mut size = consts::STANDARD_ALIGN / 2;

        // this is for assuring consistency, arr! only accepts a literal
        assert_eq!(consts::BINS_NO, 10);

        SmallAlloc {
            bins: arr_macro::arr![
                Bin::new(
                    { size <<= 1; size },
                    RcAlloc::new_in(
                        RefCell::new(pool_alloc.next_pool()),
                        meta_alloc
                    )
                );
                10
            ],
            pool_alloc,
            meta_alloc
        }
    }

    pub fn next_addr(&mut self, size: usize) -> (usize, AddrMeta<'a, A>) {
        let bin_index = self.size2bin_index(size);
        let bin = &mut self.bins[bin_index];

        let addr = bin.pool.borrow_mut().next_addr(size);
        let addr = addr.unwrap_or_else(|| {
            bin.pool = RcAlloc::new_in(
                RefCell::new(self.pool_alloc.next_pool()),
                self.meta_alloc,
            );

            bin.pool.borrow_mut().next_addr(size).unwrap()
        });

        (addr, AddrMeta::new(size, bin.pool.clone()))
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
