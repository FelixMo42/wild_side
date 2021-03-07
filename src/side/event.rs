pub enum Event {
    // key events
    Char(char),
    Delete,
    Return,
    Escape,

    // navigation
    Up,
    Down,
    Left,
    Right,

    // commands
    OpenFile(String)
}
