use crate::util::Size;

pub struct Vec2d<T: Copy> {
    size: Size,
    data: Vec<T>,
}

impl<T: Copy> Vec2d<T> {
    pub fn new(size: Size, init: T) -> Vec2d<T> {
        Vec2d {
            data: vec![init; size.w * size.w],
            size,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        self.data[y * self.size.w + x]
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.data[y * self.size.w + x] = val;
    }
}
