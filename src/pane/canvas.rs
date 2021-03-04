use super::pane::Pane;
use crate::util::{Area, Size};
use std::sync::{Arc, Mutex};

pub struct Canvas<'a> {
    size: Size,
    text: Vec<String>,
    root: &'a dyn Pane,
}

impl<'a> Canvas<'a> {
    pub fn new(root: &'a dyn Pane, size: (u16, u16)) -> Canvas<'a> {
        Canvas {
            size: Size::new(size.0 as usize, size.1 as usize),
            text: vec![" ".repeat(size.0 as usize); size.1 as usize],
            root,
        }
    }

    fn area(&self) -> Area {
        return self.size.area();
    }

    pub fn render(&mut self) -> String {
        //
        let area = self.area();

        //
        let arc = Arc::new(Mutex::new(&mut self.text));

        //
        self.root.render(Renderer::new(arc, area));

        // return the formated text
        return self.print();
    }

    fn print(&mut self) -> String {
        return self
            .text
            .iter()
            .enumerate()
            .map(|(i, line)| format!("{}{}", termion::cursor::Goto(1, (1 + i) as u16), line))
            .collect::<String>();
    }
}

pub struct Renderer<'a> {
    area: Area,
    text: Arc<Mutex<&'a mut Vec<String>>>, // canvas: &'a mut Canvas<'a>
}

impl<'a> Renderer<'a> {
    pub fn new(text: Arc<Mutex<&'a mut Vec<String>>>, area: Area) -> Renderer<'a> {
        return Renderer { text, area };
    }

    pub fn size(&self) -> Size {
        return self.area.size();
    }

    pub fn echo(&self, sx: usize, sy: usize, text: &str) {
        let x = sx + self.area.0.x;
        let y = sy + self.area.0.y;

        let mut lock = self.text.lock().unwrap();

        let line = lock.get_mut(y).unwrap();

        let start = line.char_indices().nth(x).unwrap().0;

        let end = line
            .char_indices()
            .nth(x + text.chars().count() - 1)
            .unwrap()
            .0;

        line.replace_range(start..=end, text);
    }

    pub fn draw(&self, pane: &dyn Pane, spot: Size, size: Size) {
        pane.render(Renderer::new(
            self.text.clone(),
            Area::new(self.area.0.shift(&spot), spot.shift(&size).as_tuple()),
        ));
    }
}
