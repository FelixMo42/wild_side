///
#[derive(Clone, Copy)]
pub struct Span {
    pub x: usize,
    pub y: usize,
}

impl Span {
    pub fn new(x: usize, y: usize) -> Span {
        return Span { x, y };
    }

    pub fn shift(&self, offset: &Span) -> Span {
        return Span::new(self.x + offset.x, self.y + offset.y);
    }

    pub fn area(&self) -> Area {
        Area::new((0, 0).into(), (self.x, self.y).into())
    }

    pub fn add(&self, x: usize, y: usize) -> Span {
        Span::new(self.x + x, self.x + y)
    }

    pub fn sub(&self, x: usize, y: usize) -> Span {
        Span::new(self.x - x, self.y - y)
    }

    pub fn as_tuple(&self) -> (usize, usize) {
        return (self.x, self.y);
    }
}

impl Into<Span> for (usize, usize) {
    fn into(self) -> Span {
        return Span::new(self.0 as usize, self.1 as usize);
    }
}

///
#[derive(Clone, Copy)]
pub struct Area(pub Span, pub Span);

impl Area {
    pub fn new(a: Span, b: Span) -> Area {
        return Area { 0: a, 1: b };
    }

    pub fn shift(&self, span: &Span) -> Area {
        Area {
            0: self.0.shift(span),
            1: self.1.shift(span),
        }
    }

    pub fn size(&self) -> Span {
        return Span {
            x: self.1.x - self.0.x,
            y: self.1.y - self.0.y,
        };
    }

    pub fn horizontal_slice(&self, start: usize, end: usize) -> Area {
        return Area::new((start, self.0.y).into(), (end, self.1.y).into());
    }
}
