#[derive(Debug, Clone, Copy)]
pub struct FloatColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl FloatColor {
    pub fn distance(&self, other: FloatColor) -> f64 {
        let dr = self.r - other.r;
        let dg = self.g - other.g;
        let db = self.b - other.b;

        (dr * dr + dg * dg + db * db).sqrt()
    }

    pub fn distance_squared(&self, other: FloatColor) -> f64 {
        let dr = self.r - other.r;
        let dg = self.g - other.g;
        let db = self.b - other.b;

        dr * dr + dg * dg + db * db
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
