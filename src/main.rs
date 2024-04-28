
mod algorithms;
mod config;
mod data;
mod error;
mod graph;
mod utils;

use anyhow::Context;
use directories::ProjectDirs;
use crate::data::internal::Entry;

use crate::algorithms::smith_waterman;
fn main() -> anyhow::Result<()> {

    let dirs = ProjectDirs::from("", "", "syracuse").context("failed to get project directories")?;
    if !dirs.config_dir().exists() {
        std::fs::create_dir(dirs.config_dir()).context("failed to create a config directory for the application")?
    }
    if !dirs.data_dir().exists() {
        std::fs::create_dir(dirs.data_dir()).context("failed to create a data directory for the application")?
    }
    
    let config = crate::config::Config::load(&dirs.config_dir().join("syracuse.config"));

    let entry = Entry::new("MATH-201".to_string(), vec!["ANALYSE".to_string(), "HOLOMORPH".to_string()]);

    entry.to_file(dirs.data_dir())?;

    smith_waterman("ANANUM", "ANA");
    Ok(())
}