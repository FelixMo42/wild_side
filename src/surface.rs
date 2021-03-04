use crate::color::Color;
use crate::util::Vec2d;
use crate::util::{Area, Size};

pub struct Surface {
    fg: Vec2d<Color>,
    bg: Vec2d<Color>,
    chars: Vec2d<char>,
}

impl Surface {
    pub fn new(size: Size, fg: Color, bg: Color) -> Surface {
        Surface {
            fg: Vec2d::new(size, fg),
            bg: Vec2d::new(size, bg),
            chars: Vec2d::new(size, ' '),
        }
    }
}

pub fn render(surface: Surface, area: Area) -> String {
    // put all the changes we need to make in one string so we dont need to
    // print to stdout as often as its slow
    let mut cmd = "".to_string();

    // keep track of fg and bg so we can only change it when we need too
    let mut curr_bg = surface.bg.get(0, 0);
    let mut curr_fg = surface.fg.get(0, 0);

    cmd += &curr_fg.fg()[..];
    cmd += &curr_bg.bg()[..];

    for y in area.0.y..area.1.y {
        // position the cursor at the start of the line were about to draw
        cmd += &format!("\x1b[{};{}H", y + 1, 1)[..];

        for x in area.0.x..area.1.x {
            // upgrade the fg if we need too
            let fg = surface.fg.get(x, y);
            if curr_fg != fg {
                cmd += &fg.fg()[..];
                curr_fg = fg;
            }

            // update the bg if we need too
            let bg = surface.bg.get(x, y);
            if curr_bg != bg {
                cmd += &bg.bg()[..];
                curr_bg = bg;
            }

            // add the char at this position to the screen
            cmd.push(surface.chars.get(x, y));
        }
    }

    return cmd;
}
