use crate::pane::Surface;
use crate::util::{Area, Span};
use crate::color::*;

use std::sync::{Arc, Mutex};

type Chars<'a> = std::str::Chars<'a>;

///
pub trait Pane<Event> {
    fn render(&self, canvas: Canvas, focused: bool);
    fn event(&mut self, event: Event);
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
            surface: Surface::new(
                size,
                &THEME.normal(0),
                &THEME.bg(0),
            ),
        }
    }

    pub fn render(&mut self) -> String {
        self.surface = Surface::new(self.size, &THEME.normal(0), &THEME.bg(0)); 
        
        self.pane.render(Canvas {
            area: self.size.area(),
            surface: self.surface.clone(),
        }, true);

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
    /// accessing commands
    pub fn area(&self) -> Area {
        return self.area.zero();
    }

    pub fn size(&self) -> Span {
        return self.area.size();
    }

    /// drawing commands
    pub fn draw_char(&mut self, spot: Span, chr: char) {
        self.surface.lock().unwrap().set(self.area.of(spot), chr);
    }

    pub fn draw_line(&mut self, spot: Span, line: Chars) {
        let mut surface = self.surface.lock().unwrap();
        let spot = self.area.of(spot);
        let max_len = self.area.1.x - spot.x;

        for (x, chr) in line.take(max_len).enumerate() {
            surface.set((spot.x + x, spot.y).into(), chr);
        }
    }

    pub fn draw_pane<Event>(self, pane: &Box<dyn Pane<Event>>, focused: bool) {
        pane.render(Canvas {
            area: self.area,
            surface: self.surface,
        }, focused);
    }

    /// drawing... with style
    pub fn draw_line_with_style(&self, spot: Span, line: Chars, style: Style) {
        let mut surface = self.surface.lock().unwrap();
        let spot = self.area.of(spot);
        let max_len = self.area.1.x - spot.x;

        for (x, chr) in line.take(max_len).enumerate() {
            surface.set((spot.x + x, spot.y).into(), chr);
        
            if let Some(fg) = style.fg {
                surface.fg.set(spot.x + x, spot.y, fg);
            }
            
            if let Some(bg) = style.bg {
                surface.bg.set(spot.x + x, spot.y, bg);
            }
        }
    }

    /// styling area
    pub fn style(&mut self, style: Style) {
        self.style_area(self.area.zero(), style);
    }

    pub fn style_area(&mut self, area: Area, style: Style) {
        let mut surface = self.surface.lock().unwrap();
        let area = area.shift(&self.area.0);
            
        if let Some(fg) = style.fg {
            for x in area.0.x..area.1.x {
                for y in area.0.y..area.1.y {
                    surface.fg.set(x, y, fg);
                }
            }
        }
        
        if let Some(bg) = style.bg {
            for x in area.0.x..area.1.x {
                for y in area.0.y..area.1.y {
                    surface.bg.set(x, y, bg);
                }
            }
        }
    }

    /// sub canvasing commands
    pub fn splitv(self, pos: usize) -> (Canvas, Canvas) {
        let pos = self.area.0.y + pos;

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
        let pos = self.area.0.x + pos;

        return (
            Canvas {
                area: self.area.horizontal_slice(self.area.0.x, pos),
                surface: self.surface.clone(),
            },
            Canvas {
                area: self.area.horizontal_slice(pos, self.area.1.x),
                surface: self.surface.clone(),
            },
        );
    }
}
