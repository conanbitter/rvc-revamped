use anyhow::Result;
use clap::Parser;
use colorcalc::{ColorCalc, ColorData};
use image::ImageReader;
use interface::StatusLoading;
use std::path::PathBuf;

use rvc_shared::interface::Tui;

mod colorcalc;
mod interface;

#[derive(Parser, Debug)]
struct Args {
    #[arg(required = true)]
    files: Vec<PathBuf>,
    #[arg(short, long)]
    output: String,
    #[arg(short, long, default_value_t = 256)]
    colors: u32,
    #[arg(short, long, default_value_t = 5)]
    attempts: u32,
    #[arg(short, long, default_value_t = 1000)]
    steps: u32,
}

fn main() -> Result<()> {
    let args = Args::parse_from(wild::args());

    let mut tui = Tui::new()?;
    tui.show_intro()?;

    let mut color_data = ColorData::new();
    let mut loading_status = StatusLoading::new(&mut tui, args.files.len() as u32)?;
    loading_status.timer.start();

    for (progress, filename) in args.files.iter().enumerate() {
        let img = ImageReader::open(filename)?.decode()?.to_rgb8();
        color_data.add(&img);
        if loading_status.timer.needs_update() || progress == 0 || progress == args.files.len() - 1 {
            loading_status.update(&mut tui, filename, progress as u32)?;
        };
    }

    tui.separator()?;

    let mut calculator = ColorCalc::new(args.colors, color_data, &mut tui, args.attempts, args.steps)?;
    let palette = calculator.run(&mut tui)?;
    palette.save(args.output)?;

    Ok(())
}
