use std::{cmp::min, io::stdout, thread::sleep, time::Duration};

use crossterm::{
    cursor::{Hide, MoveToColumn, MoveUp, Show},
    execute,
    style::{Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

fn main() -> std::io::Result<()> {
    let mut progress = 0.0;
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
        let (term_width, _) = terminal::size()?;
        let pb_width = min(term_width, 40);
        let pb_left = ((pb_width as f64) * progress).round() as usize;
        let pb_right = pb_width as usize - pb_left;
        let pb_line = format!(
            "[{}{}] {:3}%",
            "#".repeat(pb_left),
            "-".repeat(pb_right),
            (progress * 100.0).round()
        );
        let steps = (1000.0 * (1.0 - progress)) as i32;
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
            Print(format!("{}/1000    \n\n", steps)),
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
            Print(pb_line),
            ResetColor
        )?;
        sleep(Duration::from_millis(100));
        progress += 0.003;
        if progress > 1.0 {
            break;
        }
    }
    execute!(stdout(), Show, ResetColor)?;
    Ok(())
}
