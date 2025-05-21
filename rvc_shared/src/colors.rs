use bincode::{Decode, Encode};
use std::cmp::min;

#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct FloatColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub trait RGBColor {
    fn distance2(&self, other: &Self) -> f64;
    fn luminocity(&self) -> f64;
}

impl FloatColor {
    fn distance(&self, other: &FloatColor) -> f64 {
        let dr = self.r - other.r;
        let dg = self.g - other.g;
        let db = self.b - other.b;

        (dr * dr + dg * dg + db * db).sqrt()
    }

    pub fn new(r: i32, g: i32, b: i32) -> FloatColor {
        FloatColor {
            r: (r as f64) / 255.0,
            g: (g as f64) / 255.0,
            b: (b as f64) / 255.0,
        }
    }

    pub const BLACK: FloatColor = FloatColor { r: 0.0, g: 0.0, b: 0.0 };
}

impl RGBColor for FloatColor {
    fn distance2(&self, other: &FloatColor) -> f64 {
        let dr = self.r - other.r;
        let dg = self.g - other.g;
        let db = self.b - other.b;

        dr * dr + dg * dg + db * db
    }

    fn luminocity(&self) -> f64 {
        0.299 * self.r + 0.587 * self.g + 0.114 * self.b
    }
}

impl From<IntColor> for FloatColor {
    fn from(color: IntColor) -> Self {
        FloatColor {
            r: (color.r as f64) / 255.0,
            g: (color.g as f64) / 255.0,
            b: (color.b as f64) / 255.0,
        }
    }
}

impl From<&IntColor> for FloatColor {
    fn from(color: &IntColor) -> Self {
        FloatColor {
            r: (color.r as f64) / 255.0,
            g: (color.g as f64) / 255.0,
            b: (color.b as f64) / 255.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Decode, Encode)]
pub struct IntColor {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

impl IntColor {
    pub fn new(r: i32, g: i32, b: i32) -> IntColor {
        IntColor { r, g, b }
    }

    pub const BLACK: IntColor = IntColor { r: 0, g: 0, b: 0 };
}

impl RGBColor for IntColor {
    fn distance2(&self, other: &IntColor) -> f64 {
        let dr = self.r - other.r;
        let dg = self.g - other.g;
        let db = self.b - other.b;

        (dr * dr + dg * dg + db * db) as f64
    }

    fn luminocity(&self) -> f64 {
        0.299 * self.r as f64 + 0.587 * self.g as f64 + 0.114 * self.b as f64
    }
}

impl From<FloatColor> for IntColor {
    fn from(color: FloatColor) -> Self {
        IntColor {
            r: min(255, (color.r * 256.0) as i32),
            g: min(255, (color.g * 256.0) as i32),
            b: min(255, (color.b * 256.0) as i32),
        }
    }
}

impl From<&FloatColor> for IntColor {
    fn from(color: &FloatColor) -> Self {
        IntColor {
            r: min(255, (color.r * 256.0) as i32),
            g: min(255, (color.g * 256.0) as i32),
            b: min(255, (color.b * 256.0) as i32),
        }
    }
}
