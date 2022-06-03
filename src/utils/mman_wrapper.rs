use libc;

// DON'T ERASE THIS LINE
use errno;

pub unsafe fn mmap(addr: usize, size: usize) -> Result<(), i32> {
    let mmap_addr = libc::mmap(
        addr as *mut libc::c_void,
        size,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_FIXED,
        -1,
        0,
    );

    // because -1 causes E0600, !0 is used here instead
    if mmap_addr == !0 as *mut libc::c_void {
        Err(*libc::__errno_location())
    } else {
        Ok(())
    }
}

pub unsafe fn munmap(addr: usize, size: usize) -> Result<(), i32> {
    let err = libc::munmap(addr as *mut libc::c_void, size);
    if err == -1 {
        Err(*libc::__errno_location())
    } else {
        Ok(())
    }
}
