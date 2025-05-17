use std::io::stdout;

use crossterm::{
    execute,
    style::{Print, ResetColor, SetForegroundColor},
};

fn main() -> std::io::Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(crossterm::style::Color::Yellow),
        Print("Test"),
        ResetColor
    )?;
    Ok(())
}
