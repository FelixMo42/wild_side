///
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

    pub fn fg_cmd(&self) -> String {
        format!("\x1B[38;2;{};{};{}m", self.r, self.g, self.b)
    }

    pub fn bg_cmd(&self) -> String {
        format!("\x1B[48;2;{};{};{}m", self.r, self.g, self.b)
    }

    pub fn as_fg(self) -> Style {
        return Style::fg(self);
    }

    pub fn as_bg(self) -> Style {
        return Style::bg(self);
    }
}

///
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}

impl Style {
    pub fn new(fg: Option<Color>, bg: Option<Color>) -> Style {
        return Style { fg, bg };
    }

    pub fn fg(fg: Color) -> Style {
        return Style::new(Some(fg), None);
    }

    pub fn bg(bg: Color) -> Style {
        return Style::new(None, Some(bg));
    }
}

pub const GRAY0: Color = Color {
    r: 215,
    g: 221,
    b: 227,
};
pub const GRAY1: Color = Color {
    r: 178,
    g: 185,
    b: 192,
};
pub const GRAY2: Color = Color {
    r: 140,
    g: 148,
    b: 156,
};
pub const GRAY3: Color = Color {
    r: 116,
    g: 125,
    b: 135,
};
pub const GRAY4: Color = Color {
    r: 93,
    g: 102,
    b: 113,
};
pub const GRAY5: Color = Color {
    r: 72,
    g: 79,
    b: 89,
};
pub const GRAY6: Color = Color {
    r: 60,
    g: 67,
    b: 77,
};
pub const GRAY7: Color = Color {
    r: 48,
    g: 54,
    b: 62,
};
pub const GRAY8: Color = Color {
    r: 33,
    g: 38,
    b: 45,
};
pub const GRAY9: Color = Color {
    r: 22,
    g: 27,
    b: 34,
};

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
