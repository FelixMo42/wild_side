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
use termion::raw::IntoRawMode;
use termion::screen::*;
use termion::terminal_size;
use termion::{
    event::{Event, Key},
    input::TermRead,
};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let (x, y) = terminal_size().expect("could not get size of terminal!");

    let (tx, rx) = std::sync::mpsc::channel::<Event>();
    
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut handler = PaneHandler::new(
        Box::new(Manager::new(tx.clone())),
        (x as usize, y as usize).into()
    );

    screen.write(format!("{}", termion::cursor::SteadyBar).as_bytes())?;
    screen.write(handler.render().as_bytes())?;
    screen.flush()?;

    thread::spawn(move || {
        for event in stdin().events() {
            if let Ok(event) = event {
                tx.send(event).unwrap();
            }
        }
    });

    for event in rx.into_iter() {
        match event {
            Event::Key(Key::Esc) => break,
            e => {
                screen.write(handler.emit_event(e).as_bytes())?;
                screen.flush()?;
            }
        }
    }

    return Ok(());
}
