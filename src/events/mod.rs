use std::fs::File;
use std::os::raw::{c_int, c_short, c_ulong, c_ushort};
use std::os::unix::io::{RawFd, AsRawFd};
use std::mem;

mod reader;
mod inotify;
mod poll;

pub use reader::*;
pub use inotify::*;
pub use poll::*;

// POSIX pollfd struct
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PollFd {
    fd: c_int,
    events: c_short,
    revents: c_short
}

impl PollFd {
    fn new(fd: c_int) -> Self {
        Self {
            fd,
            // listen for every event type
            events: 0b111_1111_1111_1111,
            revents: 0
        }
    }

    fn has_event(&self) -> bool {
        return self.revents != 0;
    }
}

// Linux timeval struct
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Timeval {
    tv_sec: i64,
    tv_usec: i64
}

// Linux input_event struct
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct InputEvent {
    timestamp: Timeval,
    event_type: c_ushort,
    code: c_ushort,
    value: c_int
}

type InputEventBuf = [u8; mem::size_of::<InputEvent>()];


// enum representing the types of inotify events we care about:
// when files are created and when they are removed.
#[derive(Debug, Copy, Clone)]
pub enum InotifyEventKind {
    Created,
    Removed
}
