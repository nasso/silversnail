pub mod color;
pub mod framebuffer;

#[no_mangle]
pub extern "C" fn alloc_framebuffer(w: u32, h: u32) -> *mut u8 {
    unsafe { memalloc::allocate(w as usize * h as usize * 4) }
}

#[no_mangle]
pub extern "C" fn free_framebuffer(w: u32, h: u32, ptr: *mut u8) {
    unsafe {
        memalloc::deallocate(ptr, w as usize * h as usize * 4);
    };
}
