use crossterm::{cursor, execute, style::Stylize, terminal::{disable_raw_mode, enable_raw_mode}};
use std::io::stdout;

#[macro_export]
macro_rules! info {
    ($($args:tt)*) => {
        if $crate::config::Config::get().debug {
            eprintln!("[{}]  {}", "INFO".cyan(), format_args!($($args)*))
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($args:tt)*) => {
        eprintln!("[{}]  {}", "WARN".yellow(), format_args!($($args)*))
    };
}

pub fn enter_clean_input_mode() {
    let _ = enable_raw_mode().map_err(|err| warn!("failed to enable raw mode: '{err}'"));
    let _ = execute!(stdout(), cursor::Hide).map_err(|err| warn!("failed to hide cursor: '{err}'"));
}

pub fn exit_clean_input_mode() {
    let _ = execute!(stdout(), cursor::Show).map_err(|err| warn!("failed to show cursor: '{err}'"));
    let _ = disable_raw_mode().map_err(|err| warn!("failed to disable raw mode: '{err}'"));
}
