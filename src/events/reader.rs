use std::io::{Read, Result};
use std::path::Path;
use super::*;



extern "C" {
    static EVIOCGRAB_: c_ulong;

    fn ioctl(fd: RawFd, request: c_ulong, ...) -> c_int;
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

#[derive(Debug)]
pub struct EventReader {
    file: Option<File>,
}

impl EventReader {
    pub fn new<P>(path: P, grab_file: bool) -> Result<(Self, PollFd)>
        where P: AsRef<Path>
    {
        let file = File::options()
            .read(true)
            .write(true)
            .open(path)?;

        if grab_file {
            set_grab(&file, true);
        }

        let fd = PollFd::new(file.as_raw_fd());

        let reader = Self {
            file: Some(file),
        };

        Ok((reader, fd))
    }

    pub fn next(&mut self) -> Option<InputEvent> {
        let mut input_event = InputEvent::default();

        let buf = unsafe {
            mem::transmute::<&mut InputEvent, &mut InputEventBuf>(&mut input_event)
        };

        self.file.as_mut()?.read_exact(buf).ok()?;

        Some(input_event)
    }

    // close underlying file descriptor
    pub fn close(&mut self) {
        self.file = None;
    }

    pub fn is_closed(&self) -> bool {
        self.file.is_none()
    }
}
