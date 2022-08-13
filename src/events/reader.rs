use std::io::{Read, Result};
use std::path::Path;
use super::*;


pub struct EventReader {
    file: File,
    grab_file: bool,
    has_event: bool
}

impl EventReader {
    pub fn new<P>(path: P, grab_file: bool) -> Result<Self>
        where P: AsRef<Path>
    {
        let file = File::options()
            .read(true)
            .write(true)
            .open(path)?;

        if grab_file {
            set_grab(&file, true);
        }

        Ok(Self {
            file,
            grab_file,
            has_event: false
        })
    }

    pub fn next(&mut self) -> Result<InputEvent> {
        let mut input_event = InputEvent::default();

        let buf = unsafe {
            mem::transmute::<&mut InputEvent, &mut InputEventBuf>(&mut input_event)
        };

        self.file.read_exact(buf)?;
        Ok(input_event)
    }

    pub fn get_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }

    pub fn has_event(&self) -> bool {
        self.has_event
    }

    pub fn poll<'a>(readers: &'a mut [EventReader], pollfds: &'a mut [PollFd]) {
        const POLL_TIMEOUT_MS: c_int = 5_000;

        assert_eq!(readers.len(), pollfds.len());

        let poll_result = unsafe {
            poll(
                pollfds.as_ptr(),
                pollfds.len() as c_ulong,
                POLL_TIMEOUT_MS)
        };

        for (pollfd, reader) in pollfds.iter().zip(readers.iter_mut()) {
            if pollfd.revents != 0 {
                reader.has_event = true;
            } else {
                reader.has_event = false;
            }
        }
    }
}

impl Drop for EventReader {
    fn drop(&mut self) {
        if self.grab_file {
            set_grab(&self.file, false);
        }
    }
}
