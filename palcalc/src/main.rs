use anyhow::Result;
use clap::Parser;
use colorcalc::ColorData;
use image::ImageReader;
use interface::{StatusCalculating, StatusLoading, Tui};
use std::{path::PathBuf, thread::sleep, time::Duration};

mod colorcalc;
mod interface;

#[derive(Parser, Debug)]
struct Args {
    #[arg(required = true)]
    files: Vec<PathBuf>,
    #[arg(short, long)]
    colors: u32,
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
        loading_status.update(&mut tui, filename, progress as u32)?;
        //sleep(Duration::from_millis(500));
    }

    tui.separator()?;

    let mut status = StatusCalculating::new(&mut tui, 5, 1000)?;
    let mut progress = 0;
    status.timer.start();
    status.update(&mut tui, 1, progress, 34, 0.5, progress, 1000)?;
    loop {
        if status.timer.needs_update() {
            status.update(&mut tui, 1, progress, 34, 0.5, progress, 1000)?;
        }
        sleep(Duration::from_millis(100));
        progress += 3;
        if progress > 1000 {
            break;
        }
    }
    Ok(())
}
