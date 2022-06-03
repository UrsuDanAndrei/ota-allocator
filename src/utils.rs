use core::panic::PanicInfo;
use libc_print::libc_eprintln;

pub mod consts;
pub mod mman_wrapper;
pub mod rc_alloc;

#[inline(always)]
pub fn get_addr_space(addr: usize) -> usize {
    addr & consts::ADDR_SPACE_MASK
}

// TODO move this function in separate modules
#[inline(always)]
pub fn get_current_tid() -> usize {
    unsafe { libc::pthread_self() as usize }
}

#[inline(always)]
pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

#[inline(always)]
pub fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}

#[inline(always)]
pub fn is_small_addr(addr: usize) -> bool {
    addr & consts::LARGE_ADDR_SPACE_MASK == 0
}

#[inline(always)]
pub fn get_address_space_for_id(address_space_id: usize) -> usize {
    address_space_id << consts::ADDR_SPACE_BIT
}

#[inline(always)]
pub fn get_address_space_id(addr: usize) -> usize {
    get_addr_space(addr) >> consts::ADDR_SPACE_BIT
}

// pub fn panic_hook(panic_info: &PanicInfo) -> ! {
//     libc_eprintln!("PANICKED!");
//
//     match panic_info.message() {
//         None => { libc_eprintln!("message: none") }
//         Some(message) => { libc_eprintln!("message: {}", message) }
//     };
//
//     match panic_info.location() {
//         None => { libc_eprintln!("location: none") }
//         Some(location) => { libc_eprintln!("location: file: {}, line: {}", location.file(), location.line()) }
//     }
//
//     loop {}
// }
