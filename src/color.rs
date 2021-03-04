#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        return Color { r, g, b };
    }

    pub fn fg(&self) -> String {
        format!("\x1B[38;2;{};{};{}m", self.r, self.g, self.b)
    }

    pub fn bg(&self) -> String {
        format!("\x1B[48;2;{};{};{}m", self.r, self.g, self.b)
    }
}

pub const PINK0: Color = Color {
    r: 255,
    g: 218,
    b: 236,
};
pub const PINK1: Color = Color {
    r: 255,
    g: 190,
    b: 221,
};
pub const PINK2: Color = Color {
    r: 255,
    g: 155,
    b: 206,
};
pub const PINK3: Color = Color {
    r: 247,
    g: 120,
    b: 186,
};
pub const PINK4: Color = Color {
    r: 219,
    g: 97,
    b: 162,
};
pub const PINK5: Color = Color {
    r: 191,
    g: 75,
    b: 138,
};
pub const PINK6: Color = Color {
    r: 158,
    g: 54,
    b: 112,
};
pub const PINK7: Color = Color {
    r: 125,
    g: 36,
    b: 87,
};
pub const PINK8: Color = Color {
    r: 94,
    g: 16,
    b: 62,
};
pub const PINK9: Color = Color { r: 66, g: 6, b: 42 };
