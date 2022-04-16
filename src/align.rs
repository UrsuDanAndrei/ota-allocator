#[inline(always)]
pub fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}

#[inline(always)]
pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
