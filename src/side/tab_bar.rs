use crate::color::{GRAY3, GRAY8, GRAY9};
use crate::pane::*;
use crate::side::*;
use crate::util::*;

pub struct TabBar {
    tabs: Vec<(char, Box<dyn Pane<Event>>)>,
    selected: usize
}

impl TabBar {
    pub fn new(tabs: Vec<(char, Box<dyn Pane<Event>>)>) -> Box<TabBar> {
        Box::new(TabBar {
            tabs,
            selected: 0
        })
    }
}

impl Pane<Event> for TabBar {
    fn render(&self, canvas: Canvas) {
        let (mut left, right) = canvas.splith(4);
            
        // canvas.style_area(&GRAY8.as_bg(), area);
        // canvas.style_area(&GRAY3.as_fg(), area);
        
        for (i, c) in self.tabs.iter().enumerate() {
            left.draw_line((1, i * 3 + 1).into(), c.0.to_string());
            
            /* if i == open {
                canvas.style_area(
                    &GRAY9.as_bg(),
                    area.vertical_slice(i * 3, i * 3 + 3)
                );
            } */
        }

        right.draw_pane(&self.tabs[self.selected].1);
    }

    fn event(&mut self, event: Event) {
        self.tabs[self.selected].1.event(event);
    }
}
