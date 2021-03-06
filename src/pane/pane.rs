use crate::color::Color;
use crate::color::{Style, GRAY0, GRAY9};
use crate::util::{Area, Span, Vec2d};

use std::sync::{Arc, Mutex};

/// A pane is a genaric building block of the ui
pub trait Pane<Event> {
    fn render(&self, canvas: Canvas);
    fn event(&mut self, event: Event) -> bool;
}


///
pub struct PaneHandler<'a, Event> {
    size: Span,
    data: Surface,
    root: &'a mut dyn Pane<Event>,
}

impl<'a, Event> PaneHandler<'a, Event> {
    
    pub fn new(
        root: &'a mut dyn Pane<Event>, 
        size: Span,
    ) -> PaneHandler<'a, Event> {
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
        self.root.render(canvas);

        return self.data.render(area);
    }

    pub fn emit_event(&mut self, event: Event) -> String {
        let changed = self.root.event(event);

        if changed {
            self.render()
        } else {
            "".to_string()
        }
    }
}

///
pub struct Canvas<'a> {
    area: Area,
    data: Arc<Mutex<&'a mut Surface>>,
}

impl<'a> Canvas<'a> {
    fn new(data: Arc<Mutex<&'a mut Surface>>, area: Area) -> Canvas<'a> {
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

    pub fn draw_pane<Event>(&self, pane: &dyn Pane<Event>, area: Area) {
        pane.render(Canvas::new(
            self.data.clone(),
            area.shift(&self.area.0)
        ));
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
        self.data.lock().unwrap().style_area(style, area.shift(&self.area.0));
    }
}

///
struct Surface {
    fg: Vec2d<Color>,
    bg: Vec2d<Color>,
    chars: Vec2d<char>,
    cursor: Span,
}

fn cursor_cmd(x: usize, y: usize) -> String {
    format!("\x1b[{};{}H", y + 1, x + 1)
}

impl Surface {
    fn new(size: Span, fg: Color, bg: Color) -> Surface {
        Surface {
            fg: Vec2d::new(size, fg),
            bg: Vec2d::new(size, bg),
            chars: Vec2d::new(size, ' '),
            cursor: (0, 0).into(),
        }
    }

    fn draw_line(&mut self, spot: Span, text: &str, style: &Style) {
        for (i, c) in text.char_indices() {
            self.chars.set(spot.x + i, spot.y, c);
        }

        if let Some(fg) = style.fg {
            for (i, _c) in text.char_indices() {
                self.fg.set(spot.x + i, spot.y, fg);
            }
        }

        if let Some(bg) = style.bg {
            for (i, _c) in text.char_indices() {
                self.bg.set(spot.x + i, spot.y, bg);
            }
        }
    }

    fn draw_area(&mut self, chr: char, area: Area) {
        for x in area.0.x..area.1.x {
            for y in area.0.y..area.1.y {
                self.chars.set(x, y, chr);
            }
        }
    }

    fn style_area(&mut self, style: &Style, area: Area) {
        if let Some(fg) = style.fg { 
            for x in area.0.x..area.1.x {
                for y in area.0.y..area.1.y {
                    self.fg.set(x, y, fg);
                }
            }
        }
        
        if let Some(bg) = style.bg { 
            for x in area.0.x..area.1.x {
                for y in area.0.y..area.1.y {
                    self.bg.set(x, y, bg);
                }
            }
        }
    }

    fn set_cursor(&mut self, spot: Span) {
        self.cursor.x = spot.x;
        self.cursor.y = spot.y;
    }

    fn render(&self, area: Area) -> String {
        // put all the changes we need to make in one string so
        // we dont need to print to stdout as often as its slow
        let mut cmd = "".to_string();

        // keep track of fg and bg so we can only change it when we need too
        let mut curr_bg = self.bg.get(0, 0);
        let mut curr_fg = self.fg.get(0, 0);

        cmd += curr_fg.fg_cmd().as_str();
        cmd += curr_bg.bg_cmd().as_str();

        for y in area.0.y..area.1.y {
            // position the cursor at the start of the line
            // were about to draw too
            cmd += cursor_cmd(0, y).as_str();

            for x in area.0.x..area.1.x {
                // upgrade the fg if we need too
                let fg = self.fg.get(x, y);
                if curr_fg != fg {
                    cmd += fg.fg_cmd().as_str();
                    curr_fg = fg;
                }

                // update the bg if we need too
                let bg = self.bg.get(x, y);
                if curr_bg != bg {
                    cmd += bg.bg_cmd().as_str();
                    curr_bg = bg;
                }

                // add the char at this position to the screen
                cmd.push(self.chars.get(x, y));
            }
        }

        // place the cursor in the correct position of screen visually
        // this has to come last or the rendering will move it
        cmd += cursor_cmd(self.cursor.x, self.cursor.y).as_str();

        return cmd;
    }
}
