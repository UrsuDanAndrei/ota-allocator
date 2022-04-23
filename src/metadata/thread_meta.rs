use core::alloc::{GlobalAlloc, Layout};
use super::addr_meta::AddrMeta;
use crate::{MetaAllocWrapper, mman_wrapper};
use core::cell::RefCell;
use core::cmp;
use core::ptr;
use hashbrown::hash_map::DefaultHashBuilder;
use hashbrown::HashMap;
use spin::Mutex;
use crate::utils;
use libc_print::std_name::*;

pub struct ThreadMeta<'ma, MA: GlobalAlloc> {
    pub(crate) use_meta_alloc: RefCell<bool>,

    last_addr: RefCell<usize>,

    pub(crate) addr2meta: Mutex<HashMap<usize, AddrMeta, DefaultHashBuilder, &'ma MetaAllocWrapper<MA>>>,
}

impl<'ma, MA: GlobalAlloc> ThreadMeta<'ma, MA> {
    pub fn new_in(last_addr: usize, allocator: &'ma MetaAllocWrapper<MA>) -> Self {
        ThreadMeta {
            use_meta_alloc: RefCell::new(false),
            addr2meta: Mutex::new(HashMap::new_in(allocator)),
            last_addr: RefCell::new(last_addr),
        }
    }



    // pub fn next_addr(&self, layout: Layout) -> *mut u8 {
    //     let mut last_addr = self.last_addr.borrow_mut();
    //
    //     let next_addr = match last_addr.checked_sub(layout.size()) {
    //         None => return ptr::null_mut(),
    //         Some(next_addr) => utils::align_down(next_addr, cmp::max(layout.align(), 4096)),
    //     } as *mut u8;
    //
    //     if let Err(err) = unsafe { mman_wrapper::mmap(next_addr, layout.size()) } {
    //         // TODO maybe handle mmap errors
    //         eprintln!("Error with code: {}, when calling mmap!", err);
    //         panic!("");
    //     }
    //
    //     *last_addr = next_addr as usize;
    //
    //     next_addr
    // }
}
