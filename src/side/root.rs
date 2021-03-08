use std::sync::mpsc::Sender;
use crate::pane::{LayoutConstraint, Pane, FlexPane, Layout};
use crate::side::{Editor, Event};
use crate::side::{FileMenu, TabBar};

/// 

///
pub struct Manager {
    layout: Vec<(Box<dyn Pane<Event>>, LayoutConstraint)>,
    selected: usize,
}

impl Manager {
    pub fn new(emiter: Sender<Event>) -> Manager {
        Manager {
            layout: vec![
                (Box::new(TabBar::new(vec![
                    ('F', Box::new(FileMenu::new(emiter.clone()))),
                    ('B', Box::new(FileMenu::new(emiter))),
                ])), LayoutConstraint::Fixed(40)),
                (Box::new(Editor::new("".to_string())), LayoutConstraint::Flex(1)),
            ],
            selected: 0,
        }
    }

}

impl FlexPane<Event> for Manager {
    fn get_layout(&self) -> &Layout<Event> {
        &self.layout
    }

    fn get_selected_pane(&self) -> &dyn Pane<Event> {
        self.layout[self.selected].0.as_ref()
    }
    
    fn get_selected_pane_mut(&mut self) -> &mut dyn Pane<Event> {
        self.layout[self.selected].0.as_mut()
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
