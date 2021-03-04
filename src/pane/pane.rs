use crate::color::{GRAY0, GRAY9, Style};
use crate::util::{Area, Span, Vec2d};
use crate::color::Color;

use std::sync::{Arc, Mutex};

/// A pane is a genaric building block of the ui
pub trait Pane {
    fn get_size(&self) -> Span;
    fn render(&self, renderer: Renderer);
}

///
pub struct Canvas<'a> {
    size: Span,
    data: Surface,
    root: &'a dyn Pane,
}

impl<'a> Canvas<'a> {
    pub fn new(root: &'a dyn Pane, term_size: (u16, u16)) -> Canvas<'a> {
        let size = Span::new(term_size.0 as usize, term_size.1 as usize);

        Canvas {
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
        self.root.render(Renderer::new(data, area));
        return self.data.render(self.area());
    }
}

///
pub struct Renderer<'a> {
    area: Area,
    data: Arc<Mutex<&'a mut Surface>>,
}

impl<'a> Renderer<'a> {
    fn new(data: Arc<Mutex<&'a mut Surface>>, area: Area) -> Renderer<'a> {
        return Renderer { data, area };
    }

    pub fn size(&self) -> Span {
        return self.area.size();
    }

    pub fn draw(&self, x: usize, y: usize, text: &str) {
        self.data.lock().unwrap().write(
            self.area.0.x + x,
            self.area.0.y + y,
            text,
        );
    }

    pub fn draw_line_with_style(&self, spot: Span, text: &str, style: Style) {
        self.data.lock().unwrap().draw_line(
            spot.shift(&self.area.0),
            text,
            style
        );    
    }

    pub fn draw_pane(&self, pane: &dyn Pane, spot: Span, size: Span) {
        pane.render(Renderer::new(
            self.data.clone(),
            Area::new(self.area.0.shift(&spot), spot.shift(&size)),
        ));
    }
}

///
struct Surface {
    fg: Vec2d<Color>,
    bg: Vec2d<Color>,
    chars: Vec2d<char>,
}

impl Surface {
    fn new(size: Span, fg: Color, bg: Color) -> Surface {
        Surface {
            fg: Vec2d::new(size, fg),
            bg: Vec2d::new(size, bg),
            chars: Vec2d::new(size, ' '),
        }
    }

    fn draw_line(&mut self, spot: Span, text: &str, style: Style) {
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

    fn write(&mut self, x: usize, y: usize, text: &str) {
        for (i, c) in text.char_indices() {
            self.chars.set(x + i, y, c);
        }
    }

    fn render(&self, area: Area) -> String {
        // put all the changes we need to make in one string so we dont need to
        // print to stdout as often as its slow
        let mut cmd = "".to_string();

        // keep track of fg and bg so we can only change it when we need too
        let mut curr_bg = self.bg.get(0, 0);
        let mut curr_fg = self.fg.get(0, 0);

        cmd += &curr_fg.fg()[..];
        cmd += &curr_bg.bg()[..];

        for y in area.0.y..area.1.y {
            // position the cursor at the start of the line were about to draw too
            cmd += &format!("\x1b[{};{}H", y + 1, 1)[..];

            for x in area.0.x..area.1.x {
                // upgrade the fg if we need too
                let fg = self.fg.get(x, y);
                if curr_fg != fg {
                    cmd += &fg.fg()[..];
                    curr_fg = fg;
                }

                // update the bg if we need too
                let bg = self.bg.get(x, y);
                if curr_bg != bg {
                    cmd += &bg.bg()[..];
                    curr_bg = bg;
                }

                // add the char at this position to the screen
                cmd.push(self.chars.get(x, y));
            }
        }

        return cmd;
    }
}
