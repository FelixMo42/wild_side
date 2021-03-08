use crate::color::{Color, Style};
use crate::util::{Area, Span, Vec2d};

///
pub struct Surface {
    fg: Vec2d<Color>,
    bg: Vec2d<Color>,
    chars: Vec2d<char>,
}

fn cursor_cmd(x: usize, y: usize) -> String {
    format!("\x1b[{};{}H", y + 1, x + 1)
}

impl Surface {
    pub fn new(size: Span, fg: &Color, bg: &Color) -> Surface {
        Surface {
            fg: Vec2d::new(size, fg.clone()),
            bg: Vec2d::new(size, bg.clone()),
            chars: Vec2d::new(size, ' '),
        }
    }

    pub fn set(&mut self, spot: Span, chr: char) {
        self.chars.set(spot.x, spot.y, chr);
    }

    pub fn draw_line(&mut self, spot: Span, text: &str, style: &Style) {
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

    pub fn draw_area(&mut self, chr: char, area: Area) {
        for x in area.0.x..area.1.x {
            for y in area.0.y..area.1.y {
                self.chars.set(x, y, chr);
            }
        }
    }

    pub fn style_area(&mut self, style: &Style, area: Area) {
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

    pub fn render(&self, area: Area) -> String {
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

        return cmd;
    }
}
