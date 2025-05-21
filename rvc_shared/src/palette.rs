use std::{fs, i32, path::PathBuf};

use crate::colors::IntColor;
use anyhow::{Ok, Result};
use bincode::{Decode, Encode};

#[derive(Clone, Debug, PartialEq, Decode, Encode)]
pub struct Palette(Vec<IntColor>);

impl Palette {
    pub fn new() -> Palette {
        Palette(Vec::with_capacity(256))
    }

    pub fn add(&mut self, color: IntColor) {
        self.0.push(color);
    }

    pub fn get(&self, index: i32) -> IntColor {
        self.0[index as usize]
    }

    pub fn sort(&mut self) {
        self.0.sort_by(|a, b| a.luminocity().total_cmp(&b.luminocity()));
    }

    pub fn save(&self, filename: String) -> Result<()> {
        let mut file = fs::File::create(filename)?;
        let config = bincode::config::standard();
        bincode::encode_into_std_write(self, &mut file, config)?;
        Ok(())
    }

    pub fn from_file(filename: PathBuf) -> Result<Palette> {
        let mut file = fs::File::open(filename)?;
        let config = bincode::config::standard();
        Ok(bincode::decode_from_std_read(&mut file, config)?)
    }

    pub fn find(&self, color: IntColor) -> i32 {
        let mut best_index = 0;
        let mut best_distance = i32::MAX;
        for (i, palcol) in self.0.iter().enumerate() {
            let distance = color.distance_squared(*palcol);
            if distance < best_distance {
                best_distance = distance;
                best_index = i;
            }
        }
        best_index as i32
    }
}
