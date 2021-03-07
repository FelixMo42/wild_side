extern crate ignore;
extern crate termion;
extern crate tokio;

pub mod color;
pub mod pane;
pub mod side;
pub mod util;

use crate::pane::*;
use crate::side::*;
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;
use termion::terminal_size;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let (x, y) = terminal_size().expect("could not get size of terminal!");

    let (tx, rx) = std::sync::mpsc::channel::<Event>();

    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut handler = PaneHandler::new(
        Box::new(Manager::new(tx.clone())),
        (x as usize, y as usize).into(),
    );

    screen.write(format!("{}", termion::cursor::SteadyBar).as_bytes())?;
    screen.write(handler.render().as_bytes())?;
    screen.flush()?;

    thread::spawn(move || {
        for event in stdin().events() {
            if let Ok(event) = event {
                type TEvent = termion::event::Event;
                tx.send(match event {
                    TEvent::Key(Key::Char('\n')) => Event::Return,
                    TEvent::Key(Key::Char(chr)) => Event::Char(chr),
                    TEvent::Key(Key::Backspace) => Event::Delete,
                    TEvent::Key(Key::Delete) => Event::Delete,
                    TEvent::Key(Key::Esc) => Event::Escape,
                    TEvent::Key(Key::Up) => Event::Up,
                    TEvent::Key(Key::Down) => Event::Down,
                    TEvent::Key(Key::Left) => Event::Left,
                    TEvent::Key(Key::Right) => Event::Right,
                    _ => Event::Return
                }).unwrap();
                // tx.send(event).unwrap();
            }
        }
    });

    for event in rx.into_iter() {
        match event {
            Event::Escape => break,
            e => {
                screen.write(handler.emit_event(e).as_bytes())?;
                screen.flush()?;
            }
        }
    }

    return Ok(());
}
