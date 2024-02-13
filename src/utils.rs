use crossterm::style::Stylize;

#[macro_export]
macro_rules! info {
    ($($args:tt)*) => {
        println!("[ {} ] {}", "INFO".cyan(), format_args!($($args)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($($args:tt)*) => {
        println!("[ {} ] {}", "WARN".yellow(), format_args!($($args)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($args:tt)*) => {
        println!("[{}] {}", "ERROR".red(), format_args!($($args)*));
    };
}

pub fn user_choice<T>(choices: &[T]) -> Option<&T>
where T: std::fmt::Display
{
    for (idx, choice) in choices.iter().enumerate() {
        println!("{idx}) {choice}")
    }
    let mut user_input: String = String::new();
    if let Err(err) = std::io::stdin().read_line(&mut user_input) {
        warn!("failed to read an input from the user\n{}", err);
        return None;
    }

    match user_input.trim().parse::<usize>() {
        Ok(idx) => {
            if idx < choices.len() {Some(&choices[idx])}
            else {warn!("invalid input, out of bounds"); None}
        },
        Err(err) => {
            warn!("invalid input, could not be parsed to usize\n{}", err);
            None
        }
    }
}