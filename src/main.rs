use pane::*;
use util::Span;

use std::{
    io::{stdin, stdout, Write},
    sync::mpsc::channel,
    thread,
};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;
use termion::terminal_size;

pub mod color;
pub mod pane;
pub mod util;

///
pub struct FillPane {
    chr: char,
}

impl FillPane {
    pub fn new(chr: char) -> Box<dyn pane::Pane<Event>> {
        return Box::new(FillPane { chr });
    }
}

impl<Event> pane::Pane<Event> for FillPane {
    fn render(&self, mut canvas: Canvas) {
        let area = canvas.area();
        for x in area.0.x..area.1.x {
            for y in area.0.y..area.1.y {
                canvas.draw_char((x, y).into(), self.chr);
            }
        }
    }

    fn event(&self, _event: Event) {}
}

///
pub struct Ide {
    layout: Box<dyn pane::Pane<Event>>,
}

impl Ide {
    pub fn new() -> Ide {
        Ide {
            layout: Box::new(VertFlexPane(
                0,
                vec![
                    //(FillPane::new('#'), FlexConstraint::Fixed(10)),
                    (FillPane::new('-'), FlexConstraint::Flex(1)),
                    (FillPane::new('#'), FlexConstraint::Fixed(1)),
                ],
            )),
        }
    }
}

impl pane::Pane<Event> for Ide {
    fn event(&self, event: Event) {
        if event == Event::Key(Key::Char('\t')) {
        } else {
            self.layout.event(event);
        }
    }

    fn render(&self, canvas: Canvas) {
        self.layout.render(canvas);
    }
}

fn main() {
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let (sender, recver) = channel::<Event>();
    thread::spawn(move || {
        for event in stdin().events() {
            if let Ok(event) = event {
                sender.send(event).unwrap();
            }
        }
    });

    let (x, y) = terminal_size().expect("could not get terminal size!");
    let size = Span::new(x as usize, y as usize);

    let root = Box::new(Ide::new());
    let mut doc = Document::new(root, size);

    screen.write(doc.render().as_bytes()).unwrap();
    screen.flush().unwrap();

    for event in recver.into_iter() {
        if event == Event::Key(Key::Esc) {
            break;
        }

        screen.write(doc.emit(event).as_bytes()).unwrap();
        screen.flush().unwrap();
    }
}
