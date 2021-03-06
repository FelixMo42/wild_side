pub enum Event {
    // key events
    Char(char),
    Delete,
    Return,

    // navigation
    Up,
    Down,
    Left,
    Right,
}

pub enum Cmd {}

pub enum Key {
    Char(char),

    Delete,
    Return,
}
