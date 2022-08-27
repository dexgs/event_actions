use std::collections::HashMap;
use std::mem;
use std::convert::Into;


// Store strings in a contiguous buffer. Keep track of strings added in a map
// so that we can avoid storing duplicates.
pub struct Strings<'a> {
    buf: &'a mut String,
    map: HashMap<String, StringRange>
}

impl<'a> Strings<'a> {
    pub fn new(buf: &'a mut String) -> Self {
        Self {
            buf,
            map: HashMap::new()
        }
    }

    pub fn add<S>(&mut self, string: S) -> StringRange
    where S: AsRef<str>
    {
        let buf_ptr: *const String = self.buf;
        let string = string.as_ref();

        match self.map.get(string) {
            Some(r) => *r,
            None => {
                self.buf.push_str(string);

                let start = self.buf.len() - string.len();
                let end = self.buf.len();

                self.map.insert(string.to_string(), StringRange (buf_ptr, start, end));

                StringRange (buf_ptr, start, end)
            }
        }
    }
}


#[derive(Clone, Copy)]
pub struct StringRange (*const String, usize, usize);

impl StringRange {
    pub fn to_str<'a>(self, buf: &'a String) -> &'a str {
        let buf_ptr: *const String = buf;
        assert_eq!(buf_ptr, self.0);

        &buf[self.1..self.2]
    }
}
