use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use std::io::stdout;
pub fn enter_clean_input_mode() {
    enable_raw_mode().map_err(|err| log::warn!("Failed to enable raw mode: '{err}'"));
    execute!(stdout(), cursor::Hide).map_err(|err| log::warn!("Failed to hide cursor: '{err}'"));
}

pub fn exit_clean_input_mode() {
    execute!(stdout(), cursor::Show).map_err(|err| log::warn!("Failed to show cursor: '{err}'"));
    disable_raw_mode().map_err(|err| log::warn!("Failed to disable raw mode: '{err}'"));
}
