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

    pub fn brighten(&self, p: f32) -> Color {
        let r = 1.0 - p;
        Color {
            r: (self.r as f32 * r + 255.0 * p) as u8,
            g: (self.g as f32 * r + 255.0 * p) as u8,
            b: (self.b as f32 * r + 255.0 * p) as u8,
        }
    }
    
    pub fn darken(&self, p: f32) -> Color {
        let r = 1.0 - p;
        Color {
            r: (self.r as f32 * r) as u8,
            g: (self.g as f32 * r) as u8,
            b: (self.b as f32 * r) as u8,
        }
    }

    pub fn layer(&self, layer: usize) -> Color {
        self.brighten(layer as f32 * 0.025)
    }
}

impl Into<Color> for (u8, u8, u8) {
    fn into(self) -> Color {
        Color {
            r: self.0,
            g: self.1,
            b: self.2
        }
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

///
pub struct Theme {
    pub base: Color
}

impl Theme {
    pub fn bg(&self, layer: usize) -> Color {
        self.base.layer(layer)
    }

    
    pub fn focused(&self, layer: usize) -> Color {
        self.base.layer(layer).brighten(0.87)
    }
    
    pub fn normal(&self, layer: usize) -> Color {
        self.base.layer(layer).brighten(0.6)
    }

    pub fn disabled(&self, layer: usize) -> Color {
        self.base.layer(layer).brighten(0.38)
    }


    pub fn style(&self, layer: usize) -> Style {
        Style {
            fg: Some(self.normal(layer)),
            bg: Some(self.bg(layer)),
        }
    }
}

pub const THEME: Theme = Theme {
    base: Color { r: 33, g: 33, b: 33 }
};
