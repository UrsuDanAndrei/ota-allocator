mod addr_meta;
mod large_allocator;
mod small_allocator;

use crate::consts;
use crate::consts::BINS_NO;
use crate::metadata::thread_meta::large_allocator::LargeAllocator;
use crate::metadata::thread_meta::small_allocator::SmallAllocator;
use crate::utils::mman_wrapper;
use crate::utils::rc_alloc::RcAlloc;
use addr_meta::AddrMeta;
use arr_macro;
use core::alloc::Allocator;
use core::cell::RefCell;
use hashbrown::hash_map::DefaultHashBuilder;
use hashbrown::HashMap;
use libc_print::std_name::eprintln;

// FIXME move addr2meta in SmallAllocator
pub struct ThreadMeta<'a, A: Allocator> {
    addr2ameta: HashMap<usize, AddrMeta<'a, A>, DefaultHashBuilder, &'a A>,
    small_alloc: SmallAllocator<'a, A>,
    large_alloc: LargeAllocator,
}

impl<'a, A: Allocator> ThreadMeta<'a, A> {
    pub fn new_in(first_addr: usize, meta_alloc: &'a A) -> Self {
        ThreadMeta {
            addr2ameta: HashMap::with_capacity_in(consts::RESV_ADDRS_NO, meta_alloc),
            small_alloc: SmallAllocator::new_in(first_addr, meta_alloc),
            // FIXME large allocator should start from a different address
            large_alloc: LargeAllocator::new(first_addr),
        }
    }

    pub fn next_addr(&mut self, size: usize) -> usize {
        if size > consts::POOL_SIZE {
            self.large_alloc.next_addr(size)
        } else {
            let (addr, addr_meta) = self.small_alloc.next_addr(size);
            self.addr2ameta.insert(addr, addr_meta);
            addr
        }
    }

    pub fn free_addr(&mut self, addr: usize) {
        let ameta = self.addr2ameta.get(&addr).unwrap();

        // TODO change this when implementing big allocation management
        if let AddrMeta::LargeMeta { size } = ameta {
            if let Err(err) = unsafe { mman_wrapper::munmap(addr, *size) } {
                // TODO maybe handle mmap errors
                eprintln!("Error with code: {}, when calling munmap!", err);
                panic!("");
            }
        }

        self.addr2ameta.remove(&addr);
    }
}
