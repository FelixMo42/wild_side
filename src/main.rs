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

use pane::Canvas;
use pane::LinePane;
use pane::TextPane;

fn main() {
    // drop in the the new screen
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    // what file do we want to display?
    let path = "./src/main.rs";

    // read the contents of the file
    let contents = fs::read_to_string(path).expect("could not open file!");

    // the root node the document
    let mut root = LinePane::<Event> {
        child: &mut TextPane::new(contents),
    };

    // get the size of the terminal
    let size = terminal_size().expect("could not get size of terminal!");

    // create the actuall rendering canvas
    let mut renderer = Canvas::new(&mut root, size);

    // render it
    print!("{}{}", termion::clear::All, renderer.render());

    // wait until q is pressed
    for event in stdin.events() {
        match event.unwrap() {
            Event::Key(Key::Char('q')) => break,
            e => {
                print!("{}", renderer.emit_event(e));
            }
        }
    
        screen.flush().unwrap();
    }

    // make sure everything has been printed out
    screen.flush().unwrap();
}
