use core::alloc::Layout;
use super::addr_meta::AddrMeta;
use crate::mman_wrapper;
use core::cell::RefCell;
use core::cmp;
use core::ptr;
use hashbrown::HashMap;
use spin::Mutex;
use crate::utils;
use libc_print::std_name::*;

pub struct ThreadMeta {
    pub(crate) use_meta_alloc: RefCell<bool>,

    last_addr: RefCell<usize>,

    // TODO make your own wrapper or find a better one instead of using Option
    //  we need option here because HashMap::new can't be called from a constant function
    pub(crate) addr2meta: Mutex<Option<HashMap<*mut u8, AddrMeta>>>,
}

impl ThreadMeta {
    pub const fn new(last_addr: usize) -> Self {
        ThreadMeta {
            use_meta_alloc: RefCell::new(false),
            addr2meta: Mutex::new(None),
            last_addr: RefCell::new(last_addr),
        }
    }

    pub fn next_addr(&self, layout: Layout) -> *mut u8 {
        let mut last_addr = self.last_addr.borrow_mut();

        let next_addr = match last_addr.checked_sub(layout.size()) {
            None => return ptr::null_mut(),
            Some(next_addr) => utils::align_down(next_addr, cmp::max(layout.align(), 4096)),
        } as *mut u8;

        if let Err(err) = unsafe { mman_wrapper::mmap(next_addr, layout.size()) } {
            // TODO maybe handle mmap errors
            eprintln!("Error with code: {}, when calling mmap!", err);
            panic!("");
        }

        *last_addr = next_addr as usize;

        next_addr
    }
}
