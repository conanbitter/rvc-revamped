use anyhow::Result;
use bluenoise::generate_points;
use clap::{Args, Parser};
use image::{ImageBuffer, ImageFormat};
use rvc_shared::pattern::Pattern;

mod bluenoise;

#[derive(Parser, Debug)]
#[command(disable_help_flag = true)]
struct CliArgs {
    #[arg(required = true)]
    output: String,
    #[arg(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
    #[arg(short, long)]
    width: u32,
    #[arg(short, long)]
    height: u32,
    #[command(flatten)]
    pat_type: PatternType,
    #[arg(short, long, required = false)]
    preview: Option<String>,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct PatternType {
    #[arg(short, long)]
    levels: Option<u32>,
    #[arg(short, long)]
    bayer: bool,
}

fn is_power_of_two(value: u32) -> bool {
    (value != 0) && (value & (value - 1) == 0)
}

const BAYER2: [u32; 4] = [0, 2, 3, 1];

fn main() -> Result<()> {
    let args = CliArgs::parse();

    if args.pat_type.bayer {
        // Bayer pattern

        if args.width != args.height {
            println!("Pattern must be square.");
            return Ok(());
        }

        if args.width < 2 {
            println!("Size must be at least 2");
            return Ok(());
        }

        if !is_power_of_two(args.width) {
            println!("Size must be power of two.");
            return Ok(());
        }

        let size = args.width;
        let order = size.ilog2();
        println!("order {}", order);

        let mut pattern = Pattern::new(size, size, size * size);

        let mut block_size = 2;
        loop {
            let step = size / block_size;

            for by in 0..step {
                for bx in 0..step {
                    for py in 0..2 {
                        for px in 0..2 {
                            let color = BAYER2[(px + 2 * py) as usize];
                            for iy in 0..block_size / 2 {
                                for ix in 0..block_size / 2 {
                                    let fx = bx * block_size + px * block_size / 2 + ix;
                                    let fy = by * block_size + py * block_size / 2 + iy;
                                    let prev_color = pattern.get(fx, fy);
                                    pattern.set(fx, fy, prev_color * 4 + color);
                                }
                            }
                        }
                    }
                }
            }
            block_size *= 2;
            if block_size > size {
                break;
            }
        }

        for y in 0..args.height {
            for x in 0..args.width {
                print!("{:3} ", pattern.get(x, y));
            }
            println!();
        }
    } else if let Some(levels) = args.pat_type.levels {
        // Blue noise pattern
        let points = generate_points(args.width, args.height);

        let divisor = points.len() as f64 / levels as f64;
        let mut statistic = vec![0u32; levels as usize];
        let mut pattern = Pattern::new(args.width, args.height, levels);

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
                let color = (255.0 * pattern.get(x, y) as f64 / levels as f64) as u8;
                *pixel = image::Luma([color]);
            }
            img.save_with_format(filename, ImageFormat::Png).unwrap();
        }
    }

    Ok(())
}
