use termion::event::Event;

use crate::pane::{Pane, Canvas};
use crate::side::Editor;

pub struct SideRootPane {
    text: Editor
}

impl SideRootPane {
    pub fn new() -> SideRootPane {
        SideRootPane {
            text: Editor::load("./src/main.rs".to_string())
        }
    }
}

impl Pane<Event> for SideRootPane {
    fn render(&self, canvas: Canvas) {
        canvas.draw_pane(&self.text, (0, 0).into(), canvas.size());
    }

    fn event(&mut self, event: Event) -> bool {
        self.text.event(event)
    }
}
