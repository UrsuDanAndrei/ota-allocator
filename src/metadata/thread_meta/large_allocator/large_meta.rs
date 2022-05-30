use crate::metadata::thread_meta::large_allocator::Page;
use crate::utils::rc_alloc::RcAlloc;
use core::alloc::Allocator;

pub struct LargeMeta<'a, A: Allocator> {
    // this is the requested size, the allocated size might be larger due to alignment
    pub size: usize,
    pub first_page: RcAlloc<Page, &'a A>,
    pub last_page: RcAlloc<Page, &'a A>,
}

impl<'a, A: Allocator> LargeMeta<'a, A> {
    pub fn new_in(
        size: usize,
        first_page: RcAlloc<Page, &'a A>,
        second_page: RcAlloc<Page, &'a A>,
    ) -> Self {
        LargeMeta {
            size,
            first_page,
            last_page: second_page,
        }
    }
}
