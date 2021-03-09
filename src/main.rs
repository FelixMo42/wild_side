use pane::*;
use side::*;
use util::*;

use std::io::{stdin, stdout, Write};
use std::sync::mpsc::channel;
use std::thread;

use termion::event::{Key, Event as TEvent};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;
use termion::terminal_size;

pub mod color;
pub mod pane;
pub mod side;
pub mod util;

fn main() {
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let (emiter, recver) = channel::<Event>();
    
    let (x, y) = terminal_size().expect("could not get terminal size!");
    let size = Span::new(x as usize, y as usize);

    let root = Box::new( Manager::new(emiter.clone()) );
    let mut doc = Document::new(root, size);
    
    thread::spawn(move || {
        for event in stdin().events() {
            if let Ok(event) = event {
                emiter.send(match event {
                    TEvent::Key(Key::Esc) => Event::Escape,
                    TEvent::Key(Key::Backspace) => Event::Delete,
                    TEvent::Key(Key::Char('\n')) => Event::Return,

                    TEvent::Key(Key::Up) => Event::Up,
                    TEvent::Key(Key::Down) => Event::Down,
                    TEvent::Key(Key::Left) => Event::Left,
                    TEvent::Key(Key::Right) => Event::Right,

                    TEvent::Key(Key::Char(c)) => Event::Char(c),
                    
                    _ => Event::Up,
                }).unwrap();
            }
        }
    });

    screen.write(doc.render().as_bytes()).unwrap();
    screen.flush().unwrap();

    for event in recver.into_iter() {
        if event == Event::Escape {
            break;
        }

        screen.write(doc.emit(event).as_bytes()).unwrap();
        screen.flush().unwrap();
    }
}
