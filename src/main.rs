extern crate termion;

pub mod color;
pub mod pane;
pub mod util;
pub mod side;

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;
use termion::terminal_size;

use std::io::{stdin, stdout, Write};

use pane::*;
use side::*;

fn run(pane: &mut dyn Pane<Event>) {
    let size = terminal_size().expect("could not get size of terminal!");

    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let mut handler = PaneHandler::new(pane, size);
    screen.write(format!("{}", termion::cursor::SteadyBar).as_bytes()).unwrap();
    screen.write(handler.render().as_bytes()).unwrap();
    screen.flush().unwrap();

    for event in stdin.events() {
        match event.unwrap() {
            Event::Key(Key::Esc) => break,
            e => {
                screen.write(handler.emit_event(e).as_bytes()).unwrap();
                screen.flush().unwrap();
            }
        }
    }
}

fn main() {
    run( &mut SideRootPane::new() );
}
