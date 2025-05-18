use anyhow::Result;
use crossterm::{
    cursor, execute,
    style::{self, Color},
    terminal,
};
use std::{
    cmp::min,
    fmt::Display,
    io::{Stdout, stdout},
    path::PathBuf,
    time::{Duration, Instant},
};

pub struct Tui {
    out: Stdout,
    pub width: u16,
}

impl Tui {
    pub fn new() -> Result<Tui> {
        let width = terminal::size()?.0;
        Ok(Tui { out: stdout(), width })
    }

    pub fn show_intro(&mut self) -> Result<()> {
        execute!(
            self.out,
            cursor::Hide,
            style::SetForegroundColor(Color::White),
            style::Print("[RVC rev.]\nPalette Calculator\n\n"),
        )?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        execute!(self.out, cursor::Show, style::ResetColor).unwrap();
    }
}

pub struct ProgressBar {
    total: u32,
    current: u32,
    width: u16,
}

impl ProgressBar {
    const MAX_WIDTH: u16 = 48;

    fn new(size: u32, width: u16) -> ProgressBar {
        ProgressBar {
            total: size,
            current: 0,
            width: min(width, ProgressBar::MAX_WIDTH) - 8,
        }
    }

    fn step(&mut self, size: u32) {
        self.current = min(self.current + size, self.total - 1);
    }

    fn finish(&mut self) {
        self.current = self.total - 1;
    }
}

impl Display for ProgressBar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let progress = (self.current as f64) / self.total as f64;
        let left = ((self.width as f64) * progress).round() as u16;
        let right = self.width - left;
        write!(
            f,
            "[{}{}] {:3}%",
            "#".repeat(left as usize),
            "-".repeat(right as usize),
            (progress * 100.0).round()
        )
    }
}

pub struct Timer {
    step: u32,
    total: u32,
    start: Instant,
    last_update: Instant,
}

fn duration_format(duration: Duration) -> String {
    let hours = duration.as_secs() / 60 / 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let seconds = duration.as_secs() % 60;
    if hours > 0 {
        format!("{:>2}h {:0>2}m {:0>2}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{:0>2}m {:0>2}s", minutes, seconds)
    } else {
        format!("{:0>2}s", seconds)
    }
}

impl Timer {
    pub fn new(total: u32) -> Timer {
        Timer {
            step: 0,
            total,
            start: Instant::now(),
            last_update: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        self.start = Instant::now();
        self.last_update = Instant::now();
    }

    pub fn needs_update(&self) -> bool {
        self.last_update.elapsed() > Duration::from_millis(500)
    }

    pub fn update(&mut self, step: u32) {
        self.step = step;
        self.last_update = Instant::now();
    }

    pub fn get_elapsed(&self) -> String {
        duration_format(self.start.elapsed())
    }

    pub fn get_remaining(&self) -> String {
        if self.step == 0 {
            return duration_format(Duration::ZERO);
        }
        let t = self.total as f64;
        let s = self.step as f64;
        let rem_time = (t - s) / s;
        let remaining = self.start.elapsed().mul_f64(rem_time);
        duration_format(remaining)
    }
}

pub struct StatusCalculating {
    pbar: ProgressBar,
    pub timer: Timer,
    total_attempts: u32,
    total_steps: u32,
}

impl StatusCalculating {
    pub fn new(tui: &mut Tui, total_attempts: u32, total_steps: u32) -> Result<StatusCalculating> {
        execute!(
            tui.out,
            style::SetForegroundColor(crossterm::style::Color::Grey),
            style::Print("* Calculating palette...\n\n"),
            style::Print("       Attempt: \n"),
            style::Print("          Step: \n\n"),
            style::Print("  Points moved: \n"),
            style::Print("Total distance: \n\n"),
            style::Print("  Time elapsed: \n"),
            style::Print("Time remaining: \n\n"),
        )?;
        Ok(StatusCalculating {
            pbar: ProgressBar::new(total_attempts * total_steps, tui.width),
            timer: Timer::new(total_attempts * total_steps),
            total_attempts,
            total_steps,
        })
    }

    pub fn update(
        &mut self,
        tui: &mut Tui,
        attempt: u32,
        step: u32,
        moved: u64,
        distance: f64,
        progress: u32,
        adjusted_total: u32,
    ) -> Result<()> {
        self.timer.total = adjusted_total;
        self.timer.update(progress);
        self.pbar.total = adjusted_total;
        self.pbar.current = progress;

        execute!(
            tui.out,
            cursor::MoveUp(9),
            cursor::MoveToColumn(0),
            style::SetForegroundColor(crossterm::style::Color::Grey),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print("       Attempt: "),
            style::SetForegroundColor(crossterm::style::Color::Yellow),
            style::Print(format!("{}/{}\n", attempt, self.total_attempts)),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::SetForegroundColor(crossterm::style::Color::Grey),
            style::Print("          Step: "),
            style::SetForegroundColor(crossterm::style::Color::Yellow),
            style::Print(format!("{}/{}\n\n", step, self.total_steps)),
            style::SetForegroundColor(crossterm::style::Color::Grey),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print("  Points moved: "),
            style::SetForegroundColor(crossterm::style::Color::Yellow),
            style::Print(format!("{}\n", moved)),
            style::SetForegroundColor(crossterm::style::Color::Grey),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print("Total distance: "),
            style::SetForegroundColor(crossterm::style::Color::Yellow),
            style::Print(format!("{:.4}\n\n", distance)),
            style::SetForegroundColor(crossterm::style::Color::Grey),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print("  Time elapsed: "),
            style::SetForegroundColor(crossterm::style::Color::Yellow),
            style::Print(self.timer.get_elapsed()),
            style::Print("\n"),
            style::SetForegroundColor(crossterm::style::Color::Grey),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print("Time remaining: "),
            style::SetForegroundColor(crossterm::style::Color::Yellow),
            style::Print(self.timer.get_remaining()),
            style::Print("\n\n"),
            style::SetForegroundColor(crossterm::style::Color::Grey),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print(&self.pbar),
        )?;
        Ok(())
    }
}

pub struct StatusLoading {
    pbar: ProgressBar,
    pub timer: Timer,
    total_files: u32,
}

impl StatusLoading {
    pub fn new(tui: &mut Tui, total_files: u32) -> Result<StatusLoading> {
        execute!(
            tui.out,
            style::SetForegroundColor(crossterm::style::Color::Grey),
            style::Print("* Loading images...\n\n"),
            style::Print("          File: \n\n"),
            style::Print("  Time elapsed: \n"),
            style::Print("Time remaining: \n\n"),
        )?;
        Ok(StatusLoading {
            pbar: ProgressBar::new(total_files, tui.width),
            timer: Timer::new(total_files),
            total_files,
        })
    }

    pub fn update(&mut self, tui: &mut Tui, filename: PathBuf, progress: u32) -> Result<()> {
        self.timer.update(progress);
        self.pbar.current = progress;

        let name = match filename.file_name() {
            Some(flnm) => flnm.to_str().unwrap_or(""),
            None => "",
        };
        let name = if name.len() as u16 + 17 > tui.width {
            &name[..(tui.width - 18) as usize]
        } else {
            name
        };

        execute!(
            tui.out,
            cursor::MoveUp(5),
            cursor::MoveToColumn(0),
            style::SetForegroundColor(crossterm::style::Color::Grey),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print("          File: "),
            style::SetForegroundColor(crossterm::style::Color::Yellow),
            style::Print(name),
            style::Print("\n\n"),
            style::SetForegroundColor(crossterm::style::Color::Grey),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print("  Time elapsed: "),
            style::SetForegroundColor(crossterm::style::Color::Yellow),
            style::Print(self.timer.get_elapsed()),
            style::Print("\n"),
            style::SetForegroundColor(crossterm::style::Color::Grey),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print("Time remaining: "),
            style::SetForegroundColor(crossterm::style::Color::Yellow),
            style::Print(self.timer.get_remaining()),
            style::Print("\n\n"),
            style::SetForegroundColor(crossterm::style::Color::Grey),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print(&self.pbar),
        )?;
        Ok(())
    }
}
