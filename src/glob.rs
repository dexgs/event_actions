use std::ffi::{CStr, CString};
use std::io::Result;
use std::mem;
use std::process::{Command, Stdio};
use std::ptr;
use std::os::raw::{c_char, c_int};


mod ffi {
    use std::os::raw::{c_char, c_int, c_void};
    use super::PGlob;

    extern "C" {
        pub static GLOB_NOSORT_: c_int;
        pub static GLOB_TILDE_: c_int;

        pub fn glob(pattern: *const c_char, flags: c_int, errfunc: *const (), pglob: *const PGlob) -> c_int;
        pub fn globfree(pglob: *const PGlob) -> c_void;
    }
}

#[repr(C)]
struct PGlob {
    gl_pathc: usize,
    gl_pathv: *const *const c_char,
    gl_offs: usize
}


pub fn glob(pattern: &str) -> Vec<String> {
    let mut pglob = PGlob {
        gl_pathc: 0,
        gl_pathv: ptr::null(),
        gl_offs: 0
    };

    let err = unsafe {
        ffi::glob(
            CString::new(pattern)
                .expect("Allocating C representation of glob pattern")
                .into_raw(),
            ffi::GLOB_NOSORT_ | ffi::GLOB_TILDE_,
            ptr::null(),
            &mut pglob)
    };

    let mut paths = Vec::new();

    if err == 0 {
        let mut gl_pathv = pglob.gl_pathv;

        for _ in 0..pglob.gl_pathc {
            if let Ok(path) = unsafe { CStr::from_ptr(*gl_pathv) }.to_str() {
                paths.push(path.to_string());
            }

            gl_pathv = gl_pathv.wrapping_offset(1);
        }
    }

    unsafe {
        ffi::globfree(&mut pglob);
    }

    paths
}
