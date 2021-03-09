use crate::color::{GRAY5, GRAY9};
use crate::pane::Surface;
use crate::util::{Area, Span};

use std::sync::{Arc, Mutex};

///
pub trait Pane<Event> {
    fn render(&self, canvas: Canvas);
    fn event(&self, event: Event);
}

///
pub struct Document<Event> {
    pane: Box<dyn Pane<Event>>,
    size: Span,
    surface: Arc<Mutex<Surface>>,
}

impl<Event> Document<Event> {
    pub fn new(root: Box<dyn Pane<Event>>, size: Span) -> Document<Event> {
        Document {
            size,
            pane: root,
            surface: Arc::new(Mutex::new(Surface::new(size, &GRAY5, &GRAY9))),
        }
    }

    pub fn render(&mut self) -> String {
        self.pane.render(Canvas {
            area: self.size.area(),
            surface: self.surface.clone(),
        });

        return self.surface.lock().unwrap().render(self.size.area());
    }

    pub fn emit(&mut self, event: Event) -> String {
        self.pane.event(event);
        return self.render();
    }
}

///
pub struct Canvas {
    area: Area,
    surface: Arc<Mutex<Surface>>,
}

impl Canvas {
    pub fn area(&self) -> Area {
        return self.area.zero();
    }

    pub fn draw_char(&mut self, spot: Span, chr: char) {
        self.surface.lock().unwrap().set(self.area.of(spot), chr);
    }

    pub fn draw_line(&mut self, spot: Span, line: String) {
        let mut surface = self.surface.lock().unwrap();
        let spot = self.area.of(spot);
        let max_len = self.area.1.x - spot.x;

        for (x, chr) in line.chars().take(max_len).enumerate() {
            surface.set((spot.x + x, spot.y).into(), chr);
        }
    }

    pub fn draw_pane<Event>(self, pane: &Box<dyn Pane<Event>>) {
        pane.render(Canvas {
            area: self.area,
            surface: self.surface,
        });
    }

    pub fn splitv(self, pos: usize) -> (Canvas, Canvas) {
        let pos = pos + self.area.0.y;

        return (
            Canvas {
                area: self.area.vertical_slice(self.area.0.y, pos),
                surface: self.surface.clone(),
            },
            Canvas {
                area: self.area.vertical_slice(pos, self.area.1.y),
                surface: self.surface.clone(),
            },
        );
    }

    pub fn splith(self, pos: usize) -> (Canvas, Canvas) {
        let leng = self.area.size().x;

        return (
            Canvas {
                area: self.area.horizontal_slice(0, pos),
                surface: self.surface.clone(),
            },
            Canvas {
                area: self.area.horizontal_slice(pos, leng),
                surface: self.surface.clone(),
            },
        );
    }

    pub fn ident(self) -> Canvas {
        Canvas {
            area: self.area,
            surface: self.surface,
        }
    }
}
