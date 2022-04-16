// The GlobalAlloc trait can't be implemented for Mutex<OtaAllocator>, due to error[E0117]
// so we must use this wrapper

use spin::{Mutex, MutexGuard};

pub struct Spinlock<T> {
    mtx: Mutex<T>,
}

impl<T> Spinlock<T> {
    pub const fn new(locked_obj: T) -> Spinlock<T> {
        Spinlock {
            mtx: Mutex::new(locked_obj),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        self.mtx.lock()
    }
}
