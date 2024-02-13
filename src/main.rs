mod cli;
mod data;
use data::internal::{Blocs, Entries, Entry};
mod error;
use error::Error;
mod utils;

use crossterm::style::Stylize;

use crate::utils::user_choice;

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
            info!("successfully added a new entry")
        }
        else {
            warn!("invalid add subcommand usage, could not find a valid name")
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
            if let Some(entry_to_remove) = user_choice(&entries.search(&name, 0.50)) {
                let idx_to_remove = entries.iter().position(|entry| {entry == *entry_to_remove}).unwrap();
                let removed_entry = entries.remove(idx_to_remove);
                entries.save().map_err(|err| {error!("failed to save entries"); err})?;
                info!("removed entry: {}", removed_entry);
            }
        }
    }
    Ok(())
}