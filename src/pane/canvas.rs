use crate::color::Style;
use crate::pane::{Pane, Surface};
use crate::util::{Area, Span};

use std::sync::{Arc, Mutex};

///
pub struct Canvas<'a> {
    area: Area,
    data: Arc<Mutex<&'a mut Surface>>,
}

impl<'a> Canvas<'a> {
    pub fn new(data: Arc<Mutex<&'a mut Surface>>, area: Area) -> Canvas<'a> {
        return Canvas { data, area };
    }

    pub fn size(&self) -> Span {
        return self.area.size();
    }

    pub fn area(&self) -> Area {
        return self.area.size().area();
    }

    pub fn draw(&self, spot: Span, text: &str) {
        self.data.lock().unwrap().draw_line(
            spot.shift(&self.area.0),
            text,
            &Style::new(None, None),
        );
    }

    pub fn draw_line_with_style(&self, spot: Span, text: &str, style: &Style) {
        self.data
            .lock()
            .unwrap()
            .draw_line(spot.shift(&self.area.0), text, style);
    }

    pub fn draw_pane<Event>(&self, pane: &dyn Pane<Event>, area: Area, selected: bool) {
        pane.render(Canvas::new(self.data.clone(), area.shift(&self.area.0)), selected);
    }

    pub fn clear(&self, area: Area) {
        self.data
            .lock()
            .unwrap()
            .draw_area(' ', area.shift(&self.area.0));
    }

    pub fn set_cursor(&self, spot: Span) {
        self.data
            .lock()
            .unwrap()
            .set_cursor(spot.shift(&self.area.0));
    }

    pub fn style_area(&self, style: &Style, area: Area) {
        self.data
            .lock()
            .unwrap()
            .style_area(style, area.shift(&self.area.0));
    }
}
