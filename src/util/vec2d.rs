use crate::util::Span;

pub struct Vec2d<T: Copy> {
    size: Span,
    data: Vec<T>,
}

impl<T: Copy> Vec2d<T> {
    pub fn new(size: Span, init: T) -> Vec2d<T> {
        Vec2d {
            data: vec![init; size.x * size.y],
            size,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        self.data[y * self.size.x + x]
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.data[y * self.size.x + x] = val;
    }
}
