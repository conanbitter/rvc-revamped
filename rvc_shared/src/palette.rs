use std::{fs, path::PathBuf};

use crate::colors::{FloatColor, IntColor};
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct Palette(Vec<FloatColor>);

impl Palette {
    pub fn new() -> Palette {
        Palette(Vec::with_capacity(256))
    }

    pub fn add(&mut self, color: FloatColor) {
        self.0.push(color);
    }

    pub fn get(&self, index: i32) -> FloatColor {
        self.0[index as usize]
    }

    pub fn sort(&mut self) {
        self.0.sort_by(|a, b| a.luminocity().total_cmp(&b.luminocity()));
    }

    pub fn save(&self, filename: String) -> Result<()> {
        let mut data = vec![IntColor::BLACK; self.0.len()];

        for (icol, fcol) in data.iter_mut().zip(self.0.iter()) {
            *icol = IntColor::from(fcol);
        }

        let mut file = fs::File::create(filename)?;
        let config = bincode::config::standard();
        bincode::encode_into_std_write(data, &mut file, config)?;
        Ok(())
    }

    pub fn from_file(filename: PathBuf) -> Result<Palette> {
        let mut file = fs::File::open(filename)?;
        let config = bincode::config::standard();
        let data: Vec<IntColor> = bincode::decode_from_std_read(&mut file, config)?;

        let mut result = Palette::new();

        for icol in data {
            result.0.push(FloatColor::from(icol));
        }
        Ok(result)
    }

    pub fn find(&self, color: FloatColor) -> i32 {
        let mut best_index = 0;
        let mut best_distance = f64::MAX;
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
