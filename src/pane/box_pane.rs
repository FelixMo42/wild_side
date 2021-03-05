use crate::pane::{Renderer, Pane};
use crate::util::Span;

pub struct BoxPane<'a, T> {
    pub child: &'a mut dyn Pane<T>,
}

impl<'a, T> BoxPane<'a, T> {
    pub fn new(child: &'a mut dyn Pane<T>) -> BoxPane<T> {
        return BoxPane { child };
    }
}

impl<'a, T> Pane<T> for BoxPane<'a, T> {
    fn get_size(&self) -> Span {
        return self.child.get_size().add(2, 2);
    }

    fn render(&self, renderer: Renderer) {
        // get the size we need to fill
        let size = renderer.size().sub(1, 1);

        // render corners
        renderer.draw(0, 0, "┌");
        renderer.draw(size.x, 0, "┐");
        renderer.draw(0, size.y, "└");
        renderer.draw(size.x, size.y, "┘");

        // render horizontal bars
        for x in 1..size.x {
            renderer.draw(x, 0, "─");
            renderer.draw(x, size.y, "─");
        }

        // render vertical bars
        for y in 1..size.y {
            renderer.draw(0, y, "│");
            renderer.draw(size.x, y, "│");
        }

        // render my child
        renderer.draw_pane(self.child, Span::new(1, 1), size.sub(1, 1));
    }

    fn event(&mut self, event: T) -> bool {
        return self.child.event(event);
    }
}
