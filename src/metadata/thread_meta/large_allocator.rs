use crate::{consts, utils, utils::mman_wrapper};
use libc_print::std_name::eprintln;

pub struct LargeAllocator {
    last_mapped_addr: usize, // open endpoint
    next_addr: usize,
}

impl LargeAllocator {
    pub fn new(first_addr: usize) -> Self {
        LargeAllocator {
            last_mapped_addr: first_addr,
            next_addr: first_addr,
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

        addr
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
