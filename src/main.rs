
mod algorithms;
mod config;
mod data;
mod error;
mod graph;
mod utils;

use anyhow::Context;
use config::CONFIG;
use directories::ProjectDirs;
use crate::config::Config;
use crate::data::internal::Entry;

use crate::algorithms::*;

fn main() -> anyhow::Result<()> {
    let dirs = ProjectDirs::from("", "", "syracuse").context("failed to get project directories")?;
    if !dirs.config_dir().exists() {
        std::fs::create_dir(dirs.config_dir()).context("failed to create a config directory for the application")?
    }
    if !dirs.data_dir().exists() {
        std::fs::create_dir(dirs.data_dir()).context("failed to create a data directory for the application")?
    }
    
    // this should never fail, unwrapping is fine
    CONFIG.set(crate::config::Config::load(&dirs.config_dir().join("syracuse.config"))).unwrap();

    let entry = Entry::new("MATH-201".to_string(), vec!["ANALYSE".to_string(), "HOLOMORPH".to_string()]);

    entry.to_file(dirs.data_dir())?;

    dbg!(needleman_wunsch("ANANUMSDHSODFOSJDP", "VA"));
    dbg!(needleman_wunsch("ANUM", "ANANUM"));

    dbg!(smith_waterman("ANANUM", "A"));
    dbg!(smith_waterman("ANANUM", "NANA"));
    

    Ok(())
}