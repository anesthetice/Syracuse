// Modules
mod cli;
mod config;
mod interaction;

// Imports
use crate::data::{AnyEntry, EntryCore, IEntries, IEntry, SyrDate, UEntries, UEntry};
use clap::ArgMatches;
use color_eyre::eyre::{self, Context, OptionExt};
use config::Config;
use directories::ProjectDirs;
use itertools::Itertools;
use jiff::civil::DateTime;
use serde::{Deserialize, Serialize};

pub struct App {
    ientries: IEntries,
    uentries: UEntries,
    date: SyrDate,
    datetime: DateTime,
    config: Config,
    dirs: ProjectDirs,
}

impl App {
    #[cfg(not(debug_assertions))]
    const NAME: &'static str = "syracuse";

    #[cfg(debug_assertions)]
    const NAME: &'static str = "syracuse-dev";

    pub fn load() -> eyre::Result<Self> {
        let dirs = ProjectDirs::from("", "", Self::NAME)
            .ok_or_eyre("Failed to get project directories")?;

        let _conf_dir = dirs.config_dir();
        if !_conf_dir.exists() {
            std::fs::create_dir_all(_conf_dir).wrap_err_with(|| {
                format!(
                    "Failed to create the config directory at `{}`",
                    _conf_dir.display()
                )
            })?;
        }
        let _data_dir = dirs.data_dir();
        if !_data_dir.exists() {
            std::fs::create_dir_all(_data_dir.join("indexed")).wrap_err_with(|| {
                format!(
                    "Failed to create the data directory at `{}`",
                    _data_dir.display()
                )
            })?;
        }

        let config = config::Config::load(&_conf_dir.join("syracuse.conf"));

        let datetime = jiff::Zoned::now().datetime();

        let date: SyrDate = if datetime.time().hour() < config.night_owl_hour_extension {
            datetime.date().yesterday()?.into()
        } else {
            datetime.date().into()
        };

        let ientries: IEntries = IEntries::load_from_path(_data_dir)?;

        let uentries: UEntries = match UEntries::load_from_default_file(_data_dir) {
            Ok(uentries) => uentries,
            Err(err) => {
                if let Some(io_error) = err.downcast_ref::<std::io::Error>()
                    && io_error.kind() == std::io::ErrorKind::NotFound
                {
                    UEntries::new_empty()
                } else {
                    return Err(err);
                }
            }
        };

        Ok(Self {
            ientries,
            uentries,
            datetime,
            date,
            config,
            dirs,
        })
    }

    fn get_anyentries<'a>(&'a self) -> Vec<AnyEntry<'a>> {
        self.ientries
            .iter()
            .map(AnyEntry::from)
            .chain(self.uentries.iter().map(AnyEntry::from))
            .collect_vec()
    }

    pub fn run(self) -> eyre::Result<()> {
        let arg_matches = cli::build_cli().get_matches();

        match arg_matches.subcommand() {
            Some(("add", arg_matches)) => self.process_add(arg_matches),
            Some(("backup", arg_matches)) => self.process_backup(arg_matches, &self.datetime),
            Some(("check-in", arg_matches)) => self.process_check_in(arg_matches),
            Some(("check-out", arg_matches)) => self.process_check_out(arg_matches, &self.date),
            Some(("list", arg_matches)) => self.process_list(arg_matches),
            /*
            Some(("remove", arg_matches)) => remove::process(arg_matches, &entries),
            Some(("start", arg_matches)) => start::process(arg_matches, &entries, &today),
            Some(("update-add", arg_matches)) => update_add::process(arg_matches, &entries, &today),
            Some(("update-sub", arg_matches)) => update_sub::process(arg_matches, &entries, &today),
            Some(("today", arg_matches)) => today::process(arg_matches, &entries, &today),
            Some(("unindex", arg_matches)) => unindex::process(arg_matches, &entries),
            Some(("reindex", arg_matches)) => reindex::process(arg_matches, &entries),
            Some(("sum", arg_matches)) => sum::process(arg_matches, &entries, &today),
            Some(("prune", arg_matches)) => prune::process(arg_matches, entries),
            Some(("graph", arg_matches)) => graph::process(arg_matches, entries, &today),

            Some(("week", arg_matches)) => week::process(arg_matches, &entries, &today),
            Some(("gen-completions", arg_matches)) => gen_completions::process(arg_matches),
            */
            _ => Ok(()),
        }
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
