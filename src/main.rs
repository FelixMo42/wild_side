mod canvas;
mod pane;

use std::fs;
use canvas::Renderer;
use pane::{Pane, Size, Bounds};

/// Text panes draw, well, a bunch of text
struct TextPane {
    text: Vec<String>
}

impl TextPane {
    fn new(text: String) -> TextPane {
        return TextPane {
            text: text
                .split("\n")
                .into_iter()
                .map(|line| line.to_string())
                .collect()
        };
    }
}

impl Pane for TextPane {
    fn get_size(&self, bounds: Bounds) -> Size {
        return Size {
            w: bounds.max.w,
            h: self.text.len()
        };
    }

    fn render(&self, renderer: Renderer) {
        let size = renderer.size();
    
        for y in 0..size.h {
            let end =
                self.text[y]
                .char_indices()
                .nth(size.w)
                .unwrap_or((self.text[y].chars().count(), ' ')).0;

            if end == 0 {
                continue;
            }

            let line = &self.text[y][..end];
            renderer.echo(0, y, line);
        }
    }
}


pub struct BoxPane <'a> {
    child: &'a dyn Pane
}

impl <'a> BoxPane <'a> {
    fn new(child: &'a dyn Pane) -> BoxPane {
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


fn main() {
    // what file do we want to display?
    let path = "./src/main.rs";

    // read the contents of the file
    let contents = fs::read_to_string(path)
        .expect("could not open file!");

    // the root node the document
    let root = BoxPane {
        child: &TextPane::new(contents)
    };

    // create the actuall rendering canvas
    let mut renderer = canvas::Canvas::new(&root, Size::new(50, 50));

    // render it
    println!("{}", renderer.render());
}
