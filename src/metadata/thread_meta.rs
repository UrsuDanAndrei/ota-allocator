use super::addr_meta::AddrMeta;
use crate::align;
use crate::mman_wrapper;
use core::cell::RefCell;
use core::cmp;
use core::ptr;
use hashbrown::HashMap;
use spin::Mutex;

pub struct ThreadMeta {
    use_meta_alloc: RefCell<bool>,

    last_addr: RefCell<usize>,

    // we need option here because HashMap::new can't be called from a constant function
    addr2meta: Mutex<Option<HashMap<*mut u8, AddrMeta>>>,
}

impl ThreadMeta {
    pub const fn new() -> Self {
        ThreadMeta {
            use_meta_alloc: RefCell(false),
            addr2meta: Mutex::new(None),
            last_addr: RefCell::new(0x00006FFF_00000000),
        }
    }

    pub fn next_addr(&self) -> *mut u8 {
        let last_addr = self.last_addr.borrow_mut();

        let next_addr = match last_addr.checked_sub(layout.size()) {
            None => return ptr::null_mut(),
            Some(next_addr) => align::align_down(next_addr, cmp::max(layout.align(), 4096)),
        } as *mut u8;

        if let Err(_) = unsafe { mman_wrapper::mmap(next_addr, layout.size()) } {
            // TODO maybe handle mmap errors
            return ptr::null_mut();
        }

        *last_addr = next_addr as usize;

        next_addr
    }
}
