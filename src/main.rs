extern crate termion;

pub mod pane;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::terminal_size;
use termion::screen::*;

use std::io::{Write, stdout, stdin};
use std::fs;

use pane::canvas::Canvas;
use pane::text_pane::TextPane;
use pane::box_pane::BoxPane;

fn main() {
    // drop in the the new screen
    let stdin = stdin();
    let mut screen = AlternateScreen::from(
        stdout()
            .into_raw_mode()
            .unwrap()
    );

    // what file do we want to display?
    let path = "./src/main.rs";

    // read the contents of the file
    let contents =
        fs::read_to_string(path)
        .expect("could not open file!");

    // the root node the document
    let root = BoxPane {
        child: &TextPane::new(contents)
    };

    let size = terminal_size().unwrap();

    // create the actuall rendering canvas
    let mut renderer = Canvas::new(&root, size);

    // render it
    print!("{}{}",
        termion::clear::All,
        renderer.render()
    );

    // wait until q is pressed
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            _ => {}
        }
        screen.flush().unwrap();
    }

    // make sure everything has been printed out
    screen.flush().unwrap();
}
