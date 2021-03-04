use super::canvas::Renderer;
use super::pane::{Bounds, Pane};
use crate::util::Size;

pub struct LinePane<'a> {
    pub child: &'a dyn Pane,
}

/* impl <'a> BoxPane <'a> {
    pub fn new(child: &'a dyn Pane) -> BoxPane {
       return BoxPane { child };
    }
} */

impl<'a> Pane for LinePane<'a> {
    fn get_size(&self, bounds: Bounds) -> Size {
        return self.get_size(bounds.shrink(2)).add(2, 2);
    }

    fn render(&self, renderer: Renderer) {
        // get the size we need to fill
        let size = renderer.size().sub(1, 1);

        // render the line numbers
        for y in 0..size.h {
            renderer.echo(0, y, &format!("{}", y)[..]);
        }

        // render my child
        renderer.draw(self.child, Size::new(4, 0), size.sub(0, 0));
    }
}
