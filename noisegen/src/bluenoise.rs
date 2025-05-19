use rand::Rng;

const SIGMA: f64 = 1.5;
const DIVISOR: f64 = SIGMA * SIGMA * 2.0;

#[derive(Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

struct FloatImage {
    width: u32,
    height: u32,
    data: Vec<f64>,
}

impl FloatImage {
    fn new(width: u32, height: u32) -> FloatImage {
        FloatImage {
            width,
            height,
            data: vec![0.0; (width * height) as usize],
        }
    }

    fn set(&mut self, x: u32, y: u32, value: f64) {
        self.data[(x + y * self.width) as usize] = value;
    }

    fn get(&self, x: u32, y: u32) -> f64 {
        self.data[(x + y * self.width) as usize]
    }
}

fn generate_lut(width: u32, height: u32) -> FloatImage {
    let mut res = FloatImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let xd = if x < width / 2 { x } else { width - x };
            let yd = if y < height / 2 { y } else { height - y };
            let distance = (xd * xd + yd * yd) as f64;
            res.set(x, y, (-distance / DIVISOR).exp())
        }
    }
    res
}

fn apply_point(x: u32, y: u32, add: bool, image: &mut FloatImage, mask: &mut FloatImage, lut: &FloatImage) {
    let sign = if add {
        image.set(x, y, 1.0);
        1.0
    } else {
        image.set(x, y, 0.0);
        -1.0
    };

    for yp in 0..mask.height {
        let ylut = if yp < y { mask.height + yp - y } else { yp - y };
        for xp in 0..mask.width {
            let xlut = if xp < x { mask.width + xp - y } else { xp - x };
            mask.set(xp, yp, mask.get(xp, yp) + lut.get(xlut, ylut) * sign)
        }
    }
}

fn start_fill(image: &mut FloatImage, mask: &mut FloatImage, lut: &FloatImage, quantity: f64) -> u32 {
    // Jittered grid method
    let cols = ((image.width as f64) * quantity) as u32;
    let cell_width = image.width / cols;
    let rows = ((image.height as f64) * quantity) as u32;
    let cell_height = image.height / rows;

    let mut rng = rand::rng();
    for y in 0..rows {
        for x in 0..cols {
            let xr = x * cell_width + rng.random_range(..cell_width);
            let yr = y * cell_height + rng.random_range(..cell_height);
            apply_point(xr, yr, true, image, mask, lut);
        }
    }
    cols * rows
}

fn find_void(image: &FloatImage, mask: &FloatImage) -> Point {
    let mut void_point = Point { x: 0, y: 0 };
    let mut min_energy = f64::MAX;
    for y in 0..image.height {
        for x in 0..image.width {
            let energy = mask.get(x, y);
            if image.get(x, y) < 0.5 && energy < min_energy {
                min_energy = energy;
                void_point.x = x;
                void_point.y = y;
            }
        }
    }
    void_point
}

fn find_cluster(image: &FloatImage, mask: &FloatImage) -> Point {
    let mut cluster_point = Point { x: 0, y: 0 };
    let mut max_energy = f64::MIN;
    for y in 0..image.height {
        for x in 0..image.width {
            let energy = mask.get(x, y);
            if image.get(x, y) > 0.5 && energy > max_energy {
                max_energy = energy;
                cluster_point.x = x;
                cluster_point.y = y;
            }
        }
    }
    cluster_point
}

fn generate_bluenoise_points(width: u32, height: u32) -> Vec<Point> {
    let mut res = FloatImage::new(width, height);
    let mut energy_mask = FloatImage::new(width, height);
    let lut = generate_lut(width, height);

    let first_points_count = start_fill(&mut res, &mut energy_mask, &lut, 0.1);
    let points = vec![Point { x: 0, y: 0 }; first_points_count as usize];

    points
}
