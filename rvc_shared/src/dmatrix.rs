use anyhow::Result;
use bincode::{Decode, Encode};
use std::fs;

#[derive(Decode, Encode)]
pub struct DitherMatrix {
    pub width: u32,
    pub height: u32,
    pub levels: u32,
    data: Vec<u32>,
}

impl DitherMatrix {
    pub fn new(width: u32, height: u32, levels: u32) -> DitherMatrix {
        DitherMatrix {
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
        self.data[((x % self.width) + (y % self.height) * self.width) as usize]
    }

    pub fn save(&self, filename: String) -> Result<()> {
        let mut file = fs::File::create(filename)?;
        let config = bincode::config::standard();
        bincode::encode_into_std_write(self, &mut file, config)?;
        Ok(())
    }

    pub fn from_file(filename: String) -> Result<DitherMatrix> {
        let mut file = fs::File::open(filename)?;
        let config = bincode::config::standard();
        Ok(bincode::decode_from_std_read(&mut file, config)?)
    }
}
