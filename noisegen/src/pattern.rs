use bincode::{Decode, Encode};

#[derive(Decode, Encode)]
pub struct Pattern {
    width: u32,
    height: u32,
    levels: u32,
    data: Vec<u32>,
}

impl Pattern {
    pub fn new(width: u32, height: u32, levels: u32) -> Pattern {
        Pattern {
            width,
            height,
            levels,
            data: vec![0; (width * height) as usize],
        }
    }

    pub fn set(&mut self, x: u32, y: u32, level: u32) {
        self.data[(x + y * self.width) as usize] = level;
    }
}
