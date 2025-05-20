use anyhow::Result;
use crossterm::{cursor, execute, style, terminal};
use rvc_shared::interface::{ProgressBar, Timer, Tui};
use std::path::Path;

pub struct StatusCalculating {
    pbar: ProgressBar,
    pub timer: Timer,
    total_attempts: u32,
    total_steps: u32,
}

impl StatusCalculating {
    pub fn new(tui: &mut Tui, total_attempts: u32, total_steps: u32, total_colors: u32) -> Result<StatusCalculating> {
        execute!(
            tui.out,
            style::SetForegroundColor(crossterm::style::Color::Grey),
            style::Print("Number of colors: "),
            style::SetForegroundColor(crossterm::style::Color::Yellow),
            style::Print(format!("{}\n\n", total_colors)),
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
        //self.pbar.total = adjusted_total;
        //self.pbar.current = progress;
        self.pbar.current = step + attempt * self.total_steps;

        execute!(
            tui.out,
            cursor::MoveUp(9),
            cursor::MoveToColumn(0),
            style::SetForegroundColor(crossterm::style::Color::Grey),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print("       Attempt: "),
            style::SetForegroundColor(crossterm::style::Color::Yellow),
            style::Print(format!("{}/{}\n", attempt + 1, self.total_attempts)),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::SetForegroundColor(crossterm::style::Color::Grey),
            style::Print("          Step: "),
            style::SetForegroundColor(crossterm::style::Color::Yellow),
            style::Print(format!("{}/{}\n\n", step + 1, self.total_steps)),
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
            style::Print("          File: \n"),
            style::Print("          Name: \n\n"),
            style::Print("  Time elapsed: \n"),
            style::Print("Time remaining: \n\n"),
        )?;
        Ok(StatusLoading {
            pbar: ProgressBar::new(total_files, tui.width),
            timer: Timer::new(total_files),
            total_files,
        })
    }

    pub fn update(&mut self, tui: &mut Tui, filename: &Path, progress: u32) -> Result<()> {
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
            cursor::MoveUp(6),
            cursor::MoveToColumn(0),
            style::SetForegroundColor(crossterm::style::Color::Grey),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print("          File: "),
            style::SetForegroundColor(crossterm::style::Color::Yellow),
            style::Print(format!("{}/{}\n", progress + 1, self.total_files)),
            style::SetForegroundColor(crossterm::style::Color::Grey),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print("          Name: "),
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
