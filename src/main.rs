mod algorithms;
mod cli;
mod config;
mod data;
mod dirs;
mod error;
mod graph;
mod utils;

use anyhow::Context;
use data::internal::Entries;
use directories::ProjectDirs;

use crate::data::internal::Entry;
use crate::algorithms::*;

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
    crate::config::CONFIG.set(crate::config::Config::load(&dirs.config_dir().join("syracuse.config"))).unwrap();
    crate::dirs::DIRS.set(dirs).unwrap();
    // end of initialization

    let entries = Entries::load()?;
    println!("{}", entries.len());

    Ok(())
}