use std::ffi::{CStr, CString};
use std::os::raw::{c_int, c_char};
use std::os::unix::io::FromRawFd;
use std::path::Path;
use std::io::{Read, Result, Error, ErrorKind};
use std::fs::File;
use std::mem;
use super::*;


extern "C" {
    static F_SETFL_: c_int;
    static O_NONBLOCK_: c_int;

    fn fcntl(fd: c_int, cmd: c_int, arg: c_int) -> c_int;
    
    static IN_CREATE_: u32;
    static IN_DELETE_: u32;

    fn inotify_init() -> c_int;
    fn inotify_add_watch(fd: c_int, pathname: *const c_char, mask: u32) -> c_int;
    fn inotify_rm_watch(fd: c_int, wd: c_int) -> c_int;
}

#[repr(C)]
struct InotifyEvent {
    wd: c_int,
    mask: u32,
    cookie: u32,
    len: u32
}

const INOTIFY_EVENT_SIZE: usize = mem::size_of::<InotifyEvent>();
const MAX_NAME_LEN: usize = 2048;
type InotifyEventBuf = [u8; INOTIFY_EVENT_SIZE + MAX_NAME_LEN];

impl From<bool> for InotifyEventKind {
    fn from(created: bool) -> Self {
        if created {
            Self::Created
        } else {
            Self::Removed
        }
    }
}

impl Into<bool> for InotifyEventKind {
    fn into(self) -> bool {
        match self {
            Self::Created => true,
            Self::Removed => false
        }
    }
}


pub struct InotifyWatcher {
    fd: c_int,
    wd: c_int,
    file: File,

    buf: InotifyEventBuf,
}

impl InotifyWatcher {
    pub fn new<P>(path: P) -> Result<(Self, PollFd)>
    where P: AsRef<Path>
    {
        let fd = unsafe {
            inotify_init()
        };
        if fd < 0 {
            return Err(Error::new(ErrorKind::Other, "inotify_init"));
        }

        // set non-blocking mode so reads fail immediately if there isn't an
        // event available
        let fcntl_err = unsafe {
            fcntl(fd, F_SETFL_, O_NONBLOCK_)
        };
        if fcntl_err < 0 {
            return Err(Error::new(ErrorKind::Other, "fcntl"));
        }

        let file = unsafe {
            File::from_raw_fd(fd)
        };

        let path = path.as_ref();
        let path = CString::new(path.to_string_lossy().to_string()).unwrap();

        let wd = unsafe {
            inotify_add_watch(fd, path.as_ptr(), IN_CREATE_ | IN_DELETE_)
        };
        if wd < 0 {
            return Err(Error::new(ErrorKind::Other, "inotify_add_watch"));
        }

        let watcher = Self {
            fd,
            wd,
            file,
            buf: [0; INOTIFY_EVENT_SIZE + MAX_NAME_LEN],
        };

        Ok((watcher, PollFd::new(fd)))
    }

    pub fn next<'a>(&'a mut self) -> Option<(InotifyEventKind, &'a CStr)> {
        match self.file.read(&mut self.buf) {
            Ok(_) => unsafe {
                let event = mem::transmute::<*const u8, &InotifyEvent>(self.buf.as_ptr());

                let name = CStr::from_bytes_with_nul_unchecked(&self.buf[INOTIFY_EVENT_SIZE..]);

                Some(((event.mask == IN_CREATE_).into(), name))
            },
            Err(_) => None
        }
    }
}

impl Drop for InotifyWatcher {
    fn drop(&mut self) {
        unsafe {
            inotify_rm_watch(self.fd, self.wd);
        }
    }
}
