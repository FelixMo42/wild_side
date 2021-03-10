use crate::pane::*;
use crate::side::*;
use crate::color::*;

pub struct Menu {
    selector: String,
    options: Vec<String>,
}

impl Menu {
    pub fn new(options: Vec<String>) -> Menu {
        Menu {
            selector: "".to_string(),
            options,
        }
    }

    pub fn get_selected(&self) -> usize {
        self.selector.parse::<usize>().unwrap_or(0)
    }

    pub fn render(&self, mut canvas: Canvas, focused: bool) {
        let size = canvas.size();

        canvas.style(THEME.style(1));

        let selected = self.get_selected();

        for (y, file) in self.options.iter().enumerate() {
            let path = file
                .chars()
                .skip(2)
                .take(size.x - 4 - 1)
                .collect::<String>();

            if y == selected && focused {
                // draw line number
                canvas.draw_line_with_style(
                    (1, y + 1).into(),
                    format!("{:>2}", y).chars(),
                    THEME.normal(1).as_fg(),
                );

                // draw line
                canvas.draw_line_with_style(
                    (4, y + 1).into(),
                    path.chars(),
                    THEME.focused(1).as_fg()
                );
            } else {
                // draw line number
                canvas.draw_line_with_style(
                    (1, y + 1).into(),
                    format!("{:>2}", y).chars(),
                    THEME.disabled(1).as_fg(),
                );

                // draw line
                canvas.draw_line((4, y + 1).into(), path.chars());
            }
        }

        // draw prompt
        canvas.draw_line_with_style(
            (4, 0).into(),
            self.selector.chars(),
            THEME.focused(1).as_fg()
        );

        // set the cursor position
        if focused {
            canvas.set_cursor((4 + self.selector.chars().count(), 0).into());
        }
    }

    pub fn clear_selector(&mut self) {
        self.selector = "".to_string();
    }

    pub fn event(&mut self, event: Event) -> Option<String> {
        match event {
            Event::Char(c) => {
                self.selector.push(c);
                None
            },
            Event::Delete => {
                let leng = self.selector.chars().count();

                if leng > 0 {
                    self.selector.remove(leng - 1);
                }

                None
            }
            Event::Return => Some(self.options[self.get_selected()].clone()),
            _ => None,
        }
    }
}
