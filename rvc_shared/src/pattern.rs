use anyhow::Result;
use bincode::{Decode, Encode};
use std::fs;

#[derive(Decode, Encode)]
pub struct Pattern {
    pub width: u32,
    pub height: u32,
    pub levels: u32,
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

    pub fn get(&self, x: u32, y: u32) -> u32 {
        self.data[(x + y * self.width) as usize]
    }

    pub fn save(&self, filename: String) -> Result<()> {
        let mut file = fs::File::create(filename)?;
        let config = bincode::config::standard();
        bincode::encode_into_std_write(self, &mut file, config)?;
        Ok(())
    }
}
