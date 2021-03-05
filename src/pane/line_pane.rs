use crate::pane::{Renderer, Pane};
use crate::util::Span;
use crate::color::{Style, GRAY5};

pub struct LinePane<'a, Event> {
    pub child: &'a mut dyn Pane<Event>,
}

impl<'a, Event> Pane<Event> for LinePane<'a, Event> {
    fn get_size(&self) -> Span {
        return self.child.get_size().add(4, 0);
    }

    fn render(&self, renderer: Renderer) {
        let size = renderer.size();

        for y in 0..size.y {
            renderer.draw_line_with_style(
                (0, y).into(),
                &format!("{:>3}", y)[..],
                Style::fg(GRAY5)
            );
        }

        renderer.draw_pane(self.child, Span::new(4, 0), size.sub(0, 0));
    }
    
    fn event(&mut self, event: Event) -> bool {
        return self.child.event(event);
    }
}
