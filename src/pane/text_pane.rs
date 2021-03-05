use termion::event::{Event, Key};

use crate::pane::{Renderer, Pane};
use crate::util::Span;

use std::cmp::min;

pub struct Text {
    lines: Vec<String>
}

impl Text {
    pub fn new(text: String) -> Text {
        Text {
            lines: text 
                .split("\n")
                .into_iter()
                .map(|line| line.to_string())
                .collect()
        }
    }

    pub fn len(&self) -> usize {
        return self.lines.len()
    }

    pub fn get_line_length(&self, line: usize) -> usize {
        return self.lines[line].chars().count();
    }

    pub fn get_index(&self, spot: Span) -> usize {
        let line = &self.lines[spot.y];
        
        line.char_indices()
            .nth(spot.x)
            .map_or(line.len(), |(i, _c)| i)
    }

    pub fn insert(&mut self, spot: Span, chr: char) {
        let index = self.get_index(spot);
        self.lines[spot.y].insert(index, chr);
    }

    pub fn delete(&mut self, spot: Span) {
        if spot.x == self.get_line_length(spot.y) {
            let line = self.lines.remove(spot.y + 1);
            self.lines[spot.y] += line.as_str();
        } else {
            let index = self.get_index(spot);
            self.lines[spot.y].remove(index);
        }
    }
}

/// Text panes draw, well, a bunch of text
pub struct TextPane {
    text: Text,
    cursor: Span
}

impl TextPane {
    pub fn new(text: String) -> TextPane {
        return TextPane {
            text: Text::new(text),
            cursor: (0, 0).into()
        };
    }

    fn get_line_length(&self) -> usize {
        return self.text.get_line_length(self.cursor.y)
    }

    fn bound_cursor_x(&mut self) {
        self.cursor.x = min(
            self.cursor.x,
            self.get_line_length()
        )
    }
    
    fn move_cursor_up(&mut self) {
        if self.cursor.y != 0 {
            self.cursor.y -= 1;
            self.bound_cursor_x();
        }
    }
    
    fn move_cursor_down(&mut self) {
        if self.cursor.y != self.text.len() - 1 {
            self.cursor.y += 1;
            self.bound_cursor_x();
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor.x == 0 {
            self.move_cursor_up();
            self.cursor.x = self.get_line_length();
        } else {
            self.cursor.x -= 1;
        }
    }
    
    fn move_cursor_right(&mut self) {
        if self.cursor.x == self.get_line_length() {
            self.move_cursor_down();
            self.cursor.x = 0;
        } else {
            self.cursor.x += 1;
        }
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
            let line = &self.text.lines[y];
            let end = line
                .char_indices()
                .nth(size.x)
                .unwrap_or((line.len(), ' '))
                .0;

            if end == 0 {
                continue;
            }

            let line = &self.text.lines[y][..end];
            renderer.draw(0, y, line);
        }
    }


    fn event(&mut self, event: Event) -> bool {
        match event {
            Event::Key(Key::Up   ) => self.move_cursor_up(),
            Event::Key(Key::Down ) => self.move_cursor_down(),
            Event::Key(Key::Left ) => self.move_cursor_left(),
            Event::Key(Key::Right) => self.move_cursor_right(),
            
            Event::Key(Key::Char(chr)) => {
                self.text.insert(self.cursor, chr);
                self.move_cursor_right()
            },
            
            Event::Key(Key::Backspace) => {
                self.move_cursor_left();
                self.text.delete(self.cursor);
            }

            _ => {}
        }

        return true;
    }
}
