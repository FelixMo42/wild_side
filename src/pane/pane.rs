use crate::color::{GRAY0, GRAY9};
use crate::pane::{Canvas, Surface};
use crate::util::{Area, Span};

use std::sync::{Arc, Mutex};

/// A pane is a genaric building block of the ui
pub trait Pane<Event> {
    fn render(&self, canvas: Canvas, selected: bool);
    fn event(&mut self, event: Event);
}

///
pub struct PaneHandler<Event> {
    size: Span,
    data: Surface,
    root: Box<dyn Pane<Event>>,
}

impl<Event> PaneHandler<Event> {
    pub fn new(root: Box<dyn Pane<Event>>, size: Span) -> PaneHandler<Event> {
        PaneHandler {
            data: Surface::new(size, GRAY0, GRAY9),
            size,
            root,
        }
    }

    fn area(&self) -> Area {
        return self.size.area();
    }

    pub fn render(&mut self) -> String {
        let area = self.area();

        let data = Arc::new(Mutex::new(&mut self.data));
        let canvas = Canvas::new(data, area.clone());
        canvas.clear(area);
        self.root.render(canvas, true);

        return self.data.render(area);
    }

    pub fn emit_event(&mut self, event: Event) -> String {
        let changed = self.root.event(event);

        return self.render();
    }
}
