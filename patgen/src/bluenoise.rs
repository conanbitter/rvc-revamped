use rand::Rng;
use rvc_shared::plane::Plane;

const SIGMA: f64 = 1.5;
const DIVISOR: f64 = SIGMA * SIGMA * 2.0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

fn generate_lut(width: u32, height: u32) -> Plane<f64> {
    let mut res = Plane::new(width, height, 0.0);
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

fn apply_point(x: u32, y: u32, add: bool, image: &mut Plane<bool>, mask: &mut Plane<f64>, lut: &Plane<f64>) {
    image.set(x, y, add);
    let sign = if add { 1.0 } else { -1.0 };

    for yp in 0..mask.height {
        let ylut = if yp < y { mask.height + yp - y } else { yp - y };
        for xp in 0..mask.width {
            let xlut = if xp < x { mask.width + xp - x } else { xp - x };
            if xlut >= mask.width || ylut >= mask.height {
                println!("error");
            }
            mask.set(xp, yp, mask.get(xp, yp) + lut.get(xlut, ylut) * sign);
        }
    }
}

fn start_fill(image: &mut Plane<bool>, mask: &mut Plane<f64>, lut: &Plane<f64>, quantity: f64) -> u32 {
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

fn find_void(image: &Plane<bool>, mask: &Plane<f64>) -> Point {
    let mut void_point = Point { x: 0, y: 0 };
    let mut min_energy = f64::MAX;
    for y in 0..image.height {
        for x in 0..image.width {
            let energy = mask.get(x, y);
            if !image.get(x, y) && energy < min_energy {
                min_energy = energy;
                void_point.x = x;
                void_point.y = y;
            }
        }
    }
    void_point
}

fn find_cluster(image: &Plane<bool>, mask: &Plane<f64>) -> Point {
    let mut cluster_point = Point { x: 0, y: 0 };
    let mut max_energy = f64::MIN;
    for y in 0..image.height {
        for x in 0..image.width {
            let energy = mask.get(x, y);
            if image.get(x, y) && energy > max_energy {
                max_energy = energy;
                cluster_point.x = x;
                cluster_point.y = y;
            }
        }
    }
    cluster_point
}

pub fn generate_points(width: u32, height: u32) -> Vec<Point> {
    let mut res = Plane::new(width, height, false);
    let mut energy_mask = Plane::new(width, height, 0.0);
    let lut = generate_lut(width, height);

    let first_points_count = start_fill(&mut res, &mut energy_mask, &lut, 0.1);
    let mut points = vec![Point { x: 0, y: 0 }; first_points_count as usize];

    println!("Step 1");
    loop {
        let cluster = find_cluster(&res, &energy_mask);
        apply_point(cluster.x, cluster.y, false, &mut res, &mut energy_mask, &lut);
        let void = find_void(&res, &energy_mask);
        apply_point(void.x, void.y, true, &mut res, &mut energy_mask, &lut);
        if cluster == void {
            break;
        }
    }

    println!("Step 2");
    let mut step2temp = res.clone();
    let mut step2mask = energy_mask.clone();

    for i in (0..first_points_count as usize).rev() {
        let cluster = find_cluster(&step2temp, &step2mask);
        points[i] = cluster;
        apply_point(cluster.x, cluster.y, false, &mut step2temp, &mut step2mask, &lut);
    }

    println!("Step 3");
    for _ in first_points_count..width * height / 2 {
        let void = find_void(&res, &energy_mask);
        points.push(void);
        apply_point(void.x, void.y, true, &mut res, &mut energy_mask, &lut);
    }

    println!("Step 4");
    let mut negative = Plane::new(width, height, false);
    let mut neg_energy = Plane::new(width, height, 0.0);
    for y in 0..negative.height {
        for x in 0..negative.width {
            if !res.get(x, y) {
                apply_point(x, y, true, &mut negative, &mut neg_energy, &lut);
            }
        }
    }
    for _ in width * height / 2..width * height {
        let cluster = find_cluster(&negative, &neg_energy);
        points.push(cluster);
        apply_point(cluster.x, cluster.y, false, &mut negative, &mut neg_energy, &lut)
    }

    points
}
