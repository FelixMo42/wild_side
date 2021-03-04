use super::pane::{Pane, Bounds, Size};
use super::canvas::Renderer;

pub struct BoxPane <'a> {
    pub child: &'a dyn Pane
}

impl <'a> BoxPane <'a> {
    pub fn new(child: &'a dyn Pane) -> BoxPane {
       return BoxPane { child };
    }
}

impl <'a> Pane for BoxPane <'a> {
    fn get_size(&self, bounds: Bounds) -> Size {
        return self.get_size( bounds.shrink(2) ).add(2);
    }

    fn render(&self, renderer: Renderer) {
        // get the size we need to fill
        let size = renderer.size().sub(1);

        // render corners
        renderer.echo(0     , 0     , "┌");
        renderer.echo(size.w, 0     , "┐");
        renderer.echo(0     , size.h, "└");
        renderer.echo(size.w, size.h, "┘");
       
        // render horizontal bars
        for x in 1..size.w {
            renderer.echo(x     , 0     , "─");
            renderer.echo(x     , size.h, "─");
        }
        
        // render vertical bars
        for y in 1..size.h {
            renderer.echo(0     , y, "│");
            renderer.echo(size.w, y, "│");
        }

        // render my child
        renderer.draw(self.child, Size::new(1, 1), size.sub(1));
    }
}
