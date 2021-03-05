use termion::event::{Event, Key};

use crate::pane::{Renderer, Pane};
use crate::util::Span;

use std::cmp::min;

/// Text panes draw, well, a bunch of text
pub struct TextPane {
    text: Vec<String>,
    cursor: Span
}

impl TextPane {
    pub fn new(text: String) -> TextPane {
        return TextPane {
            text: text
                .split("\n")
                .into_iter()
                .map(|line| line.to_string())
                .collect(),
            cursor: (0, 0).into()
        };
    }
}

impl Pane<Event> for TextPane {
    fn get_size(&self) -> Span {
        return Span {
            x: 80,
            y: self.text.len(),
        };
    }

    fn render(&self, renderer: Renderer) {
        let size = renderer.size();

        renderer.set_cursor(self.cursor);

        for y in 0..min(size.y, self.text.len()) {
            let end = self.text[y]
                .char_indices()
                .nth(size.x)
                .unwrap_or((self.text[y].len(), ' '))
                .0;

            if end == 0 {
                continue;
            }

            let line = &self.text[y][..end];
            renderer.draw(0, y, line);
        }
    }

    fn event(&mut self, event: Event) -> bool {
        match event {
            Event::Key(Key::Char('w')) => self.cursor.y -= 1,
            Event::Key(Key::Char('a')) => self.cursor.x -= 1,
            Event::Key(Key::Char('s')) => self.cursor.y += 1,
            Event::Key(Key::Char('d')) => self.cursor.x += 1,
            _ => {}
        }

        return true;
    }
}
