#![allow(non_upper_case_globals)]

pub use libc::{c_int, c_long, size_t, c_char, c_void,
           ssize_t, off_t, mode_t, dev_t, ino_t, nlink_t,
           uid_t, gid_t, blksize_t, blkcnt_t, time_t};
use std::default::Default;
use std::ffi::CString;
use std::mem;
use std::ptr;

extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
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

pub static RTLD_NEXT: c_long = -1;
// call dlsym() to initialise C function pointers to all the glibc versions
// UNSAFE
lazy_static! {
    pub static ref orig_open_nocreate:unsafe fn(*const c_char, c_int) -> c_int = {
        let open_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("open").unwrap().as_ptr());
        mem::transmute(open_)
    };

    pub static ref orig_open_create:unsafe fn(*const c_char, c_int, mode_t) -> c_int = {
        let open_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("open").unwrap().as_ptr());
        mem::transmute(open_)
    };

    pub static ref orig_close:unsafe fn(c_int) -> c_int = {
        let close_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("close").unwrap().as_ptr());
        mem::transmute(close_)
    };

    pub static ref orig_read:unsafe fn(c_int, *mut c_void, size_t) -> ssize_t = {
        let read_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("read").unwrap().as_ptr());
        mem::transmute(read_)
    };

    pub static ref orig_write:unsafe fn(c_int, *mut c_void, size_t) -> ssize_t = {
        let write_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("write").unwrap().as_ptr());
        mem::transmute(write_)
    };

    pub static ref orig_lseek:unsafe fn(c_int, off_t, c_int) -> off_t = {
        let lseek_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("lseek").unwrap().as_ptr());
        mem::transmute(lseek_)
    };

    pub static ref orig_unlink:unsafe fn(*const c_char) -> c_int = {
        let unlink_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("unlink").unwrap().as_ptr());
        mem::transmute(unlink_)
    };

    pub static ref orig_stat:unsafe fn(c_int, *const c_char, *mut struct_stat_t) -> c_int = {
        let stat_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("__xstat").unwrap().as_ptr());
        mem::transmute(stat_)
    };

    pub static ref orig_gde:unsafe fn(c_int, *mut c_char, size_t, *mut off_t) -> ssize_t = {
        let gde_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("getdirentries").unwrap().as_ptr());
        mem::transmute(gde_)
    };

    pub static ref orig_gdt:unsafe fn(*const c_char) -> *mut struct_dirtreenode_t = {
        let gdt_ = dlsym(RTLD_NEXT as *mut c_void, CString::new("getdirtree").unwrap().as_ptr());
        mem::transmute(gdt_)
    };
}

