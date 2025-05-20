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
    #[command(flatten)]
    bayer_params: Option<BayerParams>,
    #[command(flatten)]
    noise_params: Option<NoiseParams>,
    #[arg(short, long, required = false)]
    preview: Option<String>,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = true, conflicts_with = "bayer")]
struct NoiseParams {
    #[arg(short, long, required = false)]
    width: u32,
    #[arg(short, long, required = false)]
    height: u32,
    #[arg(short, long, required = false)]
    levels: u32,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = true)]
struct BayerParams {
    #[arg(short, long, required = false)]
    bayer: u32,
}

fn is_power_of_two(value: u32) -> bool {
    (value != 0) && (value & (value - 1) == 0)
}

const BAYER2: [u32; 4] = [0, 2, 3, 1];

fn save_preview(pattern: &Pattern, filename: &String) {
    let mut img = ImageBuffer::new(pattern.width, pattern.height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let color = (256.0 * pattern.get(x, y) as f64 / (pattern.levels - 1) as f64).min(255.0) as u8;
        *pixel = image::Luma([color]);
    }
    img.save_with_format(filename, ImageFormat::Png).unwrap();
}

fn main() -> Result<()> {
    let args = CliArgs::parse();
    if let Some(BayerParams { bayer: size }) = args.bayer_params {
        // Bayer pattern

        if size < 2 {
            println!("Size must be at least 2");
            return Ok(());
        }

        if !is_power_of_two(size) {
            println!("Size must be power of two.");
            return Ok(());
        }

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

        pattern.save(args.output)?;

        if let Some(filename) = args.preview {
            save_preview(&pattern, &filename);
        }
    } else if let Some(params) = args.noise_params {
        // Blue noise pattern
        let points = generate_points(params.width, params.height);

        let divisor = points.len() as f64 / params.levels as f64;
        let mut statistic = vec![0u32; params.levels as usize];
        let mut pattern = Pattern::new(params.width, params.height, params.levels);

        for (i, point) in points.iter().enumerate() {
            let level = (i as f64 / divisor).floor() as u32;
            pattern.set(point.x, point.y, level);
            statistic[level as usize] += 1;
        }

        pattern.save(args.output)?;

        if let Some(filename) = args.preview {
            save_preview(&pattern, &filename);
        }
    }
    Ok(())
}
