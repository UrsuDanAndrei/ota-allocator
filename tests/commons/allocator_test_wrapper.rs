// use core::alloc::{GlobalAlloc, Layout};
// use libc;
//
// pub struct AllocatorTestWrapper<T: GlobalAlloc> {
//     pub allocator: T,
//     pub use_wrapped_allocator: bool,
// }
//
// impl<T: GlobalAlloc> AllocatorTestWrapper<T> {
//     pub const fn new(allocator: T) -> AllocatorTestWrapper<T> {
//         AllocatorTestWrapper {
//             allocator,
//             use_wrapped_allocator: false,
//         }
//     }
// }
//
// unsafe impl<T: GlobalAlloc> GlobalAlloc for AllocatorTestWrapper<T> {
//     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//         if self.use_wrapped_allocator {
//             self.allocator.alloc(layout)
//         } else {
//             libc::malloc(layout.size() as libc::size_t) as *mut u8
//         }
//     }
//
//     unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
//         if self.use_wrapped_allocator {
//             self.allocator.dealloc(ptr, layout)
//         } else {
//             libc::free(ptr as *mut libc::c_void)
//         }
//     }
// }
