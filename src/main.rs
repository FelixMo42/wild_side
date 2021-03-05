extern crate termion;

pub mod color;
pub mod pane;
pub mod util;

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;
use termion::terminal_size;

use std::fs;
use std::io::{stdin, stdout, Write};

use pane::*;

fn run(pane: &mut dyn Pane<Event>) {
    let size = terminal_size().expect("could not get size of terminal!");

    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let mut root = RootPane::new(pane, size);
    screen.write(format!("{}", termion::cursor::SteadyBar).as_bytes()).unwrap();
    screen.write(root.render().as_bytes()).unwrap();
    screen.flush().unwrap();

    for event in stdin.events() {
        match event.unwrap() {
            Event::Key(Key::Esc) => break,
            e => {
                screen.write(root.emit_event(e).as_bytes()).unwrap();
                screen.flush().unwrap();
            }
        }
    }
}

fn main() {
    // what file do we want to display?
    let path = "./src/main.rs";

    // read the contents of the file
    let contents = fs::read_to_string(path).expect("could not open file!");

    // the root node the document
    run(&mut LinePane::<Event> {
        child: &mut TextPane::new(contents),
    });
}
