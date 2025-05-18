use anyhow::Result;
use interface::{StatusCalculating, Tui};
use std::{thread::sleep, time::Duration};

mod interface;

fn main() -> Result<()> {
    let mut tui = Tui::new()?;
    tui.show_intro()?;
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
