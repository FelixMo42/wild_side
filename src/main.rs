extern crate termion;

pub mod pane;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;

use std::io::{Write, stdout, stdin};
use std::fs;

use pane::pane::Size;
use pane::canvas::Canvas;
use pane::text_pane::TextPane;
use pane::box_pane::BoxPane;

fn main() {
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    // what file do we want to display?
    let path = "./src/main.rs";

    // read the contents of the file
    let contents = fs::read_to_string(path)
        .expect("could not open file!");

    // the root node the document
    let root = BoxPane {
        child: &TextPane::new(contents)
    };

    // create the actuall rendering canvas
    let mut renderer = Canvas::new(&root, Size::new(50, 50));

    // render it
    print!("{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        renderer.render()
    );

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            _ => {}
        }
        screen.flush().unwrap();
    }

    // 
    screen.flush().unwrap();

}
