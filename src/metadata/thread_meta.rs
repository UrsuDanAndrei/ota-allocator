mod addr_meta;

use addr_meta::AddrMeta;
use crate::consts;
use crate::utils::mman_wrapper;
use crate::utils;
use core::alloc::{Allocator, Layout};
use core::cmp;
use hashbrown::hash_map::DefaultHashBuilder;
use hashbrown::HashMap;
use libc_print::std_name::*;

pub struct ThreadMeta<'a, A: Allocator> {
    last_addr: usize,
    addr2ameta: HashMap<usize, AddrMeta, DefaultHashBuilder, &'a A>,
}

impl<'a, A: Allocator> ThreadMeta<'a, A> {
    pub fn new_in(last_addr: usize, allocator: &'a A) -> Self {
        ThreadMeta {
            addr2ameta: HashMap::with_capacity_in(consts::RESV_ADDRS_NO, allocator),
            last_addr,
        }
    }

    pub fn next_addr(&mut self, layout: Layout) -> usize {
        let next_addr = utils::align_down(
            self.last_addr - layout.size(),
            cmp::max(layout.align(), consts::PAGE_SIZE),
        );

        if let Err(err) = unsafe { mman_wrapper::mmap(next_addr, layout.size()) } {
            // TODO maybe handle mmap errors
            eprintln!("Error with code: {}, when calling mmap!", err);
            panic!("");
        }

        // FIXME AddrMeta::new(layout.size() or self.last_addr - next_addr or something else)
        self.addr2ameta
            .insert(next_addr, AddrMeta::new(layout.size()));
        self.last_addr = next_addr;

        next_addr
    }

    pub fn free_addr(&mut self, addr: usize) {
        let ameta = self.addr2ameta.get(&addr).unwrap();

        if let Err(err) = unsafe { mman_wrapper::munmap(addr, ameta.size) } {
            // TODO maybe handle mmap errors
            eprintln!("Error with code: {}, when calling munmap!", err);
            panic!("");
        }

        self.addr2ameta.remove(&addr);
    }
}
