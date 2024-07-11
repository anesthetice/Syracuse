mod algorithms;
mod animation;
mod cli;
mod config;
mod data;
mod dirs;
mod error;
mod utils;

use anyhow::Context;
use crossterm::style::Stylize;
use directories::ProjectDirs;

use cli::{
    process_add_subcommand, process_backup_subcommand, process_graph_subcommand,
    process_list_subcommand, process_prune_subcommand, process_reindex_subcommand,
    process_remove_subcommand, process_start_subcommand, process_sum_subcommand,
    process_today_subcommand, process_unindex_subcommand, process_update_subcommand,
    ProcessOutput as PO,
};
use data::{internal::Entries, syrtime::SyrDate};
fn main() -> anyhow::Result<()> {
    // start of initialization
    let dirs =
        ProjectDirs::from("", "", "syracuse").context("failed to get project directories")?;
    if !dirs.config_dir().exists() {
        std::fs::create_dir(dirs.config_dir())
            .context("failed to create a config directory for the application")?
    }
    if !dirs.data_dir().exists() {
        std::fs::create_dir(dirs.data_dir())
            .context("failed to create a data directory for the application")?
    }

    // this should never fail, unwrapping is fine
    config::CONFIG
        .set(config::Config::load(
            &dirs.config_dir().join("syracuse.conf"),
        ))
        .unwrap();
    dirs::DIRS.set(dirs).unwrap();

    let datetime = {
        let lo = config::Config::get().local_offset;
        match time::UtcOffset::from_hms(lo[0], lo[1], lo[2]) {
            Ok(offset) => time::OffsetDateTime::now_utc().to_offset(offset),
            Err(err) => {
                warn!("failed to create UtcOffset with the provided local time offset: '{err}'");
                time::OffsetDateTime::now_utc()
            }
        }
    };
    let time = datetime.time();
    let date: SyrDate = {
        if time.hour() < config::Config::get().night_owl_hour_extension {
            datetime
                .date()
                .previous_day()
                .unwrap_or(datetime.date())
                .into()
        } else {
            datetime.date().into()
        }
    };

    let entries = Entries::load()?;
    // end of initialization

    let command = cli::cli();
    let arg_matches = command.get_matches();
    let _ = config::VERBOSE
        .set(arg_matches.get_flag("verbose"))
        .map_err(|err| warn!("failed to enable verbose output: '{err}'"));
    println!();

    match process_add_subcommand(&arg_matches, &entries)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    match process_list_subcommand(&arg_matches, &entries)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    match process_remove_subcommand(&arg_matches, &entries)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    match process_start_subcommand(&arg_matches, &entries, &date, &time)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    match process_update_subcommand(&arg_matches, &entries, &date)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    match process_today_subcommand(&arg_matches, &entries, &date)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    match process_backup_subcommand(&arg_matches, &entries, &datetime)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    match process_unindex_subcommand(&arg_matches, &entries)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    match process_reindex_subcommand(&arg_matches, &entries)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    match process_sum_subcommand(&arg_matches, &entries, &date)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    let entries = match process_prune_subcommand(&arg_matches, entries)? {
        PO::Continue(entries) => entries.unwrap(),
        PO::Terminate => return Ok(()),
    };

    match process_graph_subcommand(&arg_matches, entries, &date)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    Ok(())
}
