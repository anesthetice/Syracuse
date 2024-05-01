mod algorithms;
mod cli;
mod config;
mod data;
mod dirs;
mod error;
mod graph;
mod utils;

use anyhow::Context;
use data::{internal::{Entries, Entry}, syrtime::SyrDate};
use crossterm::style::Stylize;
use directories::ProjectDirs;

fn main() -> anyhow::Result<()> {
    // start of initialization
    let dirs = ProjectDirs::from("", "", "syracuse").context("failed to get project directories")?;
    if !dirs.config_dir().exists() {
        std::fs::create_dir(dirs.config_dir()).context("failed to create a config directory for the application")?
    }
    if !dirs.data_dir().exists() {
        std::fs::create_dir(dirs.data_dir()).context("failed to create a data directory for the application")?
    }

    // this should never fail, unwrapping is fine
    config::CONFIG.set(config::Config::load(&dirs.config_dir().join("syracuse.config"))).unwrap();
    dirs::DIRS.set(dirs).unwrap();
    // end of initialization

    let date: SyrDate = {
        let config = config::Config::get();
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
                warn!("failed to create UtcOffset with the provided local time offset\n{err}");
                time::OffsetDateTime::now_utc().date().into()
            }
        }
    };

    let entries = Entries::load()?;

    let command = cli::cli();
    let matches = command.get_matches();

    if let Some(argmat) = matches.subcommand_matches("add") {
    if let Some(mat) = argmat.get_many::<String>("entry") {
        let mut names: Vec<String> = mat.map(|string| string.to_uppercase()).collect();
        for entry in entries.iter() {
            let filestem = entry.get_filestem();
            for name in names.iter() {
                if filestem.contains(name) {
                    Err(error::Error{})
                        .with_context(|| format!("failed to add new entry, the name and alisases provided conflict with an existing entry or the separator characters, {}", filestem))?
                }
            }
        }
        let entry = Entry::new(names.remove(0), names);
        entry.save_to_file()?;
    }
    }

    if let Some(argmatches) = matches.subcommand_matches("list") {
        for entry in entries.iter() {
            if argmatches.get_flag("full") {
                println!("{:?}\n", entry)
            } else {
                println!("{}\n", entry)
            }
        }
    }
    
    Ok(())
}