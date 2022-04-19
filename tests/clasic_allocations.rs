// #![feature(const_fn_trait_bound)]
// #![test_runner(test_runner)]
//
// mod commons;
//
// use commons::AllocatorTestWrapper;
// use ota_allocator::OtaAllocator;
// use alloc::{boxed::Box, vec::Vec};
//
// #[global_allocator]
// static mut ALLOCATOR: AllocatorTestWrapper<OtaAllocator> =
//     AllocatorTestWrapper::new(OtaAllocator::new());
//
// pub fn test_runner(tests: &[&dyn Fn()]) {
//     unsafe {
//         ALLOCATOR.allocator.init();
//         commons::test_runner(tests, &mut ALLOCATOR);
//     }
// }
//
// #[test_case]
// fn simple_box_allocation() {
//     let x = Box::new(2);
//     assert_eq!(*x, 2);
// }
//
// #[test_case]
// fn multiple_boxes_allocation() {
//     let x = Box::new(2);
//     assert_eq!(*x, 2);
//
//     let y = Box::new(3);
//     assert_eq!(*y, 3);
//
//     let z = Box::new(4);
//     assert_eq!(*z, 4);
// }
//
// #[test_case]
// fn intertwined_box_allocation() {
//     let x = Box::new(2);
//     let y = Box::new(3);
//
//     assert_eq!(*x, 2);
//     assert_eq!(*y, 3);
//
//     let z = Box::new(4);
//
//     {
//         let u = Box::new(5);
//         assert_eq!(*u, 5);
//     }
//
//     assert_eq!(*z, 4);
// }
//
// #[test_case]
// fn vec_allocation() {
//     let mut v = Vec::new();
//     let max_size = 256;
//
//     for i in 0..max_size {
//         v.push(2 * i);
//     }
//
//     assert_eq!(v.iter().sum::<usize>(), max_size * (max_size - 1));
// }
