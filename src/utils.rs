pub mod align;
pub mod consts;
pub mod get_tid;

// reexports
pub use align::*;
pub use get_tid::*;

// TODO
pub fn is_meta_addr(addr: *mut u8) -> bool {
    true
}
