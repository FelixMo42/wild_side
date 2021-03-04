use crate::pane::{Renderer, Pane};
use crate::util::Span;

use std::cmp::min;

/// Text panes draw, well, a bunch of text
pub struct TextPane {
    text: Vec<String>,
}

impl TextPane {
    pub fn new(text: String) -> TextPane {
        return TextPane {
            text: text
                .split("\n")
                .into_iter()
                .map(|line| line.to_string())
                .collect(),
        };
    }
}

impl Pane for TextPane {
    fn get_size(&self) -> Span {
        return Span {
            x: 80,
            y: self.text.len(),
        };
    }

    fn render(&self, renderer: Renderer) {
        let size = renderer.size();

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
}
