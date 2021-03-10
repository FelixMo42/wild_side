use crate::pane::*;
use crate::side::*;
use crate::color::*;

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
    fn render(&self, canvas: Canvas, focused: bool) {
        let (mut left, right) = canvas.splith(3);
        let area = left.area();

        for (i, c) in self.tabs.iter().enumerate() {
            
            if i == self.selected {
                left.style_area(
                    area.vertical_slice(i * 3, i * 3 + 3),
                    Style::new(
                        Some(THEME.normal(1)),
                        Some(THEME.bg(1))
                    )
                );
            }
            
            left.draw_char((1, i * 3 + 1).into(), c.0);
        }

        right.draw_pane(&self.tabs[self.selected].1, focused);
    }

    fn event(&mut self, event: Event) {
        self.tabs[self.selected].1.event(event);
    }
}
