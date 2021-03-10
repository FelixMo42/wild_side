use std::sync::mpsc::Sender;

use crate::pane::*;
use crate::side::*;

///
pub struct Manager {
    layout: HorzFlexPane<Event>,
}

impl Manager {
    pub fn new(emiter: Sender<Event>) -> Manager {
        Manager {
            layout: HorzFlexPane(0, vec![
                (TabBar::new(vec![
                    ('F', FileMenu::new(emiter)),
                ]), FlexConstraint::Fixed(33)),
                (Editor::new("".to_string()), FlexConstraint::Flex(1)),
            ])
        }
    }
}

impl Pane<Event> for Manager {
    fn render(&self, canvas: Canvas, focused: bool) {
        self.layout.render(canvas, focused);
    }

    fn event(&mut self, event: Event) {
        match event {
            Event::OpenFile(_) => {
                self.layout.next();
                self.layout.event(event);
            },
            Event::Char('\t') => {
                self.layout.next();
            },
            _ => self.layout.event(event)
        }
    }
}
