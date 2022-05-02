// This code is adapted after Jon Gjengset's implementation.
// The original can be found here: https://gist.github.com/jonhoo/7cfdfe581e5108b79c2a4e9fbde38de8
//
// This module can be replaced by alloc::rc::Rc once this issue is resolved
// https://github.com/rust-lang/rust/issues/42774

// TODO understand, double check, and refactor this file

use alloc::boxed::Box;
use core::alloc::Allocator;
use core::cell::Cell;
use core::marker::PhantomData;
use core::ops::Deref;
use core::ptr::NonNull;

struct RcAllocInner<T, A: Allocator + Clone> {
    value: T,
    refcount: Cell<usize>,
    alloc: A,
}

pub struct RcAlloc<T, A: Allocator + Clone> {
    inner: NonNull<RcAllocInner<T, A>>,
    _marker: PhantomData<RcAllocInner<T, A>>,
}

impl<T, A: Allocator + Clone> RcAlloc<T, A> {
    pub fn new_in(v: T, alloc: A) -> Self {
        let inner = Box::new_in(
            RcAllocInner {
                value: v,
                refcount: Cell::new(1),
                alloc: alloc.clone(),
            },
            alloc,
        );

        let (raw, _) = Box::into_raw_with_allocator(inner);

        RcAlloc {
            // SAFETY: Box does not give us a null pointer.
            inner: unsafe { NonNull::new_unchecked(raw) },
            _marker: PhantomData,
        }
    }
}

impl<T, A: Allocator + Clone> Deref for RcAlloc<T, A> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is only deallocated when the last Rc goes away.
        // we have an Rc, therefore the Box has not been deallocated, so deref is fine.
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T, A: Allocator + Clone> Clone for RcAlloc<T, A> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        RcAlloc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

// TODO: #[may_dangle]
impl<T, A: Allocator + Clone> Drop for RcAlloc<T, A> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            // SAFETY: we are the _only_ Rc left, and we are being dropped.
            // therefore, after us, there will be no Rc's, and no references to T.
            let _ =
                unsafe { Box::from_raw_in(self.inner.as_ptr(), self.inner.as_ref().alloc.clone()) };
        } else {
            // there are other Rcs, so don't drop the Box!
            inner.refcount.set(c - 1);
        }
    }
}
