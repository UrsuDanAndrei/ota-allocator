use buddy_system_allocator::LockedHeap;
use core::alloc::{GlobalAlloc, Layout};
use libc_print::libc_eprintln;
use ota_allocator::{mman_wrapper, OtaAllocator};
use spin::Once;

// FIXME, figure out the proper ORDER value here, using 32 for now
const BUDDY_ALLOCATOR_ORDER: usize = 32;

pub type MetaAlloc = LockedHeap<{ BUDDY_ALLOCATOR_ORDER }>;
pub type CargoAlloc = LockedHeap<{ BUDDY_ALLOCATOR_ORDER }>;

pub struct AllocTestWrapper<'a> {
    ota_alloc: OtaAllocator<'a, MetaAlloc>,
    cargo_alloc: Once<CargoAlloc>,
    use_alloc: UsedAlloc,
    is_ota_init: bool,
}

enum UsedAlloc {
    Cargo,
    Ota,
}

impl<'a> AllocTestWrapper<'a> {
    pub const fn new() -> Self {
        Self {
            ota_alloc: OtaAllocator::new_in(MetaAlloc::new()),
            cargo_alloc: Once::new(),
            use_alloc: UsedAlloc::Cargo,
            is_ota_init: false,
        }
    }

    pub fn try_init(&'a mut self) {
        if !self.is_ota_init {
            self.is_ota_init = true;
            self.init_ota_alloc();
        }
    }

    pub fn use_ota_allocator(&mut self) {
        self.use_alloc = UsedAlloc::Ota;
    }

    pub fn use_cargo_allocator(&mut self) {
        self.use_alloc = UsedAlloc::Cargo;
    }

    fn init_ota_alloc(&'a mut self) {
        unsafe {
            init_buddy_allocator(
                self.ota_alloc.meta_alloc(),
                ota_allocator::META_ADDR_SPACE_START,
                ota_allocator::META_ADDR_SPACE_MAX_SIZE,
            );
        }

        self.ota_alloc.init();
    }
}

unsafe impl<'a> GlobalAlloc for AllocTestWrapper<'a> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match self.use_alloc {
            UsedAlloc::Cargo => self.cargo_alloc.call_once(new_cargo_alloc).alloc(layout),
            UsedAlloc::Ota => self.ota_alloc.alloc(layout),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        match self.use_alloc {
            UsedAlloc::Cargo => self.cargo_alloc.get().unwrap().dealloc(ptr, layout),
            UsedAlloc::Ota => self.ota_alloc.dealloc(ptr, layout),
        };
    }
}

fn new_cargo_alloc() -> CargoAlloc {
    let cargo_alloc = LockedHeap::new();

    unsafe {
        init_buddy_allocator(
            &cargo_alloc,
            ota_allocator::TEST_ADDR_SPACE_START,
            ota_allocator::TEST_ADDR_SPACE_MAX_SIZE,
        );
    }

    cargo_alloc
}

unsafe fn init_buddy_allocator(
    buddy_alloc: &LockedHeap<{ BUDDY_ALLOCATOR_ORDER }>,
    addr_space_start: usize,
    addr_space_size: usize,
) {
    if let Err(err) = mman_wrapper::mmap(addr_space_start, addr_space_size) {
        libc_eprintln!(
            "Error with code: {}, when calling mmap for allocating heap memory!",
            err
        );
        panic!("");
    }

    buddy_alloc.lock().init(addr_space_start, addr_space_size);
}
