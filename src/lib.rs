#![feature(libc)]
extern crate libc;

use libc::{c_int, c_long, size_t, ssize_t, off_t, c_char, c_void, mode_t};
use std::ffi::CStr;
use std::ffi::CString;
use std::str;
use std::mem;

static RTLD_NEXT: c_long = -1;

extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

// I'm not sure where the unsafe belongs, lol.
// Technically it's possible to do safe things within the bodies of these functions
// (like when you do RPC and stuff, I guess?)
// but the damn things are interposing for C functions anyway, so aren't they
// just, you know, completely unsafe?

#[no_mangle]
pub extern fn open_nocreate(pathname: *const c_char, flags: c_int) -> c_int {
    unsafe {
        let open_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("open").unwrap().as_ptr());
        let orig_open:unsafe fn(*const c_char, c_int) -> c_int = mem::transmute(open_);
        let slice = str::from_utf8(CStr::from_ptr(pathname).to_bytes()).unwrap();
        println!("Rust: open with {} and {}", slice, flags);
        orig_open(pathname, flags)
    }
}

#[no_mangle]
pub extern fn open_create(pathname: *const c_char, flags: c_int, mode: mode_t) -> c_int {
    unsafe {
        let open_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("open").unwrap().as_ptr());
        let orig_open:unsafe fn(*const c_char, c_int, mode_t) -> c_int = mem::transmute(open_);
        let slice = str::from_utf8(CStr::from_ptr(pathname).to_bytes()).unwrap();
        println!("Rust: open with {} and {} and {}", slice, flags, mode);
        orig_open(pathname, flags, mode)
    }
}

#[no_mangle]
pub extern fn close(fd: c_int) -> c_int {
    unsafe {
        let close_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("close").unwrap().as_ptr());
        let orig_close:unsafe fn(c_int) -> c_int = mem::transmute(close_);
        println!("Rust: close with {}", fd);
        orig_close(fd)
    }
}

#[no_mangle]
pub extern fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    unsafe {
        let read_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("read").unwrap().as_ptr());
        let orig_read:unsafe fn(c_int, *mut c_void, size_t) -> ssize_t = mem::transmute(read_);
        println!("Rust: read with {} and {}", fd, count);
        orig_read(fd, buf, count)
    }
}

// leaving this uncommented causes a segfault :O:O
// dammit, something in the rust stdlib calls write??? :(
//#[no_mangle]
//pub extern fn write(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    //unsafe {
        //let write_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("write").unwrap().as_ptr());
        //let orig_write:unsafe fn(c_int, *mut c_void, size_t) -> ssize_t = mem::transmute(write_);
        //println!("Rust: write with {} and {}", fd, count);
        //orig_write(fd, buf, count)
    //}
//}

#[no_mangle]
pub extern fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t {
    unsafe {
        let lseek_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("lseek").unwrap().as_ptr());
        let orig_lseek:unsafe fn(c_int, off_t, c_int) -> off_t = mem::transmute(lseek_);
        println!("Rust: lseek with {} and {} and {}", fd, offset, whence);
        orig_lseek(fd, offset, whence)
    }
}

#[no_mangle]
pub extern fn unlink(pathname: *const c_char) -> c_int {
    unsafe {
        let unlink_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("unlink").unwrap().as_ptr());
        let orig_unlink:unsafe fn(*const c_char) -> c_int = mem::transmute(unlink_);
        let slice = str::from_utf8(CStr::from_ptr(pathname).to_bytes()).unwrap();
        println!("Rust: unlink with {}", slice);
        orig_unlink(pathname)
    }
}

//__xstat will be left for another day :)

