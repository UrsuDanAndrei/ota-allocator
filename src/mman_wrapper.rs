use libc;
use errno_no_std;

pub unsafe fn mmap(addr: *mut u8, size: usize) -> Result<(), i32> {

    let mmap_addr = libc::mmap(addr as *mut libc::c_void, size,
                               libc::PROT_READ | libc::PROT_WRITE,
                               libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_FIXED,
                               -1, 0);

    if mmap_addr == !0 as *mut libc::c_void { // because -1 causes E0600, !0 is used here instead
        Err(errno_no_std::errno().0)
    } else {
        // TODO, research if this assert is really necessary
        assert_eq!(mmap_addr as *mut u8, addr);
        Ok(())
    }
}

pub unsafe fn munmap(addr: *mut u8, size: usize) -> Result<(), i32> {
    // TODO maybe assert if addr is page aligned
    // TODO try as _ instead
    let err = libc::munmap(addr as *mut libc::c_void, size);
    if err == -1 {
        Err(errno_no_std::errno().0)
    } else {
        Ok(())
    }
}
