pub mod consts;
pub mod mman_wrapper;
pub mod rc_alloc;

#[inline(always)]
pub fn get_addr_space(addr: usize) -> usize {
    addr & consts::ADDR_SPACE_MASK
}

#[inline(always)]
pub fn get_current_tid() -> usize {
    unsafe { libc::pthread_self() as usize }
}

#[inline(always)]
pub fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}

#[inline(always)]
pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
