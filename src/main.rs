use crossterm::{event, style::Stylize};
use std::time::{Duration, Instant};

mod animation;
use animation::{Animation, SimpleAnimation};
mod cli;
mod config;
use config::Config;
mod data;
use data::{
    graph::Graph,
    internal::{Blocs, Entries, Entry, SyrDate},
};
mod error;
use error::Error;
mod utils;
use utils::{clean_backups, expand_date_backwards, parse_date, user_choice};

use crate::utils::{duration_as_pretty_string, enter_clean_input_mode, exit_clean_input_mode};

#[allow(mutable_transmutes)]
fn main() -> anyhow::Result<()> {
    #[cfg(not(debug_assertions))]
    {
        let current_path = std::env::current_dir().map_err(|err| {
            error!("failed to get current path from which syracuse (binary) is being run");
            err
        })?;
        let bin_filepath = std::env::current_exe().map_err(|err| {
            error!("failed to get the filepath of syracuse (binary)");
            err
        })?;
        let bin_path = bin_filepath
            .parent()
            .ok_or(Error::Initialization)
            .map_err(|err| {
                error!("failed to get the parrent directory of syracuse (binary)");
                err
            })?;
        if current_path != bin_path {
            error!("syracuse (binary) must be run from the same path as itself");
            Err(Error::Initialization)?;
        }
    }

    {
        use std::path::PathBuf;
        let data_filepath: PathBuf = PathBuf::from("syracuse.json");
        let backups_path: PathBuf = PathBuf::from("./backups");
        if !data_filepath.exists() {
            warn!("syracuse.json doesn't exist");
            let entries = Entries::default();
            entries.save().map_err(|err| {
                error!("failed to save syracuse.json");
                err
            })?;
        }
        if !backups_path.exists() {
            std::fs::create_dir(backups_path).map_err(|err| {
                error!("failed to create the ./backups directory");
                err
            })?;
        }
    }

    let config = Config::new();

    let date: SyrDate = {
        match time::UtcOffset::from_hms(
            config.local_offset[0],
            config.local_offset[1],
            config.local_offset[2],
        ) {
            Ok(offset) => time::OffsetDateTime::now_utc()
                .replace_offset(offset)
                .date()
                .into(),
            Err(err) => {
                warn!("failed to create UtcOffset with the provided LOCAL_OFFSET\n{err}");
                time::OffsetDateTime::now_utc().date().into()
            }
        }
    };

    clean_backups(config.backup_expiration_time).unwrap();

    let mut entries = Entries::load().map_err(|err| {
        error!("failed to load entries");
        err
    })?;
    entries.clean();
    entries.save().map_err(|err| {
        error!("failed to save entries after running clean");
        err
    })?;
    entries.backup().map_err(|err| {
        error!("failed to backup entries");
        err
    })?;

    let command = cli::cli();
    let matches = command.get_matches();

    if let Some(argmatches) = matches.subcommand_matches("add") {
        if let Some(mat) = argmatches.get_many::<String>("entry") {
            let names: Vec<String> = mat.map(|string| string.to_uppercase()).collect();
            let mut valid: bool = true;
            'outer: for name in names.iter() {
                for entry in entries.iter() {
                    if entry.is_name(name) {
                        valid = false;
                        break 'outer;
                    }
                }
            }
            if !valid {
                warn!("invalid add subcommand usage, name is already in use");
                Err(Error::InvalidInput)?;
            }
            entries.push(Entry::new(names, Blocs::default()));
            entries.save().map_err(|err| {
                error!("failed to save entries");
                err
            })?;
            info!("successfully added a new entry");
        }
    }

    if let Some(argmatches) = matches.subcommand_matches("list") {
        for entry in entries.iter() {
            if argmatches.get_flag("full") {
                println!("{}\n{}\n", entry, entry.blocs)
            } else {
                println!("{}\n", entry)
            }
        }
    }

    if let Some(argmatches) = matches.subcommand_matches("remove") {
        if let Some(mat) = argmatches.get_one::<String>("entry") {
            let name = mat.to_uppercase();
            if let Some(entry_to_remove) =
                user_choice(&entries.search(&name, config.search_threshold))
            {
                let idx_to_remove = entries
                    .iter()
                    .position(|entry| entry == entry_to_remove)
                    .unwrap();
                let removed_entry = entries.remove(idx_to_remove);
                entries.save().map_err(|err| {
                    error!("failed to save entries");
                    err
                })?;
                info!("removed entry: {}", removed_entry);
            }
        }
    }

    if let Some(argmatches) = matches.subcommand_matches("start") {
        if let Some(mat) = argmatches.get_one::<String>("entry") {
            let name = mat.to_uppercase();
            if let Some(entry) = user_choice(&entries.search(&name, config.search_threshold)) {
                let entry: &mut Entry = unsafe { std::mem::transmute(entry) };
                println!();

                let start = Instant::now();
                let mut instant = start;
                let mut save_instant = start;
                let save_duration = Duration::from_secs_f64(config.save_period);

                let mut stdout = std::io::stdout();
                let mut frame: usize = 0;

                enter_clean_input_mode();
                loop {
                    SimpleAnimation::play(
                        &mut stdout,
                        &mut frame,
                        &duration_as_pretty_string(&instant.duration_since(start)),
                    );
                    if event::poll(std::time::Duration::from_secs_f64(0.1))? {
                        if let event::Event::Key(key) = event::read()? {
                            if key.kind == event::KeyEventKind::Press
                                && (key.code == event::KeyCode::Char('q')
                                    || key.code == event::KeyCode::Enter)
                            {
                                break;
                            }
                        }
                    }
                    if instant.duration_since(save_instant) > save_duration {
                        entries.save().map_err(|err| {
                            error!("failed to save progress");
                            err
                        })?;
                        save_instant = instant;
                    }
                    let new_instant = Instant::now();
                    entry.update_bloc_add(&date, new_instant.duration_since(instant));
                    instant = new_instant;
                }
                exit_clean_input_mode();
                println!();
                entries.save().map_err(|err| {
                    error!("failed to save progress");
                    err
                })?;
            }
        }
    }

    if let Some(argmatches) = matches.subcommand_matches("update") {
        let specified_date = match argmatches.get_one::<String>("date") {
            Some(slice) => parse_date(slice).map(SyrDate::new).unwrap_or(date),
            None => date,
        };
        if let Some(mat) = argmatches.get_one::<String>("entry") {
            let name = mat.to_uppercase();
            if let Some(entry) = user_choice(&entries.search(&name, config.search_threshold)) {
                let hour_diff: f64 = match argmatches.get_one::<String>("hour") {
                    Some(val) => val.parse::<f64>().unwrap_or(0.0),
                    None => 0.0,
                };
                let minute_diff: f64 = match argmatches.get_one::<String>("minute") {
                    Some(val) => val.parse::<f64>().unwrap_or(0.0),
                    None => 0.0,
                };
                let second_diff: f64 = match argmatches.get_one::<String>("second") {
                    Some(val) => val.parse::<f64>().unwrap_or(0.0),
                    None => 0.0,
                };
                let total_diff: u64 =
                    (hour_diff * 3600.0 + minute_diff * 60.0 + second_diff) as u64;
                let entry: &mut Entry = unsafe { std::mem::transmute(entry) };
                if argmatches.get_flag("negative") {
                    entry.update_bloc_sub(&specified_date, Duration::from_secs(total_diff))
                } else {
                    entry.update_bloc_add(&specified_date, Duration::from_secs(total_diff))
                }
                entries.save().map_err(|err| {
                    error!("failed to save progress");
                    err
                })?;
            }
        }
    }

    if let Some(argmatches) = matches.subcommand_matches("graph") {
        let end_date = match config.graph_specific_end_date {
            Some(specific_date) => specific_date,
            None => date,
        };

        if argmatches.get_flag("all") {
            entries.generate_png(expand_date_backwards(
                config.graph_num_of_days_back,
                &end_date,
            ))?;
        } else if let Some(mat) = argmatches.get_one::<String>("single") {
            let name = mat.to_uppercase();
            if let Some(entry) = user_choice(&entries.search(&name, config.search_threshold)) {
                entry.generate_png(expand_date_backwards(
                    config.graph_num_of_days_back,
                    &end_date,
                ))?;
            }
        }
    }

    Ok(())
}
