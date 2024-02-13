mod cli;
mod data;
use std::{io::Write, time::{Duration, Instant}};

use data::internal::{Blocs, Entries, Entry};
mod error;
use error::Error;
mod utils;

use crossterm::{event, execute, style::Stylize, terminal::{disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen}};

use crate::utils::user_choice;

const DEFAULT_THRESHOLD: f64 = 0.75;
const LOCAL_OFFSET: [i8; 3] = [1, 0, 0];
const SAVE_TIMER: f64 = 10.0;

fn main() -> anyhow::Result<()> {
    #[cfg(not(debug_assertions))]
    {
        let current_path = std::env::current_dir()
            .map_err(|err| {error!("failed to get current path from which syracuse (binary) is being run"); err})?;
        let bin_filepath = std::env::current_exe()
            .map_err(|err| {error!("failed to get the filepath of syracuse (binary)"); err})?;
        let bin_path = bin_filepath.parent().ok_or(Error::Initialization)
            .map_err(|err| {error!("failed to get the parrent directory of syracuse (binary)"); err})?;
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
            entries.save().map_err(|err| {error!("failed to save syracuse.json"); err})?;
        }
        if !backups_path.exists() {
            std::fs::create_dir(backups_path).map_err(|err| {error!("failed to create the ./backups directory"); err})?;
        }
    }

    let date: time::Date = {
        match time::UtcOffset::from_hms(LOCAL_OFFSET[0], LOCAL_OFFSET[1], LOCAL_OFFSET[2]) {
            Ok(offset) => {
                time::OffsetDateTime::now_utc().replace_offset(offset).date()
            },
            Err(err) => {
                warn!("failed to create UtcOffset with the provided LOCAL_OFFSET\n{err}");
                time::OffsetDateTime::now_utc().date()
            }
        }
    };
    
    let mut entries = Entries::load()
        .map_err(|err| {error!("failed to load entries"); err})?;
    entries.backup()
        .map_err(|err| {error!("failed to backup entries"); err})?;

    let command = cli::cli();
    let matches = command.get_matches();

    if let Some(argmatches) = matches.subcommand_matches("add") {
        if let Some(mat) = argmatches.get_many::<String>("entry") {
            let names: Vec<String> = mat.map(|string| {string.to_uppercase()}).collect();
            let mut valid: bool = true;
            'outer:
            for name in names.iter() {
                for entry in entries.iter() {
                    if entry.is_name(name) {valid=false; break 'outer;}
                }
            }
            if !valid {
                warn!("invalid add subcommand usage, name is already in use");
                Err(Error::InvalidInput)?;
            }
            entries.push(Entry::new(names, Blocs::default()));
            entries.save().map_err(|err| {error!("failed to save entries"); err})?;
            info!("successfully added a new entry");
        }
        else {
            warn!("invalid add subcommand usage, could not find a valid name");
        }
    }

    if let Some(argmatches) = matches.subcommand_matches("list") {
        match argmatches.get_flag("full") {
            true => {
                for entry in entries.iter() {
                    println!("{}\n{}\n", entry, entry.blocs);
                }
                
            },
            false => {
                for entry in entries.iter() {
                    println!("{}\n", entry);
                }
            },
        }
    }

    if let Some(argmatches) = matches.subcommand_matches("remove") {
        if let Some(mat) = argmatches.get_one::<String>("entry") {
            let name = mat.to_uppercase();
            if let Some(entry_to_remove) = user_choice(&entries.search(&name, DEFAULT_THRESHOLD)) {
                let idx_to_remove = entries.iter().position(|entry| {entry == *entry_to_remove}).unwrap();
                let removed_entry = entries.remove(idx_to_remove);
                entries.save().map_err(|err| {error!("failed to save entries"); err})?;
                info!("removed entry: {}", removed_entry);
            }
        }
    }

    if let Some(argmatches) = matches.subcommand_matches("start") {
        if let Some(mat) = argmatches.get_one::<String>("entry") {
            let name = mat.to_uppercase();
            if let Some(entry) = user_choice(&entries.search(&name, DEFAULT_THRESHOLD)) {
                let entry_idx = entries.iter().position(|entry| {entry.names == entry.names}).unwrap();
                let start = Instant::now();
                let mut instant = start;
                let mut save_instant = start;
                let mut stdout = std::io::stdout();
                enable_raw_mode()?;
                loop {
                    print!("\r{:.2}         ", instant.duration_since(start).as_secs_f64());
                    stdout.flush();
                    if event::poll(std::time::Duration::from_secs_f64(0.1)).unwrap() {
                        if let event::Event::Key(key) = event::read()? {
                            if key.kind == event::KeyEventKind::Press {
                                if key.code == event::KeyCode::Char('q') {
                                    break;
                                }
                            }
                        }
                    }
                    let new_instant = Instant::now();
                    entries.get_mut(entry_idx).unwrap().update_bloc(&date, new_instant.duration_since(instant));
                    instant = new_instant;
                    if instant.duration_since(save_instant) > Duration::from_secs_f64(SAVE_TIMER) {
                        entries.save().map_err(|err| {error!("failed to save progress"); err})?;
                        save_instant = new_instant;
                    } 
                }
                let _ = disable_raw_mode();
                entries.save().map_err(|err| {error!("failed to save progress"); err})?;
            }
        }
    }
    Ok(())
}