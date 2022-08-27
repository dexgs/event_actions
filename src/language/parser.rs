enum Token {
    InputBlock,
    DeviceBlock,

    LeftBrace,
    RightBrace,

    LeftParen,
    RightParen,

    String(Vec<StringSegment>),
    Glob(Vec<StringSegment>),
    Bytes(Vec<BytesSegment>),

    Key(Key),
    Event(Event)
}

enum StringSegment {
    String(String),
    Identifier(String)
}

enum BytesSegment {
    Bytes(Vec<u8>),
    Identifier(String)
}

enum Key {
    Single(String),
    Range((String, String))
}

enum Event {
    Pressed,
    Released,
    On,
    Off,
    Relative(EventValue),
    Absolute(EventValue)
}

enum EventValue {
    Equals(c_int),
    Above(c_int),
    Below(c_int),
    Between((c_int, c_int))
}
