use crate::color::{GRAY3, GRAY8, GRAY9};
use crate::pane::{Canvas, Pane};
use crate::side::Event;
use crate::util::Span;

pub struct TabBar {
    tabs: Vec<(char, Box<dyn Pane<Event>>)>,
    selected: usize
}

impl TabBar {
    pub fn new(tabs: Vec<(char, Box<dyn Pane<Event>>)>) -> TabBar {
        TabBar {
            tabs,
            selected: 0
        }
    }
}

impl Pane<Event> for TabBar {
    fn render(&self, canvas: Canvas, selected: bool) {
        let area = canvas.area();

        canvas.style_area(&GRAY8.as_bg(), area);
        canvas.style_area(&GRAY3.as_fg(), area);
        
        let open = 0;

        for (i, c) in self.tabs.iter().enumerate() {
            canvas.draw((1, i * 3 + 1).into(), c.0.to_string().as_str());
            
            if i == open {
                canvas.style_area(
                    &GRAY9.as_bg(),
                    area.vertical_slice(i * 3, i * 3 + 3)
                );
            }
        }

        canvas.draw_pane(
            self.tabs[self.selected].1.as_ref(),
            area.shrink((3, 0).into(), (0, 0).into()),
            selected
        );
    }

    fn event(&mut self, event: Event) {
        self.tabs[self.selected].1.event(event);
    }
}
