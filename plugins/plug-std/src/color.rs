#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::rgba(r, g, b, 0xFF)
    }

    pub fn rgbaf(r: f64, g: f64, b: f64, a: f64) -> Self {
        Color::rgba(
            (r * 0xFF as f64) as u8,
            (g * 0xFF as f64) as u8,
            (b * 0xFF as f64) as u8,
            (a * 0xFF as f64) as u8,
        )
    }

    pub fn rgbf(r: f64, g: f64, b: f64) -> Self {
        Color::rgbaf(r, g, b, 1.0)
    }
}
