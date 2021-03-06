use std::sync::mpsc::Sender;

use termion::event::Event;
use termion::event::Key;

use crate::pane::{Canvas, Pane, layout, LayoutConstraint};
use crate::side::Editor;
use crate::side::FileMenu;

///
pub struct Manager {
    left: FileMenu,
    text: Editor,
    selected: bool,
    // emiter: Sender<Event>
}

impl Manager {
    pub fn new(emiter: Sender<Event>) -> Manager {
        Manager {
            left: FileMenu::new(emiter),
            text: Editor::load("src/main.rs".to_string()),
            selected: true,
            // emiter,
        }
    }

    pub fn get_selected_pane(&mut self) -> &mut dyn Pane<Event> {
        if self.selected {
            return &mut self.text;
        } else {
            return &mut self.left;
        }
    }

    pub fn set_file(&mut self, path: String) {
        self.text = Editor::load(path);
    }
}

impl Pane<Event> for Manager {
    fn render(&self, canvas: Canvas) {
        layout::<Event>(canvas, vec![
               (&self.left, LayoutConstraint::Flex(1)),
               (&self.text, LayoutConstraint::Fixed(84)),
        ]);
    }

    fn event(&mut self, event: Event) -> bool {
        return match event {
            Event::Key(Key::Char('\t')) => {
                self.selected = !self.selected;
                return false;
            },

            _ => self.get_selected_pane().event(event)
        };
    }
}
