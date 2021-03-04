//
#[derive(Copy, Clone)]
pub struct Size {
    pub w: usize,
    pub h: usize,
}

impl Size {
    pub fn new(w: usize, h: usize) -> Size {
        Size { w, h }
    }

    pub fn area(&self) -> Area {
        Area::new((0, 0), (self.w, self.h))
    }

    pub fn add(&self, w: usize, h: usize) -> Size {
        Size::new(self.w + w, self.h + h)
    }

    pub fn sub(&self, w: usize, h: usize) -> Size {
        Size::new(self.w - w, self.h - h)
    }

    pub fn shift(&self, size: &Size) -> Size {
        return Size {
            w: self.w + size.w,
            h: self.h + size.h,
        };
    }

    pub fn as_tuple(&self) -> (usize, usize) {
        return (self.w, self.h);
    }
}

//
pub struct Spot {
    pub x: usize,
    pub y: usize,
}

impl Spot {
    pub fn shift(&self, size: &Size) -> (usize, usize) {
        return (self.x + size.w, self.y + size.h);
    }
}

//
pub struct Area(pub Spot, pub Spot);

impl Area {
    pub fn new(a: (usize, usize), b: (usize, usize)) -> Area {
        Area {
            0: Spot { x: a.0, y: a.1 },
            1: Spot { x: b.0, y: b.1 },
        }
    }

    pub fn size(&self) -> Size {
        return Size {
            w: self.1.x - self.0.x,
            h: self.1.y - self.0.y,
        };
    }
}
