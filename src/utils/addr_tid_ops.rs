// FIXME, this functions need adjustments, BIG adjustments

use super::consts;

#[inline(always)]
pub const fn get_last_addr_for_tid(tid: usize) -> usize {
    0xFFFF_8000_0000_0000 + (tid << 32)
}

#[inline(always)]
pub fn get_tid_for_addr(addr: usize) -> usize {
    ((addr - 0xFFFF_8000_0000_0000) >> 32) + 1
}

#[inline(always)]
pub fn get_current_tid() -> usize {
    unsafe {
        libc::gettid() as usize
    }
}

#[inline(always)]
pub fn is_meta_addr(addr: *mut u8) -> bool {
    addr as usize & consts::META_CHECK_MASK == consts::META_ADDR_START
}
