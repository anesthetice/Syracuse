// Imports
use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::stdout;

pub static ARROW: &str = "━━⮞";

pub fn enter_clean_input_mode() {
    let _ = enable_raw_mode().map_err(|err| eprintln!("Warning, Failed to enable raw mode: '{err}'"));
    let _ = execute!(stdout(), cursor::Hide).map_err(|err| eprintln!("Warning, Failed to hide cursor: '{err}'"));
}

pub fn exit_clean_input_mode() {
    let _ = execute!(stdout(), cursor::Show).map_err(|err| eprintln!("Warning: Failed to show cursor: '{err}'"));
    let _ = disable_raw_mode().map_err(|err| eprintln!("Warning: Failed to disable raw mode: '{err}'"));
}
