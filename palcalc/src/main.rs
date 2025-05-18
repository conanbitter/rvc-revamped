use std::{cmp::min, fmt::Display, io::stdout, thread::sleep, time::Duration};

use crossterm::{
    cursor::{Hide, MoveToColumn, MoveUp, Show},
    execute,
    style::{Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

struct ProgressBar {
    total: u64,
    current: u64,
    totalf: f64,
}

impl ProgressBar {
    const MAX_WIDTH: u16 = 40;

    fn new(size: u64) -> ProgressBar {
        ProgressBar {
            total: size,
            totalf: size as f64,
            current: 0,
        }
    }

    fn step(&mut self, size: u64) {
        self.current = min(self.current + size, self.total - 1);
    }

    fn finish(&mut self) {
        self.current = self.total - 1;
    }
}

impl Display for ProgressBar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (term_width, _) = terminal::size().unwrap(); // TODO convert error
        let width = min(term_width, ProgressBar::MAX_WIDTH);
        let progress = (self.current as f64) / self.totalf;
        let left = ((width as f64) * progress).round() as u16;
        let right = width - left;
        write!(
            f,
            "[{}{}] {:3}%",
            "#".repeat(left as usize),
            "-".repeat(right as usize),
            (progress * 100.0).round()
        )
    }
}

fn main() -> std::io::Result<()> {
    let mut pb = ProgressBar::new(1000);
    let mut progress = 0;
    execute!(
        stdout(),
        Hide,
        SetForegroundColor(crossterm::style::Color::White),
        Print("[RVC REV]\nPalette Calculator\n\n"),
        SetForegroundColor(crossterm::style::Color::Grey),
        Print("* Calculating palette...\n\n"),
        Print("       Attempt: \n"),
        Print("          Step: \n\n"),
        Print("  Points moved: \n"),
        Print("Total distance: \n\n"),
        Print("  Time elapsed: \n"),
        Print("Time remaining: \n\n"),
    )?;
    loop {
        execute!(
            stdout(),
            MoveUp(9),
            MoveToColumn(0),
            SetForegroundColor(crossterm::style::Color::Grey),
            Clear(ClearType::CurrentLine),
            Print("       Attempt: "),
            SetForegroundColor(crossterm::style::Color::Yellow),
            Print("1/5  \n"),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(crossterm::style::Color::Grey),
            Print("          Step: "),
            SetForegroundColor(crossterm::style::Color::Yellow),
            Print(format!("{}/1000    \n\n", progress)),
            SetForegroundColor(crossterm::style::Color::Grey),
            Clear(ClearType::CurrentLine),
            Print("  Points moved: "),
            SetForegroundColor(crossterm::style::Color::Yellow),
            Print("45673    \n"),
            SetForegroundColor(crossterm::style::Color::Grey),
            Clear(ClearType::CurrentLine),
            Print("Total distance: "),
            SetForegroundColor(crossterm::style::Color::Yellow),
            Print("0.6405423\n\n"),
            SetForegroundColor(crossterm::style::Color::Grey),
            Clear(ClearType::CurrentLine),
            Print("  Time elapsed: "),
            SetForegroundColor(crossterm::style::Color::Yellow),
            Print("34:65:76\n"),
            SetForegroundColor(crossterm::style::Color::Grey),
            Clear(ClearType::CurrentLine),
            Print("Time remaining: "),
            SetForegroundColor(crossterm::style::Color::Yellow),
            Print("32:64:42\n\n"),
            SetForegroundColor(crossterm::style::Color::Grey),
            Clear(ClearType::CurrentLine),
            Print(&pb),
            ResetColor
        )?;
        sleep(Duration::from_millis(100));
        progress += 3;
        pb.step(3);
        if progress > 1000 {
            break;
        }
    }
    pb.finish();
    execute!(stdout(), Show, ResetColor)?;
    Ok(())
}
