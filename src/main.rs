extern crate termion;

pub mod color;
pub mod pane;
pub mod side;
pub mod util;

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;
use termion::terminal_size;

use std::error::Error;
use std::io::{stdin, stdout, Write};

use pane::*;
use side::*;

fn run(pane: &mut dyn Pane<Event>) -> Result<(), Box<dyn Error + 'static>> {
    let (x, y) = terminal_size().expect("could not get size of terminal!");

    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let mut handler = PaneHandler::new(pane, (x as usize, y as usize).into());

    screen.write(format!("{}", termion::cursor::SteadyBar).as_bytes())?;
    screen.write(handler.render().as_bytes())?;
    screen.flush()?;

    for event in stdin().events() {
        match event.unwrap() {
            Event::Key(Key::Esc) => break,
            e => {
                screen.write(handler.emit_event(e).as_bytes())?;
                screen.flush()?;
            }
        }
    }

    Ok(())
}

fn main() {
    run(&mut SideRootPane::new()).unwrap();
}
