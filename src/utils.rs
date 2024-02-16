use crossterm::{
    cursor, event, execute,
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::seq::SliceRandom;
use std::io::stdout;

use crate::{config::Config, data::internal::Entries};

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

pub fn user_choice<'a, T>(choices: &'a [&'a T], config: &Config) -> Option<&'a T>
where
    T: std::fmt::Display,
{
    match choices.len() {
        0 => {
            warn!("no choices to select");
            None
        }
        1 => {
            if config.colorful {
                if let Some(color) = config.color_palette.choose(&mut rand::thread_rng()) {
                    println!(
                        "{}\n{}/{} ?",
                        format!("{}", choices[0]).with(color.into()),
                        "Yes".with(config.color_green.into()),
                        "No".with(config.color_red.into())
                    );
                } else {
                    println!(
                        "{}\n{}/{} ?",
                        choices[0],
                        "Yes".with(config.color_green.into()),
                        "No".with(config.color_red.into())
                    );
                }
            } else {
                println!("{}\nYes/No ?", choices[0])
            }
            if let Err(err) = enable_raw_mode() {
                error!("failed to enable raw mode\n{err}");
                return None;
            }
            let _ = execute!(stdout(), cursor::Hide)
                .map_err(|err| warn!("failed to hide cursor\n{err}"));

            loop {
                if event::poll(std::time::Duration::from_secs_f64(0.1)).unwrap() {
                    if let event::Event::Key(key) = event::read().ok()? {
                        if key.kind == event::KeyEventKind::Press {
                            match key.code {
                                event::KeyCode::Esc
                                | event::KeyCode::Char('q')
                                | event::KeyCode::Char('n') => {
                                    let _ = execute!(stdout(), cursor::Show)
                                        .map_err(|err| warn!("failed to show cursor\n{err}"));
                                    let _ = disable_raw_mode()
                                        .map_err(|err| warn!("failed to disable raw mode\n{err}"));
                                    break None;
                                }
                                event::KeyCode::Char('y') | event::KeyCode::Enter => {
                                    let _ = execute!(stdout(), cursor::Show)
                                        .map_err(|err| warn!("failed to show cursor\n{err}"));
                                    let _ = disable_raw_mode()
                                        .map_err(|err| warn!("failed to disable raw mode\n{err}"));
                                    break Some(choices[0]);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        2..=9 => {
            let mut idx: usize = 0;
            while idx < choices.len() {
                if config.colorful {
                    for color in config.color_palette.iter() {
                        if idx >= choices.len() {
                            break;
                        }
                        let string = format!("{idx}: {}", choices[idx]);
                        println!("{}", string.with(color.into()));
                        idx += 1;
                    }
                } else {
                    let string = format!("{idx}: {}", choices[idx]);
                    println!("{}", string);
                    idx += 1;
                }
            }
            if let Err(err) = enable_raw_mode() {
                error!("failed to enable raw mode\n{err}");
                return None;
            }
            let _ = execute!(stdout(), cursor::Show)
                .map_err(|err| warn!("failed to show cursor\n{err}"));
            loop {
                if event::poll(std::time::Duration::from_secs_f64(0.1)).unwrap() {
                    if let event::Event::Key(key) = event::read().ok()? {
                        if key.kind == event::KeyEventKind::Press {
                            match key.code {
                                event::KeyCode::Esc
                                | event::KeyCode::Char('q')
                                | event::KeyCode::Char('n') => {
                                    let _ = execute!(stdout(), cursor::Show)
                                        .map_err(|err| warn!("failed to show cursor\n{err}"));
                                    let _ = disable_raw_mode()
                                        .map_err(|err| warn!("failed to disable raw mode\n{err}"));
                                    break None;
                                }
                                event::KeyCode::Enter => {
                                    let _ = execute!(stdout(), cursor::Show)
                                        .map_err(|err| warn!("failed to show cursor\n{err}"));
                                    let _ = disable_raw_mode()
                                        .map_err(|err| warn!("failed to disable raw mode\n{err}"));
                                    break Some(choices[0]);
                                }
                                event::KeyCode::Char(chr) => {
                                    if chr.is_numeric() {
                                        if let Ok(idx) = chr.to_string().parse::<usize>() {
                                            let _ =
                                                execute!(stdout(), cursor::Show).map_err(|err| {
                                                    warn!("failed to show cursor\n{err}")
                                                });
                                            let _ = disable_raw_mode().map_err(|err| {
                                                warn!("failed to disable raw mode\n{err}")
                                            });
                                            break Some(choices[idx]);
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
        _ => {
            error!("too many choices, increase the threshold");
            None
        }
    }
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
