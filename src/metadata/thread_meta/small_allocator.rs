mod bin;
pub mod pool;
mod pool_allocator;
mod small_meta;

use crate::consts;
use crate::metadata::thread_meta::small_allocator::bin::Bin;
use crate::metadata::thread_meta::small_allocator::pool_allocator::PoolAllocator;
use crate::metadata::thread_meta::small_allocator::small_meta::SmallMeta;
use crate::utils::rc_alloc::RcAlloc;
use core::alloc::Allocator;
use core::cell::RefCell;
use core::hash::BuildHasherDefault;
use hashbrown::HashMap;
use libc_print::libc_eprintln;
use rustc_hash::FxHasher;

pub struct SmallAllocator<'a, A: Allocator> {
    pool_alloc: PoolAllocator,
    bins: [Bin<'a, A>; consts::BINS_NO],
    addr2smeta: HashMap<usize, SmallMeta<'a, A>, BuildHasherDefault<FxHasher>, &'a A>,
    meta_alloc: &'a A,
}

impl<'a, A: Allocator> SmallAllocator<'a, A> {
    pub fn new_in(first_addr: usize, meta_alloc: &'a A) -> Self {
        let mut pool_alloc = PoolAllocator::new(first_addr);

        // this is for assuring consistency, arr! only accepts a literal
        assert_eq!(consts::BINS_NO, 10);

        SmallAllocator {
            bins: arr_macro::arr![
                Bin::new(
                    RcAlloc::new_in(
                        RefCell::new(pool_alloc.next_pool()),
                        meta_alloc
                    )
                );
                10
            ],
            pool_alloc,
            addr2smeta: HashMap::with_capacity_and_hasher_in(
                consts::RESV_ADDRS_NO,
                BuildHasherDefault::<FxHasher>::default(),
                meta_alloc,
            ),
            meta_alloc,
        }
    }

    pub fn next_addr(&mut self, size: usize) -> usize {
        let bin_index = self.size2bin_index(size);
        let bin = &mut self.bins[bin_index];

        let addr = bin.pool.borrow_mut().next_addr(size);
        let addr = addr.unwrap_or_else(|| {
            bin.pool = RcAlloc::new_in(RefCell::new(self.pool_alloc.next_pool()), self.meta_alloc);

            bin.pool.borrow_mut().next_addr(size).unwrap()
        });

        self.addr2smeta
            .insert(addr, SmallMeta::new(size, bin.pool.clone()));

        addr
    }

    pub fn free(&mut self, addr: usize) {
        if self.addr2smeta.remove(&addr).is_none() {
            libc_eprintln!("Invalid or double free! addr: {}", addr);
        }
    }

    // TODO see what to do with size, maybe actually get use to this function
    pub fn usable_size(&self, addr: usize) -> usize {
        match self.addr2smeta.get(&addr) {
            None => {
                libc_eprintln!("Invalid or already freed address: {}", addr);
                0
            },
            Some(smeta) => smeta.size,
        }
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
