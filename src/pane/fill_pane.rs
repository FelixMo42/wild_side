use crate::pane::*;

pub struct FillPane {
    chr: char,
}

impl FillPane {
    pub fn new(chr: char) -> Box<FillPane> {
        return Box::new(FillPane { chr });
    }
}

impl<Event> pane::Pane<Event> for FillPane {
    fn render(&self, mut canvas: Canvas, _focused: bool) {
        let area = canvas.area();
        for x in area.0.x..area.1.x {
            for y in area.0.y..area.1.y {
                canvas.draw_char((x, y).into(), self.chr);
            }
        }
    }

    fn event(&mut self, _event: Event) {}
}
