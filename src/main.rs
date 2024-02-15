use crossterm::{event, style::Stylize, terminal::{disable_raw_mode, enable_raw_mode,}};
use std::{io::Write, time::{Duration, Instant}};

mod animation;
use animation::Animation;
mod cli;
mod config;
use config::Config;
mod data;
use data::{export::Export, internal::{Blocs, Entries, Entry}};
mod error;
use error::Error;
mod utils;
use utils::{clean_backups, user_choice};

use crate::animation::SimpleAnimation;

fn main() {
    let mut entry = Entry::default();
    entry.names.push("test".to_string());
    entry.update_bloc(&time::Date::from_ordinal_date(2023, 363).unwrap(), std::time::Duration::from_secs_f64(3600.0));
    entry.update_bloc(&time::Date::from_ordinal_date(2023, 364).unwrap(), std::time::Duration::from_secs_f64(2400.0));
    entry.generate_png(time::Date::from_ordinal_date(2023, 363).unwrap(), time::Date::from_ordinal_date(2024, 5).unwrap()).unwrap();
}

#[allow(mutable_transmutes)]
fn main2() -> anyhow::Result<()> {
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

    let config = Config::new();
    if config.colorful && config.color_palette.len() == 0 {
        error!("color palette needs at least one color");
        Err(Error::InvalidConfig)?;
    }

    let date: time::Date = {
        match time::UtcOffset::from_hms(config.local_offset[0], config.local_offset[1], config.local_offset[2]) {
            Ok(offset) => {
                time::OffsetDateTime::now_utc().replace_offset(offset).date()
            },
            Err(err) => {
                warn!("failed to create UtcOffset with the provided LOCAL_OFFSET\n{err}");
                time::OffsetDateTime::now_utc().date()
            }
        }
    };
    
    clean_backups(config.backup_expiration_time).unwrap();

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
    }

    if let Some(argmatches) = matches.subcommand_matches("list") {
        if config.colorful {
            let mut color_idx: usize = 0;
            let max_color_idx: usize = config.color_palette.len();
            for entry in entries.iter() {
                if color_idx == max_color_idx {color_idx = 0;}
                let string = {
                    if argmatches.get_flag("full") {
                        format!("{}\n{}\n", entry, entry.blocs).with(config.color_palette[color_idx])
                    }
                    else {
                        format!("{}\n", entry).with(config.color_palette[color_idx])
                    }
                };
                println!("{}", string);
                color_idx += 1;
            }
        }
        else {
            for entry in entries.iter() {
                let string = {
                    if argmatches.get_flag("full") {
                        format!("{}\n{}\n", entry, entry.blocs)
                    }
                    else {
                        format!("{}\n", entry)
                    }
                };
                println!("{}", string);
            }
                
        }
    }

    if let Some(argmatches) = matches.subcommand_matches("remove") {
        if let Some(mat) = argmatches.get_one::<String>("entry") {
            let name = mat.to_uppercase();
            if let Some(entry_to_remove) = user_choice(&entries.search(&name, config.search_threshold), &config) {
                let idx_to_remove = entries.iter().position(|entry| {entry == entry_to_remove}).unwrap();
                let removed_entry = entries.remove(idx_to_remove);
                entries.save().map_err(|err| {error!("failed to save entries"); err})?;
                info!("removed entry: {}", removed_entry);
            }
        }
    }

    if let Some(argmatches) = matches.subcommand_matches("start") {
        if let Some(mat) = argmatches.get_one::<String>("entry") {
            let name = mat.to_uppercase();
            if let Some(entry) = user_choice(&entries.search(&name, config.search_threshold), &config) {
                let entry: &mut Entry = unsafe {std::mem::transmute(entry)};
                println!("");

                let start = Instant::now();
                let mut instant = start;
                let mut save_instant = start;

                let mut stdout = std::io::stdout();
                let mut frame: usize = 0;

                enable_raw_mode()?;
                loop {
                    SimpleAnimation::play(&mut stdout, &mut frame, &instant.duration_since(start).as_secs_f64(), None);
                    let _ = stdout.flush().map_err(|err| {warn!("failed to flush to stdout\n{err}")});
                    if event::poll(std::time::Duration::from_secs_f64(0.1))? {
                        if let event::Event::Key(key) = event::read()? {
                            if key.kind == event::KeyEventKind::Press {
                                if key.code == event::KeyCode::Char('q') {
                                    break;
                                }
                            }
                        }
                    }
                    let new_instant = Instant::now();
                    entry.update_bloc(&date, new_instant.duration_since(instant));
                    instant = new_instant;
                    if instant.duration_since(save_instant) > Duration::from_secs_f64(config.save_period) {
                        entries.save().map_err(|err| {error!("failed to save progress"); err})?;
                        save_instant = new_instant;
                    } 
                }
                let _ = disable_raw_mode();
                entries.save().map_err(|err| {error!("failed to save progress"); err})?;
            }
        }
    }

    if let Some(argmatches) = matches.subcommand_matches("graph") {
        if argmatches.get_flag("all") {

        }
    }

    Ok(())
}