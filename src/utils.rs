use crossterm::style::Stylize;

#[macro_export]
macro_rules! info {
    ($($args:tt)*) => {
        if crate::config::Config::get().debug {
            eprintln!("[ {} ] {}", "INFO".cyan(), format_args!($($args)*))
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($args:tt)*) => {
        eprintln!("[ {} ] {}", "WARN".yellow(), format_args!($($args)*))
    };
}