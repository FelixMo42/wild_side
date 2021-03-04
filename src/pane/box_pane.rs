use crate::pane::{Renderer, Pane};
use crate::util::Span;

pub struct BoxPane<'a> {
    pub child: &'a dyn Pane,
}

impl<'a> BoxPane<'a> {
    pub fn new(child: &'a dyn Pane) -> BoxPane {
        return BoxPane { child };
    }
}

impl<'a> Pane for BoxPane<'a> {
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
}
