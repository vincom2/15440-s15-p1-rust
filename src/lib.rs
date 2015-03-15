#![allow(unused_imports)]

#![feature(libc)]
extern crate libc;
#[macro_use(lazy_static)]
extern crate lazy_static;

use data::{c_int, c_long, size_t, c_char, c_void,
           ssize_t, off_t, mode_t, dev_t, ino_t, nlink_t,
           uid_t, gid_t, blksize_t, blkcnt_t, time_t,
           struct_stat_t, struct_dirtreenode_t};

mod data;
mod ffi;

#[no_mangle]
pub extern fn open_nocreate(pathname: *const c_char, flags: c_int) -> c_int {
    unsafe { ffi::open_nocreate(pathname, flags) }
}

#[no_mangle]
pub extern fn open_create(pathname: *const c_char, flags: c_int, mode: mode_t) -> c_int {
    unsafe { ffi::open_create(pathname, flags, mode) }
}

#[no_mangle]
pub extern fn close(fd: c_int) -> c_int {
    unsafe { ffi::close(fd) }
}

#[no_mangle]
pub extern fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    unsafe { ffi::read(fd, buf, count) }
}

#[no_mangle]
pub extern fn write(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    unsafe { ffi::write(fd, buf, count) }
}

#[no_mangle]
pub extern fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t {
    unsafe { ffi::lseek(fd, offset, whence) }
}

#[no_mangle]
pub extern fn unlink(pathname: *const c_char) -> c_int {
    unsafe { ffi::unlink(pathname) }
}

#[no_mangle]
pub extern fn __xstat(ver: c_int, path: *const c_char, buf: *mut struct_stat_t) -> c_int {
    unsafe { ffi::__xstat(ver, path, buf) }
}

#[no_mangle]
pub extern fn getdirentries(fd: c_int, buf: *mut c_char, nbytes: size_t, basep: *mut off_t) -> ssize_t {
    unsafe { ffi::getdirentries(fd, buf, nbytes, basep) }
}

#[no_mangle]
pub extern fn getdirtree(path: *const c_char) -> *mut struct_dirtreenode_t {
    unsafe { ffi::getdirtree(path) }
}

