use termion::event::{Event, Key};

use crate::pane::{Canvas, Pane};
use crate::util::Span;
use crate::color::GRAY5;

use std::{cmp::min, fs};

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

        if chr == '\n' {
            let line = self.lines[spot.y].split_off(index);
            self.lines.insert(spot.y + 1, line);
        } else {
            self.lines[spot.y].insert(index, chr);
        }
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
pub struct Editor {
    text: Text,
    cursor: Span
}

impl Editor {
    pub fn new(text: String) -> Editor {
        return Editor {
            text: Text::new(text),
            cursor: (0, 0).into()
        };
    }

    pub fn load(path: String) -> Editor {
        return Editor::new(
            fs::read_to_string(path)
                .expect("could not read file!")
        );
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

    fn get_line(&self, line: usize, len: usize) -> &str {
        let text = &self.text.lines[line];
        let end = text
            .char_indices()
            .nth(len)
            .unwrap_or((text.len(), ' '))
            .0;

        return &text[..end];
    }
}

impl Pane<Event> for Editor {
    fn render(&self, canvas: Canvas) {
        let size = canvas.size();

        let line_num_bar_width = 4;
        let line_num_bar_style = GRAY5.clone().as_fg();
        
        // this only sets the visual position of the cursor,
        // it does not change where we draw things
        canvas.set_cursor(self.cursor.shift(&(line_num_bar_width, 0).into()));


        for y in 0..min(size.y, self.text.len()) {
            canvas.draw_line_with_style(
                (0, y).into(),
                format!("{:>1$}", y, line_num_bar_width - 1).as_str(),
                &line_num_bar_style
            );

            canvas.draw(
                (line_num_bar_width , y).into(),
                self.get_line(y, size.x - line_num_bar_width)
            );
        }
    }


    fn event(&mut self, event: Event) -> bool {
        match event {
            Event::Key(Key::Up   ) => self.move_cursor_up(),
            Event::Key(Key::Down ) => self.move_cursor_down(),
            Event::Key(Key::Left ) => self.move_cursor_left(),
            Event::Key(Key::Right) => self.move_cursor_right(),
            
            Event::Key(Key::Backspace) => {
                self.move_cursor_left();
                self.text.delete(self.cursor);
            },

            Event::Key(Key::Char(chr)) => {
                self.text.insert(self.cursor, chr);
                self.move_cursor_right()
            },
        
            _ => {}
        }

        return true;
    }
}
