use crate::colors::RGBColor;
use anyhow::Result;
use bincode::{Decode, Encode};
use std::{fs, path::PathBuf};

#[derive(Clone, Debug, PartialEq, Decode, Encode)]
pub struct Palette<T>(Vec<T>);

impl<T> Palette<T>
where
    T: RGBColor + Copy + Clone + Encode + bincode::Decode<()>,
{
    pub fn new() -> Palette<T> {
        Palette(Vec::with_capacity(256))
    }

    pub fn add(&mut self, color: T) {
        self.0.push(color);
    }

    pub fn get(&self, index: i32) -> T {
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

    pub fn from_file(filename: PathBuf) -> Result<Palette<T>> {
        let mut file = fs::File::open(filename)?;
        let config = bincode::config::standard();
        Ok(bincode::decode_from_std_read(&mut file, config)?)
    }

    pub fn find(&self, color: T) -> i32 {
        let mut best_index = 0;
        let mut best_distance = f64::MAX;
        for (i, palcol) in self.0.iter().enumerate() {
            let distance = color.distance2(palcol);
            if distance < best_distance {
                best_distance = distance;
                best_index = i;
            }
        }
        best_index as i32
    }

    pub fn from<P>(other: &Palette<P>) -> Palette<T>
    where
        P: Into<T> + Clone + Copy,
    {
        let mut result = Palette::<T>::new();
        for color in other.0.iter() {
            result.0.push(P::into(*color));
        }
        result
    }
}

impl<T> Default for Palette<T>
where
    T: RGBColor + Copy + Clone + Encode + bincode::Decode<()>,
{
    fn default() -> Self {
        Palette::new()
    }
}
