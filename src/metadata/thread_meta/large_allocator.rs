mod large_meta;

use core::alloc::Allocator;
use hashbrown::hash_map::DefaultHashBuilder;
use hashbrown::HashMap;
use crate::{consts, utils, utils::mman_wrapper};
use libc_print::std_name::eprintln;
use crate::metadata::thread_meta::large_allocator::large_meta::LargeMeta;

pub struct LargeAllocator<'a, A : Allocator> {
    last_mapped_addr: usize, // open endpoint
    next_addr: usize,
    addr2lmeta: HashMap<usize, LargeMeta, DefaultHashBuilder, &'a A>,
}

impl<'a, A : Allocator> LargeAllocator<'a, A> {
    pub fn new_in(first_addr: usize, meta_alloc: &'a A) -> Self {
        LargeAllocator {
            last_mapped_addr: first_addr,
            next_addr: first_addr,
            addr2lmeta: HashMap::with_capacity_in(consts::RESV_ADDRS_NO, meta_alloc),
        }
    }

    pub fn next_addr(&mut self, size: usize) -> usize {
        let addr = utils::align_up(self.next_addr, consts::STANDARD_ALIGN);

        self.next_addr = addr + size;

        if self.next_addr > self.last_mapped_addr {
            let size_unmapped = self.next_addr - self.last_mapped_addr;

            let expand_size = if size_unmapped >= consts::TANK_SIZE {
                size_unmapped + consts::TANK_SIZE
            } else {
                consts::TANK_SIZE
            };

            self.expand_mapped_region(expand_size);
        }

        self.addr2lmeta.insert(addr, LargeMeta::new(size));

        addr
    }

    pub fn free(&self, addr: usize) {
        // if let Err(err) = unsafe { mman_wrapper::munmap(self.last_mapped_addr, size) } {
        //     // TODO maybe handle mmap errors
        //     eprintln!("Error with code: {}, when calling mmap!", err);
        //     panic!("");
        // }
    }

    fn expand_mapped_region(&mut self, size: usize) {
        if let Err(err) = unsafe { mman_wrapper::mmap(self.last_mapped_addr, size) } {
            // TODO maybe handle mmap errors
            eprintln!("Error with code: {}, when calling mmap!", err);
            panic!("");
        }

        self.last_mapped_addr += size;
    }
}
