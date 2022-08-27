enum InputCode {
    Single(c_ushort),
    Range((c_ushort, c_ushort))
}

enum InputEvent {
    Above(c_int),
    Below(c_int),
    Between(c_int, c_int),
    Equals(c_int)
}

enum Input {
    codes: &[InputCode],
    event: InputEvent
}
