use super::consts;

#[inline(always)]
pub fn get_addr_space(addr: usize) -> usize {
    addr & consts::ADDR_SPACE_MASK
}

#[inline(always)]
pub fn get_current_tid() -> usize {
    unsafe { libc::pthread_self() as usize }
}
