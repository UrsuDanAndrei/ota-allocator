mod large_allocator;
mod small_allocator;

use crate::metadata::thread_meta::large_allocator::LargeAllocator;
use crate::metadata::thread_meta::small_allocator::SmallAllocator;
use crate::{consts, utils};
use core::alloc::Allocator;

pub struct ThreadMeta<'a, A: Allocator> {
    small_alloc: SmallAllocator<'a, A>,
    large_alloc: LargeAllocator<'a, A>,
}

impl<'a, A: Allocator> ThreadMeta<'a, A> {
    pub fn new_in(first_addr: usize, meta_alloc: &'a A) -> Self {
        ThreadMeta {
            small_alloc: SmallAllocator::new_in(first_addr, meta_alloc),
            large_alloc: LargeAllocator::new_in(
                first_addr + consts::LARGE_ADDR_SPACE_OFFSET,
                meta_alloc,
            ),
        }
    }

    pub fn next_addr(&mut self, size: usize) -> usize {
        if size < consts::POOL_SIZE {
            self.small_alloc.next_addr(size)
        } else {
            self.large_alloc.next_addr(size)
        }
    }

    pub fn free(&mut self, addr: usize) {
        if utils::is_small_addr(addr) {
            self.small_alloc.free(addr);
        } else {
            self.large_alloc.free(addr);
        }
    }
}
