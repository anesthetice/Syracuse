mod algorithms;
mod animation;
mod cli;
mod config;
mod data;
mod dirs;
mod error;
mod graph;
mod utils;

use std::time::{Duration, Instant};

use anyhow::Context;
use data::{internal::{Entries, Entry}, syrtime::SyrDate};
use crossterm::{event, style::Stylize};
use directories::ProjectDirs;

use crate::utils::{enter_clean_input_mode, exit_clean_input_mode};

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
    // end of initialization

    /*
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

    if let Some(argmat) = matches.subcommand_matches("remove") {
        if let Some(mat) = argmat.get_one::<String>("entry") {
            let name = mat.to_uppercase();
            if let Some(entry) = entries.choose(name.as_str()) {
                entry.delete()?;
            }
        }
    }

    if let Some(argmat) = matches.subcommand_matches("start") {
        if let Some(mat) = argmat.get_one::<String>("entry") {
            let name = mat.to_uppercase();
            if let Some(mut entry) = entries.choose(name.as_str()) {

                println!();
                let start = Instant::now();
                let mut instant = start;
                let mut autosave_instant = start;
                let autosave_perdiod = Duration::from_secs(config::Config::get().autosave_period as u64);

                let mut stdout = std::io::stdout();
                let mut frame: usize = 0;

                enter_clean_input_mode();
                loop {
                    /* SimpleAnimation::play(
                        &mut stdout,
                        &mut frame,
                        &duration_as_pretty_string(&instant.duration_since(start)),
                    );*/
                    if event::poll(std::time::Duration::from_secs_f64(0.1))? {
                        if let event::Event::Key(key) = event::read()? {
                            if key.kind == event::KeyEventKind::Press
                                && (key.code == event::KeyCode::Char('q') || key.code == event::KeyCode::Enter)
                            {break}
                        }
                    }
                    if instant.duration_since(autosave_instant) > autosave_perdiod {
                        entry.save_to_file().context("failed to save entry progress")?;
                        autosave_instant = instant;
                    }
                    let new_instant = Instant::now();
                    entry.increase_bloc_duration(&date, new_instant.duration_since(instant).as_secs());
                    instant = new_instant;
                }
                exit_clean_input_mode();
                println!();
                entry.save_to_file().context("failed to save entry progress")?;
            }
        }
    }
    */
    animation::test();

    Ok(())
}