use super::canvas::Renderer;
use super::pane::{Bounds, Pane};
use crate::util::Size;
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
    fn get_size(&self, bounds: Bounds) -> Size {
        return Size {
            w: bounds.max.w,
            h: self.text.len(),
        };
    }

    fn render(&self, renderer: Renderer) {
        let size = renderer.size();

        for y in 0..min(size.h, self.text.len()) {
            let end = self.text[y]
                .char_indices()
                .nth(size.w)
                .unwrap_or((self.text[y].len(), ' '))
                .0;

            if end == 0 {
                continue;
            }

            let line = &self.text[y][..end];
            renderer.echo(0, y, line);
        }
    }
}
