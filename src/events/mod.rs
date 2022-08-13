use std::fs::File;
use std::os::raw::{c_int, c_short, c_ulong, c_ushort};
use std::os::unix::io::{RawFd, AsRawFd};
use std::mem;

mod reader;
mod codes;

pub use reader::*;
pub use codes::*;


extern "C" {
    static EVIOCGRAB_: c_ulong;
    fn ioctl(fd: RawFd, request: c_ulong, ...) -> c_int;
    fn poll(fds: *const PollFd, nfds: c_ulong, timeout: c_int) -> c_int;
}


fn set_grab(file: &File, enable_grab: bool) {
    const GRAB: c_int = 1;
    const UNGRAB: c_int = 0;

    unsafe {
        if enable_grab {
            ioctl(file.as_raw_fd(), EVIOCGRAB_, GRAB)
        } else {
            ioctl(file.as_raw_fd(), EVIOCGRAB_, UNGRAB)
        }
    };
}

/// POSIX pollfd struct
#[repr(C)]
#[derive(Debug)]
pub struct PollFd {
    fd: c_int,
    events: c_short,
    revents: c_short
}

/// Linux timeval struct
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Timeval {
    tv_sec: i64,
    tv_usec: i64
}

/// Linux input_event struct
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct InputEvent {
    timestamp: Timeval,
    event_type: c_ushort,
    code: c_ushort,
    value: c_int
}

type InputEventBuf = [u8; mem::size_of::<InputEvent>()];
