mod large_meta;

use crate::metadata::thread_meta::large_allocator::large_meta::LargeMeta;
use crate::utils::rc_alloc::RcAlloc;
use crate::{consts, utils, utils::mman_wrapper};
use core::alloc::Allocator;
use core::hash::BuildHasherDefault;
use hashbrown::HashMap;
use libc_print::libc_eprintln;
use rustc_hash::FxHasher;

pub struct LargeAllocator<'a, A: Allocator> {
    last_mapped_addr: usize, // open endpoint
    next_addr: usize,
    current_page: RcAlloc<Page, &'a A>,
    addr2lmeta: HashMap<usize, LargeMeta<'a, A>, BuildHasherDefault<FxHasher>, &'a A>,
    meta_alloc: &'a A,
}

pub struct Page(usize);

impl<'a, A: Allocator> LargeAllocator<'a, A> {
    pub fn new_in(first_addr: usize, meta_alloc: &'a A) -> Self {
        let mut large_alloc = LargeAllocator {
            last_mapped_addr: first_addr,
            next_addr: first_addr,
            current_page: RcAlloc::new_in(Page(first_addr), meta_alloc),
            addr2lmeta: HashMap::with_capacity_and_hasher_in(
                consts::RESV_ADDRS_NO,
                BuildHasherDefault::<FxHasher>::default(),
                meta_alloc,
            ),
            meta_alloc,
        };

        large_alloc.expand_mapped_region(consts::TANK_SIZE);

        large_alloc
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

            self.expand_mapped_region(utils::align_up(expand_size, consts::PAGE_SIZE));
        }

        let next_page = RcAlloc::new_in(
            Page(utils::align_down(self.next_addr, consts::PAGE_SIZE)),
            self.meta_alloc,
        );

        self.addr2lmeta.insert(
            addr,
            LargeMeta::new(size, self.current_page.clone(), next_page.clone()),
        );

        self.current_page = next_page;

        addr
    }

    pub fn free(&mut self, addr: usize) {
        let lmeta = self.addr2lmeta.remove(&addr);

        if lmeta.is_none() {
            if addr != 0 {
                libc_eprintln!("Invalid or double free! addr: {}", addr);
            }

            return;
        }

        // SAFETY: the if above checks for None
        let lmeta = unsafe { lmeta.unwrap_unchecked() };

        let first_page = if lmeta.first_page.count() == 1 {
            lmeta.first_page.0
        } else {
            lmeta.first_page.0 + consts::PAGE_SIZE
        };

        let last_page = if lmeta.last_page.count() == 1 {
            lmeta.last_page.0 + consts::PAGE_SIZE
        } else {
            lmeta.last_page.0
        };

        let size = last_page - first_page;

        if let Err(err) = unsafe { mman_wrapper::munmap(first_page, size) } {
            // TODO maybe handle mmap errors
            libc_eprintln!(
                "Error with code: {}, when calling munmap! addr: {}, size: {}",
                err,
                first_page,
                size
            );
            panic!("");
        }
    }

    // TODO see what to do with size, maybe actually get use to this function
    pub fn usable_size(&self, addr: usize) -> usize {
        match self.addr2lmeta.get(&addr) {
            None => {
                if addr != 0 {
                    libc_eprintln!("Invalid or already freed address: {}", addr);
                }

                0
            }
            Some(lmeta) => lmeta.size,
        }
    }

    fn expand_mapped_region(&mut self, size: usize) {
        if let Err(err) = unsafe { mman_wrapper::mmap(self.last_mapped_addr, size) } {
            // TODO maybe handle mmap errors
            libc_eprintln!(
                "Error with code: {}, when calling mmap! addr: {}, size: {}",
                err,
                self.last_mapped_addr,
                size
            );
            panic!("");
        }

        self.last_mapped_addr += size;
    }
}
