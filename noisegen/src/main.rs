use bluenoise::generate_points;
use image::ImageBuffer;

mod bluenoise;
mod pattern;

fn main() {
    println!("Hello, world!");
    let points = generate_points(128, 128);

    let mut img = ImageBuffer::new(128, 128);

    for (i, point) in points.iter().enumerate() {
        let color = (255.0 * i as f64 / points.len() as f64) as u8;
        *img.get_pixel_mut(point.x, point.y) = image::Luma([color]);
    }

    img.save("result.png").unwrap();
}
