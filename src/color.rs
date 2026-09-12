#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn scale(self, intensity: f32) -> Self {
        Self {
            r: (self.r as f32 * intensity).clamp(0.0, 255.0) as u8,
            g: (self.g as f32 * intensity).clamp(0.0, 255.0) as u8,
            b: (self.b as f32 * intensity).clamp(0.0, 255.0) as u8,
        }
    }

    pub fn modulate(self, other: Self) -> Self {
        Self {
            r: ((self.r as u16 * other.r as u16) / 255) as u8,
            g: ((self.g as u16 * other.g as u16) / 255) as u8,
            b: ((self.b as u16 * other.b as u16) / 255) as u8,
        }
    }

    pub const fn to_hex(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | self.b as u32
    }
}
