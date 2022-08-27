use std::ffi::CStr;
use std::collections::{HashMap, HashSet};
use std::io::Result;
use std::path::{PathBuf, Path};
use super::*;


extern "C" {
    fn poll(fds: *const PollFd, nfds: c_ulong, timeout: c_int) -> c_int;
}

pub struct EventPoller {
    watcher: InotifyWatcher,
    watcher_fd: PollFd,

    readers: Vec<EventReader>,
    fds: Vec<PollFd>,

    free_indices: HashSet<usize>,
    names_to_indices: HashMap<String, usize>,

    path: PathBuf,
    null_device: File
}

impl EventPoller {
    pub fn new<P>(path: P) -> Result<Self>
    where P: AsRef<Path>
    {
        const BASE_CAPACITY: usize = 10;

        let path = path.as_ref();

        let (watcher, watcher_fd) = InotifyWatcher::new(path)?;

        Ok(Self {
            watcher,
            watcher_fd,
            readers: Vec::with_capacity(BASE_CAPACITY),
            fds: Vec::with_capacity(BASE_CAPACITY),
            free_indices: HashSet::with_capacity(BASE_CAPACITY),
            names_to_indices: HashMap::with_capacity(BASE_CAPACITY),
            path: path.to_owned(),
            null_device: File::open("/dev/null")?
        })
    }

    pub fn poll<'a>(&'a mut self) -> EventIterator<'a> {
        const POLL_TIMEOUT_MS: c_int = 5_000;

        assert_eq!(self.readers.len(), self.fds.len());

        self.fds.push(self.watcher_fd);

        unsafe {
            poll(
                self.fds.as_ptr(),
                self.fds.len() as c_ulong,
                POLL_TIMEOUT_MS);
        }

        self.watcher_fd = self.fds.pop().unwrap();

        EventIterator {
            watcher: &mut self.watcher,
            readers: &mut self.readers,
            fds: &self.fds,
            index: 0
        }
    }

    pub fn next_free_index(&self) -> Option<usize> {
        self.free_indices.iter().next().copied()
    }

    pub fn num_readers(&self) -> usize {
        self.readers.len()
    }

    pub fn add_reader(&mut self, name: &str, grab: bool) -> Option<usize> {
        if Path::new(name).is_absolute() {
            // `name` should be a file *name*, not a full path
            return None;
        }

        self.path.push(name);
        let new_reader = EventReader::new(&self.path, grab);
        self.path.pop();

        if let Ok((reader, fd)) = new_reader {
            match self.names_to_indices.get(name).copied() {
                Some(i) => {
                    self.fds[i] = fd;
                    self.readers[i] = reader;

                    Some(i)
                },
                None => match self.next_free_index() {
                    Some(i) => {
                        self.free_indices.remove(&i);

                        self.fds[i] = fd;
                        self.readers[i] = reader;
                        self.names_to_indices.insert(name.to_string(), i);

                        Some(i)
                    },
                    None => {
                        self.fds.push(fd);
                        self.readers.push(reader);
                        self.names_to_indices.insert(
                            name.to_string(), self.readers.len() - 1);

                        Some(self.readers.len() - 1)
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn remove_reader(&mut self, name: &str) -> Option<usize> {
        if let Some(i) = self.names_to_indices.get(name).copied() {
            self.names_to_indices.remove(name);

            // if the removed reader is the last element, then we don't mark
            // the index as free and just remove it (along with any trailing
            // free indices)
            if i == self.readers.len() - 1 {
                self.readers[i].close();

                for i in (0..=i).rev() {
                    if self.readers[i].is_closed() {
                        self.readers.pop();
                        self.fds.pop();
                        self.free_indices.remove(&i);
                    } else {
                        break;
                    }
                }
            } else {
                // Otherwise, close the file descriptor on the device and
                // mark the index as free to avoid moving the other elements.
                self.free_indices.insert(i);
                self.readers[i].close();
                self.fds[i].fd = self.null_device.as_raw_fd();
            }

            Some(i)
        } else {
            None
        }
    }
}


pub struct EventIterator<'a> {
    watcher: &'a mut InotifyWatcher,
    readers: &'a mut [EventReader],
    fds: &'a [PollFd],
    index: usize
}

impl<'a> Iterator for EventIterator<'a> {
    type Item = (InputEvent, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.readers.len() {
            let i = self.index;
            self.index += 1;

            if self.fds[i].has_event() {
                let event = self.readers[i].next();
                if let Some(event) = event {
                    return Some((event, i));
                }
            }
        }

        None
    }
}

impl<'a> EventIterator<'a> {
    pub fn inotify_event(&'a mut self) -> Option<(InotifyEventKind, &'a CStr)> {
        self.watcher.next()
    }
}
