// Modules
mod cli;
mod config;

// Imports
use crate::data::{IEntries, SyrDate, UEntries};
use clap::ArgMatches;
use color_eyre::eyre::{self, Context, OptionExt};
use config::Config;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

pub struct App {
    ientries: IEntries,
    uentries: UEntries,
    date: SyrDate,
    config: Config,
    dirs: ProjectDirs,
    arg_matches: ArgMatches,
}

impl App {
    pub fn load() -> eyre::Result<Self> {
        let dirs = ProjectDirs::from("", "", "syracuse").ok_or_eyre("Failed to get project directories")?;

        let _conf_dir = dirs.config_dir();
        if !_conf_dir.exists() {
            std::fs::create_dir_all(_conf_dir)
                .wrap_err_with(|| format!("Failed to create the config directory at `{}`", _conf_dir.display()))?;
        }
        let _data_dir = dirs.data_dir();
        if !_data_dir.exists() {
            std::fs::create_dir_all(_data_dir)
                .context(format!("Failed to create the data directory at `{}`", _data_dir.display()))?;
        }

        let config = config::Config::load(&_conf_dir.join("syracuse.conf"));

        let date: SyrDate = {
            let datetime = jiff::Zoned::now().datetime();
            if datetime.time().hour() < config.night_owl_hour_extension {
                datetime.date().yesterday()?.into()
            } else {
                datetime.date().into()
            }
        };

        let ientries = IEntries::load(&_data_dir.join("indexed"))?;
        let uentries = UEntries::load(&_data_dir.join("unindexed.json"))?;

        let arg_matches = unimplemented!();

        Ok(Self {
            ientries,
            uentries,
            date,
            config,
            dirs,
            arg_matches,
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) enum SortOptions {
    NameAscending,
    NameDescending,
    DurationAscending,
    DurationDescending,
}

impl Default for SortOptions {
    fn default() -> Self {
        Self::DurationDescending
    }
}
