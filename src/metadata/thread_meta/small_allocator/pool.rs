use crate::utils::align_up;
use crate::{consts, utils::mman_wrapper};
use libc_print::libc_eprintln;

// TODO maybe make POOL_SIZE a const type parameter
pub struct Pool {
    start_addr: usize,
    end_addr: usize, // open endpoint
    next_addr: usize,
}

impl Pool {
    pub fn new(start_addr: usize) -> Self {
        Pool {
            start_addr,
            end_addr: start_addr + consts::POOL_SIZE,
            next_addr: start_addr,
        }
    }

    // calling this method after it returns None will provide invalid results
    pub fn next_addr(&mut self, size: usize) -> Option<usize> {
        let addr = align_up(self.next_addr, consts::STANDARD_ALIGN);

        self.next_addr = addr + size;

        if self.next_addr > self.end_addr {
            None
        } else {
            Some(addr)
        }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        // TODO find a way to test that this is actually called
        if let Err(err) = unsafe { mman_wrapper::munmap(self.start_addr, consts::POOL_SIZE) } {
            // TODO maybe handle mmap errors
            libc_eprintln!(
                "Error with code: {}, when calling munmap! addr: {}, size: POOL_SIZE",
                err,
                self.start_addr
            );
            panic!("");
        }
    }
}
