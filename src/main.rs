mod algorithms;
mod animation;
mod cli;
mod config;
mod data;
mod dirs;
mod error;
mod utils;

use anyhow::Context;
use data::{graph, internal::Entries, syrtime::SyrDate};
use crossterm::style::Stylize;
use directories::ProjectDirs;
use cli::{
    process_add_subcommand, process_list_subcommand, process_remove_subcommand, process_start_subcommand, process_update_subcommand, ProcessOutput as PO
};

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
    let arg_matches = command.get_matches();

    match process_add_subcommand(&arg_matches, &entries)? {
        PO::Continue => (),
        PO::Terminate => return Ok(()),
    }

    match process_list_subcommand(&arg_matches, &entries)? {
        PO::Continue => (),
        PO::Terminate => return Ok(()),
    }

    match process_remove_subcommand(&arg_matches, &entries)? {
        PO::Continue => (),
        PO::Terminate => return Ok(()),
    }

    match process_start_subcommand(&arg_matches, &entries, &date)? {
        PO::Continue => (),
        PO::Terminate => return Ok(()),
    }

    match process_update_subcommand(&arg_matches, &entries, &date)? {
        PO::Continue => (),
        PO::Terminate => return Ok(()),
    }
    */

    graph::spline_interpolation_testing();

    Ok(())
}