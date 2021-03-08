use surface::Surface;
use color::{GRAY5, GRAY9};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use util::{Area, Span};
use std::io::{Write, stdin, stdout};
use termion::screen::*;
use termion::terminal_size;

pub mod surface;
pub mod color;
pub mod util;

enum Pane {
    Text(char),
}

struct Application {
    panes: Vec<Pane>,
    areas: Vec<Area>,
    surface: Surface,
}

impl Application {
    fn new(pane: Pane, size: Span) -> Application {
        Application {
            surface: Surface::new(size, &GRAY5, &GRAY9),
            panes: vec![ pane ],
            areas: vec![ size.area() ],
        }
    }

    fn layout(&mut self, pane_id: usize) {
    }

    fn render(&mut self, pane_id: usize) -> String {
        let pane = &self.panes[pane_id];
        let area = &self.areas[pane_id];

        match pane {
            Pane::Text(chr) => {
                for x in area.0.x..area.1.x {
                    for y in area.0.y..area.1.y {
                        self.surface.set((x, y).into(), chr.clone());
                    }
                }
            }
        }

        return self.surface.render(area.clone());
    }
}

fn main() {
    let (x, y) = terminal_size().expect("could not get size of terminal!");
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let size: Span = (x as usize, y as usize).into();

    let mut app = Application::new(Pane::Text('#'), size);

    screen.write(app.render(0).as_bytes()).unwrap();

    for _event in stdin().events() {
        break;
    }
}

/* extern crate ignore;
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
    let mut root: dyn Pane<Event> = Box::new(Manager::new(tx.clone());
    let mut handler = PaneHandler::new(root, (x as usize, y as usize).into());

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
                    _ => Event::Right,
                }).unwrap();
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
} */
