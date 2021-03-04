use crate::pane::{Renderer, Pane};
use crate::util::Span;

pub struct LinePane<'a> {
    pub child: &'a dyn Pane,
}

/* impl <'a> BoxPane <'a> {
    pub fn new(child: &'a dyn Pane) -> BoxPane {
       return BoxPane { child };
    }
} */

impl<'a> Pane for LinePane<'a> {
    fn get_size(&self) -> Span {
        return self.child.get_size().add(4, 0);
    }

    fn render(&self, renderer: Renderer) {
        // get the size we need to fill
        let size = renderer.size().sub(1, 1);

        // render the line numbers
        for y in 0..size.y {
            renderer.draw(0, y, &format!("{}", y)[..]);
        }

        // render my child
        renderer.draw_pane(self.child, Span::new(4, 0), size.sub(0, 0));
    }
}
