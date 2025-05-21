#[derive(Clone)]
pub struct Plane<T> {
    pub width: u32,
    pub height: u32,
    pub data: Vec<T>,
}

impl<T> Plane<T>
where
    T: Copy,
{
    pub fn new(width: u32, height: u32, init: T) -> Plane<T> {
        Plane {
            width,
            height,
            data: vec![init; (width * height) as usize],
        }
    }

    pub fn set(&mut self, x: u32, y: u32, value: T) {
        self.data[(x + y * self.width) as usize] = value;
    }

    pub fn get(&self, x: u32, y: u32) -> T {
        self.data[(x + y * self.width) as usize]
    }
}
