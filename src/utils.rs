use crossterm::{
    cursor, event, execute,
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{stdin, stdout, Read, Write};

use crate::data::internal::Entries;

#[macro_export]
macro_rules! info {
    ($($args:tt)*) => {
        eprintln!("[ {} ] {}", "INFO".cyan(), format_args!($($args)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($($args:tt)*) => {
        eprintln!("[ {} ] {}", "WARN".yellow(), format_args!($($args)*))
    };
}

#[macro_export]
macro_rules! error {
    ($($args:tt)*) => {
        eprintln!("[{}] {}", "ERROR".red(), format_args!($($args)*))
    };
}

pub fn user_choice<'a, T>(choices: &'a [&'a T]) -> Option<&'a T>
where
    T: std::fmt::Display,
{
    match choices.len() {
        0 => {
            warn!("no choices to select");
            None
        }
        1 => user_choice_single(choices),
        2..=9 => user_choice_multiple(choices),
        _ => {
            warn!("too many choices, defaulting to a less pretty mode");
            user_choice_multiple_expanded(choices)
        }
    }
}

fn user_choice_single<'a, T>(choices: &'a [&'a T]) -> Option<&'a T>
where
    T: std::fmt::Display,
{
    println!("{}\nYes/No ?", choices[0]);
    enter_clean_input_mode();
    loop {
        if event::poll(std::time::Duration::from_secs_f64(0.1)).unwrap() {
            if let event::Event::Key(key) = event::read().ok()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Esc
                        | event::KeyCode::Char('q')
                        | event::KeyCode::Char('n') => {
                            exit_clean_input_mode();
                            break None;
                        }
                        event::KeyCode::Char('y') | event::KeyCode::Enter => {
                            exit_clean_input_mode();
                            break Some(choices[0]);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

pub fn user_choice_multiple<'a, T>(choices: &'a [&'a T]) -> Option<&'a T>
where
    T: std::fmt::Display,
{
    for (idx, choice) in choices.iter().enumerate() {
        println!("{}: {}", idx + 1, choice);
    }
    enter_clean_input_mode();

    loop {
        if event::poll(std::time::Duration::from_secs_f64(0.1)).unwrap() {
            if let event::Event::Key(key) = event::read().ok()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Esc
                        | event::KeyCode::Char('q')
                        | event::KeyCode::Char('n') => {
                            exit_clean_input_mode();
                            break None;
                        }
                        event::KeyCode::Enter => {
                            exit_clean_input_mode();
                            break Some(choices[0]);
                        }
                        event::KeyCode::Char(chr) => {
                            if chr.is_numeric() {
                                if let Ok(idx) = chr.to_string().parse::<usize>() {
                                    exit_clean_input_mode();
                                    break Some(choices[idx - 1]);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

pub fn user_choice_multiple_expanded<'a, T>(choices: &'a [&'a T]) -> Option<&'a T>
where
    T: std::fmt::Display,
{
    for (idx, choice) in choices.iter().enumerate() {
        println!("{}: {}", idx + 1, choice);
    }
    let mut input: String = String::new();
    print!("> ");
    let _ = stdout().flush();
    stdin().read_to_string(&mut input).ok()?;
    choices
        .get(input.trim().parse::<usize>().ok()?.checked_sub(1)?)
        .copied()
}

pub fn enter_clean_input_mode() {
    let _ = enable_raw_mode().map_err(|err| warn!("failed to enable raw mode\n{err}"));
    let _ = execute!(stdout(), cursor::Hide).map_err(|err| warn!("failed to hide cursor\n{err}"));
}

pub fn exit_clean_input_mode() {
    let _ = execute!(stdout(), cursor::Show).map_err(|err| warn!("failed to show cursor\n{err}"));
    let _ = disable_raw_mode().map_err(|err| warn!("failed to disable raw mode\n{err}"));
}

/// older_than: seconds
pub fn clean_backups(older_than: u64) -> anyhow::Result<()> {
    let max_valid_timestamp =
        (time::OffsetDateTime::now_utc().unix_timestamp() as u64) - older_than;

    for entry in std::fs::read_dir(Entries::BACKUPS_PATH)?.flatten() {
        let entry = entry.path();
        if entry.extension().is_none() {
            continue;
        }
        let file_stem = {
            if let Some(pre_stem) = entry.file_stem() {
                if let Some(stem) = pre_stem.to_str() {
                    stem
                } else {
                    continue;
                }
            } else {
                continue;
            }
        };
        if let Ok(timestamp) = file_stem.replace("syracuse-backup-", "").parse::<u64>() {
            if timestamp < max_valid_timestamp {
                let _ = std::fs::remove_file(entry)
                    .map_err(|err| warn!("failed to remove backup file\n{err}"));
            }
        }
    }

    Ok(())
}

pub fn expand_date_backwards(
    mut number_of_days_back: u16,
    end_date: &time::Date,
) -> Vec<time::Date> {
    let mut curr_date: time::Date = *end_date;
    let mut dates: Vec<time::Date> = vec![*end_date];
    while let Some(date) = curr_date.previous_day() {
        if number_of_days_back == 0 {
            break;
        }
        dates.push(date);
        curr_date = date;
        number_of_days_back -= 1;
    }
    dates.reverse();
    dates
}

pub fn parse_date(input: &str) -> Option<time::Date> {
    let input: Vec<&str> = input.split('/').collect();
    if input.len() != 3 {
        return None;
    }
    time::Date::from_calendar_date(
        input[2].parse::<i32>().ok()?,
        time::Month::try_from(input[1].parse::<u8>().ok()?).ok()?,
        input[0].parse::<u8>().ok()?,
    )
    .ok()
}
