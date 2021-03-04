use crate::pane::canvas::Renderer;

///
#[derive(Clone, Copy)]
pub struct Size {
    pub w: usize,
    pub h: usize
}

impl Size {
    pub fn new(w: usize, h: usize) -> Size {
        return Size { w, h };
    }

    pub fn in_bounds(&self, min: Size, max: Size) -> bool {
        return
            self.w >= min.w && self.w <= max.w &&
            self.h >= min.h && self.h <= min.h; 
    }

    pub fn add(&self, num: usize) -> Size {
        Size::new(self.w + num, self.h + num)
    }

    pub fn sub(&self, num: usize) -> Size {
        Size::new(self.w - num, self.h - num)
    }

    pub fn shift(&self, size: &Size) -> Size {
        return Size {
            w: self.w + size.w,
            h: self.h + size.h
        }
    }
}

///
pub struct Bounds {
    pub min: Size,
    pub max: Size
}

impl Bounds {
    pub fn new(size: Size) -> Bounds {
        Bounds {
            min: Size::new(0, 0),
            max: size
        }
    }
    
    pub fn shrink(&self, num: usize) -> Bounds {
        return Bounds {
            min: self.min.clone(),
            max: self.max.sub(num)
        }
    }
}
/// A pane is a genaric building block of the ui
pub trait Pane {
    fn get_size(&self, bounds: Bounds) -> Size;
    fn render(&self, renderer: Renderer);
}
