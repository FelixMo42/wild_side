use crate::pane::canvas::Renderer;
use crate::util::Size;

///
pub struct Bounds {
    pub min: Size,
    pub max: Size,
}

impl Bounds {
    pub fn new(size: Size) -> Bounds {
        Bounds {
            min: Size::new(0, 0),
            max: size,
        }
    }

    pub fn shrink(&self, num: usize) -> Bounds {
        return Bounds {
            min: self.min.clone(),
            max: self.max.sub(num, num),
        };
    }
}
/// A pane is a genaric building block of the ui
pub trait Pane {
    fn get_size(&self, bounds: Bounds) -> Size;
    fn render(&self, renderer: Renderer);
}
