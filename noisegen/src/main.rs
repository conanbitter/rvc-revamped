use anyhow::Result;
use bluenoise::generate_points;
use clap::Parser;
use image::{ImageBuffer, ImageFormat};
use rvc_shared::pattern::Pattern;

mod bluenoise;

#[derive(Parser, Debug)]
#[clap(disable_help_flag = true)]
struct Args {
    #[arg(required = true)]
    output: String,
    #[arg(long)]
    help: bool,
    #[arg(short, long)]
    width: u32,
    #[arg(short, long)]
    height: u32,
    #[arg(short, long)]
    levels: u32,
    #[arg(short, long, required = false)]
    preview: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let points = generate_points(args.width, args.height);

    let divisor = points.len() as f64 / args.levels as f64;
    let mut statistic = vec![0u32; args.levels as usize];
    let mut pattern = Pattern::new(args.width, args.height, args.levels);

    for (i, point) in points.iter().enumerate() {
        let level = (i as f64 / divisor).floor() as u32;
        pattern.set(point.x, point.y, level);
        statistic[level as usize] += 1;
    }

    println!("{:?}", statistic);

    pattern.save(args.output)?;

    if let Some(filename) = args.preview {
        let mut img = ImageBuffer::new(args.width, args.height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let color = (255.0 * pattern.get(x, y) as f64 / args.levels as f64) as u8;
            *pixel = image::Luma([color]);
        }
        img.save_with_format(filename, ImageFormat::Png).unwrap();
    }

    Ok(())
}
