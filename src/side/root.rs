use std::sync::mpsc::Sender;

use crate::pane::*;
use crate::side::*;

/// 

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
                ]), FlexConstraint::Fixed(40)),
                (Editor::new("".to_string()), FlexConstraint::Flex(1)),
            ])
        }
    }

}

impl Pane<Event> for Manager {
    fn render(&self, canvas: Canvas) {
        self.layout.render(canvas);
    }

    fn event(&mut self, event: Event) {
        self.layout.event(event);
    }
}

/*
    fn render(&self, canvas: Canvas, _selected: bool) {
        layout::<Event>(canvas, self.selected, &self.layout);
    }

    fn event(&mut self, event: Event) {
        match event {
            Event::Char('\t') => {
                self.selected = (self.selected + 1) % 2;
            }

            Event::OpenFile(path) => {
                self.selected = 1;
                self.get_selected_pane_mut().event(Event::OpenFile(path));
            }

            _ => self.get_selected_pane_mut().event(event)
        };
    }
} */
