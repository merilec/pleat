use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color ({} {} {} {})", self.r, self.g, self.b, self.a)
    }
}

impl Color {
    pub fn new(value: u16) -> Color {
        Color {
            r: ((value & 31) * 8) as u8,
            g: (((value >> 5) & 31) * 8) as u8,
            b: (((value >> 10) & 31) * 8) as u8,
            a: 0xFF,
        }
    }
    pub fn to_rgb(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
    pub fn to_rgba(&self) -> u32 {
        ((self.r as u32) << 24)
            + ((self.g as u32) << 16)
            + ((self.b as u32) << 8)
            + 0xFF
    }
}
