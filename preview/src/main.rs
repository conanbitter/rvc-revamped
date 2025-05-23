use anyhow::Result;
use clap::Parser;
use image::{ImageBuffer, ImageFormat, ImageReader};
use indexing::{convert_fs, convert_matrix, convert_posterize};
use rvc_shared::{colors::IntColor, palette::Palette, pattern::Pattern, plane::Plane};
use std::{path::PathBuf, time::Instant};

mod indexing;

fn load_image(filename: &PathBuf, image: &mut Plane<IntColor>) -> Result<()> {
    let file = ImageReader::open(filename)?.decode()?.to_rgb8();
    for (file_pixel, img_pixel) in file.pixels().zip(image.data.iter_mut()) {
        img_pixel.r = file_pixel.0[0] as i32;
        img_pixel.g = file_pixel.0[1] as i32;
        img_pixel.b = file_pixel.0[2] as i32;
    }
    Ok(())
}

fn save_image(filename: &PathBuf, image: &Plane<i32>, palette: &Palette) -> Result<()> {
    let mut file = ImageBuffer::new(image.width, image.height);
    for (file_pixel, img_pixel) in file.pixels_mut().zip(image.data.iter()) {
        let c = IntColor::from(palette.get(*img_pixel));
        *file_pixel = image::Rgb([c.r as u8, c.g as u8, c.b as u8]);
    }
    file.save_with_format(filename, ImageFormat::Png)?;
    Ok(())
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(required = true)]
    files: Vec<PathBuf>,
    #[arg(short, long)]
    output: PathBuf,
    #[arg(short, long)]
    palette: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse_from(wild::args());

    let pal = Palette::from_file(args.palette)?;
    let pat = Pattern::from_file(String::from("testdata\\noise480x270x16.ptrn"))?;

    let now = Instant::now();
    for file in args.files {
        println!("{:?}", file);
        let (width, height) = ImageReader::open(&file)?.into_dimensions()?;

        let mut img = Plane::new(width, height, IntColor::BLACK);
        load_image(&file, &mut img)?;
        let mut out = Plane::new(width, height, 0i32);
        convert_matrix(&img, &mut out, &pal, &pat);

        let mut outfile = file.clone();
        outfile.set_extension("png");
        if let Some(name) = outfile.file_name() {
            let outfile = args.output.join(name);
            save_image(&outfile, &out, &pal)?;
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Ok(())
}
