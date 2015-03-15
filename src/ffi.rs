#![allow(unused_variables)]

use data::{c_int, c_long, size_t, c_char, c_void,
           ssize_t, off_t, mode_t, dev_t, ino_t, nlink_t,
           uid_t, gid_t, blksize_t, blkcnt_t, time_t};
use std::ffi::CStr;
use std::str;

use data::{RTLD_NEXT, struct_stat_t, struct_dirtreenode_t,
           orig_open_nocreate, orig_open_create, orig_close,
           orig_read, orig_write, orig_lseek,
           orig_unlink, orig_stat, orig_gde, orig_gdt};


// I'm not sure where the unsafe belongs, lol.
// Technically it's possible to do safe things within the bodies of these functions
// (like when you do RPC and stuff, I guess?)
// but the damn things are interposing for C functions anyway, so aren't they
// just, you know, completely unsafe?

pub unsafe fn open_nocreate(pathname: *const c_char, flags: c_int) -> c_int {
    let slice = str::from_utf8(CStr::from_ptr(pathname).to_bytes()).unwrap();
    //println!("Rust: open with {} and {}", slice, flags);
    orig_open_nocreate(pathname, flags)
}

pub unsafe fn open_create(pathname: *const c_char, flags: c_int, mode: mode_t) -> c_int {
    let slice = str::from_utf8(CStr::from_ptr(pathname).to_bytes()).unwrap();
    //println!("Rust: open with {} and {} and {}", slice, flags, mode);
    orig_open_create(pathname, flags, mode)
}

pub unsafe fn close(fd: c_int) -> c_int {
    //println!("Rust: close with {}", fd);
    orig_close(fd)
}

pub unsafe fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    //println!("Rust: read with {} and {}", fd, count);
    orig_read(fd, buf, count)
}

// can't call println!() from within this :(
pub unsafe fn write(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    //println!("Rust: write with {} and {}", fd, count);
    orig_write(fd, buf, count)
}

pub unsafe fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t {
    //println!("Rust: lseek with {} and {} and {}", fd, offset, whence);
    orig_lseek(fd, offset, whence)
}

pub unsafe fn unlink(pathname: *const c_char) -> c_int {
    let slice = str::from_utf8(CStr::from_ptr(pathname).to_bytes()).unwrap();
    //println!("Rust: unlink with {}", slice);
    orig_unlink(pathname)
}

pub unsafe fn __xstat(ver: c_int, path: *const c_char, buf: *mut struct_stat_t) -> c_int {
    let slice = str::from_utf8(CStr::from_ptr(path).to_bytes()).unwrap();
    //println!("Rust: __xstat with {}", slice);
    orig_stat(ver, path, buf)
}

pub unsafe fn getdirentries(fd: c_int, buf: *mut c_char, nbytes: size_t, basep: *mut off_t) -> ssize_t {
    //println!("Rust: getdirentries with {} and {}", fd, *basep);
    orig_gde(fd, buf, nbytes, basep)
}

// srsly tho, how would one expose a safe interface to this
// where the compiler frees the *mut struct_dirtreenode_t?
// impl Drop or something I really don't know anything about?
pub unsafe fn getdirtree(path: *const c_char) -> *mut struct_dirtreenode_t {
    let slice = str::from_utf8(CStr::from_ptr(path).to_bytes()).unwrap();
    //println!("Rust: getdirtree with {}", slice);
    orig_gdt(path)
}

