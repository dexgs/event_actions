mod events;
mod dlfcn;

use events::*;
use InotifyEventKind::*;

use std::env;
use std::ffi::{CString, CStr};
use std::path::Path;


fn main() {
    let handler_object_path = env::args()
        .nth(1)
        .expect("Getting path to handler object");

    let handler_object = dlfcn::Object::new(handler_object_path);
    let mut poller = EventPoller::new("/dev/input/by-id").unwrap();

    for entry in Path::new("/dev/input/by-id").read_dir().unwrap() {
        if let Ok(entry) = entry {
            let name = CString::new(entry.file_name().to_str().unwrap().to_string()).unwrap();

            device_update(&mut poller, Created, &name, &handler_object);
        }
    }

    let mut buf = Vec::with_capacity(2048);
    loop {
        let mut events = poller.poll();

        for (event, index) in &mut events {
            (handler_object.handle_input)(&event, index);
        }

        if let Some((state, name)) = events.inotify_event().take() {
            buf.clear();
            buf.extend_from_slice(name.to_bytes());

            let name = unsafe {
                CStr::from_bytes_with_nul_unchecked(&buf)
            };

            device_update(&mut poller, state, name, &handler_object);
        }
    }
}

fn device_update(poller: &mut EventPoller, state: InotifyEventKind, name: &CStr, handler_object: &dlfcn::Object) {
    let name_str = name.to_str().unwrap();
    let name_str = name_str.split_once('\0').map(|s| s.0).unwrap_or(name_str);

    let index = match state {
        Created => poller.next_free_index().unwrap_or(poller.num_readers()),
        Removed => match poller.remove_reader(name_str) {
            Some(i) => i,
            None => return
        }
    };

    let handle_result = (handler_object.device_update)(state.into(), name.as_ptr(), index);

    if let Created = state {
        match handle_result {
            1 => { poller.add_reader(name_str, false); },
            2 => { poller.add_reader(name_str, true); },
            _ => {}
        }
    }
}
