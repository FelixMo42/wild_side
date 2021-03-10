use std::cmp::min;

use crate::pane::*;
use crate::side::*;
use crate::color::*;

pub struct Menu {
    selector: String,
    selected: usize,
    filtered: Vec<usize>,
    options: Vec<String>,
}

impl Menu {
    pub fn new(options: Vec<String>) -> Menu {
        Menu {
            selector: "".to_string(),
            filtered: (0..options.len()).collect(),
            selected: 0,
            options,
        }
    }

    pub fn reset_filtered(&mut self) {
        self.filtered = (0..self.options.len()).collect()
    }

    pub fn fuzzy(&self, i: usize) -> Option<(usize, usize)> {
        let mut target = self.options[i].chars().peekable();
        let mut selector = self.selector.chars().peekable();
        
        let mut diff = 0;

        while selector.peek() != None && target.peek() != None {
            if selector.peek() == target.peek() {
                selector.next();
                target.next();
            } else {
                target.next();
                diff += 1;
            }
        }

        if target.peek() == None && selector.peek() != None {
            return None;
        }

        return Some((diff, i));
    }

    pub fn filter(&mut self) {
        let mut diffs = self.filtered.iter()
            .map(|i| i.clone())
            .filter_map(|i| self.fuzzy(i))
            .collect::<Vec<(usize, usize)>>();
        
        diffs.sort();
       
        self.filtered = diffs.iter()
            .map(|(_diff, i)| i.clone())
            .collect();
    }

    pub fn get_selected(&self) -> usize {
        return self.selected;
    }

    pub fn render(&self, mut canvas: Canvas, focused: bool) {
        let size = canvas.size();

        canvas.style(THEME.style(1));

        let selected = self.get_selected();

        for (y, i) in self.filtered.iter().enumerate() {
            // dont show the whole path
            let path = self.options[i.clone()]
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

    pub fn clamp_selected(&mut self) {
        let len = self.filtered.len();

        if len == 0 {
            self.selected = 0;
        } else  {
            self.selected = min(self.selected, len - 1);
        }
    }

    pub fn event(&mut self, event: Event) -> Option<String> {
        match event {
            Event::Char(c) => {
                self.selector.push(c);
                self.filter();
                self.clamp_selected();
            },

            Event::Delete => {
                let leng = self.selector.chars().count();

                if leng > 0 {
                    self.selector.remove(leng - 1);
                    self.reset_filtered();
                    self.filter();
                    self.clamp_selected();
                }
            },

            Event::Return => {
                let selected = self.get_selected();

                self.reset_filtered();
                self.selected = 0;

                return self.options.get(selected).map(|opt| opt.clone());
            },

            Event::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            },

            Event::Down => {
                self.selected += 1;
                self.clamp_selected();
            }
            _ => {},
        }

        return None;
    }


}
