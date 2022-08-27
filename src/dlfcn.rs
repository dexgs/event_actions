use std::ffi::{c_void, CString};
use std::os::raw::{c_int, c_char};
use std::env;
use std::path::Path;
use crate::events::*;
use std::mem;


#[link(name = "dl", kind = "dylib")]
extern "C" {
    static RTLD_NOW_: c_int;

    fn dlopen(filename: *const c_char, flags: c_int) -> *const c_void;
    fn dlsym(handle: *const c_void, symbol: *const c_char) -> *const c_void;
}

pub struct Object {
    pub device_update: extern "C" fn (is_created: bool, name: *const c_char, index: usize) -> c_int,
    pub handle_input: extern "C" fn (input_event: *const InputEvent, index: usize)
}

impl Object {
    pub fn new<P>(path: P) -> Self
    where P: AsRef<Path>
    {
        let path = path.as_ref();
        let path = if path.is_absolute() {
            path.to_owned()
        } else {
            env::current_dir().expect("Getting current dir").join(path)
        };

        let filename = CString::new(path.to_string_lossy().as_bytes()).unwrap();

        let dl = unsafe {
            dlopen(filename.as_ptr(), RTLD_NOW_)
        };

        Self {
            device_update: unsafe {
                let symbol = CString::new("device_update").unwrap();
                mem::transmute(dlsym(dl, symbol.as_ptr()))
            },
            handle_input: unsafe {
                let symbol = CString::new("handle_input").unwrap();
                mem::transmute(dlsym(dl, symbol.as_ptr()))
            }
        }
    }
}
