/*
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::terminal_size;
use termion::screen::*;

use surface::{ Surface, render };
use util::Size;

fn main() {
    // setup up terminal io
    let stdin = stdin();
    let stdout = stdout().into_raw_mode().unwrap();
    let mut screen = AlternateScreen::from(stdout);

    // make sure to clear the screen
    print!("{}", termion::clear::All);

    let term_size = terminal_size() .expect("could not find terminal size");
    let size = Size::new(
        term_size.0 as usize,
        term_size.1 as usize
    );

    let surface = Surface::new(size, color::PINK1, color::PINK9);

    println!("{}", render(surface, size.area()));

    // wait
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            _ => {}
        }
        screen.flush().unwrap();
    }

    // make sure everything has been printed out
    screen.flush().unwrap();
} */

extern crate termion;

pub mod color;
pub mod pane;
pub mod surface;
pub mod util;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;
use termion::terminal_size;

use std::fs;
use std::io::{stdin, stdout, Write};

use pane::canvas::Canvas;
use pane::line_pane::LinePane;
use pane::text_pane::TextPane;

fn main() {
    // drop in the the new screen
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    // what file do we want to display?
    let path = "./src/main.rs";

    // read the contents of the file
    let contents = fs::read_to_string(path).expect("could not open file!");

    // the root node the document
    let root = LinePane {
        child: &TextPane::new(contents),
    };

    // get the size of the terminal
    let size = terminal_size().expect("could not get size of terminal!");

    // create the actuall rendering canvas
    let mut renderer = Canvas::new(&root, size);

    // render it
    print!("{}{}", termion::clear::All, renderer.render());

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
