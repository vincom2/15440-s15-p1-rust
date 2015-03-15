#![feature(libc)]

#![allow(unused_variables)]
#![allow(non_upper_case_globals)]

extern crate libc;
#[macro_use(lazy_static)]
extern crate lazy_static;

use libc::{c_int, c_long, size_t, c_char, c_void,
           ssize_t, off_t, mode_t, dev_t, ino_t, nlink_t,
           uid_t, gid_t, blksize_t, blkcnt_t, time_t};
use std::ffi::CStr;
use std::ffi::CString;
use std::str;
use std::mem;
use std::ptr;
use std::default::Default;

extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

static RTLD_NEXT: c_long = -1;
// call dlsym() to initialise C function pointers to all the glibc versions
// UNSAFE
lazy_static! {
    static ref orig_open_nocreate:unsafe fn(*const c_char, c_int) -> c_int = {
        let open_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("open").unwrap().as_ptr());
        mem::transmute(open_)
    };

    static ref orig_open_create:unsafe fn(*const c_char, c_int, mode_t) -> c_int = {
        let open_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("open").unwrap().as_ptr());
        mem::transmute(open_)
    };

    static ref orig_close:unsafe fn(c_int) -> c_int = {
        let close_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("close").unwrap().as_ptr());
        mem::transmute(close_)
    };

    static ref orig_read:unsafe fn(c_int, *mut c_void, size_t) -> ssize_t = {
        let read_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("read").unwrap().as_ptr());
        mem::transmute(read_)
    };

    static ref orig_write:unsafe fn(c_int, *mut c_void, size_t) -> ssize_t = {
        let write_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("write").unwrap().as_ptr());
        mem::transmute(write_)
    };

    static ref orig_lseek:unsafe fn(c_int, off_t, c_int) -> off_t = {
        let lseek_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("lseek").unwrap().as_ptr());
        mem::transmute(lseek_)
    };

    static ref orig_unlink:unsafe fn(*const c_char) -> c_int = {
        let unlink_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("unlink").unwrap().as_ptr());
        mem::transmute(unlink_)
    };

    static ref orig_stat:unsafe fn(c_int, *const c_char, *mut struct_stat_t) -> c_int = {
        let stat_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("__xstat").unwrap().as_ptr());
        mem::transmute(stat_)
    };

    static ref orig_gde:unsafe fn(c_int, *mut c_char, size_t, *mut off_t) -> ssize_t = {
        let gde_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("getdirentries").unwrap().as_ptr());
        mem::transmute(gde_)
    };

    static ref orig_gdt:unsafe fn(*const c_char) -> *mut struct_dirtreenode_t = {
        let gdt_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("getdirtree").unwrap().as_ptr());
        mem::transmute(gdt_)
    };
}

#[repr(C)]
pub struct struct_stat_t {
    pub st_dev: dev_t,
    pub st_ino: ino_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_size: off_t,
    pub st_blksize: blksize_t,
    pub st_blocks: blkcnt_t,
    pub st_atime: time_t,
    pub st_mtime: time_t,
    pub st_ctime: time_t,
}

impl Default for struct_stat_t {
    fn default() -> struct_stat_t {
        struct_stat_t {
            st_dev: 0,
            st_ino: 0,
            st_mode: 0,
            st_nlink: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atime: 0,
            st_mtime: 0,
            st_ctime: 0,
        }
    }
}

#[repr(C)]
pub struct struct_dirtreenode_t {
    pub name: *mut c_char,
    pub num_subdirs: c_int,
    pub subdirs: *mut *mut struct_dirtreenode_t,
}

impl Default for struct_dirtreenode_t {
    fn default() -> struct_dirtreenode_t {
        struct_dirtreenode_t {
            name: ptr::null_mut(),
            num_subdirs: 0,
            subdirs: ptr::null_mut(),
        }
    }
}

// I'm not sure where the unsafe belongs, lol.
// Technically it's possible to do safe things within the bodies of these functions
// (like when you do RPC and stuff, I guess?)
// but the damn things are interposing for C functions anyway, so aren't they
// just, you know, completely unsafe?

#[no_mangle]
pub extern fn open_nocreate(pathname: *const c_char, flags: c_int) -> c_int {
    unsafe {
        let slice = str::from_utf8(CStr::from_ptr(pathname).to_bytes()).unwrap();
        //println!("Rust: open with {} and {}", slice, flags);
        orig_open_nocreate(pathname, flags)
    }
}

#[no_mangle]
pub extern fn open_create(pathname: *const c_char, flags: c_int, mode: mode_t) -> c_int {
    unsafe {
        let slice = str::from_utf8(CStr::from_ptr(pathname).to_bytes()).unwrap();
        //println!("Rust: open with {} and {} and {}", slice, flags, mode);
        orig_open_create(pathname, flags, mode)
    }
}

#[no_mangle]
pub extern fn close(fd: c_int) -> c_int {
    unsafe {
        //println!("Rust: close with {}", fd);
        orig_close(fd)
    }
}

#[no_mangle]
pub extern fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    unsafe {
        //println!("Rust: read with {} and {}", fd, count);
        orig_read(fd, buf, count)
    }
}

// can't call println!() from within this :(
#[no_mangle]
pub extern fn write(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    unsafe {
        //println!("Rust: write with {} and {}", fd, count);
        orig_write(fd, buf, count)
    }
}

#[no_mangle]
pub extern fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t {
    unsafe {
        //println!("Rust: lseek with {} and {} and {}", fd, offset, whence);
        orig_lseek(fd, offset, whence)
    }
}

#[no_mangle]
pub extern fn unlink(pathname: *const c_char) -> c_int {
    unsafe {
        let slice = str::from_utf8(CStr::from_ptr(pathname).to_bytes()).unwrap();
        //println!("Rust: unlink with {}", slice);
        orig_unlink(pathname)
    }
}

#[no_mangle]
pub extern fn __xstat(ver: c_int, path: *const c_char, buf: *mut struct_stat_t) -> c_int {
    unsafe {
        let slice = str::from_utf8(CStr::from_ptr(path).to_bytes()).unwrap();
        //println!("Rust: __xstat with {}", slice);
        orig_stat(ver, path, buf)
    }
}

#[no_mangle]
pub extern fn getdirentries(fd: c_int, buf: *mut c_char, nbytes: size_t, basep: *mut off_t) -> ssize_t {
    unsafe {
        //println!("Rust: getdirentries with {} and {}", fd, *basep);
        orig_gde(fd, buf, nbytes, basep)
    }
}

// srsly tho, how would one expose a safe interface to this
// where the compiler frees the *mut struct_dirtreenode_t?
// impl Drop or something I really don't know anything about?
#[no_mangle]
pub extern fn getdirtree(path: *const c_char) -> *mut struct_dirtreenode_t {
    unsafe {
        let slice = str::from_utf8(CStr::from_ptr(path).to_bytes()).unwrap();
        //println!("Rust: getdirtree with {}", slice);
        orig_gdt(path)
    }
}

