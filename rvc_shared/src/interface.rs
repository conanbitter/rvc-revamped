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
    time::{Duration, Instant},
};

pub struct Tui {
    pub out: Stdout,
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

    pub fn separator(&mut self) -> Result<()> {
        execute!(self.out, style::Print("\n\n"),)?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        execute!(self.out, cursor::Show, style::ResetColor).unwrap();
    }
}

pub struct ProgressBar {
    pub total: u32,
    pub current: u32,
    width: u16,
}

impl ProgressBar {
    const MAX_WIDTH: u16 = 48;

    pub fn new(size: u32, width: u16) -> ProgressBar {
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
        let progress = (((self.current + 1) as f64) / self.total as f64).clamp(0.0, 1.0);
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
    pub step: u32,
    pub total: u32,
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
    const MIN_DURATION: Duration = Duration::from_millis(500);

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
        self.last_update.elapsed() > Timer::MIN_DURATION
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
